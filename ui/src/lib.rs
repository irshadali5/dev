use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use api::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    
    let (theme, set_theme) = create_signal("light".to_string());
    
    let toggle_theme = move |_| {
        set_theme.update(|t| {
            *t = if t == "dark" { "light".to_string() } else { "dark".to_string() };
        });
    };

    view! {
        <Html lang="en" attr:data-theme=move || theme.get()/>
        <Stylesheet id="leptos" href="/pkg/dev-site.css"/>
        <Title text="Irshad - Developer & Writer"/>
        // Highlight.js
        <Link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/atom-one-dark.min.css"/>
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"/>
        <Script>"document.addEventListener('DOMContentLoaded', (event) => { document.querySelectorAll('pre code').forEach((block) => { hljs.highlightElement(block); }); });"</Script>

        <Router>
            <nav class="navbar" id="navbar">
                <div class="nav-container">
                    <A href="/" class="nav-logo">
                        <span class="logo-bracket">"<"</span>"IA"<span class="logo-bracket">"/>"</span>
                    </A>
                    <div class="nav-links" id="navLinks">
                        <A href="/" class="nav-link" active_class="active" exact=true>"Home"</A>
                        <A href="/projects" class="nav-link" active_class="active">"Portfolio"</A>
                        <A href="/articles" class="nav-link" active_class="active">"Articles"</A>
                        <A href="/books" class="nav-link" active_class="active">"Books"</A>
                        <A href="/about" class="nav-link" active_class="active">"About"</A>
                    </div>
                    <div class="nav-actions">
                        <button class="theme-toggle" id="themeToggle" aria-label="Toggle theme" on:click=toggle_theme>
                            <svg class="icon-sun" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="12" cy="12" r="5"></circle>
                                <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"></path>
                            </svg>
                            <svg class="icon-moon" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                            </svg>
                        </button>
                        <button class="mobile-menu-btn" id="mobileMenuBtn" aria-label="Menu">
                            <span></span><span></span><span></span>
                        </button>
                    </div>
                </div>
            </nav>
            <main id="app">
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
            </main>
            <footer class="terminal-footer" role="contentinfo">
                <div class="terminal-window">
                    <div class="terminal-header" aria-hidden="true">
                        <div class="terminal-controls">
                            <span class="control red"></span>
                            <span class="control yellow"></span>
                            <span class="control green"></span>
                        </div>
                        <div class="terminal-title">"irshad@portfolio:~$ cat ./system/info.md"</div>
                    </div>
                    <div class="terminal-body">
                        <div class="terminal-grid">
                            <div class="terminal-block">
                                <div class="terminal-comment">"# SYSTEM_IDENTITY"</div>
                                <div class="terminal-prompt">
                                    <span class="user">"irshad"</span><span class="at">"@"</span><span class="host">"dev"</span><span class="path">"~$"</span> <span class="cmd">"whoami"</span>
                                </div>
                                <div class="terminal-output">
                                    <span class="accent">"<Irshad Ali />"</span>
                                    "Systems Programmer & AI Engineer"<br/>
                                    "C++ • Rust • SQL • GNU/Linux"
                                </div>
                                <div class="terminal-prompt">
                                    <span class="user">"irshad"</span><span class="at">"@"</span><span class="host">"dev"</span><span class="path">"~$"</span> <span class="cmd">"echo $STATUS"</span>
                                </div>
                                <div class="terminal-output success">"[AVAILABLE FOR HIRE]"</div>
                            </div>
                            <div class="terminal-block">
                                <div class="terminal-comment">"# NAVIGATION"</div>
                                <div class="terminal-prompt">
                                    <span class="user">"irshad"</span><span class="at">"@"</span><span class="host">"dev"</span><span class="path">"~$"</span> <span class="cmd">"ls -la"</span>
                                </div>
                                <div class="terminal-output">
                                    <A href="/" class="terminal-link">"drwxr-xr-x  home/"</A>
                                    <A href="/projects" class="terminal-link">"drwxr-xr-x  portfolio/"</A>
                                    <A href="/articles" class="terminal-link">"drwxr-xr-x  articles/"</A>
                                    <A href="/books" class="terminal-link">"drwxr-xr-x  books/"</A>
                                    <A href="/about" class="terminal-link">"-rw-r--r--  about.md"</A>
                                </div>
                            </div>
                            <div class="terminal-block">
                                <div class="terminal-comment">"# NETWORK_PORTS"</div>
                                <div class="terminal-prompt">
                                    <span class="user">"irshad"</span><span class="at">"@"</span><span class="host">"dev"</span><span class="path">"~$"</span> <span class="cmd">"netstat -tulpn"</span>
                                </div>
                                <div class="terminal-output">
                                    <a href="https://github.com/irshadali5" target="_blank" rel="noopener" class="terminal-link">"tcp 0 0 github.com:443 ESTABLISHED"</a>
                                    <a href="https://linkedin.com/in/irshadali" target="_blank" rel="noopener" class="terminal-link">"tcp 0 0 linkedin.com:443 ESTABLISHED"</a>
                                    <a href="mailto:hello@irshadali.dev" class="terminal-link">"tcp 0 0 smtp.mail:587 LISTENING"</a>
                                </div>
                            </div>
                            <div class="terminal-block">
                                <div class="terminal-comment">"# SKILL_TREE (Beginner → Advanced)"</div>
                                <div class="terminal-prompt">
                                    <span class="user">"irshad"</span><span class="at">"@"</span><span class="host">"dev"</span><span class="path">"~$"</span> <span class="cmd">"./show_progress.sh"</span>
                                </div>
                                <div class="terminal-output skill-tree">
                                    <div class="skill-level"><span class="lvl-num">"[LVL 1]"</span> <span class="lvl-name">"Bash/Linux"</span> <span class="lvl-bar">"██████████"</span> <span class="lvl-pct">"100%"</span></div>
                                    <div class="skill-level"><span class="lvl-num">"[LVL 2]"</span> <span class="lvl-name">"SQL/Indexing"</span> <span class="lvl-bar">"██████████"</span> <span class="lvl-pct">"100%"</span></div>
                                    <div class="skill-level"><span class="lvl-num">"[LVL 3]"</span> <span class="lvl-name">"C++ & STL"</span> <span class="lvl-bar">"████████░░"</span> <span class="lvl-pct">"85%"</span></div>
                                    <div class="skill-level"><span class="lvl-num">"[LVL 4]"</span> <span class="lvl-name">"Rust/Safety"</span> <span class="lvl-bar">"███████░░░"</span> <span class="lvl-pct">"75%"</span></div>
                                    <div class="skill-level"><span class="lvl-num">"[LVL 5]"</span> <span class="lvl-name">"AI/Prompts"</span> <span class="lvl-bar">"█████████░"</span> <span class="lvl-pct">"90%"</span></div>
                                </div>
                            </div>
                        </div>
                        <div class="terminal-footer-bottom">
                            <div class="terminal-prompt">
                                <span class="user">"irshad"</span><span class="at">"@"</span><span class="host">"dev"</span><span class="path">"~$"</span> <span class="cmd">"uptime"</span>
                            </div>
                            <div class="terminal-output">
                                "© 2026 Irshad Ali. System Uptime: " <span class="uptime-val">"5y 14d 08h 32m"</span>". Compiled with Leptos & Axum. "<span class="cursor" aria-hidden="true"></span>
                            </div>
                        </div>
                    </div>
                </div>
            </footer>
        </Router>
    }
}

#[component]
fn ExperienceTimeline() -> impl IntoView {
    view! {
        <section class="page-section container" style="margin-top: 80px;">
            <div class="section-header">
                <span class="section-label">"Background"</span>
                <h2 class="section-title">"Experience & Education"</h2>
            </div>
            
            <div class="timeline-container">
                <div class="timeline-item">
                    <div class="timeline-content">
                        <span class="timeline-date">"2023 - Present"</span>
                        <h3 class="timeline-title">"Recent Graduate / Junior Developer"</h3>
                        <div class="timeline-company">"Available for Hire"</div>
                        <p class="timeline-desc">"Actively seeking roles in Systems Engineering, Backend Development, or Full-Stack Engineering. Strong foundation in Rust, C++, and PostgreSQL."</p>
                    </div>
                </div>
                
                <div class="timeline-item">
                    <div class="timeline-content">
                        <span class="timeline-date">"2022 - 2023"</span>
                        <h3 class="timeline-title">"Open Source Contributor"</h3>
                        <div class="timeline-company">"Various Projects"</div>
                        <p class="timeline-desc">"Contributed to open source systems and libraries. Built custom high-performance data structures in Rust and C++ for personal and academic projects."</p>
                    </div>
                </div>
                
                <div class="timeline-item">
                    <div class="timeline-content">
                        <span class="timeline-date">"2019 - 2023"</span>
                        <h3 class="timeline-title">"B.S. Computer Science"</h3>
                        <div class="timeline-company">"University Name"</div>
                        <p class="timeline-desc">"Focused on Systems Programming, Database Architecture, and Distributed Systems. Graduated with Honors."</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn ContactForm() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (message, set_message) = create_signal(String::new());
    let (status, set_status) = create_signal(Option::<bool>::None);
    let (is_submitting, set_submitting) = create_signal(false);

    let submit_form = create_action(move |(n, e, m): &(String, String, String)| {
        let n = n.clone();
        let e = e.clone();
        let m = m.clone();
        async move {
            set_submitting.set(true);
            let res = submit_contact_form(n, e, m).await;
            set_submitting.set(false);
            set_status.set(Some(res.is_ok()));
            if res.is_ok() {
                set_name.set(String::new());
                set_email.set(String::new());
                set_message.set(String::new());
            }
        }
    });

    view! {
        <section class="page-section container">
            <div class="section-header" style="text-align: center;">
                <span class="section-label">"Get In Touch"</span>
                <h2 class="section-title">"Ready to build something great?"</h2>
                <p class="section-subtitle mx-auto">"Fill out the form below or email me directly. I'll get back to you within 24 hours."</p>
            </div>
            
            <div class="contact-form-container">
                <form on:submit=move |ev| {
                    ev.prevent_default();
                    submit_form.dispatch((name.get(), email.get(), message.get()));
                }>
                    <div class="form-field">
                        <label for="name">"Name"</label>
                        <input id="name" type="text" required
                            prop:value=name
                            on:input=move |ev| set_name.set(event_target_value(&ev)) />
                    </div>
                    
                    <div class="form-field">
                        <label for="email">"Email Address"</label>
                        <input id="email" type="email" required
                            prop:value=email
                            on:input=move |ev| set_email.set(event_target_value(&ev)) />
                    </div>
                    
                    <div class="form-field">
                        <label for="message">"Message"</label>
                        <textarea id="message" required
                            prop:value=message
                            on:input=move |ev| set_message.set(event_target_value(&ev))></textarea>
                    </div>
                    
                    <button type="submit" class="form-submit-btn" disabled=move || is_submitting.get()>
                        {move || if is_submitting.get() { "Sending..." } else { "Send Message" }}
                    </button>
                    
                    {move || match status.get() {
                        Some(true) => view! { <div class="form-feedback success">"Message sent successfully!"</div> }.into_view(),
                        Some(false) => view! { <div class="form-feedback error">"Failed to send message. Please try again."</div> }.into_view(),
                        None => view! {}.into_view(),
                    }}
                </form>
            </div>
        </section>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <section class="hero page-section container animate-slide-up">
            <div class="hero-content">
                <span class="section-label" style="display: inline-block;">"Hi, my name is"</span>
                <h1 class="hero-title">
                    <span class="gradient-text">"Irshad Ali."</span>
                </h1>
                <h2 class="hero-subtitle" style="font-size: clamp(2rem, 5vw, 4rem); font-weight: 700; color: var(--text-secondary); margin-bottom: 2rem;">
                    "I engineer reliable systems."
                </h2>
                <p class="hero-subtitle">
                    "I am a software engineer specializing in backend development, systems programming, and AI pipelines. I focus on writing high-performance, safe, and scalable code."
                </p>
                <div class="hero-actions">
                    <a href="#" class="btn btn-primary">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></svg>
                        "Download Resume"
                    </a>
                    <A href="/projects" class="btn btn-secondary">
                        "View My Work"
                    </A>
                </div>
            </div>
        </section>
        
        <ExperienceTimeline />

        <section class="page-section container" style="margin-top: 80px;">
            <div class="section-header">
                <span class="section-label">"Expertise"</span>
                <h2 class="section-title">"Technical Skills"</h2>
            </div>
            
            <div class="skills-grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 2rem;">
                <div class="skill-category" style="background-color: var(--bg-card); padding: 2rem; border-radius: var(--radius-lg); border: 1px solid var(--border); box-shadow: var(--shadow-sm);">
                    <h3 style="color: var(--text-primary); font-size: 1.25rem; margin-bottom: 1.5rem; font-weight: 600;">"Languages & Frameworks"</h3>
                    <ul style="list-style: none; padding: 0; margin: 0; color: var(--text-tertiary);">
                        <li style="margin-bottom: 0.5rem; display: flex; align-items: center; gap: 0.5rem;"><span>"▹"</span> "Rust (Leptos, Axum, Tokio)"</li>
                        <li style="margin-bottom: 0.5rem; display: flex; align-items: center; gap: 0.5rem;"><span>"▹"</span> "C++ (C++20, STL, Boost)"</li>
                        <li style="margin-bottom: 0.5rem; display: flex; align-items: center; gap: 0.5rem;"><span>"▹"</span> "Python (FastAPI, PyTorch)"</li>
                        <li style="margin-bottom: 0.5rem; display: flex; align-items: center; gap: 0.5rem;"><span>"▹"</span> "TypeScript & JavaScript"</li>
                    </ul>
                </div>
                <div class="skill-category" style="background-color: var(--bg-card); padding: 2rem; border-radius: var(--radius-lg); border: 1px solid var(--border); box-shadow: var(--shadow-sm);">
                    <h3 style="color: var(--text-primary); font-size: 1.25rem; margin-bottom: 1.5rem; font-weight: 600;">"Databases & Cloud"</h3>
                    <ul style="list-style: none; padding: 0; margin: 0; color: var(--text-tertiary);">
                        <li style="margin-bottom: 0.5rem; display: flex; align-items: center; gap: 0.5rem;"><span>"▹"</span> "PostgreSQL & pgvector"</li>
                        <li style="margin-bottom: 0.5rem; display: flex; align-items: center; gap: 0.5rem;"><span>"▹"</span> "Redis & Memcached"</li>
                        <li style="margin-bottom: 0.5rem; display: flex; align-items: center; gap: 0.5rem;"><span>"▹"</span> "AWS (EC2, S3, RDS)"</li>
                        <li style="margin-bottom: 0.5rem; display: flex; align-items: center; gap: 0.5rem;"><span>"▹"</span> "Docker & Linux Administration"</li>
                    </ul>
                </div>
            </div>
        </section>

        <ContactForm />
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div class="page-section container animate-slide-up">
            <h1 class="page-title">"About Me"</h1>
            <div class="content-prose">
                <p class="mb-4">"I build high-performance distributed systems and web applications, predominantly using Rust."</p>
                <p>"This site itself is built using Leptos, Axum, and custom Vanilla CSS."</p>
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
        <div class="page-section animate-fade-in container">
            <h1 class="section-title">"Projects"</h1>
            <Transition fallback=move || view! { 
                <div class="projects-grid">
                    <div class="skeleton-loader project-card" style="height: 180px;"></div>
                    <div class="skeleton-loader project-card" style="height: 180px;"></div>
                </div> 
            }>
                {move || {
                    projects.get().map(|projs| {
                        if projs.is_empty() {
                            view! { <p class="text-muted">"No projects found."</p> }.into_view()
                        } else {
                            view! {
                                <div class="projects-grid">
                                    {projs.into_iter().map(|p| view! {
                                        <div class="project-card">
                                            <div class="project-content">
                                                <h2 class="project-title">{p.title}</h2>
                                                <p class="project-desc">{p.description}</p>
                                                <div class="project-tags">
                                                    {p.tags.into_iter().map(|t| view! {
                                                        <span class="tag">{t}</span>
                                                    }).collect_view()}
                                                </div>
                                                <div class="project-links">
                                                    {if let Some(url) = p.github_url {
                                                        Some(view! { <a href=url target="_blank" class="project-link">"GitHub"</a> })
                                                    } else { None }}
                                                    {if let Some(url) = p.live_url {
                                                        Some(view! { <a href=url target="_blank" class="project-link">"Live Demo"</a> })
                                                    } else { None }}
                                                </div>
                                            </div>
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
        <div class="page-section animate-fade-in container">
            <h1 class="section-title">"Reading List"</h1>
            <p class="section-subtitle">"Books I've read and recommend."</p>
            <Transition fallback=move || view! { 
                <div class="books-grid">
                    <div class="skeleton-loader book-card" style="height: 140px;"></div>
                    <div class="skeleton-loader book-card" style="height: 140px;"></div>
                    <div class="skeleton-loader book-card" style="height: 140px;"></div>
                </div> 
            }>
                {move || {
                    books.get().map(|bs| {
                        if bs.is_empty() {
                            view! { <p class="text-muted">"No books found."</p> }.into_view()
                        } else {
                            view! {
                                <div class="books-grid">
                                    {bs.into_iter().map(|b| view! {
                                        <div class="book-card">
                                            <div class="book-cover">"📚"</div>
                                            <div class="book-info">
                                                <h3>{b.title}</h3>
                                                {if let Some(desc) = b.description {
                                                    view! { <p>{desc}</p> }.into_view()
                                                } else { view! {}.into_view() }}
                                            </div>
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
        <div class="page-section animate-fade-in container">
            <div class="articles-header">
                <div>
                    <h1 class="section-title">"Articles"</h1>
                    <p class="section-subtitle">"Thoughts on Rust, Systems Engineering, and software architecture."</p>
                </div>
            </div>
            <Transition fallback=move || view! { 
                <div class="articles-grid">
                    <div class="skeleton-loader article-card" style="height: 120px;"></div>
                    <div class="skeleton-loader article-card" style="height: 120px;"></div>
                </div> 
            }>
                {move || {
                    articles.get().map(|articles| {
                        if articles.is_empty() {
                            view! { <p class="text-muted">"No articles found."</p> }.into_view()
                        } else {
                            view! {
                                <div class="articles-list">
                                    {articles.into_iter().map(|a| {
                                        let slug = a.slug.clone();
                                        view! {
                                            <article class="article-card animate-slide-up">
                                                <div class="article-meta">
                                                    <span>"Article"</span>
                                                    <span class="dot">"•"</span>
                                                    <span><time>{a.created_at.to_rfc3339()}</time></span>
                                                </div>
                                                <h2 class="article-title"><A href=format!("/writing/{}", slug)>
                                                    {a.title}
                                                </A></h2>
                                                <p class="article-excerpt">{a.description}</p>
                                                <div class="article-tags">
                                                    {a.tags.into_iter().map(|t| view! {
                                                        <span class="tag">{t}</span>
                                                    }).collect_view()}
                                                </div>
                                            </article>
                                        }
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
                            Some((art, views)) => {
                                // Now record view when loaded
                                create_effect(move |_| {
                                    let slug_val = slug();
                                    if !slug_val.is_empty() {
                                        spawn_local(async move {
                                            let _ = record_view(slug_val).await;
                                        });
                                    }
                                });
                
                                view! {
                                    <Title text=art.title.clone() />
                                    <Meta name="description" content=art.description.clone().unwrap_or_default() />
                                    <article class="article-full container">
                                        <header class="article-header">
                                            <h1 class="article-main-title">{art.title}</h1>
                                            <div class="article-meta-row">
                                                <time>{art.created_at.to_rfc3339()}</time>
                                                <span class="dot">"•"</span>
                                                <span>{views} " views"</span>
                                            </div>
                                        </header>
                                        <div class="article-content prose" inner_html=art.body></div>
                                        
                                        <footer class="article-footer">
                                            <div class="article-tags">
                                                {art.tags.into_iter().map(|t| view! {
                                                    <span class="tag">{t}</span>
                                                }).collect_view()}
                                            </div>
                                            
                                            <div class="article-nav">
                                                <A href="/writing" class="back-link">"← Back to Writing"</A>
                                            </div>
                                        </footer>
                                    </article>
                                }.into_view()
                            },
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

// Login function moved to api

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
