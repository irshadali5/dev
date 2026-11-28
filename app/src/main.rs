#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use app::server;
    use ui::App;

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load .env file
    dotenvy::dotenv().ok();
    
    #[cfg(feature = "ssr")]
    {
        use leptos::server_fn::ServerFn;
        println!("LOGIN PATH: {}", api::Login::PATH);
        println!("RECORDVIEW PATH: {}", api::RecordView::PATH);
        let _ = api::GetProjects::PATH;
        let _ = api::GetBooks::PATH;
        let _ = api::GetArticles::PATH;
        let _ = api::GetArticle::PATH;
        let _ = api::SearchContent::PATH;
        let _ = api::RecordView::PATH;
        let _ = api::GetAnalytics::PATH;
        let _ = api::RebuildSearchIndex::PATH;
        let _ = api::CheckAuth::PATH;
        let _ = api::Login::PATH;
    }
    
    server::init_start_time();

    let site_config = config::SiteConfig::load();
    let db_path = std::env::var("STOOLAP_DB_PATH")
        .unwrap_or_else(|_| std::env::var("DATABASE_URL").unwrap_or_else(|_| site_config.database.path.clone()));
    
    tracing::info!("Initializing Stoolap database at {}", db_path);
    let conn = database::get_db(&db_path).await.expect("Failed to initialize Stoolap database");

    // Optional first-run admin bootstrap via ADMIN_EMAIL + ADMIN_PASSWORD
    if let (Ok(admin_email), Ok(admin_password)) = (std::env::var("ADMIN_EMAIL"), std::env::var("ADMIN_PASSWORD")) {
        if !admin_email.trim().is_empty() && !admin_password.trim().is_empty() {
            let user_repo = database::repositories::UserRepository::new(conn.clone());
            if let Ok(None) = user_repo.get_user_by_email(&admin_email).await {
                if let Ok(hash) = auth::crypto::hash_password(&admin_password) {
                    if user_repo.add_user(&admin_email, &hash, "admin").await.is_ok() {
                        tracing::info!("Bootstrapped admin user {}", admin_email);
                    }
                }
            }
        }
    }
    
    // Initialize Asset Pipeline
    let content_dir = std::path::PathBuf::from("content");
    let asset_optimizer = assets::AssetOptimizer::new(content_dir.clone());
    tracing::info!("Running asset optimization...");
    if let Err(e) = asset_optimizer.process_all().await {
        tracing::error!("Asset optimization failed: {}", e);
    }
    tracing::info!("Asset optimization complete.");

    // Initialize Search Service
    let search_service = search::SearchService::new().expect("Failed to initialize SearchService");
    search_service.index_articles(conn.clone()).await.expect("Failed to index articles");

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", axum::routing::get({
            let conn = conn.clone();
            move |req: axum::http::Request<axum::body::Body>| leptos_axum::handle_server_fns_with_context(move || {
                provide_context(conn.clone());
            }, req)
        }))
        .route("/api/*fn_name", axum::routing::post({
            let conn = conn.clone();
            move |req: axum::http::Request<axum::body::Body>| leptos_axum::handle_server_fns_with_context(move || {
                provide_context(conn.clone());
            }, req)
        }))
        .route("/rss.xml", axum::routing::get(server::rss_handler))
        .route("/feed", axum::routing::get(server::rss_handler))
        .route("/atom.xml", axum::routing::get(server::atom_handler))
        .route("/feed.atom", axum::routing::get(server::atom_handler))
        .route("/feed.json", axum::routing::get(server::feed_json_handler))
        .route("/sitemap.xml", axum::routing::get(server::sitemap_handler))
        .route("/robots.txt", axum::routing::get(server::robots_handler))
        .route("/manifest.json", axum::routing::get(server::manifest_handler))
        .route("/api/og", axum::routing::get(server::og_image_handler))
        .route("/healthz", axum::routing::get({
            let conn = conn.clone();
            move || server::health_check_handler(axum::extract::State(conn.clone()))
        }))
        .route("/metrics", axum::routing::get({
            let conn = conn.clone();
            move || server::metrics_handler(axum::extract::State(conn.clone()))
        }))
        .route("/api/v1/articles", axum::routing::get(server::api_v1_articles_handler))
        .route("/api/v1/articles/:slug", axum::routing::get(server::api_v1_article_detail_handler))
        .route("/api/v1/projects", axum::routing::get(server::api_v1_projects_handler))
        .route("/api/v1/books", axum::routing::get(server::api_v1_books_handler))
        .route("/api/v1/skills", axum::routing::get(server::api_v1_skills_handler))
        .route("/api/v1/stats", axum::routing::get(server::api_v1_stats_handler))
        .route("/api/v1/openapi.json", axum::routing::get(server::api_v1_openapi_handler))
        .nest_service("/pkg", tower_http::services::ServeDir::new(format!("{}/pkg", leptos_options.site_root)))
        .nest_service("/media", tower_http::services::ServeDir::new("content/assets"))
        .leptos_routes_with_context(&leptos_options, routes, {
            let conn = conn.clone();
            move || {
                provide_context(conn.clone());
            }
        }, App)
        .fallback(server::file_and_error_handler)
        .with_state(leptos_options)
        .layer(axum::extract::Extension(conn))
        .layer(axum::extract::Extension(search_service))
        .layer(tower_http::compression::CompressionLayer::new())
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            axum::http::header::X_CONTENT_TYPE_OPTIONS,
            axum::http::HeaderValue::from_static("nosniff"),
        ))
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            axum::http::header::X_FRAME_OPTIONS,
            axum::http::HeaderValue::from_static("DENY"),
        ))
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            axum::http::header::HeaderName::from_static("referrer-policy"),
            axum::http::HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            axum::http::header::HeaderName::from_static("permissions-policy"),
            axum::http::HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    tracing::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { tracing::info!("Received SIGINT, shutting down gracefully..."); },
        _ = terminate => { tracing::info!("Received SIGTERM, shutting down gracefully..."); },
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
}
