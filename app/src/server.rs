use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::{IntoResponse, Response as AxumResponse},
};
use leptos::{LeptosOptions, view};
use tower::ServiceExt;
use tower_http::services::ServeDir;
use axum::extract::Extension;
use database::DatabaseConnection;
use serde::{Deserialize, Serialize};

pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    Extension(conn): Extension<DatabaseConnection>,
    req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        let handler = leptos_axum::render_app_to_stream_with_context(
            options.to_owned(),
            move || {
                leptos::provide_context(conn.clone());
            },
            move || view! { <ui::App/> },
        );
        handler(req).await.into_response()
    }
}

async fn get_static_file(
    uri: Uri,
    root: &str,
) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}

pub async fn rss_handler(Extension(conn): Extension<DatabaseConnection>) -> impl axum::response::IntoResponse {
    use rss::{ChannelBuilder, ItemBuilder, GuidBuilder};
    use database::repositories::ArticleRepository;
    
    let repo = ArticleRepository::new(conn);
    let articles = repo.get_published_articles().await.unwrap_or_default();
    
    let mut items = Vec::new();
    for article in articles {
        let url = format!("https://irshadali.org/articles/{}", article.slug);
        let pub_date = article.created_at.to_rfc2822();
        
        let guid = GuidBuilder::default()
            .value(url.clone())
            .permalink(true)
            .build();

        let item = ItemBuilder::default()
            .title(Some(article.title))
            .link(Some(url))
            .description(article.description)
            .pub_date(Some(pub_date))
            .guid(Some(guid))
            .build();
            
        items.push(item);
    }
    
    let channel = ChannelBuilder::default()
        .title("Irshad - Developer & Writer".to_string())
        .link("https://irshadali.org".to_string())
        .description("Thoughts on Rust, Systems Engineering, and software architecture.".to_string())
        .language(Some("en-us".to_string()))
        .items(items)
        .build();
    
    axum::response::Response::builder()
        .header("Content-Type", "application/rss+xml; charset=utf-8")
        .body(channel.to_string())
        .unwrap()
}

pub async fn atom_handler(Extension(conn): Extension<DatabaseConnection>) -> impl axum::response::IntoResponse {
    use database::repositories::ArticleRepository;
    
    let repo = ArticleRepository::new(conn);
    let articles = repo.get_published_articles().await.unwrap_or_default();
    let updated = chrono::Utc::now().to_rfc3339();
    
    let mut entries = String::new();
    for a in articles {
        let url = format!("https://irshadali.org/articles/{}", a.slug);
        let title_escaped = a.title.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
        let desc_escaped = a.description.unwrap_or_default().replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
        let item_updated = a.updated_at.to_rfc3339();
        
        entries.push_str(&format!(
            r#"  <entry>
    <title>{}</title>
    <link href="{}" />
    <id>{}</id>
    <updated>{}</updated>
    <summary>{}</summary>
    <author><name>Irshad Ali</name></author>
  </entry>
"#,
            title_escaped, url, url, item_updated, desc_escaped
        ));
    }
    
    let atom = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>Irshad - Developer &amp; Writer</title>
  <link href="https://irshadali.org/atom.xml" rel="self" />
  <link href="https://irshadali.org/" />
  <id>https://irshadali.org/</id>
  <updated>{}</updated>
  <author><name>Irshad Ali</name></author>
{}</feed>"#,
        updated, entries
    );
    
    axum::response::Response::builder()
        .header("Content-Type", "application/atom+xml; charset=utf-8")
        .body(atom)
        .unwrap()
}

#[derive(Serialize)]
struct JsonFeedItem {
    id: String,
    url: String,
    title: String,
    summary: Option<String>,
    date_published: String,
    date_modified: String,
}

#[derive(Serialize)]
struct JsonFeed {
    version: String,
    title: String,
    home_page_url: String,
    feed_url: String,
    description: String,
    items: Vec<JsonFeedItem>,
}

pub async fn feed_json_handler(Extension(conn): Extension<DatabaseConnection>) -> impl axum::response::IntoResponse {
    use database::repositories::ArticleRepository;
    
    let repo = ArticleRepository::new(conn);
    let articles = repo.get_published_articles().await.unwrap_or_default();
    
    let items = articles.into_iter().map(|a| {
        let url = format!("https://irshadali.org/articles/{}", a.slug);
        JsonFeedItem {
            id: url.clone(),
            url,
            title: a.title,
            summary: a.description,
            date_published: a.created_at.to_rfc3339(),
            date_modified: a.updated_at.to_rfc3339(),
        }
    }).collect();
    
    let feed = JsonFeed {
        version: "https://jsonfeed.org/version/1.1".to_string(),
        title: "Irshad - Developer & Writer".to_string(),
        home_page_url: "https://irshadali.org".to_string(),
        feed_url: "https://irshadali.org/feed.json".to_string(),
        description: "Thoughts on Rust, Systems Engineering, and software architecture.".to_string(),
        items,
    };
    
    let json_body = serde_json::to_string_pretty(&feed).unwrap_or_default();
    
    axum::response::Response::builder()
        .header("Content-Type", "application/feed+json; charset=utf-8")
        .body(json_body)
        .unwrap()
}

pub async fn sitemap_handler(Extension(conn): Extension<DatabaseConnection>) -> impl axum::response::IntoResponse {
    use database::repositories::{ArticleRepository, ProjectRepository, BookRepository};
    
    let art_repo = ArticleRepository::new(conn.clone());
    let proj_repo = ProjectRepository::new(conn.clone());
    let book_repo = BookRepository::new(conn);
    
    let articles = art_repo.get_published_articles().await.unwrap_or_default();
    let projects = proj_repo.get_all_projects().await.unwrap_or_default();
    let books = book_repo.get_all_books().await.unwrap_or_default();
    
    let mut urls = String::new();
    
    // Static core pages
    let static_pages = [
        ("/", "1.0", "daily"),
        ("/about", "0.8", "monthly"),
        ("/resume", "0.8", "monthly"),
        ("/experience", "0.9", "monthly"),
        ("/projects", "0.9", "weekly"),
        ("/articles", "0.9", "daily"),
        ("/books", "0.9", "weekly"),
        ("/contact", "0.7", "monthly"),
        ("/tools", "0.8", "monthly"),
    ];
    
    for (path, priority, freq) in static_pages {
        urls.push_str(&format!(
            "  <url>\n    <loc>https://irshadali.org{}</loc>\n    <changefreq>{}</changefreq>\n    <priority>{}</priority>\n  </url>\n",
            path, freq, priority
        ));
    }
    
    for a in articles {
        urls.push_str(&format!(
            "  <url>\n    <loc>https://irshadali.org/articles/{}</loc>\n    <lastmod>{}</lastmod>\n    <changefreq>weekly</changefreq>\n    <priority>0.8</priority>\n  </url>\n",
            a.slug,
            a.updated_at.format("%Y-%m-%d")
        ));
    }
    
    for p in projects {
        urls.push_str(&format!(
            "  <url>\n    <loc>https://irshadali.org/projects/{}</loc>\n    <lastmod>{}</lastmod>\n    <changefreq>monthly</changefreq>\n    <priority>0.8</priority>\n  </url>\n",
            p.slug,
            p.updated_at.format("%Y-%m-%d")
        ));
    }
    
    for b in books {
        urls.push_str(&format!(
            "  <url>\n    <loc>https://irshadali.org/books/{}</loc>\n    <lastmod>{}</lastmod>\n    <changefreq>weekly</changefreq>\n    <priority>0.8</priority>\n  </url>\n",
            b.slug,
            b.updated_at.format("%Y-%m-%d")
        ));
    }
    
    if let Ok(entries) = std::fs::read_dir("content/books") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let book_slug = path.file_name().and_then(|s| s.to_str()).unwrap_or_default();
                let ron_path = path.join("book.ron");
                if ron_path.exists() {
                    if let Ok(ron_content) = std::fs::read_to_string(&ron_path) {
                        if let Ok(meta) = ron::from_str::<domain::models::BookMeta>(&ron_content) {
                            for chapter in meta.chapters {
                                urls.push_str(&format!(
                                    "  <url>\n    <loc>https://irshadali.org/books/{}/{}</loc>\n    <changefreq>monthly</changefreq>\n    <priority>0.7</priority>\n  </url>\n",
                                    book_slug, chapter
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    
    let sitemap = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}</urlset>"#,
        urls
    );

    axum::response::Response::builder()
        .header("Content-Type", "application/xml; charset=utf-8")
        .body(sitemap)
        .unwrap()
}

pub async fn robots_handler() -> impl axum::response::IntoResponse {
    let robots = r#"User-agent: *
Allow: /
Disallow: /admin
Disallow: /login

Sitemap: https://irshadali.org/sitemap.xml
"#;

    axum::response::Response::builder()
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(robots.to_string())
        .unwrap()
}

#[derive(Deserialize)]
pub struct OgParams {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub tag: Option<String>,
}

pub async fn og_image_handler(
    axum::extract::Query(params): axum::extract::Query<OgParams>,
) -> impl axum::response::IntoResponse {
    let title = params.title.unwrap_or_else(|| "Irshad Ali".to_string());
    let subtitle = params.subtitle.unwrap_or_else(|| "Software Engineer & Systems Architect".to_string());
    let tag = params.tag.unwrap_or_else(|| "Rust & Systems".to_string());

    let title_escaped = title.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
    let subtitle_escaped = subtitle.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
    let tag_escaped = tag.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");

    let svg = format!(
        r##"<svg width="1200" height="630" viewBox="0 0 1200 630" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="bg" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#0f172a" />
      <stop offset="50%" stop-color="#1e293b" />
      <stop offset="100%" stop-color="#090d16" />
    </linearGradient>
  </defs>
  
  <rect width="1200" height="630" fill="url(#bg)" />
  <rect x="40" y="40" width="1120" height="550" rx="24" fill="none" stroke="#334155" stroke-width="2" />
  
  <text x="90" y="120" font-family="system-ui, -apple-system, BlinkMacSystemFont, Roboto, sans-serif" font-size="32" font-weight="bold" fill="#60a5fa">&lt;IA /&gt;</text>
  <rect x="900" y="88" width="210" height="42" rx="21" fill="#1e293b" stroke="#475569" stroke-width="1.5" />
  <text x="1005" y="115" font-family="system-ui, -apple-system, BlinkMacSystemFont, Roboto, sans-serif" font-size="15" font-weight="600" fill="#94a3b8" text-anchor="middle">{}</text>
  
  <text x="90" y="290" font-family="system-ui, -apple-system, BlinkMacSystemFont, Roboto, sans-serif" font-size="56" font-weight="800" fill="#f8fafc">
    {}
  </text>
  
  <text x="90" y="380" font-family="system-ui, -apple-system, BlinkMacSystemFont, Roboto, sans-serif" font-size="26" font-weight="400" fill="#94a3b8">
    {}
  </text>
  
  <line x1="90" y1="470" x2="1110" y2="470" stroke="#334155" stroke-width="1.5" />
  <text x="90" y="525" font-family="system-ui, -apple-system, BlinkMacSystemFont, Roboto, sans-serif" font-size="22" font-weight="600" fill="#64748b">irshadali.org</text>
  <text x="1110" y="525" font-family="system-ui, -apple-system, BlinkMacSystemFont, Roboto, sans-serif" font-size="22" font-weight="500" fill="#64748b" text-anchor="end">Irshad Ali</text>
</svg>"##,
        tag_escaped, title_escaped, subtitle_escaped
    );

    axum::response::Response::builder()
        .header("Content-Type", "image/svg+xml; charset=utf-8")
        .header("Cache-Control", "public, max-age=86400")
        .body(svg)
        .unwrap()
}

pub async fn manifest_handler() -> impl axum::response::IntoResponse {
    let manifest = r##"{
  "name": "Irshad Ali - Developer & Systems Architect",
  "short_name": "Irshad",
  "description": "High-Performance Distributed Systems, Rust, and Cloud Architecture.",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#0f172a",
  "theme_color": "#0f172a",
  "icons": [
    {
      "src": "/api/og?title=IA&subtitle=Dev",
      "sizes": "192x192 512x512",
      "type": "image/svg+xml"
    }
  ]
}"##;

    axum::response::Response::builder()
        .header("Content-Type", "application/manifest+json; charset=utf-8")
        .header("Cache-Control", "public, max-age=86400")
        .body(manifest.to_string())
        .unwrap()
}

static START_TIME: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();

pub fn init_start_time() {
    START_TIME.get_or_init(std::time::Instant::now);
}

pub fn get_uptime_secs() -> u64 {
    START_TIME.get().map(|t| t.elapsed().as_secs()).unwrap_or(0)
}

pub async fn health_check_handler(
    axum::extract::State(conn): axum::extract::State<database::DatabaseConnection>,
) -> impl axum::response::IntoResponse {
    let db_ok = conn.inner().execute("SELECT 1", ()).is_ok();
    let uptime = get_uptime_secs();
    let search_index_exists = std::path::Path::new("content/.search_index").exists();

    let status = if db_ok { "ok" } else { "degraded" };

    let body = serde_json::json!({
        "status": status,
        "uptime_seconds": uptime,
        "database": {
            "engine": "stoolap",
            "status": if db_ok { "connected" } else { "disconnected" },
            "mvcc": true,
            "pure_rust": true
        },
        "search_index": {
            "persistent": search_index_exists,
            "path": "content/.search_index"
        },
        "version": env!("CARGO_PKG_VERSION")
    });

    let status_code = if db_ok {
        axum::http::StatusCode::OK
    } else {
        axum::http::StatusCode::SERVICE_UNAVAILABLE
    };

    (
        status_code,
        [("Content-Type", "application/json; charset=utf-8")],
        body.to_string(),
    )
}

pub async fn metrics_handler(
    axum::extract::State(conn): axum::extract::State<database::DatabaseConnection>,
) -> impl axum::response::IntoResponse {
    let uptime = get_uptime_secs();
    let db_status = if conn.inner().execute("SELECT 1", ()).is_ok() { 1 } else { 0 };

    let metrics = format!(
        "# HELP app_uptime_seconds Total application uptime in seconds.\n\
         # TYPE app_uptime_seconds counter\n\
         app_uptime_seconds {}\n\n\
         # HELP db_engine_status Stoolap embedded database engine status (1=up, 0=down).\n\
         # TYPE db_engine_status gauge\n\
         db_engine_status{{engine=\"stoolap\"}} {}\n",
        uptime, db_status
    );

    (
        axum::http::StatusCode::OK,
        [("Content-Type", "text/plain; version=0.0.4; charset=utf-8")],
        metrics,
    )
}
