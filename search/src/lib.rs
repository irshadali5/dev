use tantivy::{doc, Index, schema::*, TantivyDocument};
use tantivy::query::QueryParser;
use tantivy::collector::TopDocs;
use tantivy::snippet::SnippetGenerator;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Tantivy error: {0}")]
    Tantivy(#[from] tantivy::TantivyError),
    #[error("Query parsing error: {0}")]
    Query(#[from] tantivy::query::QueryParserError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SearchResult {
    pub title: String,
    pub slug: String,
    pub doc_type: String,
    pub snippet: String,
}

#[derive(Clone)]
pub struct SearchService {
    index: Index,
    _schema: Schema,
    title_field: Field,
    body_field: Field,
    tags_field: Field,
    slug_field: Field,
    type_field: Field,
}

impl SearchService {
    pub fn new() -> Result<Self, SearchError> {
        let mut schema_builder = Schema::builder();
        
        let text_options = TextOptions::default().set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer("en_stem")
                .set_index_option(IndexRecordOption::WithFreqsAndPositions),
        ).set_stored();
        
        let title_field = schema_builder.add_text_field("title", text_options.clone());
        let body_field = schema_builder.add_text_field("body", text_options.clone());
        let tags_field = schema_builder.add_text_field("tags", text_options.clone());
        let slug_field = schema_builder.add_text_field("slug", STRING | STORED);
        let type_field = schema_builder.add_text_field("doc_type", STRING | STORED);
        
        let schema = schema_builder.build();
        
        let index_dir = Path::new("content/.search_index");
        let index = if index_dir.exists() {
            Index::open_in_dir(index_dir).unwrap_or_else(|_| {
                let _ = fs::remove_dir_all(index_dir);
                fs::create_dir_all(index_dir).unwrap();
                Index::create_in_dir(index_dir, schema.clone()).unwrap()
            })
        } else {
            let _ = fs::create_dir_all(index_dir);
            Index::create_in_dir(index_dir, schema.clone()).unwrap_or_else(|_| Index::create_in_ram(schema.clone()))
        };
        
        Ok(Self {
            index,
            _schema: schema,
            title_field,
            body_field,
            tags_field,
            slug_field,
            type_field,
        })
    }

    pub async fn index_articles(&self, conn: database::DatabaseConnection) -> Result<(), SearchError> {
        self.index_all(conn).await
    }

    pub async fn index_all(&self, conn: database::DatabaseConnection) -> Result<(), SearchError> {
        let mut index_writer = self.index.writer(50_000_000)?;
        index_writer.delete_all_documents()?;
        
        use database::repositories::{ProjectRepository, BookRepository, PortfolioRepository};
        
        // 1. Index Articles (Database + content/articles/)
        let article_service = articles::service::ArticleService::new(conn.clone());
        if let Ok(articles) = article_service.get_all_articles().await {
            for article in articles {
                let mut doc = TantivyDocument::default();
                doc.add_text(self.title_field, &article.title);
                doc.add_text(self.body_field, &article.body);
                doc.add_text(self.tags_field, &article.tags.join(" "));
                doc.add_text(self.slug_field, format!("articles/{}", article.slug));
                doc.add_text(self.type_field, "Article");
                index_writer.add_document(doc)?;
            }
        }
        
        // 2. Index Projects
        let project_repo = ProjectRepository::new(conn.clone());
        if let Ok(projects) = project_repo.get_all_projects().await {
            for p in projects {
                let mut doc = TantivyDocument::default();
                doc.add_text(self.title_field, &p.title);
                let body = format!("{} {}", p.description, p.content_html);
                doc.add_text(self.body_field, &body);
                doc.add_text(self.slug_field, "projects");
                doc.add_text(self.type_field, "Project");
                index_writer.add_document(doc)?;
            }
        }
        
        // 3. Index Books & Chapters
        let book_repo = BookRepository::new(conn.clone());
        if let Ok(books) = book_repo.get_all_books().await {
            for b in books {
                let mut doc = TantivyDocument::default();
                doc.add_text(self.title_field, &b.title);
                doc.add_text(self.body_field, b.description.unwrap_or_default());
                doc.add_text(self.slug_field, format!("books/{}", b.slug));
                doc.add_text(self.type_field, "Book");
                index_writer.add_document(doc)?;
            }
        }
        
        // 4. Index local book chapters from content/books/
        if let Ok(entries) = fs::read_dir("content/books") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let book_slug = path.file_name().and_then(|s| s.to_str()).unwrap_or_default();
                    if let Ok(ch_entries) = fs::read_dir(&path) {
                        for ch in ch_entries.flatten() {
                            let ch_path = ch.path();
                            if ch_path.extension().and_then(|s| s.to_str()) == Some("md") {
                                if let Ok(md_content) = fs::read_to_string(&ch_path) {
                                    let filename = ch_path.file_name().and_then(|s| s.to_str()).unwrap_or_default();
                                    let title = filename.replace(".md", "").replace("-", " ");
                                    
                                    let mut doc = TantivyDocument::default();
                                    doc.add_text(self.title_field, &title);
                                    doc.add_text(self.body_field, &md_content);
                                    doc.add_text(self.slug_field, format!("books/{}/{}", book_slug, filename));
                                    doc.add_text(self.type_field, "Book Chapter");
                                    index_writer.add_document(doc)?;
                                }
                            }
                        }
                    }
                }
            }
        }

        // 5. Index Skills & Experience
        let portfolio_repo = PortfolioRepository::new(conn);
        if let Ok(skills) = portfolio_repo.get_skills().await {
            for s in skills {
                let mut doc = TantivyDocument::default();
                doc.add_text(self.title_field, &s.name);
                doc.add_text(self.body_field, format!("Technical Skill in {}", s.category));
                doc.add_text(self.slug_field, "resume");
                doc.add_text(self.type_field, "Skill");
                index_writer.add_document(doc)?;
            }
        }

        index_writer.commit()?;
        Ok(())
    }

    pub fn search(&self, query_str: &str) -> Result<Vec<SearchResult>, SearchError> {
        let reader = self.index
            .reader_builder()
            .try_into()?;
            
        let searcher = reader.searcher();
        
        let query_parser = QueryParser::for_index(&self.index, vec![
            self.title_field,
            self.body_field,
            self.tags_field
        ]);
        
        let query = query_parser.parse_query(query_str)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(15))?;
        
        let mut snippet_generator = SnippetGenerator::create(&searcher, &*query, self.body_field)?;
        snippet_generator.set_max_num_chars(160);
        
        let mut results = Vec::new();
        for (_score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
            
            let title = retrieved_doc.get_first(self.title_field)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
                
            let slug = retrieved_doc.get_first(self.slug_field)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
                
            let doc_type = retrieved_doc.get_first(self.type_field)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
                
            let snippet_html = snippet_generator.snippet_from_doc(&retrieved_doc).to_html();
            let snippet = if snippet_html.trim().is_empty() {
                title.clone()
            } else {
                snippet_html
            };
                
            results.push(SearchResult {
                title,
                slug,
                doc_type,
                snippet,
            });
        }
        
        Ok(results)
    }
}
