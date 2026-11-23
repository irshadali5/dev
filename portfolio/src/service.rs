use database::repositories::ProjectRepository;
use database::DatabaseConnection;
use domain::models::Project;
use crate::models::ProjectMetadata;
use markdown::MarkdownService;
use std::fs;
use std::path::Path;

pub struct PortfolioService {
    conn: DatabaseConnection,
}

impl PortfolioService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn get_all_projects(&self) -> Result<Vec<Project>, Box<dyn std::error::Error + Send + Sync>> {
        let repo = ProjectRepository::new(self.conn.clone());
        let mut projects = repo.get_all_projects().await?;
        
        // Discover local project markdown files in content/projects/ or ../content/projects/
        let possible_dirs = [
            Path::new("content/projects"),
            Path::new("../content/projects"),
            Path::new("../../content/projects"),
        ];
        let projects_dir = possible_dirs.iter().find(|p| p.exists());

        if let Some(dir) = projects_dir {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("ron") {
                        if let Ok(ron_str) = fs::read_to_string(&path) {
                            if let Ok(meta) = ron::from_str::<ProjectMetadata>(&ron_str) {
                                if !projects.iter().any(|p| p.slug == meta.slug) {
                                    let md_path = path.with_extension("md");
                                    let content_html = if md_path.exists() {
                                        let md_raw = fs::read_to_string(md_path).unwrap_or_default();
                                        MarkdownService::parse_to_html(&md_raw)
                                    } else {
                                        String::new()
                                    };

                                    projects.push(Project {
                                        id: uuid::Uuid::new_v4(),
                                        slug: meta.slug,
                                        title: meta.title,
                                        description: meta.description,
                                        content_html,
                                        github_url: meta.github_url,
                                        live_url: meta.live_url,
                                        created_at: meta.created_at,
                                        updated_at: meta.created_at,
                                        tags: meta.tags,
                                        architecture: meta.architecture,
                                        benchmarks: meta.benchmarks,
                                        lessons_learned: meta.lessons_learned,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(projects)
    }

    pub async fn get_project_by_slug(&self, slug: &str) -> Result<Option<Project>, Box<dyn std::error::Error + Send + Sync>> {
        let all = self.get_all_projects().await?;
        Ok(all.into_iter().find(|p| p.slug == slug))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_portfolio_service_loads_all_projects() {
        let db = database::get_db(":memory:").await.expect("Failed to create in-memory db");
        let service = PortfolioService::new(db);
        let projects = service.get_all_projects().await.expect("Failed to get projects");
        
        assert!(projects.len() >= 6, "Expected at least 6 projects, found {}", projects.len());

        let stoolap = projects.iter().find(|p| p.slug == "stoolap-engine");
        assert!(stoolap.is_some(), "stoolap-engine project should exist");
        let stoolap = stoolap.unwrap();
        assert!(stoolap.architecture.is_some(), "stoolap-engine architecture should be set");
        assert!(stoolap.benchmarks.is_some(), "stoolap-engine benchmarks should be set");
        assert!(!stoolap.content_html.is_empty(), "stoolap-engine case study HTML should not be empty");

        let raft = projects.iter().find(|p| p.slug == "raft-kv-distributed");
        assert!(raft.is_some(), "raft-kv-distributed project should exist");

        let proxy = projects.iter().find(|p| p.slug == "zero-copy-proxy");
        assert!(proxy.is_some(), "zero-copy-proxy project should exist");

        let wasm = projects.iter().find(|p| p.slug == "wasm-edge-runtime");
        assert!(wasm.is_some(), "wasm-edge-runtime project should exist");

        let vector = projects.iter().find(|p| p.slug == "vector-simd-search");
        assert!(vector.is_some(), "vector-simd-search project should exist");

        let ecommerce = projects.iter().find(|p| p.slug == "rust-ecommerce");
        assert!(ecommerce.is_some(), "rust-ecommerce project should exist");
    }
}
