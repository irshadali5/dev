use domain::models::{User, Article, Book, Project};
use sqlx::PgPool;


pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await
    }
}

pub struct ArticleRepository {
    pool: PgPool,
}

impl ArticleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_published_articles(&self) -> Result<Vec<Article>, sqlx::Error> {
        // Mocking tags since sqlx::query_as! expects exactly the struct fields, 
        // we map it manually or ignore tags for now, or use query! and map it.
        let records = sqlx::query!(
            r#"
            SELECT id, slug, title, description, body, published, author_id, created_at, updated_at
            FROM articles 
            WHERE published = true 
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records.into_iter().map(|r| Article {
            id: r.id,
            slug: r.slug,
            title: r.title,
            description: r.description,
            body: r.body,
            published: r.published,
            author_id: r.author_id,
            created_at: r.created_at,
            updated_at: r.updated_at,
            tags: vec![], // Populate later with JOIN if needed
        }).collect())
    }

    pub async fn get_article_by_slug(&self, slug: &str) -> Result<Option<Article>, sqlx::Error> {
        let r = sqlx::query!(
            r#"
            SELECT id, slug, title, description, body, published, author_id, created_at, updated_at
            FROM articles 
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(r.map(|r| Article {
            id: r.id,
            slug: r.slug,
            title: r.title,
            description: r.description,
            body: r.body,
            published: r.published,
            author_id: r.author_id,
            created_at: r.created_at,
            updated_at: r.updated_at,
            tags: vec![],
        }))
    }
}

pub struct ProjectRepository {
    pool: PgPool,
}

impl ProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_projects(&self) -> Result<Vec<Project>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT id, slug, title, description, content_html, github_url, live_url, created_at, updated_at
            FROM projects
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records.into_iter().map(|r| Project {
            id: r.id,
            slug: r.slug,
            title: r.title,
            description: r.description,
            content_html: r.content_html,
            github_url: r.github_url,
            live_url: r.live_url,
            created_at: r.created_at,
            updated_at: r.updated_at,
            tags: vec![],
        }).collect())
    }
}

pub struct BookRepository {
    pool: PgPool,
}

impl BookRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_books(&self) -> Result<Vec<Book>, sqlx::Error> {
        sqlx::query_as!(Book, "SELECT * FROM books ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
    }
}
