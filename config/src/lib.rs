use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub engine: String,
    pub path: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            engine: "stoolap".to_string(),
            path: "data/dev_site.stoolap".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub title: String,
    pub tagline: String,
    pub author: String,
    pub email: String,
    pub bio: String,
    pub site_url: String,
    pub github_url: String,
    pub linkedin_url: String,
    pub twitter_url: Option<String>,
    #[serde(default)]
    pub database: DatabaseConfig,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: "Irshad - Software Engineer & Systems Architect".to_string(),
            tagline: "High-Performance Distributed Systems, Rust, and Cloud Architecture".to_string(),
            author: "Irshad Ali".to_string(),
            email: "contact@irshadali.org".to_string(),
            bio: "Systems architect building resilient distributed backends, developer tooling, and modern web applications with Rust.".to_string(),
            site_url: "https://irshadali.org".to_string(),
            github_url: "https://github.com/irshadali5".to_string(),
            linkedin_url: "https://linkedin.com/in/irshadali5".to_string(),
            twitter_url: Some("https://twitter.com/irshadali5".to_string()),
            database: DatabaseConfig::default(),
        }
    }
}

impl SiteConfig {
    pub fn load() -> Self {
        let config_path = Path::new("content/site_config.ron");
        if config_path.exists() {
            if let Ok(content) = fs::read_to_string(config_path) {
                if let Ok(config) = ron::from_str::<SiteConfig>(&content) {
                    return config;
                }
            }
        }
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_site_config_ron_roundtrip() {
        let config = SiteConfig::default();
        let ron_str = ron::ser::to_string_pretty(&config, ron::ser::PrettyConfig::default()).expect("RON serialization failed");
        let parsed: SiteConfig = ron::from_str(&ron_str).expect("RON deserialization failed");
        assert_eq!(config.title, parsed.title);
        assert_eq!(config.database.engine, "stoolap");
    }
}
