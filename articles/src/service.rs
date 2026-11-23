use database::repositories::ArticleRepository;
use database::DatabaseConnection;
use domain::models::Article;
use thiserror::Error;

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

impl ArticleService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn get_all_articles(&self) -> Result<Vec<Article>, ArticleError> {
        let repo = ArticleRepository::new(self.conn.clone());
        repo.get_published_articles()
            .await
            .map_err(|e| ArticleError::Database(e.to_string()))
    }

    pub async fn get_article(&self, slug: &str) -> Result<Article, ArticleError> {
        let repo = ArticleRepository::new(self.conn.clone());
        match repo
            .get_article_by_slug(slug)
            .await
            .map_err(|e| ArticleError::Database(e.to_string()))?
        {
            Some(article) => Ok(article),
            None => Err(ArticleError::NotFound),
        }
    }
}
