use sqlx::PgPool;

pub mod repositories;

pub async fn get_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(database_url).await
}
