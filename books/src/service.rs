use crate::models::{BookError, BookMeta, Chapter, ChapterMeta, FullBook};
use database::repositories::BookRepository;
use domain::models::Book;
use markdown::MarkdownService;
use sqlx::SqlitePool;
use std::fs;
use std::path::Path;

pub struct BookService {
    repo: Option<BookRepository>,
}

impl BookService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repo: Some(BookRepository::new(pool)),
        }
    }

    pub fn new_offline() -> Self {
        Self { repo: None }
    }

    pub async fn get_reading_list(&self) -> Result<Vec<Book>, sqlx::Error> {
        if let Some(ref repo) = self.repo {
            repo.get_all_books().await
        } else {
            Ok(vec![])
        }
    }

    /// Load book metadata and chapters from a local directory (e.g. `content/books/production-grade-rust`)
    pub fn load_book_from_dir(&self, book_dir: &Path) -> Result<FullBook, BookError> {
        let ron_path = book_dir.join("book.ron");
        if !ron_path.exists() {
            return Err(BookError::NotFound(format!(
                "book.ron not found in {}",
                book_dir.display()
            )));
        }

        let ron_content = fs::read_to_string(&ron_path)?;
        let meta: BookMeta = ron::from_str(&ron_content)?;

        let mut chapters = Vec::new();
        for chapter_file in &meta.chapters {
            let chapter_path = book_dir.join(chapter_file);
            if chapter_path.exists() {
                let markdown_raw = fs::read_to_string(&chapter_path)?;
                let (ch_meta, body) = parse_chapter_markdown(&markdown_raw, chapter_file)?;
                let html = MarkdownService::parse_to_html(&body);
                chapters.push(Chapter {
                    slug: chapter_file.trim_end_matches(".md").to_string(),
                    meta: ch_meta,
                    markdown: body,
                    html,
                });
            }
        }

        Ok(FullBook { meta, chapters })
    }

    /// List all available books from `content/books/` directory
    pub fn list_books(&self, books_dir: &Path) -> Result<Vec<BookMeta>, BookError> {
        if !books_dir.exists() {
            return Ok(vec![]);
        }

        let mut books = Vec::new();
        for entry in fs::read_dir(books_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let ron_path = path.join("book.ron");
                if ron_path.exists() {
                    let ron_content = fs::read_to_string(&ron_path)?;
                    if let Ok(meta) = ron::from_str::<BookMeta>(&ron_content) {
                        books.push(meta);
                    }
                }
            }
        }

        Ok(books)
    }
}

fn parse_chapter_markdown(content: &str, filename: &str) -> Result<(ChapterMeta, String), BookError> {
    if content.starts_with("---") {
        if let Some(end_idx) = content[3..].find("---") {
            let fm_str = &content[3..3 + end_idx];
            let body = &content[3 + end_idx + 3..];

            let mut title = filename.replace(".md", "");
            let mut description = None;
            let mut order = 0u32;

            for line in fm_str.lines() {
                let l = line.trim();
                if l.starts_with("title:") {
                    title = l["title:".len()..].trim().trim_matches('"').to_string();
                } else if l.starts_with("description:") {
                    let desc = l["description:".len()..].trim().trim_matches('"').to_string();
                    if !desc.is_empty() {
                        description = Some(desc);
                    }
                } else if l.starts_with("order:") {
                    if let Ok(ord) = l["order:".len()..].trim().parse::<u32>() {
                        order = ord;
                    }
                }
            }

            return Ok((
                ChapterMeta {
                    title,
                    description,
                    order,
                },
                body.trim().to_string(),
            ));
        }
    }

    Ok((
        ChapterMeta {
            title: filename.replace(".md", ""),
            description: None,
            order: 0,
        },
        content.trim().to_string(),
    ))
}
