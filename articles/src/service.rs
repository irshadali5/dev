use thiserror::Error;
use sqlx::SqlitePool;
use database::repositories::ArticleRepository;
use domain::models::Article;

#[derive(Error, Debug)]
pub enum ArticleError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Article not found")]
    NotFound,
}

pub struct ArticleService {
    pool: SqlitePool,
}

impl ArticleService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_all_articles(&self) -> Result<Vec<Article>, ArticleError> {
        let repo = ArticleRepository::new(self.pool.clone());
        let articles = repo.get_published_articles().await?;
        Ok(articles)
    }

    pub async fn get_article(&self, slug: &str) -> Result<Article, ArticleError> {
        let repo = ArticleRepository::new(self.pool.clone());
        match repo.get_article_by_slug(slug).await? {
            Some(article) => Ok(article),
            None => Err(ArticleError::NotFound),
        }
    }
}
