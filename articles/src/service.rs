use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::models::{Article, ArticleMetadata};
use markdown::parser::MarkdownService;
use moka::sync::Cache;

#[derive(Error, Debug)]
pub enum ArticleError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Markdown Parse Error: {0}")]
    Parse(String),
}

pub struct ArticleService {
    base_path: PathBuf,
    cache: Cache<String, Article>,
}

impl ArticleService {
    pub fn new(base_path: PathBuf) -> Self {
        let cache = Cache::builder()
            .max_capacity(100)
            .time_to_live(std::time::Duration::from_secs(60 * 60))
            .build();
            
        Self { base_path, cache }
    }

    pub fn get_all_articles(&self) -> Result<Vec<Article>, ArticleError> {
        let mut articles = Vec::new();
        let articles_dir = self.base_path.join("articles");

        if !articles_dir.exists() {
            return Ok(articles);
        }

        for entry in fs::read_dir(articles_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Ok(article) = self.load_article_from_dir(&path) {
                    if article.metadata.published {
                        articles.push(article);
                    }
                }
            }
        }

        // Sort by created_at descending
        articles.sort_by(|a, b| b.metadata.created_at.cmp(&a.metadata.created_at));

        Ok(articles)
    }

    pub fn get_article(&self, slug: &str) -> Result<Article, ArticleError> {
        if let Some(cached) = self.cache.get(slug) {
            return Ok(cached);
        }

        let path = self.base_path.join("articles").join(slug);
        if !path.exists() || !path.is_dir() {
            return Err(ArticleError::Parse("Article not found".to_string()));
        }
        
        let article = self.load_article_from_dir(&path)?;
        self.cache.insert(slug.to_string(), article.clone());
        
        Ok(article)
    }

    fn load_article_from_dir(&self, dir: &Path) -> Result<Article, ArticleError> {
        let meta_path = dir.join("meta.ron");
        let content_path = dir.join("article.md");

        let meta_content = fs::read_to_string(meta_path)?;
        let md_content = fs::read_to_string(content_path)?;

        let metadata: ArticleMetadata = ron::from_str(&meta_content)
            .map_err(|e| ArticleError::Parse(e.to_string()))?;

        let content_html = MarkdownService::parse_to_html(&md_content);

        Ok(Article {
            metadata,
            content_html,
        })
    }
}
