use leptos::*;
use serde::{Serialize, Deserialize};
use domain::models::{Book, Article, Project};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UiSearchResult {
    pub title: String,
    pub slug: String,
    pub doc_type: String,
    pub snippet: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UiPageView {
    pub slug: String,
    pub view_count: i64,
}

#[server(SearchContent, "/api")]
pub async fn search_content(query: String) -> Result<Vec<UiSearchResult>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let search_service = expect_context::<search::SearchService>();
        let results = search_service.search(&query).unwrap_or_default();
        
        let ui_results = results.into_iter().map(|r| UiSearchResult {
            title: r.title,
            slug: r.slug,
            doc_type: r.doc_type,
            snippet: r.snippet,
        }).collect();
        
        Ok(ui_results)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetProjects, "/api")]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use portfolio::service::PortfolioService;

        let pool = expect_context::<sqlx::PgPool>();
        let service = PortfolioService::new(pool);
        
        let projects = service.get_all_projects().await.map_err(|e| {
            let err: ServerFnError = ServerFnError::ServerError(e.to_string());
            err
        })?;
        Ok(projects)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetBooks, "/api")]
pub async fn get_books() -> Result<Vec<Book>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = expect_context::<sqlx::PgPool>();
        let service = books::service::BookService::new(pool);
        let books = service.get_reading_list().await.map_err(|e| {
            let err: ServerFnError = ServerFnError::ServerError(e.to_string());
            err
        })?;
        Ok(books)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetArticles, "/api")]
pub async fn get_articles() -> Result<Vec<Article>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use articles::service::ArticleService;

        let pool = expect_context::<sqlx::PgPool>();
        let service = ArticleService::new(pool);
        let articles = service.get_all_articles().await.map_err(|e| {
            let err: ServerFnError = ServerFnError::ServerError(e.to_string());
            err
        })?;
        Ok(articles)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetArticle, "/api")]
pub async fn get_article(slug: String) -> Result<(Article, i64), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use articles::service::ArticleService;

        let pool = expect_context::<sqlx::PgPool>();
        let service = ArticleService::new(pool.clone());
        let article = service.get_article(&slug).await.map_err(|e| {
            let err: ServerFnError = ServerFnError::ServerError(e.to_string());
            err
        })?;
        
        let analytics = analytics::AnalyticsService::new(pool);
        let views = analytics.get_view_count(&slug).await.unwrap_or(0);
        
        Ok((article, views))
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(RecordView, "/api")]
pub async fn record_view(slug: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = expect_context::<sqlx::PgPool>();
        let analytics = analytics::AnalyticsService::new(pool);
        let _ = analytics.record_view(&slug).await;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetAnalytics, "/api")]
pub async fn get_analytics() -> Result<Vec<UiPageView>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = expect_context::<sqlx::PgPool>();
        let analytics_service = analytics::AnalyticsService::new(pool);
        let views = analytics_service.get_all_views().await.unwrap_or_default();
        
        let ui_views = views.into_iter().map(|v| UiPageView {
            slug: v.slug,
            view_count: v.view_count,
        }).collect();
        
        Ok(ui_views)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(RebuildSearchIndex, "/api")]
pub async fn rebuild_search_index() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let search_service = expect_context::<search::SearchService>();
        search_service.index_articles(&std::path::PathBuf::from("content")).map_err(|e| {
            let err: ServerFnError = ServerFnError::ServerError(e.to_string());
            err
        })?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(CheckAuth, "/api")]
pub async fn check_auth() -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        Ok(true)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(Login, "/api")]
pub async fn login(email: String, password: String) -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use auth::service::AuthService;
        use std::env;
        use leptos_axum::ResponseOptions;

        let pool = expect_context::<sqlx::PgPool>();
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "dev_secret_key_12345".to_string());
        
        let auth_service = AuthService::new(pool, secret);
        match auth_service.login(&email, &password).await {
            Ok(token) => {
                if let Some(resp_options) = use_context::<ResponseOptions>() {
                    resp_options.insert_header(
                        axum::http::header::SET_COOKIE,
                        axum::http::HeaderValue::from_str(&format!("auth_token={}; Path=/; HttpOnly; SameSite=Strict; Max-Age=604800", token)).unwrap(),
                    );
                }
                Ok(true)
            },
            Err(e) => {
                Err(ServerFnError::ServerError(e.to_string()))
            }
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}
