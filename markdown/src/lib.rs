pub mod parser;

pub use parser::*;

pub type MarkdownParser = MarkdownService;

impl MarkdownService {
    pub fn to_html(md_content: &str) -> String {
        Self::parse_to_html(md_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_time_estimation() {
        let text_100_words = "word ".repeat(100);
        assert_eq!(MarkdownService::estimate_reading_time(&text_100_words), 1);

        let text_500_words = "word ".repeat(500);
        assert_eq!(MarkdownService::estimate_reading_time(&text_500_words), 3);
    }

    #[test]
    fn test_markdown_parsing_with_toc() {
        let md = r#"
# Heading One

[!NOTE] Remember to verify your invariants.

## Subheading Two

```rust
fn main() {
    println!("Hello World");
}
```
"#;
        let (html, toc, reading_time) = MarkdownService::parse_with_toc(md);
        assert!(!html.is_empty());
        assert_eq!(reading_time, 1);
        assert!(toc.iter().any(|t| t.title == "Heading One"));
        assert!(toc.iter().any(|t| t.title == "Subheading Two"));
        assert!(html.contains("callout-note"));
        assert!(html.contains("btn-run-code"));
    }
}
