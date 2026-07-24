use domain::models::{User, Article, Book};
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
    
    // Additional methods like create_user, update_user etc.
}

pub struct ArticleRepository {
    pool: PgPool,
}

impl ArticleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_published_articles(&self) -> Result<Vec<Article>, sqlx::Error> {
        sqlx::query_as!(
            Article,
            "SELECT * FROM articles WHERE published = true ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
    }

    // Additional methods like create_article, update_article, etc.
}

pub struct BookRepository {
    pool: PgPool,
}

impl BookRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_books(&self) -> Result<Vec<Book>, sqlx::Error> {
        sqlx::query_as!(
            Book,
            "SELECT * FROM books ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
    }
}
