use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BookError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("RON Error: {0}")]
    Ron(#[from] ron::error::SpannedError),

    #[error("Parse Error: {0}")]
    Parse(String),

    #[error("Book Not Found: {0}")]
    NotFound(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChapterMeta {
    pub title: String,
    pub description: Option<String>,
    pub order: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub slug: String,
    pub meta: ChapterMeta,
    pub markdown: String,
    pub html: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullBook {
    pub meta: BookMeta,
    pub chapters: Vec<Chapter>,
}
