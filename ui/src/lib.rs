use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_router::*;
use portfolio::models::Project;
use domain::models::Book;
use articles::models::Article;
use serde::{Serialize, Deserialize};

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
        
        // Map search::SearchResult to UiSearchResult
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
        use std::path::PathBuf;

        // Use current directory content/ as base path
        let content_dir = PathBuf::from("content");
        let service = PortfolioService::new(content_dir);
        
        let projects = service.get_all_projects().map_err(|e| {
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
        use std::path::PathBuf;

        let content_dir = PathBuf::from("content");
        let service = ArticleService::new(content_dir);
        let articles = service.get_all_articles().map_err(|e| {
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
        use std::path::PathBuf;

        let content_dir = PathBuf::from("content");
        let service = ArticleService::new(content_dir);
        let article = service.get_article(&slug).map_err(|e| {
            let err: ServerFnError = ServerFnError::ServerError(e.to_string());
            err
        })?;
        
        let pool = expect_context::<sqlx::PgPool>();
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
        // For now, assume true, we will lock it down later.
        Ok(true)
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en"/>
        <Stylesheet id="leptos" href="/pkg/dev-site.css"/>
        <Title text="Irshad - Developer & Writer"/>
        // Highlight.js
        <Link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/atom-one-dark.min.css"/>
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"/>
        <Script>"document.addEventListener('DOMContentLoaded', (event) => { document.querySelectorAll('pre code').forEach((block) => { hljs.highlightElement(block); }); });"</Script>

        <Router>
            <main class="app-layout">
                <nav class="app-navbar glass-panel">
                    <div class="navbar-brand bg-gradient-primary">
                        <A href="/">"Irshad"</A>
                    </div>
                    <div class="navbar-links">
                        <A href="/about" class="nav-link" active_class="nav-link-active">"About"</A>
                        <A href="/projects" class="nav-link" active_class="nav-link-active">"Projects"</A>
                        <A href="/books" class="nav-link" active_class="nav-link-active">"Books"</A>
                        <A href="/articles" class="nav-link" active_class="nav-link-active">"Articles"</A>
                    </div>
                    <div class="navbar-actions">
                        <A href="/search" class="icon-link">
                            <svg xmlns="http://www.w3.org/2000/svg" class="icon" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                            </svg>
                        </A>
                        <A href="/admin" class="nav-link text-sm">"Admin"</A>
                    </div>
                </nav>
                <div class="app-content">
                    <Routes>
                        <Route path="" view=HomePage/>
                        <Route path="about" view=AboutPage/>
                        <Route path="search" view=SearchPage/>
                        <Route path="projects" view=ProjectsPage/>
                        <Route path="books" view=BooksPage/>
                        <Route path="articles" view=ArticlesPage/>
                        <Route path="articles/:slug" view=ArticlePage/>
                        <Route path="login" view=LoginPage/>
                        <Route path="admin" view=AdminDashboard>
                            <Route path="" view=AdminOverview/>
                            <Route path="analytics" view=AdminAnalytics/>
                            <Route path="settings" view=AdminSettings/>
                        </Route>
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </div>
                <footer class="terminal-footer">
                    <div class="terminal-footer-content">
                        <p class="terminal-prompt">"cat sys_info.txt"</p>
                        <p>"OS: IrshadOS v1.0.0"</p>
                        <p>"Kernel: Rust x86_64"</p>
                        <p class="terminal-prompt">"ls -la links/"</p>
                        <p>
                            "drwxr-xr-x 2 irshad users 4096 "
                            <a href="https://github.com/irshad" target="_blank" class="terminal-link">"github"</a>
                            "  "
                            <a href="https://twitter.com/irshad" target="_blank" class="terminal-link">"twitter"</a>
                        </p>
                        <p class="terminal-prompt mt-4" style="color: #64748b; font-size: 0.75rem;">"© 2026 Irshad. EOF."</p>
                    </div>
                </footer>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="hero-section animate-slide-up">
            <h1 class="hero-title bg-gradient-secondary">
                "Irshad"
            </h1>
            <p class="hero-subtitle">
                "Software Engineer, Open Source Contributor, and Writer."
            </p>
            <div class="hero-actions">
                <A href="/projects" class="glass-button font-medium">"View Projects"</A>
                <A href="/articles" class="text-link">"Read Articles →"</A>
            </div>
        </div>
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div class="page-section animate-slide-up">
            <h1 class="page-title">"About Me"</h1>
            <div class="content-prose">
                <p class="mb-4">"I build high-performance distributed systems and web applications, predominantly using Rust."</p>
                <p>"This site itself is built using Leptos, Axum, and Tailwind CSS."</p>
            </div>
        </div>
    }
}

#[component]
fn ProjectsPage() -> impl IntoView {
    let projects = create_resource(|| (), |_| async move { get_projects().await.unwrap_or_default() });

    view! {
        <Title text="Projects - Irshad"/>
        <Meta name="description" content="Open source projects, libraries, and systems I've built."/>
        <div class="page-section animate-fade-in">
            <h1 class="page-title">"Projects"</h1>
            <Transition fallback=move || view! { 
                <div class="project-grid">
                    <div class="skeleton-loader glass-panel" style="height: 180px;"></div>
                    <div class="skeleton-loader glass-panel" style="height: 180px;"></div>
                </div> 
            }>
                {move || {
                    projects.get().map(|projs| {
                        if projs.is_empty() {
                            view! { <p class="text-muted">"No projects found."</p> }.into_view()
                        } else {
                            view! {
                                <div class="project-grid">
                                    {projs.into_iter().map(|p| view! {
                                        <div class="project-card glass-panel">
                                            <h2 class="project-title">{p.metadata.title}</h2>
                                            <p class="project-description">{p.metadata.description}</p>
                                            <div class="tag-list">
                                                {p.metadata.tags.into_iter().map(|t| view! { 
                                                    <span class="tag">{t}</span> 
                                                }).collect_view()}
                                            </div>
                                            <div class="project-prose" inner_html=p.content_html></div>
                                            {if let Some(url) = p.metadata.github_url {
                                                view! { <a href=url target="_blank" class="project-link">"View on GitHub →"</a> }.into_view()
                                            } else { view! {}.into_view() }}
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_view()
                        }
                    })
                }}
            </Transition>
        </div>
    }
}

#[component]
fn BooksPage() -> impl IntoView {
    let books = create_resource(|| (), |_| async move { get_books().await.unwrap_or_default() });

    view! {
        <div class="page-section animate-fade-in">
            <h1 class="page-title">"Reading List"</h1>
            <p class="page-subtitle">"Books I've read and recommend."</p>
            <Transition fallback=move || view! { 
                <div class="book-grid">
                    <div class="skeleton-loader glass-panel" style="height: 140px;"></div>
                    <div class="skeleton-loader glass-panel" style="height: 140px;"></div>
                    <div class="skeleton-loader glass-panel" style="height: 140px;"></div>
                </div> 
            }>
                {move || {
                    books.get().map(|bs| {
                        if bs.is_empty() {
                            view! { <p class="text-muted">"No books found."</p> }.into_view()
                        } else {
                            view! {
                                <div class="book-grid">
                                    {bs.into_iter().map(|b| view! {
                                        <div class="search-result-card glass-panel">
                                            <h2 class="book-title">{b.title}</h2>
                                            {if let Some(desc) = b.description {
                                                view! { <p class="book-description">{desc}</p> }.into_view()
                                            } else { view! {}.into_view() }}
                                        </div>
                                    }).collect_view()}
                                </div>
                            }.into_view()
                        }
                    })
                }}
            </Transition>
        </div>
    }
}

#[component]
fn ArticlesPage() -> impl IntoView {
    let articles = create_resource(|| (), |_| async move { get_articles().await.unwrap_or_default() });

    view! {
        <Title text="Articles - Irshad"/>
        <Meta name="description" content="Thoughts on Rust, Systems Engineering, and software architecture."/>
        <div class="page-section animate-fade-in">
            <h1 class="page-title">"Articles"</h1>
            <p class="page-subtitle">"Thoughts on Rust, Systems Engineering, and software architecture."</p>
            <Transition fallback=move || view! { 
                <div class="article-list">
                    <div class="skeleton-loader glass-panel" style="height: 120px;"></div>
                    <div class="skeleton-loader glass-panel" style="height: 120px;"></div>
                </div> 
            }>
                {move || {
                    articles.get().map(|arts| {
                        if arts.is_empty() {
                            view! { <p class="text-muted">"No articles found."</p> }.into_view()
                        } else {
                            view! {
                                <div class="article-list">
                                    {arts.into_iter().map(|a| view! {
                                        <article class="article-card glass-panel">
                                            <h2 class="article-title-wrap">
                                                <A href=format!("/articles/{}", a.metadata.slug) class="article-title-link">
                                                    {a.metadata.title}
                                                </A>
                                            </h2>
                                            <p class="project-description">{a.metadata.description}</p>
                                            <div class="article-meta">
                                                <time>{a.metadata.created_at}</time>
                                            </div>
                                        </article>
                                    }).collect_view()}
                                </div>
                            }.into_view()
                        }
                    })
                }}
            </Transition>
        </div>
    }
}

#[component]
fn ArticlePage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").cloned().unwrap_or_default());
    
    // We get a tuple: (Article, views)
    let article = create_resource(slug, |s| async move { get_article(s).await.ok() });
    
    // Create an effect to record view once on the client
    create_effect(move |_| {
        let current_slug = slug();
        if !current_slug.is_empty() {
            spawn_local(async move {
                let _ = record_view(current_slug).await;
            });
        }
    });

    view! {
        <div class="page-section max-w-3xl animate-fade-in">
            <Transition fallback=move || view! { 
                <div class="max-w-3xl mx-auto">
                    <div class="skeleton-loader skeleton-title" style="width: 70%; height: 3.5rem; margin-bottom: 2rem;"></div>
                    <div class="skeleton-loader skeleton-text" style="height: 1.5rem; margin-bottom: 1rem;"></div>
                    <div class="skeleton-loader skeleton-text" style="height: 1.5rem; margin-bottom: 1rem;"></div>
                    <div class="skeleton-loader skeleton-text short" style="height: 1.5rem; margin-bottom: 1rem;"></div>
                </div> 
            }>
                {move || {
                    article.get().map(|opt| {
                        match opt {
                            Some((art, views)) => view! {
                                <Title text=art.metadata.title.clone() />
                                <Meta name="description" content=art.metadata.description.clone() />
                                <article>
                                    <h1 class="article-main-title">{art.metadata.title}</h1>
                                    <div class="article-header-meta">
                                        <time>{art.metadata.created_at}</time>
                                        <span>" • " {views} " views"</span>
                                    </div>
                                    
                                    <div class="content-prose article-prose" inner_html=art.content_html></div>
                                    
                                    <div class="article-footer">
                                        <div class="tag-list">
                                            {art.metadata.tags.into_iter().map(|t| view! { 
                                                <span class="tag tag-rounded">{t}</span> 
                                            }).collect_view()}
                                        </div>
                                    </div>
                                    <Script>"setTimeout(() => document.querySelectorAll('.prose pre code').forEach(hljs.highlightElement), 100);"</Script>
                                </article>
                            }.into_view(),
                            None => view! { <NotFound/> }.into_view()
                        }
                    })
                }}
            </Transition>
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_axum::ResponseOptions>();
        resp.set_status(axum::http::StatusCode::NOT_FOUND);
    }

    view! {
        <div class="hero-section animate-fade-in">
            <h1 class="error-title">"404"</h1>
            <h2 class="error-subtitle">"Page Not Found"</h2>
            <p class="page-subtitle">"The page you are looking for does not exist or has been moved."</p>
            <A href="/" class="glass-button">"Return Home"</A>
        </div>
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

#[component]
pub fn LoginPage() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error_msg, set_error_msg) = create_signal(String::new());
    let (success_msg, set_success_msg) = create_signal(String::new());

    let login_action = create_action(move |(e, p): &(String, String)| {
        let e = e.clone();
        let p = p.clone();
        async move {
            match login(e, p).await {
                Ok(_) => {
                    set_success_msg.set("Login successful!".to_string());
                    set_error_msg.set(String::new());
                    // Normally we would redirect here
                }
                Err(err) => {
                    set_error_msg.set(format!("Login failed: {:?}", err));
                    set_success_msg.set(String::new());
                }
            }
        }
    });

    view! {
        <div class="auth-layout animate-slide-up">
            <div class="auth-panel glass-panel">
                <h1 class="auth-title">"Admin Login"</h1>
                <form on:submit=move |ev| {
                    ev.prevent_default();
                    login_action.dispatch((email.get(), password.get()));
                } class="auth-form">
                    <div class="admin-sidebar-menu">
                        <label class="form-label">"Email"</label>
                        <input type="email" class="input-field" on:input=move |ev| set_email.set(event_target_value(&ev)) prop:value=email/>
                    </div>
                    <div class="admin-sidebar-menu">
                        <label class="form-label">"Password"</label>
                        <input type="password" class="input-field" on:input=move |ev| set_password.set(event_target_value(&ev)) prop:value=password/>
                    </div>
                    <button type="submit" class="glass-button btn-primary btn-block">"Login"</button>
                </form>
                
                {move || {
                    let err = error_msg.get();
                    if !err.is_empty() {
                        view! { <p class="form-error">{err}</p> }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    }
                }}
                
                {move || {
                    let succ = success_msg.get();
                    if !succ.is_empty() {
                        view! { <p class="form-success">{succ}</p> }.into_view()
                    } else {
                        view! { <span></span> }.into_view()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
pub fn SearchPage() -> impl IntoView {
    let (query, set_query) = create_signal("".to_string());
    
    let search_results = create_resource(move || query.get(), |q| async move { 
        if q.is_empty() {
            Ok(vec![])
        } else {
            search_content(q).await 
        }
    });

    view! {
        <Title text="Search - Irshad"/>
        <Meta name="description" content="Search across articles, projects, and books."/>
        
        <div class="page-section max-w-3xl animate-fade-in">
            <h1 class="page-title">"Search"</h1>
            <div class="search-container">
                <input 
                    type="text" 
                    class="input-field search-input-large"
                    placeholder="Type to search articles, projects, and books..."
                    on:input=move |ev| {
                        set_query.set(event_target_value(&ev));
                    }
                    prop:value=query
                />
                <svg xmlns="http://www.w3.org/2000/svg" class="search-icon" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
            </div>
            
            <div class="search-results-wrap">
                <Transition fallback=move || view! { 
                    <div class="search-results-list mt-8">
                        <div class="skeleton-loader glass-panel" style="height: 100px;"></div>
                        <div class="skeleton-loader glass-panel" style="height: 100px;"></div>
                    </div> 
                }>
                    {move || {
                        search_results.get().map(|res| {
                            match res {
                                Ok(results) => {
                                    if results.is_empty() {
                                        if query.get().is_empty() {
                                            view! { <p class="search-empty-state">"Enter a search term above to find content."</p> }.into_view()
                                        } else {
                                            view! { <p class="search-empty-state">"No results found."</p> }.into_view()
                                        }
                                    } else {
                                        view! {
                                            <ul class="search-results-list">
                                                {results.into_iter().map(|r| {
                                                    let url = format!("/{}s/{}", r.doc_type, r.slug);
                                                    view! {
                                                        <li class="search-result-card glass-panel">
                                                            <a href=url class="search-result-layout">
                                                                <div>
                                                                    <h3 class="search-result-title">{r.title}</h3>
                                                                    <p class="search-result-snippet">{r.snippet}</p>
                                                                </div>
                                                                <span class="search-result-type">{r.doc_type}</span>
                                                            </a>
                                                        </li>
                                                    }
                                                }).collect_view()}
                                            </ul>
                                        }.into_view()
                                    }
                                }
                                Err(_) => view! { <p class="text-error">"An error occurred while searching."</p> }.into_view()
                            }
                        })
                    }}
                </Transition>
            </div>
        </div>
    }
}

#[component]
pub fn Authenticated(children: ChildrenFn) -> impl IntoView {
    let auth = create_resource(|| (), |_| async move { check_auth().await.unwrap_or(false) });
    
    let children_view = children().into_view();

    view! {
        <Transition fallback=move || view! { <p>"Checking authentication..."</p> }>
            {
                let children_view = children_view.clone();
                move || {
                    let children_view = children_view.clone();
                    auth.get().map(move |is_auth| {
                        if is_auth {
                            children_view.clone().into_view()
                        } else {
                            view! {
                                <div class="error-section animate-fade-in">
                                    <h1 class="text-4xl font-bold text-slate-100 mb-4">"Unauthorized"</h1>
                                    <p class="text-muted">"You must be logged in to view this page. "<a href="/login" class="text-link">"Login here"</a></p>
                                </div>
                            }.into_view()
                        }
                    })
                }
            }
        </Transition>
    }
}

#[component]
pub fn AdminDashboard() -> impl IntoView {
    view! {
        <Authenticated>
            <div class="admin-layout animate-fade-in">
                <nav class="admin-sidebar glass-panel">
                    <h2 class="admin-sidebar-title">"Admin Panel"</h2>
                    <ul class="admin-sidebar-menu">
                        <li><A href="/admin" exact=true class="admin-nav-link" active_class="bg-brand-500/20 text-brand-400 font-medium">"Overview"</A></li>
                        <li><A href="/admin/analytics" class="admin-nav-link" active_class="bg-brand-500/20 text-brand-400 font-medium">"Analytics"</A></li>
                        <li><A href="/admin/settings" class="admin-nav-link" active_class="bg-brand-500/20 text-brand-400 font-medium">"Settings"</A></li>
                    </ul>
                </nav>
                <div class="admin-content glass-panel">
                    <Outlet/>
                </div>
            </div>
        </Authenticated>
    }
}

#[component]
pub fn AdminOverview() -> impl IntoView {
    view! {
        <div class="animate-in fade-in">
            <div class="flex items-center justify-between mb-6" style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 1.5rem;">
                <h1 class="admin-page-title mb-0" style="margin-bottom: 0;">"System Dashboard"</h1>
                <span class="status-badge success">"System Online"</span>
            </div>
            
            <div class="dashboard-grid">
                <div class="metric-card glass-panel">
                    <span class="metric-title">"Database Connections"</span>
                    <span class="metric-value">"4"</span>
                    <span class="metric-trend up">"Healthy"</span>
                </div>
                <div class="metric-card glass-panel">
                    <span class="metric-title">"Indexed Articles"</span>
                    <span class="metric-value">"12"</span>
                    <span class="metric-trend info">"Tantivy Engine"</span>
                </div>
                <div class="metric-card glass-panel">
                    <span class="metric-title">"Unique Visitors (30d)"</span>
                    <span class="metric-value">"1,492"</span>
                    <span class="metric-trend up">"+12%"</span>
                </div>
                <div class="metric-card glass-panel">
                    <span class="metric-title">"API Latency"</span>
                    <span class="metric-value">"24ms"</span>
                    <span class="metric-trend info">"P95"</span>
                </div>
            </div>
            
            <div class="admin-card glass-panel mt-6" style="margin-top: 1.5rem;">
                <h2 class="admin-card-title">"Recent System Logs"</h2>
                <div class="table-responsive">
                    <table class="admin-table">
                        <tr class="table-header-row">
                            <th class="table-th">"Time"</th>
                            <th class="table-th">"Event"</th>
                            <th class="table-th">"Status"</th>
                        </tr>
                        <tr class="table-row">
                            <td class="table-td text-muted font-mono">"10:45 AM"</td>
                            <td class="table-td">"Database Backup Completed"</td>
                            <td class="table-td"><span class="status-badge success">"Success"</span></td>
                        </tr>
                        <tr class="table-row">
                            <td class="table-td text-muted font-mono">"09:12 AM"</td>
                            <td class="table-td">"Search Index Synced"</td>
                            <td class="table-td"><span class="status-badge info">"Info"</span></td>
                        </tr>
                        <tr class="table-row">
                            <td class="table-td text-muted font-mono">"08:30 AM"</td>
                            <td class="table-td">"Failed Login Attempt (admin)"</td>
                            <td class="table-td"><span class="status-badge warning">"Warning"</span></td>
                        </tr>
                    </table>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn AdminAnalytics() -> impl IntoView {
    let analytics = create_resource(|| (), |_| async move { get_analytics().await.unwrap_or_default() });
    
    view! {
        <div class="animate-in fade-in">
            <h1 class="admin-page-title">"Analytics"</h1>
            <Transition fallback=move || view! { 
                <div>
                    <div class="skeleton-loader skeleton-title"></div>
                    <div class="skeleton-loader skeleton-text"></div>
                    <div class="skeleton-loader skeleton-text short"></div>
                </div> 
            }>
                {move || {
                    analytics.get().map(|views| {
                        view! {
                            <div class="table-responsive">
                                <table class="admin-table">
                                    <thead>
                                        <tr class="table-header-row">
                                            <th class="table-th">"Path / Slug"</th>
                                            <th class="table-th text-right">"Views"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {views.into_iter().map(|v| view! {
                                            <tr class="table-row">
                                                <td class="table-td">{v.slug}</td>
                                                <td class="table-td text-right font-mono">{v.view_count}</td>
                                            </tr>
                                        }).collect_view()}
                                    </tbody>
                                </table>
                            </div>
                        }
                    })
                }}
            </Transition>
        </div>
    }
}

#[component]
pub fn AdminSettings() -> impl IntoView {
    let rebuild_action = create_action(|_: &()| async move {
        rebuild_search_index().await
    });
    
    view! {
        <div class="animate-in fade-in">
            <h1 class="admin-page-title">"Settings"</h1>
            <div class="admin-card glass-panel">
                <h2 class="admin-card-title">"Search Engine"</h2>
                <p class="text-slate-400 mb-6">"Rebuild the Tantivy search index from the current content directory."</p>
                <button 
                    on:click=move |_| rebuild_action.dispatch(()) 
                    disabled=rebuild_action.pending()
                    class="glass-button glow-button auto-width"
                >
                    {move || if rebuild_action.pending().get() { "Rebuilding..." } else { "Rebuild Index" }}
                </button>
                {move || {
                    rebuild_action.value().get().map(|res| {
                        match res {
                            Ok(_) => view! { <p class="text-green-400 text-sm mt-4">"Search index rebuilt successfully."</p> }.into_view(),
                            Err(e) => view! { <p class="text-red-400 text-sm mt-4">"Failed to rebuild index: " {e.to_string()}</p> }.into_view()
                        }
                    })
                }}
            </div>
        </div>
    }
}
