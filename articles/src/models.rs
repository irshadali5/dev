use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleMetadata {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub published: bool,
    pub tags: Vec<String>,
    pub created_at: String, // Kept as String for simplicity, or chrono::DateTime
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub metadata: ArticleMetadata,
    pub content_html: String,
}
