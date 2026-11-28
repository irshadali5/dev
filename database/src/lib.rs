use std::sync::Arc;
use stoolap::Database;

pub mod repositories;

#[derive(Clone)]
pub struct DatabaseConnection {
    db: Arc<Database>,
}

impl DatabaseConnection {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn inner(&self) -> &Database {
        &self.db
    }
}

pub async fn get_db(db_path: &str) -> Result<DatabaseConnection, Box<dyn std::error::Error + Send + Sync>> {
    let normalized_path = db_path.trim();
    let db = if normalized_path == ":memory:" || normalized_path == "memory://" || normalized_path.is_empty() {
        Database::open_in_memory()?
    } else {
        // Strip legacy sqlite schemes if present
        let clean_path = if let Some(p) = normalized_path.strip_prefix("sqlite://") {
            p
        } else if let Some(p) = normalized_path.strip_prefix("sqlite:") {
            p
        } else if let Some(p) = normalized_path.strip_prefix("file://") {
            p
        } else {
            normalized_path
        };

        // Ensure target directory exists
        let path_obj = std::path::Path::new(clean_path);
        if let Some(parent) = path_obj.parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }

        let dsn = format!("file://{}", clean_path);
        Database::open(&dsn)?
    };

    let conn = DatabaseConnection::new(Arc::new(db));
    init_schema(&conn)?;
    Ok(conn)
}

pub fn get_in_memory_db() -> Result<DatabaseConnection, Box<dyn std::error::Error + Send + Sync>> {
    let db = Database::open_in_memory()?;
    let conn = DatabaseConnection::new(Arc::new(db));
    init_schema(&conn)?;
    Ok(conn)
}

fn init_schema(conn: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = conn.inner();

    db.execute(
        "CREATE TABLE IF NOT EXISTS articles (
            row_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            id TEXT NOT NULL,
            slug TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            body TEXT NOT NULL,
            published INTEGER NOT NULL,
            author_id TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            tags TEXT
        )",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            row_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            id TEXT NOT NULL,
            slug TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            content_html TEXT NOT NULL,
            github_url TEXT,
            live_url TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS books (
            row_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            id TEXT NOT NULL,
            slug TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            author_id TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS timeline_items (
            row_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            id TEXT NOT NULL,
            item_type TEXT NOT NULL,
            title TEXT NOT NULL,
            organization TEXT NOT NULL,
            period TEXT NOT NULL,
            description TEXT NOT NULL,
            order_index INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS skills (
            row_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            id TEXT NOT NULL,
            category TEXT NOT NULL,
            name TEXT NOT NULL,
            order_index INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS contact_messages (
            row_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            id TEXT NOT NULL,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            message TEXT NOT NULL,
            is_read INTEGER NOT NULL,
            created_at TEXT NOT NULL
        )",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS newsletter_subscribers (
            row_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            id TEXT NOT NULL,
            email TEXT NOT NULL,
            is_active INTEGER NOT NULL,
            subscribed_at TEXT NOT NULL
        )",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS page_views (
            row_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            id TEXT NOT NULL,
            slug TEXT NOT NULL,
            view_count INTEGER NOT NULL,
            last_viewed_at TEXT NOT NULL
        )",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS users (
            row_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            id TEXT NOT NULL,
            email TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        (),
    )?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS article_likes (
            row_id INTEGER PRIMARY KEY AUTO_INCREMENT,
            slug TEXT NOT NULL,
            likes_count INTEGER NOT NULL
        )",
        (),
    )?;

    seed_initial_data(conn)?;

    Ok(())
}

#[derive(serde::Deserialize)]
struct ResumeSeedTimelineItem {
    item_type: String,
    title: String,
    organization: String,
    period: String,
    description: String,
    order_index: i32,
}

#[derive(serde::Deserialize)]
struct ResumeSeedSkill {
    category: String,
    name: String,
    order_index: i32,
}

#[derive(serde::Deserialize)]
struct ResumeSeedData {
    #[serde(default)]
    timeline: Vec<ResumeSeedTimelineItem>,
    #[serde(default)]
    skills: Vec<ResumeSeedSkill>,
}

fn load_resume_seed() -> Option<ResumeSeedData> {
    let candidates = [
        "content/resume.ron",
        "../content/resume.ron",
        "../../content/resume.ron",
    ];
    for path in candidates {
        if let Ok(raw) = std::fs::read_to_string(path) {
            if let Ok(parsed) = ron::from_str::<ResumeSeedData>(&raw) {
                return Some(parsed);
            }
        }
    }
    None
}

fn seed_initial_data(conn: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = conn.inner();
    let Some(seed) = load_resume_seed() else {
        return Ok(());
    };

    // Check if timeline_items is empty
    let timeline_count = match db.query("SELECT count(*) FROM timeline_items", ()) {
        Ok(mut rows) => {
            if let Some(Ok(row)) = rows.next() {
                row.get::<i64>(0).unwrap_or(0)
            } else {
                0
            }
        }
        Err(_) => 0,
    };

    if timeline_count == 0 {
        let now = chrono::Utc::now().to_rfc3339();
        for item in &seed.timeline {
            let id = uuid::Uuid::new_v4().to_string();
            let sql = format!(
                "INSERT INTO timeline_items (id, item_type, title, organization, period, description, order_index, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', {}, '{}', '{}')",
                id,
                item.item_type.replace('\'', "''"),
                item.title.replace('\'', "''"),
                item.organization.replace('\'', "''"),
                item.period.replace('\'', "''"),
                item.description.replace('\'', "''"),
                item.order_index,
                now,
                now
            );
            let _ = db.execute(&sql, ());
        }
    }

    // Check if skills is empty
    let skills_count = match db.query("SELECT count(*) FROM skills", ()) {
        Ok(mut rows) => {
            if let Some(Ok(row)) = rows.next() {
                row.get::<i64>(0).unwrap_or(0)
            } else {
                0
            }
        }
        Err(_) => 0,
    };

    if skills_count == 0 {
        let now = chrono::Utc::now().to_rfc3339();
        for skill in &seed.skills {
            let id = uuid::Uuid::new_v4().to_string();
            let sql = format!(
                "INSERT INTO skills (id, category, name, order_index, created_at, updated_at) VALUES ('{}', '{}', '{}', {}, '{}', '{}')",
                id,
                skill.category.replace('\'', "''"),
                skill.name.replace('\'', "''"),
                skill.order_index,
                now,
                now
            );
            let _ = db.execute(&sql, ());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_db_dsn_normalization() {
        let db_mem = get_db(":memory:").await;
        assert!(db_mem.is_ok());

        let db_sqlite = get_db("sqlite:dev_site.db").await;
        assert!(db_sqlite.is_ok());

        let db_plain = get_db("data/test_dsn.stoolap").await;
        assert!(db_plain.is_ok());

        let db_file_uri = get_db("file://data/test_dsn_uri.stoolap").await;
        assert!(db_file_uri.is_ok());

        // Cleanup
        let _ = std::fs::remove_file("data/test_dsn.stoolap");
        let _ = std::fs::remove_file("data/test_dsn_uri.stoolap");
        let _ = std::fs::remove_file("dev_site.db");
    }
}
