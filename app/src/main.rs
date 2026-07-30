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
    
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = database::get_pool(&database_url).await.expect("Failed to connect to database");
    
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
    search_service.index_articles(pool.clone()).await.expect("Failed to index articles");


    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", axum::routing::get({
            let pool = pool.clone();
            move |req: axum::http::Request<axum::body::Body>| leptos_axum::handle_server_fns_with_context(move || {
                provide_context(pool.clone());
            }, req)
        }))
        .route("/api/*fn_name", axum::routing::post({
            let pool = pool.clone();
            move |req: axum::http::Request<axum::body::Body>| leptos_axum::handle_server_fns_with_context(move || {
                provide_context(pool.clone());
            }, req)
        }))
        .route("/rss.xml", axum::routing::get(server::rss_handler))
        .route("/sitemap.xml", axum::routing::get(server::sitemap_handler))
        .nest_service("/pkg", tower_http::services::ServeDir::new(format!("{}/pkg", leptos_options.site_root)))
        .leptos_routes_with_context(&leptos_options, routes, {
            let pool = pool.clone();
            move || {
                provide_context(pool.clone());
            }
        }, App)
        .fallback(server::file_and_error_handler)
        .with_state(leptos_options)
        .layer(axum::extract::Extension(pool))
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
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
