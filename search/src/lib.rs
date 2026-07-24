use tantivy::{doc, Index, IndexWriter, schema::*, TantivyDocument};
use tantivy::query::QueryParser;
use tantivy::collector::TopDocs;
use std::path::PathBuf;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use articles::service::ArticleService;

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Tantivy error: {0}")]
    Tantivy(#[from] tantivy::TantivyError),
    #[error("Query parsing error: {0}")]
    Query(#[from] tantivy::query::QueryParserError),
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
    schema: Schema,
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
        let type_field = schema_builder.add_text_field("type", STRING | STORED);
        
        let schema = schema_builder.build();
        let index = Index::create_in_ram(schema.clone());
        
        Ok(Self {
            index,
            schema,
            title_field,
            body_field,
            tags_field,
            slug_field,
            type_field,
        })
    }

    pub fn index_articles(&self, content_dir: &PathBuf) -> Result<(), SearchError> {
        let mut index_writer: IndexWriter = self.index.writer(50_000_000)?;
        let article_service = ArticleService::new(content_dir.clone());
        
        if let Ok(articles) = article_service.get_all_articles() {
            for article in articles {
                let tags_str = article.metadata.tags.join(" ");
                index_writer.add_document(doc!(
                    self.title_field => article.metadata.title,
                    self.body_field => article.content_html, // indexing HTML for simplicity, though raw markdown is better
                    self.tags_field => tags_str,
                    self.slug_field => article.metadata.slug,
                    self.type_field => "article"
                ))?;
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
        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
        
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
                
            // For a real implementation, we'd use a SnippetGenerator.
            let snippet = title.clone();
                
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
