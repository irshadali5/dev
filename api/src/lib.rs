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

        let conn = expect_context::<database::DatabaseConnection>();
        let service = PortfolioService::new(conn);
        
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

#[server(GetProject, "/api")]
pub async fn get_project(slug: String) -> Result<Project, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use portfolio::service::PortfolioService;

        let conn = expect_context::<database::DatabaseConnection>();
        let service = PortfolioService::new(conn);
        
        let project = service.get_project_by_slug(&slug).await
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .ok_or_else(|| ServerFnError::new("Project not found".to_string()))?;
            
        Ok(project)
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
        let conn = expect_context::<database::DatabaseConnection>();
        let service = books::service::BookService::new(conn);
        let mut books = service.get_reading_list().await.unwrap_or_default();
        
        // Merge filesystem books in content/books/
        if let Ok(fs_books) = service.list_books(std::path::Path::new("content/books")) {
            for b in fs_books {
                if !books.iter().any(|existing| existing.slug == b.slug) {
                    books.push(Book {
                        id: uuid::Uuid::new_v4(),
                        slug: b.slug,
                        title: b.title,
                        description: Some(b.description),
                        author_id: uuid::Uuid::nil(),
                        created_at: chrono::Utc::now(),
                        updated_at: chrono::Utc::now(),
                    });
                }
            }
        }
        
        Ok(books)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetBookManifest, "/api")]
pub async fn get_book_manifest(slug: String) -> Result<domain::models::BookMeta, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let file_path = std::path::PathBuf::from(format!("content/books/{}/book.ron", slug));
        if !file_path.exists() {
            return Err(ServerFnError::new("Book manifest not found"));
        }
        let ron_content = std::fs::read_to_string(file_path).map_err(|e| ServerFnError::new(e.to_string()))?;
        let metadata: domain::models::BookMeta = ron::de::from_str(&ron_content).map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(metadata)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetBookChapter, "/api")]
pub async fn get_book_chapter(book_slug: String, chapter_file: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let file_path = std::path::PathBuf::from(format!("content/books/{}/{}", book_slug, chapter_file));
        if !file_path.exists() {
            return Err(ServerFnError::new("Chapter not found"));
        }
        let md_content = std::fs::read_to_string(file_path).map_err(|e| ServerFnError::new(e.to_string()))?;
        
        // Strip frontmatter
        let content = if md_content.starts_with("---") {
            if let Some(end_idx) = md_content[3..].find("---") {
                &md_content[3 + end_idx + 3..]
            } else {
                &md_content
            }
        } else {
            &md_content
        };

        let html_output = markdown::MarkdownService::parse_to_html(content.trim());
        Ok(html_output)
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

        let conn = expect_context::<database::DatabaseConnection>();
        let service = ArticleService::new(conn);
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

        let conn = expect_context::<database::DatabaseConnection>();
        let service = ArticleService::new(conn.clone());
        let article = service.get_article(&slug).await.map_err(|e| {
            let err: ServerFnError = ServerFnError::new(e.to_string());
            err
        })?;
        
        let analytics = analytics::AnalyticsService::new(conn);
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
    if name.is_empty() || email.is_empty() || message.is_empty() {
        return Err(ServerFnError::new("All fields are required".to_string()));
    }
    
    #[cfg(feature = "ssr")]
    {
        use database::repositories::ContactRepository;
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = ContactRepository::new(conn);
        repo.save_message(&name, &email, &message).await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(true)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetArticleLikes, "/api")]
pub async fn get_article_likes(slug: String) -> Result<u64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use database::repositories::ArticleRepository;
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = ArticleRepository::new(conn);
        repo.get_likes(&slug).await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = slug;
        unreachable!()
    }
}

#[server(LikeArticle, "/api")]
pub async fn like_article(slug: String) -> Result<u64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use database::repositories::ArticleRepository;
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = ArticleRepository::new(conn);
        repo.increment_likes(&slug).await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = slug;
        unreachable!()
    }
}

#[server(SubscribeNewsletter, "/api")]
pub async fn subscribe_newsletter(email: String) -> Result<bool, ServerFnError> {
    if email.is_empty() || !email.contains('@') {
        return Err(ServerFnError::new("Valid email address is required".to_string()));
    }
    
    #[cfg(feature = "ssr")]
    {
        use database::repositories::NewsletterRepository;
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = NewsletterRepository::new(conn);
        repo.subscribe(&email).await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(true)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetContactMessages, "/api")]
pub async fn get_contact_messages() -> Result<Vec<domain::models::ContactMessage>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        verify_admin().await?;
        use database::repositories::ContactRepository;
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = ContactRepository::new(conn);
        let msgs = repo.get_messages().await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(msgs)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(RunRustCode, "/api")]
pub async fn run_rust_code(code: String) -> Result<domain::models::PlaygroundResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use serde_json::json;
        
        let client = reqwest::Client::new();
        let payload = json!({
            "channel": "stable",
            "mode": "debug",
            "edition": "2021",
            "crateType": "bin",
            "tests": false,
            "code": code,
            "backtrace": false
        });

        let resp = client.post("https://play.rust-lang.org/execute")
            .json(&payload)
            .send()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to connect to Rust Playground: {}", e)))?;

        let res_json: serde_json::Value = resp.json()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to parse response: {}", e)))?;

        let success = res_json.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
        let stdout = res_json.get("stdout").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let stderr = res_json.get("stderr").and_then(|v| v.as_str()).unwrap_or("").to_string();

        Ok(domain::models::PlaygroundResponse {
            success,
            stdout,
            stderr,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetGithubActivity, "/api")]
pub async fn get_github_activity() -> Result<Vec<domain::models::GithubActivityItem>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use shared::MemoryCache;
        let cache_key = "github_activity_events".to_string();
        if let Some(items) = MemoryCache::get_postcard::<Vec<domain::models::GithubActivityItem>>(&cache_key) {
            return Ok(items);
        }

        let cfg = config::SiteConfig::load();
        let gh_user = cfg
            .github_url
            .trim_end_matches('/')
            .rsplit('/')
            .next()
            .filter(|s| !s.is_empty())
            .unwrap_or("username");

        let client = reqwest::Client::new();
        let api_url = format!("https://api.github.com/users/{}/events/public", gh_user);
        let resp = client.get(&api_url)
            .header("User-Agent", "dev-portfolio-app")
            .send()
            .await;

        let mut results = Vec::new();
        if let Ok(res) = resp {
            if let Ok(events) = res.json::<serde_json::Value>().await {
                if let Some(arr) = events.as_array() {
                    for ev in arr.iter().take(6) {
                        let event_type = ev.get("type").and_then(|v| v.as_str()).unwrap_or("PushEvent").to_string();
                        let repo = ev.get("repo").and_then(|r| r.get("name")).and_then(|v| v.as_str()).unwrap_or("dev").to_string();
                        let created_at = ev.get("created_at").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        
                        let message = ev.get("payload")
                            .and_then(|p| p.get("commits"))
                            .and_then(|c| c.as_array())
                            .and_then(|a| a.first())
                            .and_then(|commit| commit.get("message"))
                            .and_then(|m| m.as_str())
                            .unwrap_or("Repository update")
                            .to_string();

                        let url = format!("https://github.com/{}", repo);

                        results.push(domain::models::GithubActivityItem {
                            repo,
                            event_type,
                            message,
                            created_at,
                            url,
                        });
                    }
                }
            }
        }

        if results.is_empty() {
            results.push(domain::models::GithubActivityItem {
                repo: format!("{}/dev", gh_user),
                event_type: "PushEvent".to_string(),
                message: "feat: high performance distributed systems architecture & full text search".to_string(),
                created_at: "Recent".to_string(),
                url: cfg.github_url,
            });
        }

        MemoryCache::insert_postcard(cache_key, &results);

        Ok(results)
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
        let conn = expect_context::<database::DatabaseConnection>();
        let analytics = analytics::AnalyticsService::new(conn);
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
        let conn = expect_context::<database::DatabaseConnection>();
        let analytics_service = analytics::AnalyticsService::new(conn);
        let total = analytics_service.get_total_views().await.unwrap_or(0);
        
        let ui_views = vec![UiPageView {
            slug: "total_views".to_string(),
            view_count: total as i64,
        }];
        
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
        let conn = expect_context::<database::DatabaseConnection>();
        search_service.index_articles(conn).await.map_err(|e| {
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
    
    let conn = expect_context::<database::DatabaseConnection>();
    let auth_service = AuthService::new(conn, get_jwt_secret());
    
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
        use leptos_axum::ResponseOptions;

        let conn = expect_context::<database::DatabaseConnection>();
        let secret = get_jwt_secret();
        
        let auth_service = AuthService::new(conn, secret);
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
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = PortfolioRepository::new(conn);
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
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = PortfolioRepository::new(conn);
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
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = PortfolioRepository::new(conn);
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
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = PortfolioRepository::new(conn);
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
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = PortfolioRepository::new(conn);
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
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = PortfolioRepository::new(conn);
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
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = ProjectRepository::new(conn);
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
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = ProjectRepository::new(conn);
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
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = ArticleRepository::new(conn);
        let author_id = uuid::Uuid::nil();
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
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = ArticleRepository::new(conn);
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
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = BookRepository::new(conn);
        let author_id = uuid::Uuid::nil();
        repo.add_book(&slug, &title, description, author_id)
            .await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(UpdateProject, "/api")]
pub async fn update_project(id: String, slug: String, title: String, description: String, content_html: String, github_url: Option<String>, live_url: Option<String>) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        verify_admin().await?;
        use database::repositories::ProjectRepository;
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = ProjectRepository::new(conn);
        let uuid = uuid::Uuid::parse_str(&id).map_err(|e| ServerFnError::new(e.to_string()))?;
        repo.update_project(uuid, &slug, &title, &description, &content_html, github_url, live_url)
            .await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(UpdateArticle, "/api")]
pub async fn update_article(id: String, slug: String, title: String, description: String, body: String, published: bool) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        verify_admin().await?;
        use database::repositories::ArticleRepository;
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = ArticleRepository::new(conn);
        let uuid = uuid::Uuid::parse_str(&id).map_err(|e| ServerFnError::new(e.to_string()))?;
        repo.update_article(uuid, &slug, &title, &description, &body, published)
            .await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(UpdateBook, "/api")]
pub async fn update_book(id: String, slug: String, title: String, description: Option<String>) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        verify_admin().await?;
        use database::repositories::BookRepository;
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = BookRepository::new(conn);
        let uuid = uuid::Uuid::parse_str(&id).map_err(|e| ServerFnError::new(e.to_string()))?;
        repo.update_book(uuid, &slug, &title, description)
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
        let conn = expect_context::<database::DatabaseConnection>();
        let repo = BookRepository::new(conn);
        let uuid = uuid::Uuid::parse_str(&id).map_err(|e| ServerFnError::new(e.to_string()))?;
        repo.delete_book(uuid).await.map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetSiteProfile, "/api")]
pub async fn get_site_profile() -> Result<domain::models::SiteProfile, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let cfg = config::SiteConfig::load();
        Ok(domain::models::SiteProfile {
            title: cfg.title,
            short_name: cfg.short_name,
            tagline: cfg.tagline,
            author: cfg.author,
            handle: cfg.handle,
            role: cfg.role,
            email: cfg.email,
            bio: cfg.bio,
            site_url: cfg.site_url,
            github_url: cfg.github_url,
            linkedin_url: cfg.linkedin_url,
            twitter_url: cfg.twitter_url,
            twitter_handle: cfg.twitter_handle,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetAboutHtml, "/api")]
pub async fn get_about_html() -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let path = std::path::Path::new("content/about.md");
        if path.exists() {
            let md = std::fs::read_to_string(path).map_err(|e| ServerFnError::new(e.to_string()))?;
            Ok(markdown::MarkdownService::parse_to_html(&md))
        } else {
            Ok("<p>About content not configured yet. Add <code>content/about.md</code>.</p>".to_string())
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetResumeSummary, "/api")]
pub async fn get_resume_summary() -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        #[derive(serde::Deserialize)]
        struct ResumeSummaryOnly {
            #[serde(default)]
            summary: String,
        }
        let path = std::path::Path::new("content/resume.ron");
        if path.exists() {
            let raw = std::fs::read_to_string(path).map_err(|e| ServerFnError::new(e.to_string()))?;
            if let Ok(parsed) = ron::from_str::<ResumeSummaryOnly>(&raw) {
                return Ok(parsed.summary);
            }
        }
        Ok(String::new())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetUsesConfig, "/api")]
pub async fn get_uses_config() -> Result<domain::models::UsesConfig, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let path = std::path::Path::new("content/uses.ron");
        if path.exists() {
            let raw = std::fs::read_to_string(path).map_err(|e| ServerFnError::new(e.to_string()))?;
            let parsed: domain::models::UsesConfig =
                ron::from_str(&raw).map_err(|e| ServerFnError::new(e.to_string()))?;
            return Ok(parsed);
        }
        Ok(domain::models::UsesConfig::default())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[server(GetHomeConfig, "/api")]
pub async fn get_home_config() -> Result<domain::models::HomeConfig, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let path = std::path::Path::new("content/home.ron");
        if path.exists() {
            let raw = std::fs::read_to_string(path).map_err(|e| ServerFnError::new(e.to_string()))?;
            let parsed: domain::models::HomeConfig =
                ron::from_str(&raw).map_err(|e| ServerFnError::new(e.to_string()))?;
            return Ok(parsed);
        }
        Ok(domain::models::HomeConfig::default())
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}
