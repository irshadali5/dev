use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub body: String, // This will store the HTML or MD content
    pub published: bool,
    pub author_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    // We can populate tags manually if needed
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub content_html: String,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub architecture: Option<String>,
    #[serde(default)]
    pub benchmarks: Option<String>,
    #[serde(default)]
    pub lessons_learned: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub author_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMeta {
    pub slug: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: String,
    pub author: String,
    pub published: bool,
    pub tags: Vec<String>,
    pub created_at: String,
    pub chapters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineItem {
    pub id: Uuid,
    pub item_type: String,
    pub title: String,
    pub organization: String,
    pub period: String,
    pub description: String,
    pub order_index: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: Uuid,
    pub category: String,
    pub name: String,
    pub order_index: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactMessage {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub message: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsletterSubscriber {
    pub id: Uuid,
    pub email: String,
    pub is_active: bool,
    pub subscribed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaygroundResponse {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct GithubActivityItem {
    pub repo: String,
    pub event_type: String,
    pub message: String,
    pub created_at: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct BookmarkItem {
    pub slug: String,
    pub title: String,
    pub doc_type: String,
    pub url: String,
    pub timestamp: String,
}
