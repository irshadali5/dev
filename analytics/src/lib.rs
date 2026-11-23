use database::repositories::AnalyticsRepository;
use database::DatabaseConnection;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PageView {
    pub slug: String,
    pub view_count: i64,
}

#[derive(Error, Debug)]
pub enum AnalyticsError {
    #[error("Database error: {0}")]
    Database(String),
}

pub struct AnalyticsService {
    repo: AnalyticsRepository,
}

impl AnalyticsService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self {
            repo: AnalyticsRepository::new(conn),
        }
    }

    /// Records a page view for a given slug.
    pub async fn record_view(&self, slug: &str) -> Result<(), AnalyticsError> {
        self.repo
            .record_view(slug)
            .await
            .map_err(|e| AnalyticsError::Database(e.to_string()))
    }

    /// Gets the view count for a given slug.
    pub async fn get_view_count(&self, slug: &str) -> Result<i64, AnalyticsError> {
        self.repo
            .get_view_count(slug)
            .await
            .map(|c| c as i64)
            .map_err(|e| AnalyticsError::Database(e.to_string()))
    }

    /// Gets total page views across all pages.
    pub async fn get_total_views(&self) -> Result<u64, AnalyticsError> {
        self.repo
            .get_total_views()
            .await
            .map_err(|e| AnalyticsError::Database(e.to_string()))
    }
}
