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
        .nest_service("/pkg", tower_http::services::ServeDir::new(format!("{}/pkg", leptos_options.site_root)))
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
        .layer(tower_http::trace::TraceLayer::new_for_http());

    tracing::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
}
