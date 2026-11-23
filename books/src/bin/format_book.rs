use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
struct Frontmatter {
    title: String,
    description: String,
    order: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct BookMeta {
    slug: String,
    title: String,
    subtitle: Option<String>,
    description: String,
    author: String,
    published: bool,
    tags: Vec<String>,
    created_at: String,
    chapters: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source_dir = Path::new("/home/irshad/projects/02prod/webapp/src/content/chapters");
    let target_dir = Path::new("/home/irshad/Projects/dev/content/books/production-grade-rust");

    println!("Starting book content copy & formatting...");
    println!("Source: {}", source_dir.display());
    println!("Target: {}", target_dir.display());

    if !source_dir.exists() {
        return Err(format!("Source directory does not exist: {}", source_dir.display()).into());
    }

    fs::create_dir_all(target_dir)?;

    let mut entries: Vec<PathBuf> = fs::read_dir(source_dir)?
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().is_some_and(|ext| ext == "md"))
        .collect();

    entries.sort();

    let mut chapter_filenames: Vec<String> = Vec::new();

    // Regexes for cleaning markdown escapes
    let re_escaped_filename = Regex::new(r"\\\*\\\*\\`([^`]+)\\`\\\*\\\*")?;
    let re_escaped_alert = Regex::new(r">\s*\\\[!(TIP|IMPORTANT|NOTE|WARNING|CAUTION)\\\]")?;
    let re_escaped_blockquote = Regex::new(r"(?m)^\\>\s*")?;
    let re_escaped_attr = Regex::new(r"#\\\[")?;
    let re_escaped_underscore = Regex::new(r"\\_")?;
    let re_escaped_asterisk = Regex::new(r"\\\*")?;
    let re_escaped_brackets_open = Regex::new(r"\\\[")?;
    let re_escaped_brackets_close = Regex::new(r"\\\]")?;

    for entry in &entries {
        let filename = entry.file_name().unwrap().to_str().unwrap().to_string();
        println!("Processing chapter: {}", filename);

        let raw_content = fs::read_to_string(entry)?;

        let (frontmatter_str, body_str) = parse_frontmatter_and_body(&raw_content);

        let fm: Frontmatter = parse_yaml_frontmatter(&frontmatter_str)
            .unwrap_or_else(|_| Frontmatter {
                title: filename.replace(".md", ""),
                description: String::new(),
                order: extract_order_from_filename(&filename),
            });

        let cleaned_body = clean_markdown_body(
            &body_str,
            &re_escaped_filename,
            &re_escaped_alert,
            &re_escaped_blockquote,
            &re_escaped_attr,
            &re_escaped_underscore,
            &re_escaped_asterisk,
            &re_escaped_brackets_open,
            &re_escaped_brackets_close,
        );

        let clean_markdown = format!(
            "---\ntitle: \"{}\"\ndescription: \"{}\"\norder: {}\n---\n\n{}",
            escape_quotes(&fm.title),
            escape_quotes(&fm.description),
            fm.order,
            cleaned_body.trim()
        );

        let target_path = target_dir.join(&filename);
        fs::write(&target_path, clean_markdown)?;
        chapter_filenames.push(filename);
    }

    // Generate book.ron
    let book_meta = BookMeta {
        slug: "production-grade-rust".to_string(),
        title: "Zero to Production in Rust".to_string(),
        subtitle: Some("An Opinionated Guide to Backend Development".to_string()),
        description: "Learn how to build production-grade backend services in Rust with Axum, SQLx, Tokio, background workers, telemetry, and deployment pipelines.".to_string(),
        author: "Irshad Ali".to_string(),
        published: true,
        tags: vec![
            "Rust".to_string(),
            "Backend".to_string(),
            "Axum".to_string(),
            "SQLx".to_string(),
            "Tokio".to_string(),
            "Production".to_string(),
            "Architecture".to_string(),
        ],
        created_at: "2026-06-01".to_string(),
        chapters: chapter_filenames.clone(),
    };

    let ron_config = ron::ser::PrettyConfig::default();
    let ron_str = ron::ser::to_string_pretty(&book_meta, ron_config)?;
    let ron_path = target_dir.join("book.ron");
    fs::write(&ron_path, ron_str)?;

    println!("\nSuccessfully processed {} chapters!", chapter_filenames.len());
    println!("Output book metadata: {}", ron_path.display());
    println!("Output chapter files located at: {}", target_dir.display());

    Ok(())
}

fn parse_frontmatter_and_body(content: &str) -> (String, String) {
    if let Some(stripped) = content.strip_prefix("---") {
        if let Some(end_idx) = stripped.find("---") {
            let fm = &stripped[..end_idx];
            let body = &stripped[end_idx + 3..];
            return (fm.trim().to_string(), body.trim().to_string());
        }
    }
    (String::new(), content.trim().to_string())
}

fn parse_yaml_frontmatter(fm_str: &str) -> Result<Frontmatter, String> {
    let mut title = String::new();
    let mut description = String::new();
    let mut order = 0u32;

    for line in fm_str.lines() {
        let line = line.trim();
        if let Some(val) = line.strip_prefix("title:") {
            title = val.trim().trim_matches('"').to_string();
        } else if let Some(val) = line.strip_prefix("description:") {
            description = val.trim().trim_matches('"').to_string();
        } else if let Some(val) = line.strip_prefix("order:") {
            if let Ok(ord) = val.trim().parse::<u32>() {
                order = ord;
            }
        }
    }

    Ok(Frontmatter {
        title,
        description,
        order,
    })
}

fn extract_order_from_filename(filename: &str) -> u32 {
    let parts: Vec<&str> = filename.split('-').collect();
    if !parts.is_empty() {
        if let Ok(n) = parts[0].parse::<u32>() {
            return n;
        }
    }
    0
}

fn escape_quotes(s: &str) -> String {
    s.replace('"', "\\\"")
}

#[allow(clippy::too_many_arguments)]
fn clean_markdown_body(
    body: &str,
    re_filename: &Regex,
    re_alert: &Regex,
    re_blockquote: &Regex,
    re_attr: &Regex,
    re_underscore: &Regex,
    re_asterisk: &Regex,
    re_brackets_open: &Regex,
    re_brackets_close: &Regex,
) -> String {
    // 1. Strip repetitive tail appendix if present
    let content = if let Some(idx) = body.find("## Exhaustive Implementation Appendix") {
        &body[..idx]
    } else {
        body
    };

    // 2. Clean escapes line-by-line / block-by-block
    let mut lines: Vec<String> = Vec::new();

    let mut in_mermaid = false;

    for line in content.lines() {
        let mut l = line.to_string();

        // Fix escaped filename headers **`filename`**
        l = re_filename.replace_all(&l, "**`$1`**").to_string();

        // Fix alerts > [!TIP]
        l = re_alert.replace_all(&l, "> [!$1]").to_string();

        // Fix blockquote markers
        l = re_blockquote.replace_all(&l, "> ").to_string();

        // Fix attribute macros #[async_trait]
        l = re_attr.replace_all(&l, "#[").to_string();

        // Fix mermaid graph declarations outside fenced code blocks
        if (l.trim().starts_with("graph TD")
            || l.trim().starts_with("flowchart TD")
            || l.trim().starts_with("sequenceDiagram"))
            && !in_mermaid
        {
            lines.push("```mermaid".to_string());
            in_mermaid = true;
        }

        // Unescape escaped underscores and asterisks outside raw code blocks
        if !l.trim().starts_with("```") {
            l = re_underscore.replace_all(&l, "_").to_string();
            l = re_asterisk.replace_all(&l, "*").to_string();
            l = re_brackets_open.replace_all(&l, "[").to_string();
            l = re_brackets_close.replace_all(&l, "]").to_string();
        }

        if in_mermaid && l.trim().is_empty() {
            lines.push("```".to_string());
            in_mermaid = false;
        }


        lines.push(l);
    }

    if in_mermaid {
        lines.push("```".to_string());
    }

    // Join and deduplicate blank lines
    let result = lines.join("\n");
    let re_multi_newline = Regex::new(r"\n{3,}").unwrap();
    re_multi_newline.replace_all(&result, "\n\n").to_string()
}
