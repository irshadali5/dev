use crate::DatabaseConnection;
use chrono::{DateTime, Utc};
use domain::models::{Article, Book, ContactMessage, NewsletterSubscriber, Project, Skill, TimelineItem, User};
use std::error::Error;
use uuid::Uuid;

type DbResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub struct UserRepository {
    conn: DatabaseConnection,
}

impl UserRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn get_user_by_email(&self, email: &str) -> DbResult<Option<User>> {
        let db = self.conn.inner();
        let query_sql = format!(
            "SELECT id, email, password_hash, role, created_at, updated_at FROM users WHERE email = '{}' LIMIT 1",
            email.replace('\'', "''")
        );
        let mut rows = db.query(&query_sql, ())?;
        if let Some(row_res) = rows.next() {
            let row = row_res?;
            let id_str: String = row.get(0)?;
            let email: String = row.get(1)?;
            let password_hash: String = row.get(2)?;
            let role: String = row.get(3)?;
            let created_at_str: String = row.get(4)?;
            let updated_at_str: String = row.get(5)?;

            let id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            return Ok(Some(User {
                id,
                email,
                password_hash,
                role,
                created_at,
                updated_at,
            }));
        }
        Ok(None)
    }

    pub async fn add_user(&self, email: &str, password_hash: &str, role: &str) -> DbResult<User> {
        let db = self.conn.inner();
        let id = Uuid::new_v4();
        let id_str = id.to_string();
        let now = Utc::now();
        let now_str = now.to_rfc3339();

        let sql = format!(
            "INSERT INTO users (id, email, password_hash, role, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')",
            id_str,
            email.replace('\'', "''"),
            password_hash.replace('\'', "''"),
            role.replace('\'', "''"),
            now_str,
            now_str
        );
        db.execute(&sql, ())?;

        Ok(User {
            id,
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            role: role.to_string(),
            created_at: now,
            updated_at: now,
        })
    }
}

pub struct ArticleRepository {
    conn: DatabaseConnection,
}

impl ArticleRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn get_published_articles(&self) -> DbResult<Vec<Article>> {
        let db = self.conn.inner();
        let rows = db.query(
            "SELECT id, slug, title, description, body, published, author_id, created_at, updated_at, tags FROM articles WHERE published = 1 ORDER BY created_at DESC",
            (),
        )?;

        let mut articles = Vec::new();
        for row_res in rows {
            let row = row_res?;
            let id_str: String = row.get(0)?;
            let slug: String = row.get(1)?;
            let title: String = row.get(2)?;
            let description: Option<String> = row.get(3).ok();
            let body: String = row.get(4)?;
            let published: i64 = row.get(5)?;
            let author_id_str: String = row.get(6)?;
            let created_at_str: String = row.get(7)?;
            let updated_at_str: String = row.get(8)?;
            let tags_str: Option<String> = row.get(9).ok();

            let id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
            let author_id = Uuid::parse_str(&author_id_str).unwrap_or_else(|_| Uuid::new_v4());
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            let tags = tags_str
                .and_then(|s| serde_json::from_str::<Vec<String>>(&s).ok())
                .unwrap_or_default();

            articles.push(Article {
                id,
                slug,
                title,
                description,
                body,
                published: published != 0,
                author_id,
                created_at,
                updated_at,
                tags,
            });
        }

        Ok(articles)
    }

    pub async fn get_article_by_slug(&self, slug: &str) -> DbResult<Option<Article>> {
        let db = self.conn.inner();
        let sql = format!(
            "SELECT id, slug, title, description, body, published, author_id, created_at, updated_at, tags FROM articles WHERE slug = '{}' LIMIT 1",
            slug.replace('\'', "''")
        );
        let mut rows = db.query(&sql, ())?;

        if let Some(row_res) = rows.next() {
            let row = row_res?;
            let id_str: String = row.get(0)?;
            let slug_val: String = row.get(1)?;
            let title: String = row.get(2)?;
            let description: Option<String> = row.get(3).ok();
            let body: String = row.get(4)?;
            let published: i64 = row.get(5)?;
            let author_id_str: String = row.get(6)?;
            let created_at_str: String = row.get(7)?;
            let updated_at_str: String = row.get(8)?;
            let tags_str: Option<String> = row.get(9).ok();

            let id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
            let author_id = Uuid::parse_str(&author_id_str).unwrap_or_else(|_| Uuid::new_v4());
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            let tags = tags_str
                .and_then(|s| serde_json::from_str::<Vec<String>>(&s).ok())
                .unwrap_or_default();

            return Ok(Some(Article {
                id,
                slug: slug_val,
                title,
                description,
                body,
                published: published != 0,
                author_id,
                created_at,
                updated_at,
                tags,
            }));
        }

        Ok(None)
    }

    pub async fn add_article(
        &self,
        slug: &str,
        title: &str,
        description: &str,
        body: &str,
        published: bool,
        author_id: Uuid,
    ) -> DbResult<Article> {
        let db = self.conn.inner();
        let id = Uuid::new_v4();
        let id_str = id.to_string();
        let author_id_str = author_id.to_string();
        let now = Utc::now();
        let now_str = now.to_rfc3339();
        let pub_int = if published { 1 } else { 0 };

        let sql = format!(
            "INSERT INTO articles (id, slug, title, description, body, published, author_id, created_at, updated_at, tags) VALUES ('{}', '{}', '{}', '{}', '{}', {}, '{}', '{}', '{}', '[]')",
            id_str,
            slug.replace('\'', "''"),
            title.replace('\'', "''"),
            description.replace('\'', "''"),
            body.replace('\'', "''"),
            pub_int,
            author_id_str,
            now_str,
            now_str
        );
        db.execute(&sql, ())?;

        Ok(Article {
            id,
            slug: slug.to_string(),
            title: title.to_string(),
            description: Some(description.to_string()),
            body: body.to_string(),
            published,
            author_id,
            created_at: now,
            updated_at: now,
            tags: vec![],
        })
    }

    pub async fn update_article(
        &self,
        id: Uuid,
        slug: &str,
        title: &str,
        description: &str,
        body: &str,
        published: bool,
    ) -> DbResult<()> {
        let db = self.conn.inner();
        let id_str = id.to_string();
        let now_str = Utc::now().to_rfc3339();
        let pub_int = if published { 1 } else { 0 };

        let sql = format!(
            "UPDATE articles SET slug = '{}', title = '{}', description = '{}', body = '{}', published = {}, updated_at = '{}' WHERE id = '{}'",
            slug.replace('\'', "''"),
            title.replace('\'', "''"),
            description.replace('\'', "''"),
            body.replace('\'', "''"),
            pub_int,
            now_str,
            id_str
        );
        db.execute(&sql, ())?;
        Ok(())
    }

    pub async fn delete_article(&self, id: Uuid) -> DbResult<()> {
        let db = self.conn.inner();
        let sql = format!("DELETE FROM articles WHERE id = '{}'", id);
        db.execute(&sql, ())?;
        Ok(())
    }

    pub async fn get_likes(&self, slug: &str) -> DbResult<u64> {
        let db = self.conn.inner();
        let sql = format!(
            "SELECT likes_count FROM article_likes WHERE slug = '{}' LIMIT 1",
            slug.replace('\'', "''")
        );
        let mut rows = db.query(&sql, ())?;
        if let Some(row_res) = rows.next() {
            let row = row_res?;
            let count: i64 = row.get(0)?;
            return Ok(count.max(0) as u64);
        }
        Ok(0)
    }

    pub async fn increment_likes(&self, slug: &str) -> DbResult<u64> {
        let db = self.conn.inner();
        let check_sql = format!(
            "SELECT row_id, likes_count FROM article_likes WHERE slug = '{}' LIMIT 1",
            slug.replace('\'', "''")
        );
        let mut existing = db.query(&check_sql, ())?;
        if let Some(row_res) = existing.next() {
            let row = row_res?;
            let row_id: i64 = row.get(0)?;
            let current: i64 = row.get(1)?;
            let next_count = current + 1;
            let update_sql = format!(
                "UPDATE article_likes SET likes_count = {} WHERE row_id = {}",
                next_count, row_id
            );
            db.execute(&update_sql, ())?;
            return Ok(next_count as u64);
        }

        let insert_sql = format!(
            "INSERT INTO article_likes (slug, likes_count) VALUES ('{}', 1)",
            slug.replace('\'', "''")
        );
        db.execute(&insert_sql, ())?;
        Ok(1)
    }
}

pub struct ProjectRepository {
    conn: DatabaseConnection,
}

impl ProjectRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn get_all_projects(&self) -> DbResult<Vec<Project>> {
        let db = self.conn.inner();
        let rows = db.query(
            "SELECT id, slug, title, description, content_html, github_url, live_url, created_at, updated_at FROM projects ORDER BY created_at DESC",
            (),
        )?;

        let mut projects = Vec::new();
        for row_res in rows {
            let row = row_res?;
            let id_str: String = row.get(0)?;
            let slug: String = row.get(1)?;
            let title: String = row.get(2)?;
            let description: String = row.get(3)?;
            let content_html: String = row.get(4)?;
            let github_url: Option<String> = row.get(5).ok();
            let live_url: Option<String> = row.get(6).ok();
            let created_at_str: String = row.get(7)?;
            let updated_at_str: String = row.get(8)?;

            let id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            projects.push(Project {
                id,
                slug,
                title,
                description,
                content_html,
                github_url,
                live_url,
                created_at,
                updated_at,
                tags: vec![],
                architecture: None,
                benchmarks: None,
                lessons_learned: None,
            });
        }

        Ok(projects)
    }

    pub async fn add_project(
        &self,
        slug: &str,
        title: &str,
        description: &str,
        content_html: &str,
        github_url: Option<String>,
        live_url: Option<String>,
    ) -> DbResult<Project> {
        let db = self.conn.inner();
        let id = Uuid::new_v4();
        let id_str = id.to_string();
        let now = Utc::now();
        let now_str = now.to_rfc3339();

        let gh_val = github_url.as_deref().unwrap_or("");
        let live_val = live_url.as_deref().unwrap_or("");

        let sql = format!(
            "INSERT INTO projects (id, slug, title, description, content_html, github_url, live_url, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')",
            id_str,
            slug.replace('\'', "''"),
            title.replace('\'', "''"),
            description.replace('\'', "''"),
            content_html.replace('\'', "''"),
            gh_val.replace('\'', "''"),
            live_val.replace('\'', "''"),
            now_str,
            now_str
        );
        db.execute(&sql, ())?;

        Ok(Project {
            id,
            slug: slug.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            content_html: content_html.to_string(),
            github_url,
            live_url,
            created_at: now,
            updated_at: now,
            tags: vec![],
            architecture: None,
            benchmarks: None,
            lessons_learned: None,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update_project(
        &self,
        id: Uuid,
        slug: &str,
        title: &str,
        description: &str,
        content_html: &str,
        github_url: Option<String>,
        live_url: Option<String>,
    ) -> DbResult<()> {
        let db = self.conn.inner();
        let id_str = id.to_string();
        let now_str = Utc::now().to_rfc3339();
        let gh_val = github_url.as_deref().unwrap_or("");
        let live_val = live_url.as_deref().unwrap_or("");

        let sql = format!(
            "UPDATE projects SET slug = '{}', title = '{}', description = '{}', content_html = '{}', github_url = '{}', live_url = '{}', updated_at = '{}' WHERE id = '{}'",
            slug.replace('\'', "''"),
            title.replace('\'', "''"),
            description.replace('\'', "''"),
            content_html.replace('\'', "''"),
            gh_val.replace('\'', "''"),
            live_val.replace('\'', "''"),
            now_str,
            id_str
        );
        db.execute(&sql, ())?;
        Ok(())
    }

    pub async fn delete_project(&self, id: Uuid) -> DbResult<()> {
        let db = self.conn.inner();
        let sql = format!("DELETE FROM projects WHERE id = '{}'", id);
        db.execute(&sql, ())?;
        Ok(())
    }
}

pub struct BookRepository {
    conn: DatabaseConnection,
}

impl BookRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn get_all_books(&self) -> DbResult<Vec<Book>> {
        let db = self.conn.inner();
        let rows = db.query(
            "SELECT id, slug, title, description, author_id, created_at, updated_at FROM books ORDER BY created_at DESC",
            (),
        )?;

        let mut books = Vec::new();
        for row_res in rows {
            let row = row_res?;
            let id_str: String = row.get(0)?;
            let slug: String = row.get(1)?;
            let title: String = row.get(2)?;
            let description: Option<String> = row.get(3).ok();
            let author_id_str: String = row.get(4)?;
            let created_at_str: String = row.get(5)?;
            let updated_at_str: String = row.get(6)?;

            let id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
            let author_id = Uuid::parse_str(&author_id_str).unwrap_or_else(|_| Uuid::new_v4());
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            books.push(Book {
                id,
                slug,
                title,
                description,
                author_id,
                created_at,
                updated_at,
            });
        }

        Ok(books)
    }

    pub async fn add_book(
        &self,
        slug: &str,
        title: &str,
        description: Option<String>,
        author_id: Uuid,
    ) -> DbResult<Book> {
        let db = self.conn.inner();
        let id = Uuid::new_v4();
        let id_str = id.to_string();
        let author_id_str = author_id.to_string();
        let now = Utc::now();
        let now_str = now.to_rfc3339();
        let desc_val = description.as_deref().unwrap_or("");

        let sql = format!(
            "INSERT INTO books (id, slug, title, description, author_id, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}')",
            id_str,
            slug.replace('\'', "''"),
            title.replace('\'', "''"),
            desc_val.replace('\'', "''"),
            author_id_str,
            now_str,
            now_str
        );
        db.execute(&sql, ())?;

        Ok(Book {
            id,
            slug: slug.to_string(),
            title: title.to_string(),
            description,
            author_id,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn update_book(
        &self,
        id: Uuid,
        slug: &str,
        title: &str,
        description: Option<String>,
    ) -> DbResult<()> {
        let db = self.conn.inner();
        let id_str = id.to_string();
        let now_str = Utc::now().to_rfc3339();
        let desc_val = description.as_deref().unwrap_or("");

        let sql = format!(
            "UPDATE books SET slug = '{}', title = '{}', description = '{}', updated_at = '{}' WHERE id = '{}'",
            slug.replace('\'', "''"),
            title.replace('\'', "''"),
            desc_val.replace('\'', "''"),
            now_str,
            id_str
        );
        db.execute(&sql, ())?;
        Ok(())
    }

    pub async fn delete_book(&self, id: Uuid) -> DbResult<()> {
        let db = self.conn.inner();
        let sql = format!("DELETE FROM books WHERE id = '{}'", id);
        db.execute(&sql, ())?;
        Ok(())
    }
}

pub struct PortfolioRepository {
    conn: DatabaseConnection,
}

impl PortfolioRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn get_timeline_items(&self) -> DbResult<Vec<TimelineItem>> {
        let db = self.conn.inner();
        let rows = db.query(
            "SELECT id, item_type, title, organization, period, description, order_index, created_at, updated_at FROM timeline_items ORDER BY order_index ASC",
            (),
        )?;

        let mut items = Vec::new();
        for row_res in rows {
            let row = row_res?;
            let id_str: String = row.get(0)?;
            let item_type: String = row.get(1)?;
            let title: String = row.get(2)?;
            let organization: String = row.get(3)?;
            let period: String = row.get(4)?;
            let description: String = row.get(5)?;
            let order_index: i64 = row.get(6)?;
            let created_at_str: String = row.get(7)?;
            let updated_at_str: String = row.get(8)?;

            let id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            items.push(TimelineItem {
                id,
                item_type,
                title,
                organization,
                period,
                description,
                order_index: order_index as i32,
                created_at,
                updated_at,
            });
        }

        Ok(items)
    }

    pub async fn add_timeline_item(
        &self,
        item_type: &str,
        title: &str,
        organization: &str,
        period: &str,
        description: &str,
        order_index: i32,
    ) -> DbResult<TimelineItem> {
        let db = self.conn.inner();
        let id = Uuid::new_v4();
        let id_str = id.to_string();
        let now = Utc::now();
        let now_str = now.to_rfc3339();

        let sql = format!(
            "INSERT INTO timeline_items (id, item_type, title, organization, period, description, order_index, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', {}, '{}', '{}')",
            id_str,
            item_type.replace('\'', "''"),
            title.replace('\'', "''"),
            organization.replace('\'', "''"),
            period.replace('\'', "''"),
            description.replace('\'', "''"),
            order_index,
            now_str,
            now_str
        );
        db.execute(&sql, ())?;

        Ok(TimelineItem {
            id,
            item_type: item_type.to_string(),
            title: title.to_string(),
            organization: organization.to_string(),
            period: period.to_string(),
            description: description.to_string(),
            order_index,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn delete_timeline_item(&self, id: Uuid) -> DbResult<()> {
        let db = self.conn.inner();
        let sql = format!("DELETE FROM timeline_items WHERE id = '{}'", id);
        db.execute(&sql, ())?;
        Ok(())
    }

    pub async fn get_skills(&self) -> DbResult<Vec<Skill>> {
        let db = self.conn.inner();
        let rows = db.query(
            "SELECT id, category, name, order_index, created_at, updated_at FROM skills ORDER BY category ASC, order_index ASC",
            (),
        )?;

        let mut skills = Vec::new();
        for row_res in rows {
            let row = row_res?;
            let id_str: String = row.get(0)?;
            let category: String = row.get(1)?;
            let name: String = row.get(2)?;
            let order_index: i64 = row.get(3)?;
            let created_at_str: String = row.get(4)?;
            let updated_at_str: String = row.get(5)?;

            let id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            skills.push(Skill {
                id,
                category,
                name,
                order_index: order_index as i32,
                created_at,
                updated_at,
            });
        }

        Ok(skills)
    }

    pub async fn add_skill(&self, category: &str, name: &str, order_index: i32) -> DbResult<Skill> {
        let db = self.conn.inner();
        let id = Uuid::new_v4();
        let id_str = id.to_string();
        let now = Utc::now();
        let now_str = now.to_rfc3339();

        let sql = format!(
            "INSERT INTO skills (id, category, name, order_index, created_at, updated_at) VALUES ('{}', '{}', '{}', {}, '{}', '{}')",
            id_str,
            category.replace('\'', "''"),
            name.replace('\'', "''"),
            order_index,
            now_str,
            now_str
        );
        db.execute(&sql, ())?;

        Ok(Skill {
            id,
            category: category.to_string(),
            name: name.to_string(),
            order_index,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn delete_skill(&self, id: Uuid) -> DbResult<()> {
        let db = self.conn.inner();
        let sql = format!("DELETE FROM skills WHERE id = '{}'", id);
        db.execute(&sql, ())?;
        Ok(())
    }
}

pub struct ContactRepository {
    conn: DatabaseConnection,
}

impl ContactRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn save_message(&self, name: &str, email: &str, message: &str) -> DbResult<ContactMessage> {
        let db = self.conn.inner();
        let id = Uuid::new_v4();
        let id_str = id.to_string();
        let now = Utc::now();
        let now_str = now.to_rfc3339();

        let sql = format!(
            "INSERT INTO contact_messages (id, name, email, message, is_read, created_at) VALUES ('{}', '{}', '{}', '{}', 0, '{}')",
            id_str,
            name.replace('\'', "''"),
            email.replace('\'', "''"),
            message.replace('\'', "''"),
            now_str
        );
        db.execute(&sql, ())?;

        Ok(ContactMessage {
            id,
            name: name.to_string(),
            email: email.to_string(),
            message: message.to_string(),
            is_read: false,
            created_at: now,
        })
    }

    pub async fn get_messages(&self) -> DbResult<Vec<ContactMessage>> {
        let db = self.conn.inner();
        let rows = db.query(
            "SELECT id, name, email, message, is_read, created_at FROM contact_messages ORDER BY created_at DESC",
            (),
        )?;

        let mut messages = Vec::new();
        for row_res in rows {
            let row = row_res?;
            let id_str: String = row.get(0)?;
            let name: String = row.get(1)?;
            let email: String = row.get(2)?;
            let message: String = row.get(3)?;
            let is_read: i64 = row.get(4)?;
            let created_at_str: String = row.get(5)?;

            let id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            messages.push(ContactMessage {
                id,
                name,
                email,
                message,
                is_read: is_read != 0,
                created_at,
            });
        }

        Ok(messages)
    }

    pub async fn mark_as_read(&self, id: Uuid) -> DbResult<()> {
        let db = self.conn.inner();
        let sql = format!("UPDATE contact_messages SET is_read = 1 WHERE id = '{}'", id);
        db.execute(&sql, ())?;
        Ok(())
    }
}

pub struct NewsletterRepository {
    conn: DatabaseConnection,
}

impl NewsletterRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn subscribe(&self, email: &str) -> DbResult<NewsletterSubscriber> {
        let db = self.conn.inner();
        let clean_email = email.trim().to_lowercase();
        let now = Utc::now();
        let now_str = now.to_rfc3339();

        // Check if exists
        let check_sql = format!(
            "SELECT id, email, is_active, subscribed_at FROM newsletter_subscribers WHERE email = '{}' LIMIT 1",
            clean_email.replace('\'', "''")
        );
        let mut existing = db.query(&check_sql, ())?;
        if let Some(row_res) = existing.next() {
            let row = row_res?;
            let id_str: String = row.get(0)?;
            let id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());

            // Re-activate if inactive
            let update_sql = format!("UPDATE newsletter_subscribers SET is_active = 1 WHERE id = '{}'", id_str);
            db.execute(&update_sql, ())?;

            return Ok(NewsletterSubscriber {
                id,
                email: clean_email,
                is_active: true,
                subscribed_at: now,
            });
        }

        // Insert new
        let id = Uuid::new_v4();
        let id_str = id.to_string();
        let insert_sql = format!(
            "INSERT INTO newsletter_subscribers (id, email, is_active, subscribed_at) VALUES ('{}', '{}', 1, '{}')",
            id_str,
            clean_email.replace('\'', "''"),
            now_str
        );
        db.execute(&insert_sql, ())?;

        Ok(NewsletterSubscriber {
            id,
            email: clean_email,
            is_active: true,
            subscribed_at: now,
        })
    }

    pub async fn get_subscribers(&self) -> DbResult<Vec<NewsletterSubscriber>> {
        let db = self.conn.inner();
        let rows = db.query(
            "SELECT id, email, is_active, subscribed_at FROM newsletter_subscribers WHERE is_active = 1 ORDER BY subscribed_at DESC",
            (),
        )?;

        let mut subs = Vec::new();
        for row_res in rows {
            let row = row_res?;
            let id_str: String = row.get(0)?;
            let email: String = row.get(1)?;
            let is_active: i64 = row.get(2)?;
            let subscribed_at_str: String = row.get(3)?;

            let id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
            let subscribed_at = DateTime::parse_from_rfc3339(&subscribed_at_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());

            subs.push(NewsletterSubscriber {
                id,
                email,
                is_active: is_active != 0,
                subscribed_at,
            });
        }

        Ok(subs)
    }
}

pub struct AnalyticsRepository {
    conn: DatabaseConnection,
}

impl AnalyticsRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn record_view(&self, slug: &str) -> DbResult<()> {
        let db = self.conn.inner();
        let now_str = Utc::now().to_rfc3339();
        let check_sql = format!(
            "SELECT id, view_count FROM page_views WHERE slug = '{}' LIMIT 1",
            slug.replace('\'', "''")
        );
        let mut existing = db.query(&check_sql, ())?;
        if let Some(row_res) = existing.next() {
            let row = row_res?;
            let id_str: String = row.get(0)?;
            let view_count: i64 = row.get(1)?;
            let update_sql = format!(
                "UPDATE page_views SET view_count = {}, last_viewed_at = '{}' WHERE id = '{}'",
                view_count + 1,
                now_str,
                id_str
            );
            db.execute(&update_sql, ())?;
            return Ok(());
        }

        let id = Uuid::new_v4().to_string();
        let insert_sql = format!(
            "INSERT INTO page_views (id, slug, view_count, last_viewed_at) VALUES ('{}', '{}', 1, '{}')",
            id,
            slug.replace('\'', "''"),
            now_str
        );
        db.execute(&insert_sql, ())?;
        Ok(())
    }

    pub async fn get_view_count(&self, slug: &str) -> DbResult<u64> {
        let db = self.conn.inner();
        let sql = format!(
            "SELECT view_count FROM page_views WHERE slug = '{}' LIMIT 1",
            slug.replace('\'', "''")
        );
        let mut rows = db.query(&sql, ())?;
        if let Some(row_res) = rows.next() {
            let row = row_res?;
            let count: i64 = row.get(0)?;
            return Ok(count as u64);
        }
        Ok(0)
    }

    pub async fn get_total_views(&self) -> DbResult<u64> {
        let db = self.conn.inner();
        let rows = db.query("SELECT view_count FROM page_views", ())?;
        let mut total = 0u64;
        for row_res in rows {
            let row = row_res?;
            let count: i64 = row.get(0)?;
            total += count as u64;
        }
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_in_memory_db;

    #[tokio::test]
    async fn test_stoolap_repositories_crud() {
        let conn = get_in_memory_db().expect("Failed to create in-memory Stoolap DB");

        // Test Contact
        let contact_repo = ContactRepository::new(conn.clone());
        let msg = contact_repo.save_message("Alice", "alice@example.com", "Hello from Stoolap!").await.unwrap();
        assert_eq!(msg.name, "Alice");
        let messages = contact_repo.get_messages().await.unwrap();
        assert_eq!(messages.len(), 1);

        // Test Newsletter
        let news_repo = NewsletterRepository::new(conn.clone());
        let sub = news_repo.subscribe("alice@example.com").await.unwrap();
        assert_eq!(sub.email, "alice@example.com");
        let subs = news_repo.get_subscribers().await.unwrap();
        assert_eq!(subs.len(), 1);

        // Test Articles
        let article_repo = ArticleRepository::new(conn.clone());
        let author_id = Uuid::new_v4();
        let art = article_repo.add_article(
            "test-post",
            "Test Post",
            "A test description",
            "# Heading",
            true,
            author_id,
        ).await.unwrap();
        assert_eq!(art.slug, "test-post");

        let fetched = article_repo.get_article_by_slug("test-post").await.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().title, "Test Post");

        // Test Analytics
        let analytics_repo = AnalyticsRepository::new(conn.clone());
        analytics_repo.record_view("test-post").await.unwrap();
        analytics_repo.record_view("test-post").await.unwrap();
        let count = analytics_repo.get_view_count("test-post").await.unwrap();
        assert_eq!(count, 2);
    }
}
