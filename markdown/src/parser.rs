use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use ron::de::from_str;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
use syntect::parsing::SyntaxSet;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocItem {
    pub level: u8,
    pub title: String,
    pub slug: String,
}

pub struct ParsedContent<T> {
    pub metadata: T,
    pub html_content: String,
    pub toc: Vec<TocItem>,
    pub reading_time_mins: u32,
}

static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

fn get_syntax_set() -> &'static SyntaxSet {
    SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn get_theme_set() -> &'static ThemeSet {
    THEME_SET.get_or_init(ThemeSet::load_defaults)
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
        let (html_content, toc, reading_time_mins) = Self::parse_markdown_extended(&md_content);

        Ok(ParsedContent {
            metadata,
            html_content,
            toc,
            reading_time_mins,
        })
    }

    pub fn parse_to_html(md_content: &str) -> String {
        let (html, _, _) = Self::parse_markdown_extended(md_content);
        html
    }

    pub fn parse_with_toc(md_content: &str) -> (String, Vec<TocItem>, u32) {
        Self::parse_markdown_extended(md_content)
    }

    pub fn estimate_reading_time(text: &str) -> u32 {
        let words = text.split_whitespace().count();
        words.div_ceil(200).max(1) as u32
    }

    fn slugify(text: &str) -> String {
        let mut slug = text
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "-")
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-");
        if slug.is_empty() {
            slug = "section".to_string();
        }
        slug
    }

    fn escape_html(input: &str) -> String {
        let mut out = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '&' => out.push_str("&amp;"),
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
                '"' => out.push_str("&quot;"),
                '\'' => out.push_str("&#39;"),
                _ => out.push(c),
            }
        }
        out
    }

    fn highlight_code(code: &str, lang: &str) -> String {
        let ss = get_syntax_set();
        let ts = get_theme_set();
        
        let syntax = ss.find_syntax_by_token(lang)
            .or_else(|| ss.find_syntax_by_extension(lang))
            .unwrap_or_else(|| ss.find_syntax_plain_text());

        let theme = &ts.themes["base16-ocean.dark"];
        let mut highlighter = HighlightLines::new(syntax, theme);
        
        let mut output = String::new();
        for line in syntect::util::LinesWithEndings::from(code) {
            match highlighter.highlight_line(line, ss) {
                Ok(ranges) => {
                    let escaped = styled_line_to_highlighted_html(&ranges[..], IncludeBackground::No);
                    if let Ok(html_line) = escaped {
                        output.push_str(&html_line);
                    } else {
                        output.push_str(&Self::escape_html(line));
                    }
                }
                Err(_) => {
                    output.push_str(&Self::escape_html(line));
                }
            }
        }
        output
    }

    fn parse_markdown_extended(md_content: &str) -> (String, Vec<TocItem>, u32) {
        let reading_time = Self::estimate_reading_time(md_content);

        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

        let parser = Parser::new_ext(md_content, options);

        let mut custom_events = Vec::new();
        let mut toc = Vec::new();

        let mut in_code_block = false;
        let mut code_lang = String::new();
        let mut code_text = String::new();

        let mut in_heading = false;
        let mut heading_level: u8 = 1;
        let mut heading_text = String::new();

        for event in parser {
            match event {
                Event::Start(Tag::CodeBlock(ref kind)) => {
                    in_code_block = true;
                    code_lang = match kind {
                        CodeBlockKind::Fenced(lang) => lang.to_string(),
                        CodeBlockKind::Indented => "text".to_string(),
                    };
                    code_text.clear();
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                    let display_lang = if code_lang.is_empty() { "text" } else { &code_lang };
                    let highlighted = Self::highlight_code(&code_text, display_lang);
                    
                    let copy_svg = r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>"#;
                    let is_rust = display_lang == "rust" || display_lang == "rs";
                    let run_btn = if is_rust {
                        r#"<button class="btn-run-code" onclick="const wrapper=this.closest('.code-block-wrapper'); const code=wrapper.querySelector('code').innerText; const drawer=wrapper.querySelector('.playground-drawer'); const out=wrapper.querySelector('.playground-output'); drawer.style.display='block'; out.textContent='Compiling and running on Rust Playground...'; fetch('/api/run_rust_code', { method: 'POST', headers: {'Content-Type': 'application/json'}, body: JSON.stringify([code]) }).then(r=>r.json()).then(res=>{ if(res.Ok){ out.textContent = (res.Ok.stdout || '') + (res.Ok.stderr ? '\n' + res.Ok.stderr : ''); } else { out.textContent = 'Error: ' + JSON.stringify(res.Err); } }).catch(e=>{ out.textContent = 'Execution failed: ' + e; });">▶ Run</button>"#
                    } else {
                        ""
                    };

                    let drawer_html = if is_rust {
                        r#"<div class="playground-drawer" style="display: none;"><div style="display: flex; justify-content: space-between; margin-bottom: 4px; color: var(--accent); font-size: 0.75rem; font-weight: bold;"><span>TERMINAL OUTPUT</span><button style="background: none; border: none; color: var(--text-muted); cursor: pointer;" onclick="this.closest('.playground-drawer').style.display='none';">✕</button></div><pre class="playground-output"></pre></div>"#
                    } else {
                        ""
                    };

                    let wrapped_html = format!(
                        r#"<div class="code-block-wrapper">
                            <div class="code-block-header">
                                <span class="code-lang-badge">{}</span>
                                <div style="display: flex; gap: 0.5rem; align-items: center;">
                                    {}
                                    <button class="code-copy-btn" onclick="navigator.clipboard.writeText(this.closest('.code-block-wrapper').querySelector('code').innerText); const self=this; self.textContent='Copied!'; setTimeout(() => self.innerHTML='{} Copy', 2000);" aria-label="Copy Code">
                                        {} Copy
                                    </button>
                                </div>
                            </div>
                            <pre class="highlight-code"><code class="language-{}">{}</code></pre>
                            {}
                        </div>"#,
                        display_lang,
                        run_btn,
                        copy_svg,
                        copy_svg,
                        display_lang,
                        highlighted,
                        drawer_html
                    );
                    custom_events.push(Event::Html(CowStr::Boxed(wrapped_html.into_boxed_str())));
                }
                Event::Start(Tag::Heading { level, .. }) => {
                    in_heading = true;
                    heading_level = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    heading_text.clear();
                }
                Event::End(TagEnd::Heading(_)) => {
                    in_heading = false;
                    let slug = Self::slugify(&heading_text);
                    if heading_level <= 3 {
                        toc.push(TocItem {
                            level: heading_level,
                            title: heading_text.clone(),
                            slug: slug.clone(),
                        });
                    }
                    let heading_html = format!(
                        r##"<h{} id="{}"><a href="#{}" class="heading-anchor" aria-hidden="true">#</a>{}</h{}>"##,
                        heading_level, slug, slug, heading_text, heading_level
                    );
                    custom_events.push(Event::Html(CowStr::Boxed(heading_html.into_boxed_str())));
                }
                Event::Text(ref text) => {
                    if in_code_block {
                        code_text.push_str(text);
                    } else if in_heading {
                        heading_text.push_str(text);
                    } else {
                        // Check for callouts in text
                        let s = text.as_ref();
                        if s.starts_with("[!NOTE]") {
                            custom_events.push(Event::Html(CowStr::Boxed(r#"<div class="callout callout-note"><span class="callout-badge">NOTE</span>"#.to_string().into_boxed_str())));
                            custom_events.push(Event::Text(CowStr::Boxed(s.trim_start_matches("[!NOTE]").trim().to_string().into_boxed_str())));
                            custom_events.push(Event::Html(CowStr::Boxed("</div>".to_string().into_boxed_str())));
                        } else if s.starts_with("[!TIP]") {
                            custom_events.push(Event::Html(CowStr::Boxed(r#"<div class="callout callout-tip"><span class="callout-badge">TIP</span>"#.to_string().into_boxed_str())));
                            custom_events.push(Event::Text(CowStr::Boxed(s.trim_start_matches("[!TIP]").trim().to_string().into_boxed_str())));
                            custom_events.push(Event::Html(CowStr::Boxed("</div>".to_string().into_boxed_str())));
                        } else if s.starts_with("[!WARNING]") {
                            custom_events.push(Event::Html(CowStr::Boxed(r#"<div class="callout callout-warning"><span class="callout-badge">WARNING</span>"#.to_string().into_boxed_str())));
                            custom_events.push(Event::Text(CowStr::Boxed(s.trim_start_matches("[!WARNING]").trim().to_string().into_boxed_str())));
                            custom_events.push(Event::Html(CowStr::Boxed("</div>".to_string().into_boxed_str())));
                        } else if s.starts_with("[!CAUTION]") {
                            custom_events.push(Event::Html(CowStr::Boxed(r#"<div class="callout callout-caution"><span class="callout-badge">CAUTION</span>"#.to_string().into_boxed_str())));
                            custom_events.push(Event::Text(CowStr::Boxed(s.trim_start_matches("[!CAUTION]").trim().to_string().into_boxed_str())));
                            custom_events.push(Event::Html(CowStr::Boxed("</div>".to_string().into_boxed_str())));
                        } else if s.starts_with("[!IMPORTANT]") {
                            custom_events.push(Event::Html(CowStr::Boxed(r#"<div class="callout callout-important"><span class="callout-badge">IMPORTANT</span>"#.to_string().into_boxed_str())));
                            custom_events.push(Event::Text(CowStr::Boxed(s.trim_start_matches("[!IMPORTANT]").trim().to_string().into_boxed_str())));
                            custom_events.push(Event::Html(CowStr::Boxed("</div>".to_string().into_boxed_str())));
                        } else {
                            custom_events.push(event);
                        }
                    }
                }
                _ => {
                    if !in_code_block && !in_heading {
                        custom_events.push(event);
                    }
                }
            }
        }

        let mut html_output = String::new();
        html::push_html(&mut html_output, custom_events.into_iter());

        let html_output = html_output
            .replace("<p>[!NOTE]", r#"<div class="callout callout-note"><span class="callout-badge">NOTE</span><p>"#)
            .replace("<p>[!TIP]", r#"<div class="callout callout-tip"><span class="callout-badge">TIP</span><p>"#)
            .replace("<p>[!WARNING]", r#"<div class="callout callout-warning"><span class="callout-badge">WARNING</span><p>"#)
            .replace("<p>[!CAUTION]", r#"<div class="callout callout-caution"><span class="callout-badge">CAUTION</span><p>"#)
            .replace("<p>[!IMPORTANT]", r#"<div class="callout callout-important"><span class="callout-badge">IMPORTANT</span><p>"#);

        (html_output, toc, reading_time)
    }
}

