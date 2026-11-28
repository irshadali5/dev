pub mod models;
pub mod service;

pub use models::*;
pub use service::*;

/// Pure-Rust HTML to GitHub-Flavored Markdown converter used by offline book importers.
pub fn html_to_markdown(html: &str) -> String {
    use regex::Regex;

    let mut out = html.to_string();

    // Preformatted code blocks first
    let pre_re = Regex::new(r#"(?is)<pre[^>]*>\s*<code(?:\s+class="language-([^"]*)")?[^>]*>(.*?)</code>\s*</pre>"#).unwrap();
    out = pre_re
        .replace_all(&out, |caps: &regex::Captures| {
            let lang = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let code = unescape_html(caps.get(2).map(|m| m.as_str()).unwrap_or(""));
            format!("\n```{}\n{}\n```\n\n", lang, code.trim_matches('\n'))
        })
        .into_owned();

    // Headings
    for (tag, prefix) in [("h1", "# "), ("h2", "## "), ("h3", "### "), ("h4", "#### ")] {
        let re = Regex::new(&format!(r#"(?is)<{tag}[^>]*>(.*?)</{tag}>"#)).unwrap();
        out = re
            .replace_all(&out, |caps: &regex::Captures| {
                format!("\n{prefix}{}\n\n", caps[1].trim())
            })
            .into_owned();
    }

    // Links
    let link_re = Regex::new(r#"(?is)<a\s+[^>]*href="([^"]*)"[^>]*>(.*?)</a>"#).unwrap();
    out = link_re
        .replace_all(&out, |caps: &regex::Captures| {
            format!("[{}]({})", caps[2].trim(), &caps[1])
        })
        .into_owned();

    // Inline formatting & lists
    for (pat, rep) in [
        (r#"(?is)<code[^>]*>(.*?)</code>"#, "`$1`"),
        (r#"(?is)<(?:strong|b)[^>]*>(.*?)</(?:strong|b)>"#, "**$1**"),
        (r#"(?is)<(?:em|i)[^>]*>(.*?)</(?:em|i)>"#, "*$1*"),
        (r#"(?is)<li[^>]*>(.*?)</li>"#, "- $1\n"),
        (r#"(?is)<p[^>]*>(.*?)</p>"#, "$1\n\n"),
        (r#"(?is)<br\s*/?>"#, "\n"),
    ] {
        let re = Regex::new(pat).unwrap();
        out = re.replace_all(&out, rep).into_owned();
    }

    // Strip any remaining HTML tags
    let tag_re = Regex::new(r#"(?is)<[^>]+>"#).unwrap();
    out = tag_re.replace_all(&out, "").into_owned();

    unescape_html(&out).trim().to_string()
}

fn unescape_html(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_to_markdown() {
        let html = r#"<h1>Title</h1><p>Hello <strong>world</strong> and <code>rust</code>.</p><pre><code class="language-rust">fn main() {}</code></pre>"#;
        let md = html_to_markdown(html);
        assert!(md.contains("# Title"));
        assert!(md.contains("Hello **world** and `rust`."));
        assert!(md.contains("```rust\nfn main() {}\n```"));
    }
}
