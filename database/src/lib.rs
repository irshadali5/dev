use sqlx::SqlitePool;

pub mod repositories;

pub async fn get_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    SqlitePool::connect(database_url).await
}
