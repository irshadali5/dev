use database::repositories::ProjectRepository;
use sqlx::SqlitePool;
use domain::models::Project;

pub struct PortfolioService {
    pool: SqlitePool,
}

impl PortfolioService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_all_projects(&self) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
        let repo = ProjectRepository::new(self.pool.clone());
        let projects = repo.get_all_projects().await?;
        Ok(projects)
    }
}
