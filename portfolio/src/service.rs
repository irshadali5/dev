use database::repositories::ProjectRepository;
use sqlx::PgPool;
use domain::models::Project;

pub struct PortfolioService {
    pool: PgPool,
}

impl PortfolioService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_projects(&self) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
        let repo = ProjectRepository::new(self.pool.clone());
        let projects = repo.get_all_projects().await?;
        Ok(projects)
    }
}
