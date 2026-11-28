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
    #[serde(default = "default_short_name")]
    pub short_name: String,
    pub tagline: String,
    pub author: String,
    #[serde(default = "default_handle")]
    pub handle: String,
    #[serde(default = "default_role")]
    pub role: String,
    pub email: String,
    pub bio: String,
    pub site_url: String,
    pub github_url: String,
    pub linkedin_url: String,
    pub twitter_url: Option<String>,
    #[serde(default)]
    pub twitter_handle: Option<String>,
    #[serde(default)]
    pub database: DatabaseConfig,
}

fn default_short_name() -> String {
    "Dev".to_string()
}

fn default_handle() -> String {
    "dev".to_string()
}

fn default_role() -> String {
    "Software Engineer".to_string()
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: "Developer Portfolio & Blog".to_string(),
            short_name: default_short_name(),
            tagline: "Software Engineering, Systems, and Writing".to_string(),
            author: "Your Name".to_string(),
            handle: default_handle(),
            role: default_role(),
            email: "hello@example.com".to_string(),
            bio: "Software engineer building reliable systems, developer tools, and web applications.".to_string(),
            site_url: "https://example.com".to_string(),
            github_url: "https://github.com/username".to_string(),
            linkedin_url: "https://linkedin.com/in/username".to_string(),
            twitter_url: None,
            twitter_handle: None,
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

    #[test]
    fn test_site_config_loads_workspace_ron() {
        let path = Path::new("../content/site_config.ron");
        if path.exists() {
            let raw = fs::read_to_string(path).expect("Failed to read site_config.ron");
            let parsed: SiteConfig = ron::from_str(&raw).expect("Failed to parse site_config.ron");
            assert!(!parsed.author.is_empty());
            assert!(!parsed.handle.is_empty());
        }
    }
}
