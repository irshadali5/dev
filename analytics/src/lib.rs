use sqlx::PgPool;
use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PageView {
    pub slug: String,
    pub view_count: i64,
}

#[derive(Error, Debug)]
pub enum AnalyticsError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub struct AnalyticsService {
    pool: PgPool,
}

impl AnalyticsService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Records a page view for a given slug.
    pub async fn record_view(&self, slug: &str) -> Result<(), AnalyticsError> {
        sqlx::query!(
            r#"
            INSERT INTO page_views (slug, view_count, last_viewed_at)
            VALUES ($1, 1, CURRENT_TIMESTAMP)
            ON CONFLICT (slug) DO UPDATE
            SET view_count = page_views.view_count + 1,
                last_viewed_at = CURRENT_TIMESTAMP
            "#,
            slug
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Gets the view count for a given slug.
    pub async fn get_view_count(&self, slug: &str) -> Result<i64, AnalyticsError> {
        let record = sqlx::query!(
            r#"
            SELECT view_count FROM page_views WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(|r| r.view_count).unwrap_or(0))
    }

    /// Gets all page views ordered by view count descending.
    pub async fn get_all_views(&self) -> Result<Vec<PageView>, AnalyticsError> {
        let records = sqlx::query!(
            r#"
            SELECT slug, view_count FROM page_views ORDER BY view_count DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let views = records.into_iter().map(|r| PageView {
            slug: r.slug,
            view_count: r.view_count,
        }).collect();

        Ok(views)
    }
}
