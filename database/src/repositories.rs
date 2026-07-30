use domain::models::{User, Article, Book, Project, TimelineItem, Skill};
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

    pub async fn add_article(&self, slug: &str, title: &str, description: &str, body: &str, published: bool, author_id: uuid::Uuid) -> Result<Article, sqlx::Error> {
        let r = sqlx::query!(
            r#"
            INSERT INTO articles (slug, title, description, body, published, author_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, slug, title, description, body, published, author_id, created_at, updated_at
            "#,
            slug, title, description, body, published, author_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(Article {
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
        })
    }

    pub async fn delete_article(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM articles WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
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

    pub async fn add_project(&self, slug: &str, title: &str, description: &str, content_html: &str, github_url: Option<String>, live_url: Option<String>) -> Result<Project, sqlx::Error> {
        let r = sqlx::query!(
            r#"
            INSERT INTO projects (slug, title, description, content_html, github_url, live_url)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, slug, title, description, content_html, github_url, live_url, created_at, updated_at
            "#,
            slug, title, description, content_html, github_url, live_url
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Project {
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
        })
    }

    pub async fn delete_project(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM projects WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
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

    pub async fn add_book(&self, slug: &str, title: &str, description: Option<String>, author_id: uuid::Uuid) -> Result<Book, sqlx::Error> {
        sqlx::query_as!(
            Book,
            "INSERT INTO books (slug, title, description, author_id) VALUES ($1, $2, $3, $4) RETURNING *",
            slug, title, description, author_id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_book(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM books WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

pub struct PortfolioRepository {
    pool: PgPool,
}

impl PortfolioRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_timeline_items(&self) -> Result<Vec<TimelineItem>, sqlx::Error> {
        sqlx::query_as!(TimelineItem, "SELECT * FROM timeline_items ORDER BY order_index ASC")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn add_timeline_item(&self, item_type: &str, title: &str, organization: &str, period: &str, description: &str, order_index: i32) -> Result<TimelineItem, sqlx::Error> {
        sqlx::query_as!(
            TimelineItem,
            "INSERT INTO timeline_items (item_type, title, organization, period, description, order_index) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            item_type, title, organization, period, description, order_index
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_timeline_item(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM timeline_items WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_skills(&self) -> Result<Vec<Skill>, sqlx::Error> {
        sqlx::query_as!(Skill, "SELECT * FROM skills ORDER BY category ASC, order_index ASC")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn add_skill(&self, category: &str, name: &str, order_index: i32) -> Result<Skill, sqlx::Error> {
        sqlx::query_as!(
            Skill,
            "INSERT INTO skills (category, name, order_index) VALUES ($1, $2, $3) RETURNING *",
            category, name, order_index
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_skill(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM skills WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
