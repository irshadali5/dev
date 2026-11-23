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

fn seed_initial_data(conn: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db = conn.inner();

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
        let items = [
            ("experience", "Lead Systems Architect", "Distributed Cloud Systems", "2023 - Present", "Architecting high-throughput distributed database engines, zero-copy network reverse proxies, and low-latency asynchronous microservices using Rust, Tokio, and C++. Leading cloud native migrations and fault-tolerant Raft replication clusters.", 1),
            ("experience", "Senior Backend & Infrastructure Engineer", "Scalable Systems Corp", "2021 - 2023", "Built high-concurrency event-driven processing pipelines handling 50k+ QPS with sub-15ms p99 latency using Rust, PostgreSQL, Kafka, and Redis. Engineered transactional outbox patterns and distributed consensus primitives.", 2),
            ("experience", "Software Engineer - Systems & Platforms", "Core Engineering Labs", "2019 - 2021", "Developed high-performance Linux kernel integration tooling, eBPF telemetry collectors, and WebAssembly serverless execution runtimes with strict capability-based isolation.", 3),
            ("certification", "AWS Certified Solutions Architect – Professional", "Amazon Web Services (AWS)", "2024", "Advanced cloud architecture, multi-region distributed disaster recovery, high-availability VPC networks, and low-latency infrastructure design.", 4),
            ("certification", "Certified Kubernetes Administrator (CKA)", "Cloud Native Computing Foundation (CNCF)", "2023", "Cluster lifecycle management, etcd backup/restore, custom controllers, multi-tenant network policies, and production ingress scaling.", 5),
            ("certification", "Rust Systems & Concurrency Specialization", "Linux Foundation", "2023", "Zero-cost abstractions, unsafe memory invariants, atomics, lock-free data structures, and SIMD hardware acceleration.", 6),
            ("education", "Bachelor of Technology in Computer Science & Engineering", "National Institute of Technology", "2015 - 2019", "Core coursework in Distributed Operating Systems, Database Engine Theory, Algorithms & Data Structures, Advanced Computer Networks, and Compiler Construction. Graduated with Honors.", 7),
        ];

        let now = chrono::Utc::now().to_rfc3339();
        for (itype, title, org, period, desc, order) in items {
            let id = uuid::Uuid::new_v4().to_string();
            let sql = format!(
                "INSERT INTO timeline_items (id, item_type, title, organization, period, description, order_index, created_at, updated_at) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', {}, '{}', '{}')",
                id, itype, title.replace('\'', "''"), org.replace('\'', "''"), period.replace('\'', "''"), desc.replace('\'', "''"), order, now, now
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
        let skills = [
            ("Languages & Core", "Rust (Advanced Systems)", 1),
            ("Languages & Core", "C++20 / C", 2),
            ("Languages & Core", "Go", 3),
            ("Languages & Core", "SQL / Database Internals", 4),
            ("Languages & Core", "WebAssembly / WASI", 5),
            ("Languages & Core", "TypeScript / Node.js", 6),
            ("Distributed Systems & Storage", "Stoolap (Embedded MVCC)", 1),
            ("Distributed Systems & Storage", "Raft Consensus & Linearizability", 2),
            ("Distributed Systems & Storage", "PostgreSQL & Query Optimization", 3),
            ("Distributed Systems & Storage", "Kafka & Event-Driven CQRS", 4),
            ("Distributed Systems & Storage", "Redis & In-Memory Caching", 5),
            ("Distributed Systems & Storage", "Tantivy Full-Text Search", 6),
            ("Networking & Performance", "Linux io_uring & epoll", 1),
            ("Networking & Performance", "Zero-Copy TCP Splicing", 2),
            ("Networking & Performance", "SIMD (AVX-512 / NEON) Acceleration", 3),
            ("Networking & Performance", "gRPC & Protocol Buffers", 4),
            ("Networking & Performance", "Async Tokio Actor Concurrency", 5),
            ("Cloud, DevOps & Security", "Kubernetes & Cloud-Native", 1),
            ("Cloud, DevOps & Security", "Kiri (Rust Container Runtime) & Sandboxing", 2),
            ("Cloud, DevOps & Security", "Linux eBPF & Observability", 3),
            ("Cloud, DevOps & Security", "AWS Distributed Infrastructure", 4),
            ("Cloud, DevOps & Security", "Prometheus Telemetry & Grafana", 5),
            ("Cloud, DevOps & Security", "Automated CI/CD Workflows", 6),
        ];

        let now = chrono::Utc::now().to_rfc3339();
        for (category, name, order) in skills {
            let id = uuid::Uuid::new_v4().to_string();
            let sql = format!(
                "INSERT INTO skills (id, category, name, order_index, created_at, updated_at) VALUES ('{}', '{}', '{}', {}, '{}', '{}')",
                id, category.replace('\'', "''"), name.replace('\'', "''"), order, now, now
            );
            let _ = db.execute(&sql, ());
        }
    } else {
        // Automatically migrate any legacy Docker entry to Kiri
        let _ = db.execute(
            "UPDATE skills SET name = 'Kiri (Rust Container Runtime) & Sandboxing' WHERE name LIKE '%Docker%'",
            (),
        );
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
