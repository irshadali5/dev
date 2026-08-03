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
use sqlx::SqlitePool;

pub async fn file_and_error_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
    Extension(pool): Extension<SqlitePool>,
    req: Request<Body>,
) -> AxumResponse {
    let root = options.site_root.clone();
    println!("=== file_and_error_handler ===");
    println!("URI: {}", uri);
    println!("CWD: {:?}", std::env::current_dir().unwrap());
    println!("ROOT: {}", root);
    
    let res = get_static_file(uri.clone(), &root).await.unwrap();
    println!("ServeDir status: {}", res.status());

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        let handler = leptos_axum::render_app_to_stream_with_context(
            options.to_owned(),
            move || {
                leptos::provide_context(pool.clone());
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
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}

pub async fn rss_handler() -> impl axum::response::IntoResponse {
    use rss::{ChannelBuilder, ItemBuilder};
    
    let mut channel = ChannelBuilder::default()
        .title("Irshad - Developer & Writer".to_string())
        .link("https://dev.irshad.org.in".to_string())
        .description("Thoughts on Rust, Systems Engineering, and software architecture.".to_string())
        .build();
    
    // In a real application, we would fetch the latest articles from the ArticleService
    let mut items = vec![];
    let item = ItemBuilder::default()
        .title(Some("Rust Ownership".to_string()))
        .link(Some("https://dev.irshad.org.in/articles/rust-ownership".to_string()))
        .description(Some("Understanding Rust ownership and borrowing.".to_string()))
        .build();
    items.push(item);
    
    channel.set_items(items);
    
    axum::response::Response::builder()
        .header("Content-Type", "application/rss+xml; charset=utf-8")
        .body(channel.to_string())
        .unwrap()
}

pub async fn sitemap_handler() -> impl axum::response::IntoResponse {
    let sitemap = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://dev.irshad.org.in/</loc>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>https://dev.irshad.org.in/articles</loc>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>https://dev.irshad.org.in/projects</loc>
    <priority>0.8</priority>
  </url>
</urlset>"#;

    axum::response::Response::builder()
        .header("Content-Type", "application/xml; charset=utf-8")
        .body(sitemap.to_string())
        .unwrap()
}
