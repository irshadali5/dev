use crate::models::ArticleMetadata;
use chrono::{DateTime, NaiveDate, Utc};
use database::repositories::ArticleRepository;
use database::DatabaseConnection;
use domain::models::Article;
use markdown::MarkdownService;
use std::fs;
use std::path::Path;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ArticleError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Article not found")]
    NotFound,
}

pub struct ArticleService {
    conn: DatabaseConnection,
}

fn parse_created_at(raw: &str) -> DateTime<Utc> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(raw) {
        return dt.with_timezone(&Utc);
    }
    if let Ok(d) = NaiveDate::parse_from_str(raw, "%Y-%m-%d") {
        if let Some(dt) = d.and_hms_opt(0, 0, 0) {
            return DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc);
        }
    }
    Utc::now()
}

fn load_fs_articles() -> Vec<Article> {
    let possible_dirs = [
        Path::new("content/articles"),
        Path::new("../content/articles"),
        Path::new("../../content/articles"),
    ];
    let Some(dir) = possible_dirs.iter().find(|p| p.exists()) else {
        return Vec::new();
    };

    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let meta_path = path.join("meta.ron");
        let md_path = path.join("article.md");
        if !meta_path.exists() || !md_path.exists() {
            continue;
        }
        let Ok(meta_raw) = fs::read_to_string(&meta_path) else {
            continue;
        };
        let Ok(meta) = ron::from_str::<ArticleMetadata>(&meta_raw) else {
            continue;
        };
        if !meta.published {
            continue;
        }
        let md_raw = fs::read_to_string(&md_path).unwrap_or_default();
        let body = MarkdownService::parse_to_html(&md_raw);
        let created_at = parse_created_at(&meta.created_at);

        out.push(Article {
            id: Uuid::new_v4(),
            slug: meta.slug,
            title: meta.title,
            description: Some(meta.description),
            body,
            published: meta.published,
            author_id: Uuid::nil(),
            created_at,
            updated_at: created_at,
            tags: meta.tags,
        });
    }

    out
}

impl ArticleService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn get_all_articles(&self) -> Result<Vec<Article>, ArticleError> {
        let repo = ArticleRepository::new(self.conn.clone());
        let mut articles = repo
            .get_published_articles()
            .await
            .map_err(|e| ArticleError::Database(e.to_string()))?;

        for fs_art in load_fs_articles() {
            if !articles.iter().any(|a| a.slug == fs_art.slug) {
                articles.push(fs_art);
            }
        }

        articles.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(articles)
    }

    pub async fn get_article(&self, slug: &str) -> Result<Article, ArticleError> {
        let repo = ArticleRepository::new(self.conn.clone());
        if let Some(article) = repo
            .get_article_by_slug(slug)
            .await
            .map_err(|e| ArticleError::Database(e.to_string()))?
        {
            return Ok(article);
        }

        if let Some(fs_art) = load_fs_articles().into_iter().find(|a| a.slug == slug) {
            return Ok(fs_art);
        }

        Err(ArticleError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_article_service_loads_fs_articles() {
        let db = database::get_db(":memory:").await.expect("Failed to create in-memory db");
        let service = ArticleService::new(db);
        let articles = service.get_all_articles().await.expect("Failed to load articles");
        assert!(articles.iter().any(|a| a.slug == "rust-ownership"));
        let detail = service.get_article("rust-ownership").await.expect("Should find rust-ownership");
        assert!(!detail.body.is_empty());
    }
}
