use leptos::*;
use serde::{Serialize, Deserialize};
use domain::models::{Book, Article, Project, TimelineItem, Skill};

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
            let err: ServerFnError = ServerFnError::new(e.to_string());
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
            let err: ServerFnError = ServerFnError::new(e.to_string());
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
            let err: ServerFnError = ServerFnError::new(e.to_string());
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
            let err: ServerFnError = ServerFnError::new(e.to_string());
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

#[server(SubmitContactForm, "/api")]
pub async fn submit_contact_form(name: String, email: String, message: String) -> Result<bool, ServerFnError> {
    // Basic validation
    if name.is_empty() || email.is_empty() || message.is_empty() {
        return Err(ServerFnError::new("All fields are required".to_string()));
    }
    
    #[cfg(feature = "ssr")]
    {
        // For now, just log it. Later, we can hook it up to an email service or save to DB.
        println!("Received Contact Form Submission:\nName: {}\nEmail: {}\nMessage: {}", name, email, message);
        Ok(true)
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
        let pool = expect_context::<sqlx::PgPool>();
        search_service.index_articles(pool).await.map_err(|e| {
            let err: ServerFnError = ServerFnError::new(e.to_string());
            err
        })?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[cfg(feature = "ssr")]
pub fn get_jwt_secret() -> String {
    use std::env;
    use std::sync::OnceLock;
    
    static SECRET: OnceLock<String> = OnceLock::new();
    
    SECRET.get_or_init(|| {
        env::var("JWT_SECRET").unwrap_or_else(|_| {
            uuid::Uuid::new_v4().to_string()
        })
    }).clone()
}

#[cfg(feature = "ssr")]
pub async fn verify_admin() -> Result<(), ServerFnError> {
    use auth::service::AuthService;
    use axum::http::request::Parts as RequestParts;
    use axum::http::header::COOKIE;

    let req_parts = use_context::<RequestParts>()
        .ok_or_else(|| ServerFnError::new("Request parts not found".to_string()))?;
        
    let mut token = None;
    if let Some(cookie_header) = req_parts.headers.get(COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if cookie.starts_with("auth_token=") {
                    token = Some(cookie["auth_token=".len()..].to_string());
                    break;
                }
            }
        }
    }
    
    let token = token.ok_or_else(|| ServerFnError::new("Unauthorized".to_string()))?;
    
    let pool = expect_context::<sqlx::PgPool>();
    let auth_service = AuthService::new(pool, get_jwt_secret());
    
    let claims = auth_service.verify_token(&token)
        .map_err(|_| ServerFnError::new("Invalid token".to_string()))?;
        
    if claims.role != "admin" {
        return Err(ServerFnError::new("Forbidden".to_string()));
    }
    
    Ok(())
}

#[server(CheckAuth, "/api")]
pub async fn check_auth() -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        Ok(verify_admin().await.is_ok())
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
        let secret = get_jwt_secret();
        
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
                Err(ServerFnError::new(e.to_string()))
            }
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetTimeline, "/api")]
pub async fn get_timeline() -> Result<Vec<TimelineItem>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use database::repositories::PortfolioRepository;
        let pool = expect_context::<sqlx::PgPool>();
        let repo = PortfolioRepository::new(pool);
        let items = repo.get_timeline_items().await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(items)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(AddTimelineItem, "/api")]
pub async fn add_timeline_item(item_type: String, title: String, organization: String, period: String, description: String, order_index: i32) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        verify_admin().await?;
        use database::repositories::PortfolioRepository;
        let pool = expect_context::<sqlx::PgPool>();
        let repo = PortfolioRepository::new(pool);
        repo.add_timeline_item(&item_type, &title, &organization, &period, &description, order_index)
            .await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(DeleteTimelineItem, "/api")]
pub async fn delete_timeline_item(id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        verify_admin().await?;
        use database::repositories::PortfolioRepository;
        let pool = expect_context::<sqlx::PgPool>();
        let repo = PortfolioRepository::new(pool);
        let uuid = uuid::Uuid::parse_str(&id).map_err(|e| ServerFnError::new(e.to_string()))?;
        repo.delete_timeline_item(uuid).await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetSkills, "/api")]
pub async fn get_skills() -> Result<Vec<Skill>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use database::repositories::PortfolioRepository;
        let pool = expect_context::<sqlx::PgPool>();
        let repo = PortfolioRepository::new(pool);
        let skills = repo.get_skills().await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(skills)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(AddSkill, "/api")]
pub async fn add_skill(category: String, name: String, order_index: i32) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        verify_admin().await?;
        use database::repositories::PortfolioRepository;
        let pool = expect_context::<sqlx::PgPool>();
        let repo = PortfolioRepository::new(pool);
        repo.add_skill(&category, &name, order_index)
            .await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(DeleteSkill, "/api")]
pub async fn delete_skill(id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        verify_admin().await?;
        use database::repositories::PortfolioRepository;
        let pool = expect_context::<sqlx::PgPool>();
        let repo = PortfolioRepository::new(pool);
        let uuid = uuid::Uuid::parse_str(&id).map_err(|e| ServerFnError::new(e.to_string()))?;
        repo.delete_skill(uuid).await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(AddProject, "/api")]
pub async fn add_project(slug: String, title: String, description: String, content_html: String, github_url: Option<String>, live_url: Option<String>) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        verify_admin().await?;
        use database::repositories::ProjectRepository;
        let pool = expect_context::<sqlx::PgPool>();
        let repo = ProjectRepository::new(pool);
        repo.add_project(&slug, &title, &description, &content_html, github_url, live_url)
            .await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(DeleteProject, "/api")]
pub async fn delete_project(id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        verify_admin().await?;
        use database::repositories::ProjectRepository;
        let pool = expect_context::<sqlx::PgPool>();
        let repo = ProjectRepository::new(pool);
        let uuid = uuid::Uuid::parse_str(&id).map_err(|e| ServerFnError::new(e.to_string()))?;
        repo.delete_project(uuid).await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(AddArticle, "/api")]
pub async fn add_article(slug: String, title: String, description: String, body: String, published: bool) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        verify_admin().await?;
        use database::repositories::ArticleRepository;
        let pool = expect_context::<sqlx::PgPool>();
        let repo = ArticleRepository::new(pool.clone());
        // Default author_id for now or fetch from token. We will use a dummy uuid or fetch the admin uuid.
        // Assuming admin is the only author. We need the auth_id. For now, use uuid::Uuid::nil() or parse from claims.
        let author_id = sqlx::query!("SELECT id FROM users LIMIT 1").fetch_one(&pool).await.map(|r| r.id).map_err(|e| ServerFnError::new(e.to_string()))?; 
        repo.add_article(&slug, &title, &description, &body, published, author_id)
            .await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(DeleteArticle, "/api")]
pub async fn delete_article(id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        verify_admin().await?;
        use database::repositories::ArticleRepository;
        let pool = expect_context::<sqlx::PgPool>();
        let repo = ArticleRepository::new(pool);
        let uuid = uuid::Uuid::parse_str(&id).map_err(|e| ServerFnError::new(e.to_string()))?;
        repo.delete_article(uuid).await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(AddBook, "/api")]
pub async fn add_book(slug: String, title: String, description: Option<String>) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        verify_admin().await?;
        use database::repositories::BookRepository;
        let pool = expect_context::<sqlx::PgPool>();
        let repo = BookRepository::new(pool.clone());
        let author_id = sqlx::query!("SELECT id FROM users LIMIT 1").fetch_one(&pool).await.map(|r| r.id).map_err(|e| ServerFnError::new(e.to_string()))?;
        repo.add_book(&slug, &title, description, author_id)
            .await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(DeleteBook, "/api")]
pub async fn delete_book(id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        verify_admin().await?;
        use database::repositories::BookRepository;
        let pool = expect_context::<sqlx::PgPool>();
        let repo = BookRepository::new(pool);
        let uuid = uuid::Uuid::parse_str(&id).map_err(|e| ServerFnError::new(e.to_string()))?;
        repo.delete_book(uuid).await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}
