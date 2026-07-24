use pulldown_cmark::{html, Options, Parser};
use ron::de::from_str;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MarkdownError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("RON Parse Error: {0}")]
    RonParse(#[from] ron::error::SpannedError),
    #[error("Missing RON Metadata file")]
    MissingMetadata,
}

pub struct ParsedContent<T> {
    pub metadata: T,
    pub html_content: String,
}

pub struct MarkdownService;

impl MarkdownService {
    /// Reads a `.md` file and its sibling `.ron` metadata file.
    pub fn parse_file<T: DeserializeOwned>(
        dir_path: &Path,
        file_stem: &str,
    ) -> Result<ParsedContent<T>, MarkdownError> {
        let md_path = dir_path.join(format!("{}.md", file_stem));
        let ron_path = dir_path.join(format!("{}.ron", file_stem));

        if !ron_path.exists() {
            return Err(MarkdownError::MissingMetadata);
        }

        let md_content = fs::read_to_string(md_path)?;
        let ron_content = fs::read_to_string(ron_path)?;

        let metadata: T = from_str(&ron_content)?;

        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

        let parser = Parser::new_ext(&md_content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Ok(ParsedContent {
            metadata,
            html_content: html_output,
        })
    }

    pub fn parse_to_html(md_content: &str) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

        let parser = Parser::new_ext(md_content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}
