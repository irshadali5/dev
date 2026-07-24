use crate::models::{Project, ProjectMetadata};
use markdown::parser::MarkdownService;
use std::fs;
use std::path::{Path, PathBuf};
use moka::sync::Cache;

pub struct PortfolioService {
    content_dir: PathBuf,
    cache: Cache<String, Vec<Project>>,
}

impl PortfolioService {
    pub fn new(content_dir: PathBuf) -> Self {
        let cache = Cache::builder()
            .max_capacity(10) // Small capacity since we just store the whole list
            .time_to_live(std::time::Duration::from_secs(60 * 60))
            .build();
        Self { content_dir, cache }
    }

    pub fn get_all_projects(&self) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
        if let Some(cached) = self.cache.get("all_projects") {
            return Ok(cached);
        }

        let projects_dir = self.content_dir.join("projects");
        let mut projects = Vec::new();

        if !projects_dir.exists() {
            return Ok(projects);
        }

        for entry in fs::read_dir(projects_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                let parsed = MarkdownService::parse_file::<ProjectMetadata>(
                    &self.content_dir.join("projects"),
                    file_stem,
                )?;

                projects.push(Project {
                    metadata: parsed.metadata,
                    content_html: parsed.html_content,
                });
            }
        }

        // Sort by created_at descending
        projects.sort_by(|a, b| b.metadata.created_at.cmp(&a.metadata.created_at));

        self.cache.insert("all_projects".to_string(), projects.clone());

        Ok(projects)
    }
}
