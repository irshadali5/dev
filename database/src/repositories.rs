use domain::models::{User, Article, Book, Project, TimelineItem, Skill};
use sqlx::SqlitePool;


pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        // sqlx maps SQLite's DATETIME to NaiveDateTime, which we need to convert to DateTime<Utc>
        let r = sqlx::query!("SELECT id, email, password_hash, role, created_at, updated_at FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(r.map(|r| User {
            id: r.id.unwrap_or_default().parse().unwrap(),
            email: r.email,
            password_hash: r.password_hash,
            role: r.role,
            created_at: r.created_at.and_utc(),
            updated_at: r.updated_at.and_utc(),
        }))
    }
}

pub struct ArticleRepository {
    pool: SqlitePool,
}

impl ArticleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_published_articles(&self) -> Result<Vec<Article>, sqlx::Error> {
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
            id: r.id.unwrap_or_default().parse().unwrap(),
            slug: r.slug,
            title: r.title,
            description: r.description,
            body: r.body,
            published: r.published,
            author_id: r.author_id.parse().unwrap(),
            created_at: r.created_at.and_utc(),
            updated_at: r.updated_at.and_utc(),
            tags: vec![],
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
            id: r.id.unwrap_or_default().parse().unwrap(),
            slug: r.slug,
            title: r.title,
            description: r.description,
            body: r.body,
            published: r.published,
            author_id: r.author_id.parse().unwrap(),
            created_at: r.created_at.and_utc(),
            updated_at: r.updated_at.and_utc(),
            tags: vec![],
        }))
    }

    pub async fn add_article(&self, slug: &str, title: &str, description: &str, body: &str, published: bool, author_id: uuid::Uuid) -> Result<Article, sqlx::Error> {
        let id_str = uuid::Uuid::new_v4().to_string();
        let author_id_str = author_id.to_string();
        let r = sqlx::query!(
            r#"
            INSERT INTO articles (id, slug, title, description, body, published, author_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, slug, title, description, body, published, author_id, created_at, updated_at
            "#,
            id_str, slug, title, description, body, published, author_id_str
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(Article {
            id: r.id.unwrap_or_default().parse().unwrap(),
            slug: r.slug,
            title: r.title,
            description: r.description,
            body: r.body,
            published: r.published,
            author_id: r.author_id.parse().unwrap(),
            created_at: r.created_at.and_utc(),
            updated_at: r.updated_at.and_utc(),
            tags: vec![],
        })
    }

    pub async fn delete_article(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        let id_str = id.to_string();
        sqlx::query!("DELETE FROM articles WHERE id = $1", id_str)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

pub struct ProjectRepository {
    pool: SqlitePool,
}

impl ProjectRepository {
    pub fn new(pool: SqlitePool) -> Self {
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
            id: r.id.unwrap_or_default().parse().unwrap(),
            slug: r.slug,
            title: r.title,
            description: r.description,
            content_html: r.content_html,
            github_url: r.github_url,
            live_url: r.live_url,
            created_at: r.created_at.and_utc(),
            updated_at: r.updated_at.and_utc(),
            tags: vec![],
        }).collect())
    }

    pub async fn add_project(&self, slug: &str, title: &str, description: &str, content_html: &str, github_url: Option<String>, live_url: Option<String>) -> Result<Project, sqlx::Error> {
        let id_str = uuid::Uuid::new_v4().to_string();
        let r = sqlx::query!(
            r#"
            INSERT INTO projects (id, slug, title, description, content_html, github_url, live_url)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, slug, title, description, content_html, github_url, live_url, created_at, updated_at
            "#,
            id_str, slug, title, description, content_html, github_url, live_url
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Project {
            id: r.id.unwrap_or_default().parse().unwrap(),
            slug: r.slug,
            title: r.title,
            description: r.description,
            content_html: r.content_html,
            github_url: r.github_url,
            live_url: r.live_url,
            created_at: r.created_at.and_utc(),
            updated_at: r.updated_at.and_utc(),
            tags: vec![],
        })
    }

    pub async fn delete_project(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        let id_str = id.to_string();
        sqlx::query!("DELETE FROM projects WHERE id = $1", id_str)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

pub struct BookRepository {
    pool: SqlitePool,
}

impl BookRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_all_books(&self) -> Result<Vec<Book>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT id, slug, title, description, author_id, created_at, updated_at
            FROM books
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(records.into_iter().map(|r| Book {
            id: r.id.unwrap_or_default().parse().unwrap(),
            slug: r.slug,
            title: r.title,
            description: r.description,
            author_id: r.author_id.parse().unwrap(),
            created_at: r.created_at.and_utc(),
            updated_at: r.updated_at.and_utc(),
        }).collect())
    }

    pub async fn add_book(&self, slug: &str, title: &str, description: Option<String>, author_id: uuid::Uuid) -> Result<Book, sqlx::Error> {
        let id_str = uuid::Uuid::new_v4().to_string();
        let author_id_str = author_id.to_string();
        let r = sqlx::query!(
            "INSERT INTO books (id, slug, title, description, author_id) VALUES ($1, $2, $3, $4, $5) RETURNING id, slug, title, description, author_id, created_at, updated_at",
            id_str, slug, title, description, author_id_str
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(Book {
            id: r.id.unwrap_or_default().parse().unwrap(),
            slug: r.slug,
            title: r.title,
            description: r.description,
            author_id: r.author_id.parse().unwrap(),
            created_at: r.created_at.and_utc(),
            updated_at: r.updated_at.and_utc(),
        })
    }

    pub async fn delete_book(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        let id_str = id.to_string();
        sqlx::query!("DELETE FROM books WHERE id = $1", id_str)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

pub struct PortfolioRepository {
    pool: SqlitePool,
}

impl PortfolioRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_timeline_items(&self) -> Result<Vec<TimelineItem>, sqlx::Error> {
        let records = sqlx::query!(
            r#"SELECT id, item_type, title, organization, period, description, order_index, created_at, updated_at FROM timeline_items ORDER BY order_index ASC"#
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(records.into_iter().map(|r| TimelineItem {
            id: r.id.unwrap_or_default().parse().unwrap(),
            item_type: r.item_type,
            title: r.title,
            organization: r.organization,
            period: r.period,
            description: r.description,
            order_index: r.order_index as i32,
            created_at: r.created_at.and_utc(),
            updated_at: r.updated_at.and_utc(),
        }).collect())
    }

    pub async fn add_timeline_item(&self, item_type: &str, title: &str, organization: &str, period: &str, description: &str, order_index: i32) -> Result<TimelineItem, sqlx::Error> {
        let id_str = uuid::Uuid::new_v4().to_string();
        let r = sqlx::query!(
            "INSERT INTO timeline_items (id, item_type, title, organization, period, description, order_index) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, item_type, title, organization, period, description, order_index, created_at, updated_at",
            id_str, item_type, title, organization, period, description, order_index
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(TimelineItem {
            id: r.id.unwrap_or_default().parse().unwrap(),
            item_type: r.item_type,
            title: r.title,
            organization: r.organization,
            period: r.period,
            description: r.description,
            order_index: r.order_index as i32,
            created_at: r.created_at.and_utc(),
            updated_at: r.updated_at.and_utc(),
        })
    }

    pub async fn delete_timeline_item(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        let id_str = id.to_string();
        sqlx::query!("DELETE FROM timeline_items WHERE id = $1", id_str)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_skills(&self) -> Result<Vec<Skill>, sqlx::Error> {
        let records = sqlx::query!(
            "SELECT id, category, name, order_index, created_at, updated_at FROM skills ORDER BY category ASC, order_index ASC"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(records.into_iter().map(|r| Skill {
            id: r.id.unwrap_or_default().parse().unwrap(),
            category: r.category,
            name: r.name,
            order_index: r.order_index as i32,
            created_at: r.created_at.and_utc(),
            updated_at: r.updated_at.and_utc(),
        }).collect())
    }

    pub async fn add_skill(&self, category: &str, name: &str, order_index: i32) -> Result<Skill, sqlx::Error> {
        let id_str = uuid::Uuid::new_v4().to_string();
        let r = sqlx::query!(
            "INSERT INTO skills (id, category, name, order_index) VALUES ($1, $2, $3, $4) RETURNING id, category, name, order_index, created_at, updated_at",
            id_str, category, name, order_index
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(Skill {
            id: r.id.unwrap_or_default().parse().unwrap(),
            category: r.category,
            name: r.name,
            order_index: r.order_index as i32,
            created_at: r.created_at.and_utc(),
            updated_at: r.updated_at.and_utc(),
        })
    }

    pub async fn delete_skill(&self, id: uuid::Uuid) -> Result<(), sqlx::Error> {
        let id_str = id.to_string();
        sqlx::query!("DELETE FROM skills WHERE id = $1", id_str)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
