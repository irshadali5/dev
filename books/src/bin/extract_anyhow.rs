use std::fs;
use std::process::Command;

fn extract_title(html: &str) -> String {
    if let Some(start) = html.find("<h1>") {
        if let Some(end) = html[start..].find("</h1>") {
            return html[start + 4..start + end].trim().to_string();
        }
    }
    "Untitled".to_string()
}

fn convert_callouts(html: &str) -> String {
    let mut out = html.to_string();
    
    out = out.replace("<div class=\"callout callout-info\">", "<div class=\"callout callout-info\">\n\n> [!NOTE]\n> ");
    out = out.replace("<div class=\"callout callout-tip\">", "<div class=\"callout callout-tip\">\n\n> [!TIP]\n> ");
    out = out.replace("<div class=\"callout callout-warn\">", "<div class=\"callout callout-warn\">\n\n> [!WARNING]\n> ");
    out = out.replace("<div class=\"callout callout-err\">", "<div class=\"callout callout-err\">\n\n> [!CAUTION]\n> ");
    
    out = out.replace("<strong>", "**");
    out = out.replace("</strong>", "**\n> ");
    out = out.replace("</div>", "\n</div>");
    out
}

fn main() {
    let source = fs::read_to_string("/home/irshad/projects/anyhow-tutorial/index.html").unwrap();
    let dest_dir = "/home/irshad/Projects/dev/content/books/anyhow-tutorial";
    fs::create_dir_all(dest_dir).unwrap();

    let parts: Vec<&str> = source.split("<div id=\"s-").collect();
    
    // Skip the first part (header / nav)
    for (i, part) in parts.iter().skip(1).enumerate() {
        let end_idx = part.find("\"").unwrap();
        let slug = &part[..end_idx];
        
        // Find end of section (either the next section or </main>)
        let content_end = part.find("</div>\n\n<!--").unwrap_or(part.find("</main>").unwrap_or(part.len()));
        let mut html_content = part[..content_end].to_string();
        
        let title = extract_title(&html_content);
        
        // Convert custom callouts before pandoc
        html_content = convert_callouts(&html_content);

        // Convert code headers
        html_content = html_content.replace("<div class=\"code-header\"><span class=\"code-lang\">rust</span><button class=\"copy-btn\" onclick=\"copy(this)\">copy</button></div>", "");
        html_content = html_content.replace("<div class=\"code-header\"><span class=\"code-lang\">toml</span><button class=\"copy-btn\" onclick=\"copy(this)\">copy</button></div>", "");
        html_content = html_content.replace("<pre><code>", "<pre><code class=\"language-rust\">");
        
        let use_pandoc = std::env::args().any(|a| a == "--pandoc");
        let mut md = if use_pandoc {
            let temp_html = format!("/tmp/section_{}.html", i);
            fs::write(&temp_html, &html_content).unwrap();

            let temp_md = format!("/tmp/section_{}.md", i);
            Command::new("pandoc")
                .args(["-i", &temp_html, "-f", "html", "-t", "gfm", "-o", &temp_md])
                .status()
                .unwrap();

            fs::read_to_string(&temp_md).unwrap()
        } else {
            books::html_to_markdown(&html_content)
        };
        
        // Cleanup pandoc output
        md = md.replace("<div class=\"callout callout-info\">", "");
        md = md.replace("<div class=\"callout callout-tip\">", "");
        md = md.replace("<div class=\"callout callout-warn\">", "");
        md = md.replace("<div class=\"callout callout-err\">", "");
        md = md.replace("</div>", "");
        md = md.replace("<span class=\"pill\">", "`");
        md = md.replace("</span>", "`");
        md = md.replace("\\*", "*");
        md = md.replace("\\_", "_");
        md = md.replace("\\[", "[");
        md = md.replace("\\]", "]");
        md = md.replace("\\<", "<");
        md = md.replace("\\>", ">");
        
        // Some formatting fixes for the markdown blockquotes
        let mut final_md = String::new();
        for line in md.lines() {
            if line.starts_with(">") && !line.starts_with("> [!") && line.len() > 1 && !line.starts_with(">  ") {
                final_md.push_str(&format!("> {}\n", line[1..].trim_start()));
            } else {
                final_md.push_str(line);
                final_md.push('\n');
            }
        }

        // Prepend frontmatter
        let frontmatter = format!(
            "---\ntitle: \"{}\"\ndescription: \"\"\n---\n\n",
            title
        );
        let output_content = format!("{}{}", frontmatter, final_md.trim());
        
        let out_filename = format!("{}/{:02}-{}.md", dest_dir, i, slug);
        fs::write(&out_filename, output_content).unwrap();
        
        println!("Processed chapter: {}", out_filename);
    }
}
