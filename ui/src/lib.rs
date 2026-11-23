use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use api::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    
    let (theme, set_theme) = create_signal("dark".to_string());
    
    // Theme initialization & sync on client
    create_effect(move |_| {
        #[cfg(not(feature = "ssr"))]
        {
            if let Some(win) = web_sys::window() {
                if let Ok(Some(storage)) = win.local_storage() {
                    if let Ok(Some(saved)) = storage.get_item("theme") {
                        set_theme.set(saved);
                    }
                }
            }
        }
    });

    let toggle_theme = move |_| {
        set_theme.update(|t| {
            let next = if t == "dark" { "light".to_string() } else { "dark".to_string() };
            #[cfg(not(feature = "ssr"))]
            {
                if let Some(win) = web_sys::window() {
                    if let Ok(Some(storage)) = win.local_storage() {
                        let _ = storage.set_item("theme", &next);
                    }
                }
            }
            *t = next;
        });
    };

    let (is_menu_open, set_menu_open) = create_signal(false);
    let toggle_menu = move |_| {
        set_menu_open.update(|open| *open = !*open);
    };

    let (is_palette_open, set_palette_open) = create_signal(false);

    // Global keyboard shortcut: Cmd+K / Ctrl+K / Escape
    let _ = window_event_listener(ev::keydown, move |ev: web_sys::KeyboardEvent| {
        let key = ev.key();
        if (ev.meta_key() || ev.ctrl_key()) && (key == "k" || key == "K") {
            ev.prevent_default();
            set_palette_open.update(|open| *open = !*open);
        } else if key == "Escape" {
            set_palette_open.set(false);
        }
    });

    view! {
        <Html lang="en" attr:data-theme=move || theme.get()/>
        <Stylesheet id="leptos" href="/pkg/dev-site.css"/>
        <Title text="Irshad - Developer & Writer"/>
        <Meta name="description" content="Irshad Ali — Software Engineer & Systems Architect. High-Performance Distributed Systems, Rust, and Cloud Architecture."/>
        <Meta property="og:site_name" content="Irshad Ali"/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:title" content="Irshad Ali — Software Engineer & Systems Architect"/>
        <Meta property="og:description" content="High-Performance Distributed Systems, Rust, and Cloud Architecture."/>
        <Meta property="og:image" content="/api/og"/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:creator" content="@irshadali5"/>
        <Meta name="twitter:title" content="Irshad Ali — Software Engineer & Systems Architect"/>
        <Meta name="twitter:description" content="High-Performance Distributed Systems, Rust, and Cloud Architecture."/>
        <Meta name="twitter:image" content="/api/og"/>
        <Link rel="alternate" type_="application/rss+xml" title="Irshad RSS Feed" href="/rss.xml"/>
        <Link rel="alternate" type_="application/atom+xml" title="Irshad Atom Feed" href="/atom.xml"/>
        <Link rel="alternate" type_="application/feed+json" title="Irshad JSON Feed" href="/feed.json"/>
        <Link rel="manifest" href="/manifest.json"/>
        
        // Zero-FOUC inline theme script
        <Script>
            "try { const t = localStorage.getItem('theme') || (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'); document.documentElement.setAttribute('data-theme', t); } catch(e) {}"
        </Script>
        
        // Highlight.js
        <Link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/atom-one-dark.min.css"/>
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"/>
        <Script>"document.addEventListener('DOMContentLoaded', () => { document.querySelectorAll('pre code').forEach((block) => { hljs.highlightElement(block); }); });"</Script>

        // KaTeX Math
        <Link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css"/>
        <Script src="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.js"/>
        <Script src="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/contrib/auto-render.min.js"/>
        <Script>"document.addEventListener('DOMContentLoaded', () => { if (window.renderMathInElement) { renderMathInElement(document.body, { delimiters: [ {left: '$$', right: '$$', display: true}, {left: '$', right: '$', display: false} ] }); } });"</Script>

        // Mermaid Diagrams
        <Script src="https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.min.js"/>
        <Script>"document.addEventListener('DOMContentLoaded', () => { if (window.mermaid) { mermaid.initialize({ startOnLoad: true, theme: 'dark' }); } });"</Script>

        <Router>
            <CommandPalette is_open=is_palette_open set_open=set_palette_open/>
            <nav class="navbar" id="navbar">
                <div class="nav-container">
                    <A href="/" class="nav-logo">
                        <span class="logo-bracket">"<"</span>"IA"<span class="logo-bracket">"/>"</span>
                    </A>
                    <div class=move || format!("nav-links {}", if is_menu_open.get() { "open" } else { "" }) id="navLinks">
                        <A href="/" class="nav-link" active_class="active" exact=true on:click=move |_| set_menu_open.set(false)>"Home"</A>
                        <A href="/projects" class="nav-link" active_class="active" on:click=move |_| set_menu_open.set(false)>"Portfolio"</A>
                        <A href="/experience" class="nav-link" active_class="active" on:click=move |_| set_menu_open.set(false)>"Experience"</A>
                        <A href="/articles" class="nav-link" active_class="active" on:click=move |_| set_menu_open.set(false)>"Articles"</A>
                        <A href="/books" class="nav-link" active_class="active" on:click=move |_| set_menu_open.set(false)>"Books"</A>
                        <A href="/uses" class="nav-link" active_class="active" on:click=move |_| set_menu_open.set(false)>"Uses"</A>
                        <A href="/tools" class="nav-link" active_class="active" on:click=move |_| set_menu_open.set(false)>"Tools"</A>
                        <A href="/about" class="nav-link" active_class="active" on:click=move |_| set_menu_open.set(false)>"About"</A>
                        <A href="/contact" class="nav-link" active_class="active" on:click=move |_| set_menu_open.set(false)>"Contact"</A>
                        <A href="/resume" class="nav-link" active_class="active" on:click=move |_| set_menu_open.set(false)>"Resume"</A>
                    </div>
                    <div class="nav-actions">
                        <button class="theme-toggle" aria-label="Search Palette (Cmd+K)" style="margin-right: 4px; display: inline-flex; align-items: center; gap: 4px;" on:click=move |_| set_palette_open.set(true)>
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="11" cy="11" r="8"></circle>
                                <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
                            </svg>
                            <span class="cmd-palette-kbd" style="font-size: 0.65rem; padding: 1px 4px;">"⌘K"</span>
                        </button>
                        <button class="theme-toggle" id="themeToggle" aria-label="Toggle theme" on:click=toggle_theme>
                            <svg class="icon-sun" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="12" cy="12" r="5"></circle>
                                <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"></path>
                            </svg>
                            <svg class="icon-moon" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                            </svg>
                        </button>
                        <button class="mobile-menu-btn" id="mobileMenuBtn" aria-label="Menu" on:click=toggle_menu>
                            <span class=move || if is_menu_open.get() { "open" } else { "" }></span>
                            <span class=move || if is_menu_open.get() { "open" } else { "" }></span>
                            <span class=move || if is_menu_open.get() { "open" } else { "" }></span>
                        </button>
                    </div>
                </div>
            </nav>
            <main id="app">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="about" view=AboutPage/>
                    <Route path="contact" view=ContactPage/>
                    <Route path="tools" view=ToolsPage/>
                    <Route path="resume" view=ResumePage/>
                    <Route path="experience" view=ExperiencePage/>
                    <Route path="career" view=ExperiencePage/>
                    <Route path="credentials" view=ExperiencePage/>
                    <Route path="search" view=SearchPage/>
                    <Route path="projects" view=ProjectsPage/>
                    <Route path="projects/:slug" view=ProjectDetailPage/>
                    <Route path="books" view=BooksPage/>
                    <Route path="books/:slug" view=BookIndexPage/>
                    <Route path="books/:slug/:chapter" view=BookChapterPage/>
                    <Route path="uses" view=UsesPage/>
                    <Route path="articles" view=ArticlesPage/>
                    <Route path="articles/:slug" view=ArticlePage/>
                    <Route path="writing" view=ArticlesPage/>
                    <Route path="writing/:slug" view=ArticlePage/>
                    <Route path="blog" view=ArticlesPage/>
                    <Route path="blog/:slug" view=ArticlePage/>
                    <Route path="login" view=LoginPage/>
                    <Route path="admin" view=AdminDashboard>
                        <Route path="" view=AdminOverview/>
                        <Route path="analytics" view=AdminAnalytics/>
                        <Route path="messages" view=AdminMessages/>
                        <Route path="portfolio" view=AdminPortfolio/>
                        <Route path="projects" view=AdminProjects/>
                        <Route path="articles" view=AdminArticles/>
                        <Route path="books" view=AdminBooks/>
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
                                    <A href="/uses" class="terminal-link">"-rw-r--r--  uses.md"</A>
                                    <A href="/about" class="terminal-link">"-rw-r--r--  about.md"</A>
                                    <A href="/resume" class="terminal-link">"-rw-r--r--  resume.md"</A>
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
                                    <a href="mailto:contact@irshadali.org" class="terminal-link">"tcp 0 0 smtp.mail:587 LISTENING"</a>
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
fn TimelineSection(
    label: &'static str,
    title: &'static str,
    item_type_filter: &'static str,
    timeline_res: leptos::Resource<(), Vec<domain::models::TimelineItem>>,
) -> impl IntoView {
    view! {
        <section class="page-section container" style="margin-top: 80px;">
            <div class="section-header">
                <span class="section-label">{label}</span>
                <h2 class="section-title">{title}</h2>
            </div>
            
            <div class="timeline-container">
                <Suspense fallback=move || view! { <div class="timeline-item"><div class="timeline-content">"Loading..."</div></div> }>
                    {move || timeline_res.get().map(|items| {
                        let filtered: Vec<_> = items.into_iter().filter(|i| i.item_type == item_type_filter).collect();
                        if filtered.is_empty() {
                            view! {
                                <div class="timeline-item">
                                    <div class="timeline-content">
                                        <p class="timeline-desc" style="text-align: center; font-style: italic;">"Details are currently being updated."</p>
                                    </div>
                                </div>
                            }.into_view()
                        } else {
                            filtered.into_iter().map(|item| {
                                view! {
                                    <div class="timeline-item">
                                        <div class="timeline-content">
                                            <span class="timeline-date">{item.period}</span>
                                            <h3 class="timeline-title">{item.title}</h3>
                                            <div class="timeline-company">{item.organization}</div>
                                            <p class="timeline-desc">{item.description}</p>
                                        </div>
                                    </div>
                                }
                            }).collect_view()
                        }
                    })}
                </Suspense>
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
fn SkillsSection() -> impl IntoView {
    let skills_res = create_resource(|| (), |_| async move { api::get_skills().await.unwrap_or_default() });

    view! {
        <section class="page-section container" style="margin-top: 80px;">
            <div class="section-header">
                <span class="section-label">"Expertise"</span>
                <h2 class="section-title">"Technical Skills"</h2>
            </div>
            
            <Suspense fallback=move || view! { <div class="skills-grid"><div class="skill-category">"Loading skills..."</div></div> }>
                {move || skills_res.get().map(|skills| {
                    if skills.is_empty() {
                        view! {
                            <div class="skills-grid">
                                <div class="skill-category" style="background-color: var(--bg-card); padding: 2rem; border-radius: var(--radius-lg); border: 1px solid var(--border); box-shadow: var(--shadow-sm); text-align: center; color: var(--text-tertiary); font-style: italic;">
                                    "Skills are currently being updated."
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        let mut categories: Vec<(String, Vec<domain::models::Skill>)> = Vec::new();
                        for skill in skills {
                            if let Some(pos) = categories.iter().position(|(c, _)| c == &skill.category) {
                                categories[pos].1.push(skill);
                            } else {
                                categories.push((skill.category.clone(), vec![skill]));
                            }
                        }
                        
                        view! {
                            <div class="skills-grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 2rem;">
                                {categories.into_iter().map(|(category, cat_skills)| {
                                    view! {
                                        <div class="skill-category" style="background-color: var(--bg-card); padding: 2rem; border-radius: var(--radius-lg); border: 1px solid var(--border); box-shadow: var(--shadow-sm);">
                                            <h3 style="color: var(--text-primary); font-size: 1.25rem; margin-bottom: 1.5rem; font-weight: 600;">{category}</h3>
                                            <ul style="list-style: none; padding: 0; margin: 0; color: var(--text-tertiary);">
                                                {cat_skills.into_iter().map(|skill| {
                                                    view! {
                                                        <li style="margin-bottom: 0.5rem; display: flex; align-items: center; gap: 0.5rem;"><span>"▹"</span> {skill.name}</li>
                                                    }
                                                }).collect_view()}
                                            </ul>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_view()
                    }
                })}
            </Suspense>
        </section>
    }
}


#[component]
fn FeaturedProjectsSection() -> impl IntoView {
    let projects_res = create_resource(|| (), |_| async move { api::get_projects().await.unwrap_or_default() });

    view! {
        <section class="page-section container" style="margin-top: 80px;">
            <div class="section-header" style="display: flex; justify-content: space-between; align-items: flex-end; flex-wrap: wrap; gap: 1rem;">
                <div>
                    <span class="section-label">"Selected Engineering Systems"</span>
                    <h2 class="section-title">"Featured Architecture & Projects"</h2>
                    <p class="section-subtitle">"Deep-dive into production-grade distributed consensus, embedded database engines, and zero-copy Linux networking."</p>
                </div>
                <A href="/projects" class="btn btn-secondary">
                    "View All Projects &rarr;"
                </A>
            </div>

            <Suspense fallback=move || view! { <div class="projects-grid"><div class="skeleton-loader project-card" style="height: 260px;"></div><div class="skeleton-loader project-card" style="height: 260px;"></div></div> }>
                {move || projects_res.get().map(|projs| {
                    let featured: Vec<_> = projs.into_iter().take(3).collect();
                    view! {
                        <div class="projects-grid">
                            {featured.into_iter().map(|p| {
                                let case_study_url = format!("/projects/{}", p.slug);
                                view! {
                                    <div class="project-card glass-panel" style="display: flex; flex-direction: column; justify-content: space-between;">
                                        <div>
                                            <div style="display: flex; justify-content: space-between; align-items: flex-start; gap: 0.5rem; margin-bottom: 0.75rem;">
                                                <h3 class="project-title" style="margin: 0; font-size: 1.3rem;">
                                                    <A href=case_study_url.clone()>{p.title}</A>
                                                </h3>
                                            </div>
                                            <p class="project-desc" style="color: var(--text-secondary); line-height: 1.6; margin-bottom: 1.25rem; font-size: 0.95rem;">
                                                {p.description}
                                            </p>

                                            {p.benchmarks.as_ref().map(|bench| view! {
                                                <div class="project-spec-card" style="margin-bottom: 1.25rem; padding: 0.85rem 1rem;">
                                                    <div class="project-spec-title">"⚡ Key Benchmark"</div>
                                                    <div class="project-spec-value" style="font-size: 0.85rem; font-weight: 600; color: var(--accent);">{bench.clone()}</div>
                                                </div>
                                            })}

                                            <div class="project-tags" style="display: flex; flex-wrap: wrap; gap: 0.4rem; margin-bottom: 1.5rem;">
                                                {p.tags.into_iter().map(|t| view! {
                                                    <span class="tag">{t}</span>
                                                }).collect_view()}
                                            </div>
                                        </div>

                                        <div class="project-links" style="display: flex; gap: 0.75rem; align-items: center; border-top: 1px solid var(--border); padding-top: 1rem; flex-wrap: wrap;">
                                            <A href=case_study_url class="btn btn-primary">
                                                "Case Study &rarr;"
                                            </A>
                                            {p.github_url.map(|url| view! { <a href=url target="_blank" class="project-link" style="font-size: 0.85rem;">"GitHub &nearr;"</a> })}
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }.into_view()
                })}
            </Suspense>
        </section>
    }
}

#[component]
fn ArchitecturePillarsSection() -> impl IntoView {
    view! {
        <section class="page-section container" style="margin-top: 80px;">
            <div class="section-header">
                <span class="section-label">"Core Engineering Specialties"</span>
                <h2 class="section-title">"Systems Architecture Pillars"</h2>
                <p class="section-subtitle">"Architectural foundations built on memory safety, formal verification, low latency, and mechanical sympathy."</p>
            </div>

            <div class="architecture-bento">
                <div class="bento-card">
                    <div>
                        <div class="bento-icon">"🏛️"</div>
                        <h3 class="bento-title">"Distributed Consensus & Storage"</h3>
                        <p class="bento-desc">"Building fault-tolerant linearizable state machines with Raft protocol, dynamic joint consensus membership, snapshot log compaction, and leader read leases."</p>
                    </div>
                    <ul class="bento-features">
                        <li><span>"▹"</span> "Raft Consensus Algorithm"</li>
                        <li><span>"▹"</span> "Dynamic Cluster Reconfiguration"</li>
                        <li><span>"▹"</span> "Linearizable Read Leases"</li>
                    </ul>
                </div>

                <div class="bento-card">
                    <div>
                        <div class="bento-icon">"⚡"</div>
                        <h3 class="bento-title">"Embedded MVCC Database Engines"</h3>
                        <p class="bento-desc">"Engineering pure-Rust ACID transactional storage engines featuring slotted-page B+ tree indexing, append-only WAL group commits, and lock-free snapshot isolation."</p>
                    </div>
                    <ul class="bento-features">
                        <li><span>"▹"</span> "Multi-Version Concurrency (MVCC)"</li>
                        <li><span>"▹"</span> "Slotted-Page B+ Tree Layouts"</li>
                        <li><span>"▹"</span> "Group-Commit WAL Logging"</li>
                    </ul>
                </div>

                <div class="bento-card">
                    <div>
                        <div class="bento-icon">"🚀"</div>
                        <h3 class="bento-title">"Zero-Copy Linux Networking"</h3>
                        <p class="bento-desc">"Eliminating user/kernel space memory copies through Linux io_uring SQPOLL ring submission, kernel TCP socket splicing, and SIMD HTTP/1.1 & HTTP/2 tokenizers."</p>
                    </div>
                    <ul class="bento-features">
                        <li><span>"▹"</span> "Linux io_uring & epoll"</li>
                        <li><span>"▹"</span> "Zero-Copy splice(2) Sockets"</li>
                        <li><span>"▹"</span> "SIMD AVX2 Parser Kernels"</li>
                    </ul>
                </div>

                <div class="bento-card">
                    <div>
                        <div class="bento-icon">"🔒"</div>
                        <h3 class="bento-title">"Edge WASM & Vector Search"</h3>
                        <p class="bento-desc">"Deploying sub-millisecond WebAssembly serverless sandboxes with capability-based security boundaries and HNSW vector databases accelerated by AVX-512 SIMD."</p>
                    </div>
                    <ul class="bento-features">
                        <li><span>"▹"</span> "WASI Capability Security"</li>
                        <li><span>"▹"</span> "HNSW Graph Vector Indexing"</li>
                        <li><span>"▹"</span> "AVX-512 Distance Acceleration"</li>
                    </ul>
                </div>
            </div>
        </section>
    }
}

#[component]
fn PublicationsPreviewSection() -> impl IntoView {
    view! {
        <section class="page-section container" style="margin-top: 80px;">
            <div class="section-header" style="display: flex; justify-content: space-between; align-items: flex-end; flex-wrap: wrap; gap: 1rem;">
                <div>
                    <span class="section-label">"Written Guides & Books"</span>
                    <h2 class="section-title">"Engineering Manuals & Publications"</h2>
                    <p class="section-subtitle">"Deep-dive technical books and long-form publications on production Rust architecture."</p>
                </div>
                <div style="display: flex; gap: 0.75rem;">
                    <A href="/articles" class="btn btn-secondary">"All Articles &rarr;"</A>
                    <A href="/books" class="btn btn-secondary">"All Books &rarr;"</A>
                </div>
            </div>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 1.5rem;">
                <div class="glass-panel" style="padding: 2rem; border-radius: var(--radius-lg); display: flex; flex-direction: column; justify-content: space-between;">
                    <div>
                        <span class="experience-badge" style="margin-bottom: 1rem; display: inline-block;">"📖 Technical Book"</span>
                        <h3 style="font-size: 1.3rem; font-weight: 700; color: var(--text-primary); margin-bottom: 0.75rem;">
                            <A href="/books/production-grade-rust">"Production-Grade Rust"</A>
                        </h3>
                        <p style="color: var(--text-secondary); font-size: 0.95rem; line-height: 1.6; margin-bottom: 1.5rem;">
                            "An extensive 30-chapter architectural guide to engineering mission-critical, high-throughput web backends, distributed consensus engines, and cloud infrastructure with modern Rust."
                        </p>
                    </div>
                    <A href="/books/production-grade-rust" class="btn btn-primary">
                        "Read Book Chapters &rarr;"
                    </A>
                </div>

                <div class="glass-panel" style="padding: 2rem; border-radius: var(--radius-lg); display: flex; flex-direction: column; justify-content: space-between;">
                    <div>
                        <span class="experience-badge" style="margin-bottom: 1rem; display: inline-block;">"📖 Technical Book"</span>
                        <h3 style="font-size: 1.3rem; font-weight: 700; color: var(--text-primary); margin-bottom: 0.75rem;">
                            <A href="/books/zero-to-production">"Zero to Production in Rust"</A>
                        </h3>
                        <p style="color: var(--text-secondary); font-size: 0.95rem; line-height: 1.6; margin-bottom: 1.5rem;">
                            "Comprehensive foundations on cloud deployments, Hexagonal Architecture, type-driven state machines, structured logging with tracing, and CI/CD pipelines."
                        </p>
                    </div>
                    <A href="/books/zero-to-production" class="btn btn-primary">
                        "Read Book Chapters &rarr;"
                    </A>
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="hero page-section container animate-slide-up">
            <div class="hero-content">
                <div class="hero-focus-pills">
                    <span class="hero-focus-pill"><span class="dot"></span> "Available for Principal Systems Roles"</span>
                    <span class="hero-focus-pill">"⚡ Sub-millisecond P99"</span>
                    <span class="hero-focus-pill">"🦀 Pure Rust & WASM"</span>
                    <span class="hero-focus-pill">"🔒 ACID MVCC & Raft"</span>
                </div>
                <span class="section-label" style="display: inline-block;">"Hi, my name is"</span>
                <h1 class="hero-title">
                    <span class="gradient-text">"Irshad Ali."</span>
                </h1>
                <h2 class="hero-subtitle" style="font-size: clamp(2rem, 5vw, 4rem); font-weight: 700; color: var(--text-secondary); margin-bottom: 1.5rem;">
                    "I engineer reliable distributed systems."
                </h2>
                <p class="hero-subtitle" style="font-size: 1.2rem; line-height: 1.7; max-width: 760px;">
                    "Systems architect specializing in high-throughput distributed database engines, zero-copy network proxies, and low-latency Rust infrastructure. I build safe, concurrent, and mechanically sympathetic backends."
                </p>
                <div class="hero-actions" style="margin-top: 2rem;">
                    <A href="/projects" class="btn btn-primary">
                        "Explore Systems & Projects &rarr;"
                    </A>
                    <A href="/experience" class="btn btn-secondary">
                        "Career & Experience"
                    </A>
                    <A href="/tools" class="btn btn-secondary">
                        "⚡ Systems Latency Tools"
                    </A>
                </div>
            </div>
        </section>
        
        <FeaturedProjectsSection />
        <ArchitecturePillarsSection />
        <SkillsSection />
        <PublicationsPreviewSection />
        <GithubActivityFeed />
        <ContactForm />
    }
}

#[component]
pub fn ExperiencePage() -> impl IntoView {
    let timeline_res = create_resource(|| (), |_| async move { api::get_timeline().await.unwrap_or_default() });
    let (selected_filter, set_selected_filter) = create_signal("all".to_string());

    view! {
        <Title text="Experience & Career - Irshad Ali | Systems Architect"/>
        <Meta name="description" content="Professional engineering background, distributed systems roles, cloud certifications, and academic degrees of Irshad Ali."/>

        <div class="page-section container animate-fade-in">
            <div class="section-header">
                <span class="section-label">"Career & Credentials"</span>
                <h1 class="section-title">"Experience, Certifications & Education"</h1>
                <p class="section-subtitle">
                    "Track record architecting fault-tolerant distributed engines, low-latency microservices, and high-performance computing infrastructure."
                </p>
            </div>

            <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 1rem; margin-bottom: 2rem;">
                <div class="tag-filter-bar" style="margin-bottom: 0;">
                    <button 
                        class=move || format!("tag-chip {}", if selected_filter.get() == "all" { "active" } else { "" })
                        on:click=move |_| set_selected_filter.set("all".to_string())
                    >
                        "All Records"
                    </button>
                    <button 
                        class=move || format!("tag-chip {}", if selected_filter.get() == "experience" { "active" } else { "" })
                        on:click=move |_| set_selected_filter.set("experience".to_string())
                    >
                        "💼 Work Experience"
                    </button>
                    <button 
                        class=move || format!("tag-chip {}", if selected_filter.get() == "certification" { "active" } else { "" })
                        on:click=move |_| set_selected_filter.set("certification".to_string())
                    >
                        "🏆 Certifications & Licenses"
                    </button>
                    <button 
                        class=move || format!("tag-chip {}", if selected_filter.get() == "education" { "active" } else { "" })
                        on:click=move |_| set_selected_filter.set("education".to_string())
                    >
                        "🎓 Education & Academics"
                    </button>
                </div>

                <A href="/resume" class="btn btn-secondary">
                    "📄 View Full Resume / CV &rarr;"
                </A>
            </div>

            <Suspense fallback=move || view! { <div class="skeleton-loader glass-panel" style="height: 350px;"></div> }>
                {move || timeline_res.get().map(|items| {
                    let filter = selected_filter.get();
                    let filtered: Vec<_> = items.into_iter().filter(|i| {
                        if filter == "all" { true } else { i.item_type == filter }
                    }).collect();

                    if filtered.is_empty() {
                        view! {
                            <div class="glass-panel" style="padding: 3rem; text-align: center; color: var(--text-tertiary);">
                                <p>"No records found matching this category."</p>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div class="experience-timeline">
                                {filtered.into_iter().map(|item| {
                                    let badge_label = match item.item_type.as_str() {
                                        "experience" => "Work Experience",
                                        "certification" => "Certification",
                                        "education" => "Academic Degree",
                                        _ => "Credential",
                                    };
                                    view! {
                                        <div class="experience-card">
                                            <div class="experience-dot"></div>
                                            <div class="experience-header">
                                                <div>
                                                    <h2 class="experience-org">{item.organization}</h2>
                                                    <div class="experience-role">{item.title}</div>
                                                </div>
                                                <div style="display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap;">
                                                    <span class="experience-badge">{badge_label}</span>
                                                    <span style="font-size: 0.85rem; font-weight: 600; color: var(--text-tertiary); font-family: var(--font-mono);">{item.period}</span>
                                                </div>
                                            </div>
                                            <p style="color: var(--text-secondary); line-height: 1.7; font-size: 0.95rem; margin-top: 1rem;">
                                                {item.description}
                                            </p>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_view()
                    }
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div class="page-section container animate-slide-up">
            <h1 class="page-title">"About Me"</h1>
            <div class="content-prose">
                <p class="mb-4">"I build high-performance distributed systems and web applications, predominantly using Rust, C++, and SQL."</p>
                <p class="mb-4">"My core expertise centers around low-latency architectures, systems safety, database engines, and AI pipelines."</p>
                <p>"This website itself is engineered with pure Rust using Leptos SSR, Axum, Tokio, and Tantivy full-text search."</p>
            </div>
        </div>
    }
}

#[component]
fn ResumePage() -> impl IntoView {
    let timeline_res = create_resource(|| (), |_| async move { api::get_timeline().await.unwrap_or_default() });
    let skills_res = create_resource(|| (), |_| async move { api::get_skills().await.unwrap_or_default() });
    let projects_res = create_resource(|| (), |_| async move { api::get_projects().await.unwrap_or_default() });

    view! {
        <Title text="Resume - Irshad Ali | Systems & AI Engineer"/>
        <Meta name="description" content="Resume and Curriculum Vitae of Irshad Ali - Systems Engineer, C++, Rust, AI Pipelines."/>
        
        <div class="page-section container animate-slide-up" style="max-width: 900px;">
            <div class="resume-card glass-panel" style="padding: 3rem;">
                <div style="display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 1.5rem; border-bottom: 2px solid var(--border); padding-bottom: 2rem; margin-bottom: 2rem;">
                    <div>
                        <h1 class="page-title" style="margin-bottom: 0.5rem; font-size: 2.75rem;">"Irshad Ali"</h1>
                        <h2 style="font-size: 1.25rem; font-weight: 600; color: var(--accent); margin-bottom: 1rem;">"Systems Programmer & AI Engineer"</h2>
                        <div style="display: flex; gap: 1rem; flex-wrap: wrap; font-size: 0.95rem; color: var(--text-secondary);">
                            <span>"📧 contact@irshadali.org"</span>
                            <span>"•"</span>
                            <span>"🌐 irshadali.org"</span>
                            <span>"•"</span>
                            <a href="https://github.com/irshadali5" target="_blank" rel="noopener" style="color: var(--accent);">"🐙 GitHub"</a>
                            <span>"•"</span>
                            <a href="https://linkedin.com/in/irshadali" target="_blank" rel="noopener" style="color: var(--accent);">"💼 LinkedIn"</a>
                        </div>
                    </div>
                </div>

                <section style="margin-bottom: 2.5rem;">
                    <h3 style="font-size: 1.25rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-primary); border-bottom: 1px solid var(--border); padding-bottom: 0.5rem; margin-bottom: 1rem;">"Summary"</h3>
                    <p style="color: var(--text-secondary); line-height: 1.7;">
                        "High-performance systems programmer specializing in Rust, C++, distributed architectures, and scalable AI infrastructure. Experienced in designing robust web services, database storage engines, and type-safe systems with zero-cost abstractions."
                    </p>
                </section>

                <section style="margin-bottom: 2.5rem;">
                    <h3 style="font-size: 1.25rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-primary); border-bottom: 1px solid var(--border); padding-bottom: 0.5rem; margin-bottom: 1rem;">"Technical Skills"</h3>
                    <Suspense fallback=move || view! { <p class="text-muted">"Loading skills..."</p> }>
                        {move || skills_res.get().map(|skills| {
                            let mut categories: Vec<(String, Vec<domain::models::Skill>)> = Vec::new();
                            for skill in skills {
                                if let Some(pos) = categories.iter().position(|(c, _)| c == &skill.category) {
                                    categories[pos].1.push(skill);
                                } else {
                                    categories.push((skill.category.clone(), vec![skill]));
                                }
                            }
                            view! {
                                <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                                    {categories.into_iter().map(|(cat, cat_skills)| {
                                        let skill_names = cat_skills.into_iter().map(|s| s.name).collect::<Vec<_>>().join(", ");
                                        view! {
                                            <div style="font-size: 0.95rem;">
                                                <strong style="color: var(--text-primary);">{cat}": "</strong>
                                                <span style="color: var(--text-secondary);">{skill_names}</span>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }
                        })}
                    </Suspense>
                </section>

                <section style="margin-bottom: 2.5rem;">
                    <h3 style="font-size: 1.25rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-primary); border-bottom: 1px solid var(--border); padding-bottom: 0.5rem; margin-bottom: 1rem;">"Experience"</h3>
                    <Suspense fallback=move || view! { <p class="text-muted">"Loading experience..."</p> }>
                        {move || timeline_res.get().map(|items| {
                            let exp: Vec<_> = items.into_iter().filter(|i| i.item_type == "experience").collect();
                            view! {
                                <div style="display: flex; flex-direction: column; gap: 1.5rem;">
                                    {exp.into_iter().map(|item| {
                                        view! {
                                            <div>
                                                <div style="display: flex; justify-content: space-between; align-items: baseline; flex-wrap: wrap; margin-bottom: 0.25rem;">
                                                    <h4 style="font-size: 1.1rem; font-weight: 700; color: var(--text-primary);">{item.title} " — " <span style="color: var(--accent); font-weight: 600;">{item.organization}</span></h4>
                                                    <span style="font-size: 0.85rem; color: var(--text-tertiary); font-weight: 600;">{item.period}</span>
                                                </div>
                                                <p style="color: var(--text-secondary); font-size: 0.95rem; line-height: 1.6;">{item.description}</p>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }
                        })}
                    </Suspense>
                </section>

                <section style="margin-bottom: 2.5rem;">
                    <h3 style="font-size: 1.25rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-primary); border-bottom: 1px solid var(--border); padding-bottom: 0.5rem; margin-bottom: 1rem;">"Featured Projects"</h3>
                    <Suspense fallback=move || view! { <p class="text-muted">"Loading projects..."</p> }>
                        {move || projects_res.get().map(|projs| {
                            view! {
                                <div style="display: flex; flex-direction: column; gap: 1.25rem;">
                                    {projs.into_iter().map(|p| {
                                        view! {
                                            <div>
                                                <div style="display: flex; justify-content: space-between; align-items: baseline; flex-wrap: wrap;">
                                                    <h4 style="font-size: 1.05rem; font-weight: 700; color: var(--text-primary);">{p.title}</h4>
                                                    <div style="display: flex; gap: 0.75rem; font-size: 0.85rem;">
                                                        {p.github_url.map(|url| view! { <a href=url target="_blank" style="color: var(--accent); font-weight: 600;">"GitHub"</a> })}
                                                        {p.live_url.map(|url| view! { <a href=url target="_blank" style="color: var(--accent); font-weight: 600;">"Live Demo"</a> })}
                                                    </div>
                                                </div>
                                                <p style="color: var(--text-secondary); font-size: 0.95rem; line-height: 1.5; margin-top: 0.25rem;">{p.description}</p>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }
                        })}
                    </Suspense>
                </section>

                <section>
                    <h3 style="font-size: 1.25rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-primary); border-bottom: 1px solid var(--border); padding-bottom: 0.5rem; margin-bottom: 1rem;">"Education & Certifications"</h3>
                    <Suspense fallback=move || view! { <p class="text-muted">"Loading education..."</p> }>
                        {move || timeline_res.get().map(|items| {
                            let edu: Vec<_> = items.into_iter().filter(|i| i.item_type == "education" || i.item_type == "certification").collect();
                            view! {
                                <div style="display: flex; flex-direction: column; gap: 1.25rem;">
                                    {edu.into_iter().map(|item| {
                                        view! {
                                            <div>
                                                <div style="display: flex; justify-content: space-between; align-items: baseline; flex-wrap: wrap;">
                                                    <h4 style="font-size: 1rem; font-weight: 700; color: var(--text-primary);">{item.title} " — " <span style="color: var(--accent);">{item.organization}</span></h4>
                                                    <span style="font-size: 0.85rem; color: var(--text-tertiary);">{item.period}</span>
                                                </div>
                                                <p style="color: var(--text-secondary); font-size: 0.9rem;">{item.description}</p>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }
                        })}
                    </Suspense>
                </section>
            </div>
        </div>
    }
}

#[component]
fn ProjectsPage() -> impl IntoView {
    let projects = create_resource(|| (), |_| async move { api::get_projects().await.unwrap_or_default() });
    let (selected_tag, set_selected_tag) = create_signal(Option::<String>::None);

    view! {
        <Title text="Portfolio & Distributed Systems - Irshad Ali"/>
        <Meta name="description" content="Production distributed systems, embedded database engines, WebAssembly sandboxes, and low-latency benchmarks."/>
        <div class="page-section animate-fade-in container">
            <div class="section-header">
                <span class="section-label">"Systems Engineering & Architecture"</span>
                <h1 class="section-title">"Featured Work & Systems"</h1>
                <p class="section-subtitle">"Production engineering, distributed consensus, embedded database engines, and performance benchmarks."</p>
            </div>
            
            <Transition fallback=move || view! { 
                <div class="projects-grid">
                    <div class="skeleton-loader project-card" style="height: 220px;"></div>
                    <div class="skeleton-loader project-card" style="height: 220px;"></div>
                </div> 
            }>
                {move || {
                    projects.get().map(|projs| {
                        if projs.is_empty() {
                            view! { <p class="text-muted">"No projects found."</p> }.into_view()
                        } else {
                            let mut tags_set = std::collections::BTreeSet::new();
                            for p in &projs {
                                for t in &p.tags {
                                    tags_set.insert(t.clone());
                                }
                            }
                            let tags_list: Vec<String> = tags_set.into_iter().collect();

                            let active_tag = selected_tag.get();
                            let filtered: Vec<_> = projs.into_iter().filter(|p| {
                                match &active_tag {
                                    Some(tag) => p.tags.contains(tag),
                                    None => true,
                                }
                            }).collect();

                            view! {
                                <div>
                                    <div class="tag-filter-bar">
                                        <button 
                                             class=move || format!("tag-chip {}", if selected_tag.get().is_none() { "active" } else { "" })
                                            on:click=move |_| set_selected_tag.set(None)
                                        >
                                            "All Projects"
                                        </button>
                                        {tags_list.into_iter().map(|tag| {
                                            let tag_for_click = tag.clone();
                                            let tag_for_active = tag.clone();
                                            view! {
                                                <button 
                                                    class=move || format!("tag-chip {}", if selected_tag.get().as_deref() == Some(&tag_for_active) { "active" } else { "" })
                                                    on:click=move |_| set_selected_tag.set(Some(tag_for_click.clone()))
                                                >
                                                    {tag}
                                                </button>
                                            }
                                        }).collect_view()}
                                    </div>

                                    {if filtered.is_empty() {
                                        view! { <p class="text-muted" style="padding: 2rem 0;">"No projects found matching this tag."</p> }.into_view()
                                    } else {
                                        view! {
                                            <div class="projects-grid">
                                                {filtered.into_iter().map(|p| {
                                                    let has_case_study = !p.content_html.is_empty();
                                                    let case_study_url = format!("/projects/{}", p.slug);
                                                    view! {
                                                        <div class="project-card glass-panel" style="display: flex; flex-direction: column; justify-content: space-between;">
                                                            <div class="project-content">
                                                                <div style="display: flex; justify-content: space-between; align-items: flex-start; gap: 0.5rem; margin-bottom: 0.75rem;">
                                                                    <h2 class="project-title" style="margin: 0; font-size: 1.3rem;">
                                                                        {if has_case_study {
                                                                            view! { <A href=case_study_url.clone()>{p.title}</A> }.into_view()
                                                                        } else {
                                                                            view! { <span>{p.title}</span> }.into_view()
                                                                        }}
                                                                    </h2>
                                                                </div>
                                                                <p class="project-desc" style="color: var(--text-secondary); line-height: 1.6; margin-bottom: 1.25rem; font-size: 0.95rem;">{p.description}</p>
                                                                
                                                                {p.benchmarks.as_ref().map(|bench| view! {
                                                                    <div class="project-spec-card" style="margin-bottom: 1.25rem; padding: 0.85rem 1rem;">
                                                                        <div class="project-spec-title">"⚡ Verified Benchmark"</div>
                                                                        <div class="project-spec-value" style="font-size: 0.85rem; font-weight: 600; color: var(--accent);">{bench.clone()}</div>
                                                                    </div>
                                                                })}

                                                                <div class="project-tags" style="display: flex; flex-wrap: wrap; gap: 0.4rem; margin-bottom: 1.5rem;">
                                                                    {p.tags.into_iter().map(|t| view! {
                                                                        <span class="tag">{t}</span>
                                                                    }).collect_view()}
                                                                </div>
                                                            </div>
                                                            <div class="project-links" style="display: flex; gap: 0.75rem; align-items: center; border-top: 1px solid var(--border); padding-top: 1rem; flex-wrap: wrap;">
                                                                {if has_case_study {
                                                                    Some(view! { <A href=case_study_url class="btn btn-primary">"Case Study &rarr;"</A> })
                                                                } else { None }}
                                                                {p.github_url.map(|url| view! { <a href=url target="_blank" class="project-link" style="font-size: 0.85rem;">"GitHub &nearr;"</a> })}
                                                                {p.live_url.map(|url| view! { <a href=url target="_blank" class="project-link" style="font-size: 0.85rem;">"Live Demo &nearr;"</a> })}
                                                            </div>
                                                        </div>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        }.into_view()
                                    }}
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
fn ProjectDetailPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").cloned().unwrap_or_default());
    
    let project = create_resource(slug, |s| async move {
        if s.is_empty() { return None; }
        api::get_project(s).await.ok()
    });

    view! {
        <div class="page-section animate-fade-in container" style="max-width: 960px;">
            <div style="margin-bottom: 1.5rem;">
                <A href="/projects" class="btn btn-secondary">
                    "&larr; Back to Projects"
                </A>
            </div>

            <Transition fallback=move || view! { <div class="skeleton-loader glass-panel" style="height: 300px;"></div> }>
                {move || project.get().flatten().map(|p| {
                    view! {
                        <div class="project-case-study">
                            <Title text=format!("{} - Architecture & Case Study", p.title)/>
                            <Meta name="description" content=p.description.clone()/>
                            <div class="glass-panel" style="padding: 2.5rem; margin-bottom: 2rem;">
                                <div style="display: flex; gap: 0.5rem; flex-wrap: wrap; margin-bottom: 1rem;">
                                    {p.tags.into_iter().map(|t| view! { <span class="tag">{t}</span> }).collect_view()}
                                </div>
                                <h1 class="section-title" style="margin-bottom: 0.75rem;">{p.title}</h1>
                                <p class="text-muted" style="font-size: 1.15rem; line-height: 1.7; margin-bottom: 1.5rem;">{p.description}</p>
                                
                                <div class="project-spec-grid">
                                    {p.architecture.as_ref().map(|arch| view! {
                                        <div class="project-spec-card">
                                            <div class="project-spec-title">"🏛️ Architecture Model"</div>
                                            <div class="project-spec-value" style="font-size: 0.95rem; font-weight: 500; font-family: var(--font-sans);">{arch.clone()}</div>
                                        </div>
                                    })}

                                    {p.benchmarks.as_ref().map(|bench| view! {
                                        <div class="project-spec-card">
                                            <div class="project-spec-title">"⚡ Performance & Benchmarks"</div>
                                            <div class="project-spec-value" style="font-size: 0.95rem; font-weight: 500; font-family: var(--font-sans); color: var(--accent);">{bench.clone()}</div>
                                        </div>
                                    })}

                                    {p.lessons_learned.as_ref().map(|lessons| view! {
                                        <div class="project-spec-card">
                                            <div class="project-spec-title">"💡 Invariants & Lessons"</div>
                                            <div class="project-spec-value" style="font-size: 0.95rem; font-weight: 500; font-family: var(--font-sans);">{lessons.clone()}</div>
                                        </div>
                                    })}
                                </div>

                                <div class="architecture-topology-card" style="margin-top: 1.25rem; padding: 1.25rem 1.5rem; border: 1px solid var(--border); border-radius: var(--radius-md); background: rgba(15, 23, 42, 0.4);">
                                    <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.5rem; flex-wrap: wrap; gap: 0.5rem;">
                                        <span style="font-size: 0.85rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em; color: var(--accent);">"System Architecture Topology"</span>
                                        <span class="tag" style="font-size: 0.75rem; padding: 2px 8px;">"Kiri Pure-Rust OCI Sandboxing"</span>
                                    </div>
                                    <div style="font-family: var(--font-mono); font-size: 0.85rem; color: var(--text-secondary); line-height: 1.6;">
                                        "Client (Browser / WASM Hydrated) ──[HTTP/2 TLS]──> Edge Gateway (Axum 0.7) ──> In-Memory MVCC Store (Stoolap v0.5.1) + Full-Text Inverted Index (Tantivy)"
                                    </div>
                                </div>

                                <div style="display: flex; gap: 1rem; flex-wrap: wrap; margin-top: 1.5rem; border-top: 1px solid var(--border); padding-top: 1.25rem; align-items: center;">
                                    {p.github_url.map(|url| view! { <a href=url target="_blank" class="btn btn-primary">"View on GitHub &nearr;"</a> })}
                                    {p.live_url.map(|url| view! { <a href=url target="_blank" class="btn btn-secondary">"Live Demo &nearr;"</a> })}
                                    <ClapButton slug=p.slug.clone()/>
                                </div>
                            </div>

                            {if !p.content_html.is_empty() {
                                view! {
                                    <article class="markdown-body glass-panel" style="padding: 3rem;" inner_html=p.content_html/>
                                }.into_view()
                            } else {
                                view! { <div></div> }.into_view()
                            }}
                        </div>
                    }
                })}
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
                                        <div class="book-card-wrapper" style="text-decoration: none; color: inherit; display: block;">
                                            <A href=format!("/books/{}", b.slug) class="book-card-link">
                                                <div class="book-card">
                                                    <div class="book-cover">"📚"</div>
                                                    <div class="book-info">
                                                        <h3>{b.title}</h3>
                                                        {if let Some(desc) = b.description {
                                                            view! { <p>{desc}</p> }.into_view()
                                                        } else { view! {}.into_view() }}
                                                    </div>
                                                </div>
                                            </A>
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
    let (selected_tag, set_selected_tag) = create_signal(Option::<String>::None);

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
                    articles.get().map(|all_articles| {
                        if all_articles.is_empty() {
                            view! { <p class="text-muted">"No articles found."</p> }.into_view()
                        } else {
                            let mut tags_set = std::collections::BTreeSet::new();
                            for a in &all_articles {
                                for t in &a.tags {
                                    tags_set.insert(t.clone());
                                }
                            }
                            let tags_list: Vec<String> = tags_set.into_iter().collect();

                            let active_tag = selected_tag.get();
                            let filtered: Vec<_> = all_articles.into_iter().filter(|a| {
                                match &active_tag {
                                    Some(tag) => a.tags.contains(tag),
                                    None => true,
                                }
                            }).collect();

                            view! {
                                <div>
                                    <div class="tag-filter-bar">
                                        <button 
                                            class=move || format!("tag-chip {}", if selected_tag.get().is_none() { "active" } else { "" })
                                            on:click=move |_| set_selected_tag.set(None)
                                        >
                                            "All Articles"
                                        </button>
                                        {tags_list.into_iter().map(|tag| {
                                            let tag_for_click = tag.clone();
                                            let tag_for_active = tag.clone();
                                            view! {
                                                <button 
                                                    class=move || format!("tag-chip {}", if selected_tag.get().as_deref() == Some(&tag_for_active) { "active" } else { "" })
                                                    on:click=move |_| set_selected_tag.set(Some(tag_for_click.clone()))
                                                >
                                                    {tag}
                                                </button>
                                            }
                                        }).collect_view()}
                                    </div>

                                    {if filtered.is_empty() {
                                        view! { <p class="text-muted" style="padding: 2rem 0;">"No articles found matching this tag."</p> }.into_view()
                                    } else {
                                        view! {
                                            <div class="articles-list">
                                                {filtered.into_iter().map(|a| {
                                                    let slug = a.slug.clone();
                                                    view! {
                                                        <article class="article-card animate-slide-up">
                                                            <div class="article-meta">
                                                                <span>"Article"</span>
                                                                <span class="dot">"•"</span>
                                                                <span><time>{a.created_at.to_rfc3339()}</time></span>
                                                            </div>
                                                            <h2 class="article-title"><A href=format!("/articles/{}", slug)>
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
                                    }}
                                </div>
                            }.into_view()
                        }
                    })
                }}
            </Transition>

            <div class="newsletter-card glass-panel" style="margin-top: 3.5rem; padding: 2.5rem; text-align: center; border-radius: var(--radius-lg);">
                <h3 style="font-size: 1.4rem; font-weight: 700; margin-bottom: 0.5rem;">"Subscribe to Systems & Rust Dispatch"</h3>
                <p class="text-muted" style="max-width: 520px; margin: 0 auto 1.5rem auto; line-height: 1.6;">"Receive occasional deep dives on Rust systems engineering, performance optimization, and architectural blueprints. Zero spam."</p>
                <NewsletterForm/>
            </div>
        </div>
    }
}

#[component]
fn NewsletterForm() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (submitted, set_submitted) = create_signal(false);
    let (err, set_err) = create_signal(String::new());

    let subscribe_action = create_action(move |em: &String| {
        let em = em.clone();
        async move {
            api::subscribe_newsletter(em).await
        }
    });

    view! {
        <div>
            {move || if submitted.get() {
                view! {
                    <p style="color: var(--accent); font-weight: 600; padding: 0.75rem; background: rgba(59, 130, 246, 0.1); border-radius: var(--radius-md); max-width: 400px; margin: 0 auto;">
                        "&check; You're subscribed to the newsletter!"
                    </p>
                }.into_view()
            } else {
                view! {
                    <form on:submit=move |ev| {
                        ev.prevent_default();
                        let val = email.get();
                        if !val.is_empty() && val.contains('@') {
                            subscribe_action.dispatch(val);
                            set_submitted.set(true);
                            set_err.set(String::new());
                        } else {
                            set_err.set("Please enter a valid email address".to_string());
                        }
                    } style="display: flex; gap: 0.5rem; justify-content: center; max-width: 440px; margin: 0 auto; flex-wrap: wrap;">
                        <input 
                            type="email" 
                            class="form-input" 
                            placeholder="your.email@domain.com" 
                            required=true 
                            style="flex: 1; min-width: 220px;"
                            prop:value=email
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                        />
                        <button type="submit" class="btn btn-primary" style="white-space: nowrap;">"Subscribe"</button>
                    </form>
                }.into_view()
            }}
            {move || {
                let error = err.get();
                if !error.is_empty() {
                    view! { <p style="color: #ef4444; font-size: 0.85rem; margin-top: 0.5rem;">{error}</p> }.into_view()
                } else {
                    view! { <span></span> }.into_view()
                }
            }}
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
        <ReadingProgressBar/>
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
                                    <Title text=format!("{} - Irshad", art.title) />
                                    <Meta name="description" content=art.description.clone().unwrap_or_default() />
                                    <Meta property="og:type" content="article" />
                                    <Meta property="og:title" content=art.title.clone() />
                                    <Meta property="og:description" content=art.description.clone().unwrap_or_default() />
                                    <Meta name="twitter:title" content=art.title.clone() />
                                    <Meta name="twitter:description" content=art.description.clone().unwrap_or_default() />
                                    <article class="article-full container">
                                        <header class="article-header">
                                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.25rem; flex-wrap: wrap; gap: 0.5rem;">
                                                <A href="/articles" class="btn btn-secondary">"&larr; All Articles"</A>
                                                <div style="display: flex; gap: 0.5rem; align-items: center;">
                                                    <ClapButton slug=art.slug.clone()/>
                                                    <BookmarkButton 
                                                        slug=art.slug.clone() 
                                                        title=art.title.clone() 
                                                        doc_type="Article".to_string() 
                                                        url=format!("/articles/{}", art.slug) 
                                                    />
                                                </div>
                                            </div>
                                            <h1 class="article-main-title">{art.title.clone()}</h1>
                                            <div class="article-meta-row">
                                                <time>{art.created_at.to_rfc3339()}</time>
                                                <span class="dot">"•"</span>
                                                <span>{views} " views"</span>
                                            </div>
                                        </header>

                                        <AudioReader content_selector=".article-content"/>

                                        <div class="article-content prose" inner_html=art.body></div>
                                        
                                        <footer class="article-footer">
                                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem; flex-wrap: wrap; gap: 1rem;">
                                                <div class="article-tags">
                                                    {art.tags.into_iter().map(|t| view! {
                                                        <span class="tag">{t}</span>
                                                    }).collect_view()}
                                                </div>
                                                <ClapButton slug=art.slug.clone()/>
                                            </div>
                                            
                                            <div class="article-nav">
                                                <A href="/articles" class="back-link">"← Back to Articles"</A>
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
                    <div class="form-group">
                        <label class="form-label">"Email"</label>
                        <input type="email" class="form-input" on:input=move |ev| set_email.set(event_target_value(&ev)) prop:value=email/>
                    </div>
                    <div class="form-group">
                        <label class="form-label">"Password"</label>
                        <input type="password" class="form-input" on:input=move |ev| set_password.set(event_target_value(&ev)) prop:value=password/>
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
    let (selected_filter, set_selected_filter) = create_signal("All".to_string());
    
    let search_results = create_resource(move || query.get(), |q| async move { 
        if q.is_empty() {
            Ok(vec![])
        } else {
            search_content(q).await 
        }
    });

    let filtered_results = move || {
        search_results.get().and_then(|res| res.ok()).map(|results| {
            let filter = selected_filter.get();
            if filter == "All" {
                results
            } else if filter == "Book" {
                results.into_iter().filter(|r| r.doc_type == "Book" || r.doc_type == "Book Chapter").collect::<Vec<_>>()
            } else {
                results.into_iter().filter(|r| r.doc_type.eq_ignore_ascii_case(&filter)).collect::<Vec<_>>()
            }
        })
    };

    view! {
        <Title text="Search - Irshad"/>
        <Meta name="description" content="Search across articles, projects, books, and systems architecture."/>
        
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
            
            <div class="search-filter-chips">
                {["All", "Article", "Project", "Book", "Skill"].into_iter().map(|f| {
                    let f_str = f.to_string();
                    let is_active = move || selected_filter.get() == f_str;
                    let f_click = f.to_string();
                    view! {
                        <button 
                            type="button"
                            class=move || format!("filter-chip {}", if is_active() { "active" } else { "" })
                            on:click=move |_| set_selected_filter.set(f_click.clone())
                        >
                            {f}
                        </button>
                    }
                }).collect_view()}
            </div>

            <div class="search-results-wrap">
                <Transition fallback=move || view! { 
                    <div class="search-results-list mt-8">
                        <div class="skeleton-loader glass-panel" style="height: 100px;"></div>
                        <div class="skeleton-loader glass-panel" style="height: 100px;"></div>
                    </div> 
                }>
                    {move || {
                        filtered_results().map(|results| {
                            if results.is_empty() {
                                if query.get().is_empty() {
                                    view! { <p class="search-empty-state">"Enter a search term above to find content."</p> }.into_view()
                                } else {
                                    view! { <p class="search-empty-state">"No results found matching \"" {query.get()} "\" for filter \"" {selected_filter.get()} "\"."</p> }.into_view()
                                }
                            } else {
                                view! {
                                    <div style="margin-bottom: 0.75rem; font-size: 0.85rem; color: var(--text-muted);">
                                        {format!("Found {} result{}", results.len(), if results.len() == 1 { "" } else { "s" })}
                                    </div>
                                    <ul class="search-results-list">
                                        {results.into_iter().map(|r| {
                                            let url = if r.slug.starts_with('/') {
                                                r.slug
                                            } else {
                                                format!("/{}", r.slug)
                                            };
                                            view! {
                                                <li class="search-result-card glass-panel" style="margin-bottom: 1rem;">
                                                    <a href=url class="search-result-layout" style="display: flex; justify-content: space-between; align-items: flex-start; padding: 1.5rem; gap: 1rem;">
                                                        <div style="flex: 1;">
                                                            <div style="display: flex; align-items: center; gap: 0.5rem; margin-bottom: 0.5rem;">
                                                                <span class="tag" style="font-size: 0.75rem; padding: 2px 8px;">{r.doc_type}</span>
                                                                <h3 class="search-result-title" style="margin: 0; font-size: 1.2rem;">{r.title}</h3>
                                                            </div>
                                                            <p class="search-result-snippet" inner_html=r.snippet style="color: var(--text-secondary); font-size: 0.95rem; line-height: 1.6;"></p>
                                                        </div>
                                                        <span style="color: var(--accent); font-weight: 700;">"&rarr;"</span>
                                                    </a>
                                                </li>
                                            }
                                        }).collect_view()}
                                    </ul>
                                }.into_view()
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
                                <div class="unauthorized-card animate-slide-up">
                                    <h1>"Unauthorized"</h1>
                                    <p class="text-muted mb-6" style="margin-bottom: 2rem;">"You must be logged in to view this page."</p>
                                    <A href="/login" class="glass-button btn-primary">"Login here"</A>
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
                        <li><A href="/admin/messages" class="admin-nav-link" active_class="bg-brand-500/20 text-brand-400 font-medium">"Messages"</A></li>
                        <li><A href="/admin/portfolio" class="admin-nav-link" active_class="bg-brand-500/20 text-brand-400 font-medium">"Portfolio"</A></li>
                        <li><A href="/admin/projects" class="admin-nav-link" active_class="bg-brand-500/20 text-brand-400 font-medium">"Projects"</A></li>
                        <li><A href="/admin/articles" class="admin-nav-link" active_class="bg-brand-500/20 text-brand-400 font-medium">"Articles"</A></li>
                        <li><A href="/admin/books" class="admin-nav-link" active_class="bg-brand-500/20 text-brand-400 font-medium">"Books"</A></li>
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

#[component]
pub fn AdminPortfolio() -> impl IntoView {
    let timeline_action = create_server_action::<api::AddTimelineItem>();
    let delete_timeline_action = create_server_action::<api::DeleteTimelineItem>();
    
    let skills_action = create_server_action::<api::AddSkill>();
    let delete_skill_action = create_server_action::<api::DeleteSkill>();

    let timeline_res = create_resource(
        move || (timeline_action.version().get(), delete_timeline_action.version().get()),
        |_| async move { api::get_timeline().await.unwrap_or_default() }
    );
    
    let skills_res = create_resource(
        move || (skills_action.version().get(), delete_skill_action.version().get()),
        |_| async move { api::get_skills().await.unwrap_or_default() }
    );

    view! {
        <div class="animate-in fade-in">
            <h1 class="admin-page-title">"Portfolio Management"</h1>
            
            <div class="admin-card glass-panel mb-6" style="margin-bottom: 2rem;">
                <h2 class="admin-card-title">"Add Timeline Item (Experience/Education/Certification)"</h2>
                <ActionForm action=timeline_action class="auth-form">
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Type"</label>
                        <select name="item_type" class="form-input">
                            <option value="experience">"Experience"</option>
                            <option value="certification">"Certification / Course"</option>
                            <option value="education">"Education"</option>
                        </select>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Title (e.g. Junior Developer)"</label>
                        <input type="text" name="title" class="form-input" required=true/>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Organization (e.g. Company Name)"</label>
                        <input type="text" name="organization" class="form-input" required=true/>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Period (e.g. 2023 - Present)"</label>
                        <input type="text" name="period" class="form-input" required=true/>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Description"</label>
                        <textarea name="description" class="form-input" required=true></textarea>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Order Index (lower is first)"</label>
                        <input type="number" name="order_index" class="form-input" value="0"/>
                    </div>
                    <button type="submit" class="glass-button btn-primary">"Add Timeline Item"</button>
                </ActionForm>
            </div>
            
            <div class="admin-card glass-panel mb-6" style="margin-bottom: 2rem;">
                <h2 class="admin-card-title">"Existing Timeline Items"</h2>
                <div class="table-responsive">
                    <table class="admin-table">
                        <tr class="table-header-row">
                            <th class="table-th">"Type"</th>
                            <th class="table-th">"Title"</th>
                            <th class="table-th">"Org"</th>
                            <th class="table-th">"Actions"</th>
                        </tr>
                        <Suspense fallback=move || view! { <tr><td colspan="4">"Loading..."</td></tr> }>
                            {move || timeline_res.get().map(|items| {
                                items.into_iter().map(|item| {
                                    let id = item.id.to_string();
                                    view! {
                                        <tr class="table-row">
                                            <td class="table-td">{item.item_type}</td>
                                            <td class="table-td">{item.title}</td>
                                            <td class="table-td">{item.organization}</td>
                                            <td class="table-td">
                                                <ActionForm action=delete_timeline_action>
                                                    <input type="hidden" name="id" value=id/>
                                                    <button type="submit" class="text-link" style="color: #ff5555;">"Delete"</button>
                                                </ActionForm>
                                            </td>
                                        </tr>
                                    }
                                }).collect_view()
                            })}
                        </Suspense>
                    </table>
                </div>
            </div>
            
            <div class="admin-card glass-panel mb-6" style="margin-bottom: 2rem;">
                <h2 class="admin-card-title">"Add Skill"</h2>
                <ActionForm action=skills_action class="auth-form">
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Category (e.g. Languages & Frameworks)"</label>
                        <input type="text" name="category" class="form-input" required=true/>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Skill Name (e.g. Rust)"</label>
                        <input type="text" name="name" class="form-input" required=true/>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Order Index"</label>
                        <input type="number" name="order_index" class="form-input" value="0"/>
                    </div>
                    <button type="submit" class="glass-button btn-primary">"Add Skill"</button>
                </ActionForm>
            </div>
            
            <div class="admin-card glass-panel mb-6" style="margin-bottom: 2rem;">
                <h2 class="admin-card-title">"Existing Skills"</h2>
                <div class="table-responsive">
                    <table class="admin-table">
                        <tr class="table-header-row">
                            <th class="table-th">"Category"</th>
                            <th class="table-th">"Name"</th>
                            <th class="table-th">"Actions"</th>
                        </tr>
                        <Suspense fallback=move || view! { <tr><td colspan="3">"Loading..."</td></tr> }>
                            {move || skills_res.get().map(|items| {
                                items.into_iter().map(|item| {
                                    let id = item.id.to_string();
                                    view! {
                                        <tr class="table-row">
                                            <td class="table-td">{item.category}</td>
                                            <td class="table-td">{item.name}</td>
                                            <td class="table-td">
                                                <ActionForm action=delete_skill_action>
                                                    <input type="hidden" name="id" value=id/>
                                                    <button type="submit" class="text-link" style="color: #ff5555;">"Delete"</button>
                                                </ActionForm>
                                            </td>
                                        </tr>
                                    }
                                }).collect_view()
                            })}
                        </Suspense>
                    </table>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn AdminProjects() -> impl IntoView {
    let add_action = create_server_action::<api::AddProject>();
    let delete_action = create_server_action::<api::DeleteProject>();

    let projects_res = create_resource(
        move || (add_action.version().get(), delete_action.version().get()),
        |_| async move { api::get_projects().await.unwrap_or_default() }
    );

    view! {
        <div class="animate-in fade-in">
            <h1 class="admin-page-title">"Projects Management"</h1>
            
            <div class="admin-card glass-panel mb-6" style="margin-bottom: 2rem;">
                <h2 class="admin-card-title">"Add Project"</h2>
                <ActionForm action=add_action class="auth-form">
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Slug"</label>
                        <input type="text" name="slug" class="form-input" required=true/>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Title"</label>
                        <input type="text" name="title" class="form-input" required=true/>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Description"</label>
                        <textarea name="description" class="form-input" required=true></textarea>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Content HTML"</label>
                        <textarea name="content_html" class="form-input" required=true></textarea>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"GitHub URL"</label>
                        <input type="text" name="github_url" class="form-input"/>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Live URL"</label>
                        <input type="text" name="live_url" class="form-input"/>
                    </div>
                    <button type="submit" class="glass-button btn-primary">"Add Project"</button>
                </ActionForm>
            </div>
            
            <div class="admin-card glass-panel mb-6" style="margin-bottom: 2rem;">
                <h2 class="admin-card-title">"Existing Projects"</h2>
                <div class="table-responsive">
                    <table class="admin-table">
                        <tr class="table-header-row">
                            <th class="table-th">"Title"</th>
                            <th class="table-th">"Slug"</th>
                            <th class="table-th">"Actions"</th>
                        </tr>
                        <Suspense fallback=move || view! { <tr><td colspan="3">"Loading..."</td></tr> }>
                            {move || projects_res.get().map(|items| {
                                items.into_iter().map(|item| {
                                    let id = item.id.to_string();
                                    view! {
                                        <tr class="table-row">
                                            <td class="table-td">{item.title}</td>
                                            <td class="table-td">{item.slug}</td>
                                            <td class="table-td">
                                                <ActionForm action=delete_action>
                                                    <input type="hidden" name="id" value=id/>
                                                    <button type="submit" class="text-link" style="color: #ff5555;">"Delete"</button>
                                                </ActionForm>
                                            </td>
                                        </tr>
                                    }
                                }).collect_view()
                            })}
                        </Suspense>
                    </table>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn AdminArticles() -> impl IntoView {
    let add_action = create_server_action::<api::AddArticle>();
    let delete_action = create_server_action::<api::DeleteArticle>();

    let articles_res = create_resource(
        move || (add_action.version().get(), delete_action.version().get()),
        |_| async move { api::get_articles().await.unwrap_or_default() }
    );

    view! {
        <div class="animate-in fade-in">
            <h1 class="admin-page-title">"Articles Management"</h1>
            
            <div class="admin-card glass-panel mb-6" style="margin-bottom: 2rem;">
                <h2 class="admin-card-title">"Add Article"</h2>
                <ActionForm action=add_action class="auth-form">
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Slug"</label>
                        <input type="text" name="slug" class="form-input" required=true/>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Title"</label>
                        <input type="text" name="title" class="form-input" required=true/>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Description"</label>
                        <textarea name="description" class="form-input" required=true></textarea>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Body"</label>
                        <textarea name="body" class="form-input" required=true></textarea>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Published (true/false)"</label>
                        <input type="text" name="published" class="form-input" value="true" required=true/>
                    </div>
                    <button type="submit" class="glass-button btn-primary">"Add Article"</button>
                </ActionForm>
            </div>
            
            <div class="admin-card glass-panel mb-6" style="margin-bottom: 2rem;">
                <h2 class="admin-card-title">"Existing Articles"</h2>
                <div class="table-responsive">
                    <table class="admin-table">
                        <tr class="table-header-row">
                            <th class="table-th">"Title"</th>
                            <th class="table-th">"Slug"</th>
                            <th class="table-th">"Published"</th>
                            <th class="table-th">"Actions"</th>
                        </tr>
                        <Suspense fallback=move || view! { <tr><td colspan="4">"Loading..."</td></tr> }>
                            {move || articles_res.get().map(|items| {
                                items.into_iter().map(|item| {
                                    let id = item.id.to_string();
                                    view! {
                                        <tr class="table-row">
                                            <td class="table-td">{item.title}</td>
                                            <td class="table-td">{item.slug}</td>
                                            <td class="table-td">{if item.published { "Yes" } else { "No" }}</td>
                                            <td class="table-td">
                                                <ActionForm action=delete_action>
                                                    <input type="hidden" name="id" value=id/>
                                                    <button type="submit" class="text-link" style="color: #ff5555;">"Delete"</button>
                                                </ActionForm>
                                            </td>
                                        </tr>
                                    }
                                }).collect_view()
                            })}
                        </Suspense>
                    </table>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn AdminBooks() -> impl IntoView {
    let add_action = create_server_action::<api::AddBook>();
    let delete_action = create_server_action::<api::DeleteBook>();

    let books_res = create_resource(
        move || (add_action.version().get(), delete_action.version().get()),
        |_| async move { api::get_books().await.unwrap_or_default() }
    );

    view! {
        <div class="animate-in fade-in">
            <h1 class="admin-page-title">"Books Management"</h1>
            
            <div class="admin-card glass-panel mb-6" style="margin-bottom: 2rem;">
                <h2 class="admin-card-title">"Add Book"</h2>
                <ActionForm action=add_action class="auth-form">
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Slug"</label>
                        <input type="text" name="slug" class="form-input" required=true/>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Title"</label>
                        <input type="text" name="title" class="form-input" required=true/>
                    </div>
                    <div class="form-group" style="margin-bottom: 1rem;">
                        <label class="form-label">"Description"</label>
                        <textarea name="description" class="form-input"></textarea>
                    </div>
                    <button type="submit" class="glass-button btn-primary">"Add Book"</button>
                </ActionForm>
            </div>
            
            <div class="admin-card glass-panel mb-6" style="margin-bottom: 2rem;">
                <h2 class="admin-card-title">"Existing Books"</h2>
                <div class="table-responsive">
                    <table class="admin-table">
                        <tr class="table-header-row">
                            <th class="table-th">"Title"</th>
                            <th class="table-th">"Slug"</th>
                            <th class="table-th">"Actions"</th>
                        </tr>
                        <Suspense fallback=move || view! { <tr><td colspan="3">"Loading..."</td></tr> }>
                            {move || books_res.get().map(|items| {
                                items.into_iter().map(|item| {
                                    let id = item.id.to_string();
                                    view! {
                                        <tr class="table-row">
                                            <td class="table-td">{item.title}</td>
                                            <td class="table-td">{item.slug}</td>
                                            <td class="table-td">
                                                <ActionForm action=delete_action>
                                                    <input type="hidden" name="id" value=id/>
                                                    <button type="submit" class="text-link" style="color: #ff5555;">"Delete"</button>
                                                </ActionForm>
                                            </td>
                                        </tr>
                                    }
                                }).collect_view()
                            })}
                        </Suspense>
                    </table>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn BookIndexPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").cloned().unwrap_or_default());
    
    let manifest = create_resource(slug, |s| async move {
        if s.is_empty() { return None; }
        api::get_book_manifest(s).await.ok()
    });

    view! {
        <div class="page-section animate-fade-in container" style="max-width: 900px;">
            <div style="margin-bottom: 1.5rem;">
                <A href="/books" class="btn btn-secondary">
                    "&larr; Back to Books"
                </A>
            </div>
            
            <Transition fallback=move || view! { <div class="skeleton-loader glass-panel" style="height: 200px;"></div> }>
                {move || {
                    manifest.get().flatten().map(|m| {
                        let slug_val = slug();
                        let total_chapters = m.chapters.len();
                        view! {
                            <div class="book-index glass-panel" style="padding: 3rem;">
                                <div style="display: flex; gap: 0.5rem; flex-wrap: wrap; margin-bottom: 1rem;">
                                    {m.tags.into_iter().map(|t| view! { <span class="tag">{t}</span> }).collect_view()}
                                    <span class="tag" style="background: var(--bg-tertiary); color: var(--text-tertiary);">{total_chapters} " Chapters"</span>
                                </div>
                                <h1 class="section-title" style="margin-bottom: 0.5rem;">{m.title}</h1>
                                {if let Some(subtitle) = m.subtitle {
                                    view! { <h3 class="section-subtitle" style="margin-bottom: 1.5rem;">{subtitle}</h3> }.into_view()
                                } else { view! {}.into_view() }}
                                <p class="text-muted" style="margin-bottom: 2.5rem; line-height: 1.7; font-size: 1.05rem;">{m.description}</p>
                                
                                <h2 style="font-size: 1.4rem; font-weight: 700; margin-bottom: 1.25rem; border-bottom: 2px solid var(--border); padding-bottom: 0.75rem;">"Table of Contents"</h2>
                                <ol class="chapter-list" style="margin-top: 1rem; line-height: 1.8; list-style-position: inside;">
                                    {m.chapters.into_iter().enumerate().map(|(idx, chap)| {
                                        let display_name = chap.replace(".md", "").replace(".mdx", "").replace("-", " ");
                                        view! {
                                            <li style="margin-bottom: 0.5rem;">
                                                <A href=format!("/books/{}/{}", slug_val, chap) class="chapter-link">
                                                    <span style="font-weight: 700; color: var(--accent); margin-right: 0.75rem;">{format!("{:02}.", idx + 1)}</span>
                                                    <span>{display_name}</span>
                                                </A>
                                            </li>
                                        }
                                    }).collect_view()}
                                </ol>
                            </div>
                        }
                    })
                }}
            </Transition>
        </div>
    }
}

#[component]
pub fn BookChapterPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").cloned().unwrap_or_default());
    let chapter = move || params.with(|p| p.get("chapter").cloned().unwrap_or_default());
    
    let manifest = create_resource(slug, |s| async move {
        if s.is_empty() { return None; }
        api::get_book_manifest(s).await.ok()
    });

    let content = create_resource(
        move || (slug(), chapter()),
        |(s, c)| async move {
            if s.is_empty() || c.is_empty() { return None; }
            api::get_book_chapter(s, c).await.ok()
        }
    );

    view! {
        <ReadingProgressBar/>
        <div class="page-section animate-fade-in container" style="max-width: 860px;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem; flex-wrap: wrap; gap: 1rem;">
                <A href=move || format!("/books/{}", slug()) class="btn btn-secondary">
                    "&larr; Table of Contents"
                </A>
                <BookmarkButton 
                    slug=format!("{}/{}", slug(), chapter())
                    title=format!("{} — Chapter {}", slug(), chapter().replace(".md", ""))
                    doc_type="Book Chapter".to_string()
                    url=format!("/books/{}/{}", slug(), chapter())
                />
            </div>

            <AudioReader content_selector=".markdown-body"/>
            
            <Transition fallback=move || view! { <div class="skeleton-loader glass-panel" style="height: 300px;"></div> }>
                {move || content.get().map(|c| view! {
                    <article class="markdown-body glass-panel" style="padding: 3rem; margin-bottom: 2rem;" inner_html=c/>
                })}
            </Transition>

            <Transition fallback=move || view! { <div></div> }>
                {move || {
                    let cur_chap = chapter();
                    let slug_val = slug();
                    manifest.get().flatten().map(move |m| {
                        let chapters = m.chapters;
                        let cur_idx = chapters.iter().position(|ch| ch == &cur_chap);
                        let prev_chap = cur_idx.and_then(|idx| if idx > 0 { chapters.get(idx - 1) } else { None });
                        let next_chap = cur_idx.and_then(|idx| chapters.get(idx + 1));
                        
                        view! {
                            <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 2rem; padding-top: 1.5rem; border-top: 1px solid var(--border); gap: 1rem; flex-wrap: wrap;">
                                {if let Some(prev) = prev_chap {
                                    let prev_url = format!("/books/{}/{}", slug_val, prev);
                                    let prev_name = prev.replace(".md", "").replace("-", " ");
                                    view! {
                                        <A href=prev_url class="btn btn-secondary">
                                            "&larr; Previous: " {prev_name}
                                        </A>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }}
                                
                                {if let Some(next) = next_chap {
                                    let next_url = format!("/books/{}/{}", slug_val, next);
                                    let next_name = next.replace(".md", "").replace("-", " ");
                                    view! {
                                        <A href=next_url class="btn btn-primary">
                                            "Next: " {next_name} " &rarr;"
                                        </A>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }}
                            </div>
                        }
                    })
                }}
            </Transition>
        </div>
    }
}

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <Title text="Contact - Irshad Ali"/>
        <Meta name="description" content="Get in touch for systems engineering, consulting, Rust development, or technical inquiries."/>
        <div class="page-section animate-fade-in container" style="max-width: 800px; padding-top: 2rem;">
            <ContactForm/>
        </div>
    }
}

#[component]
pub fn AdminMessages() -> impl IntoView {
    let messages_res = create_resource(|| (), |_| async move { api::get_contact_messages().await.unwrap_or_default() });

    view! {
        <div class="animate-in fade-in">
            <h1 class="admin-page-title">"Contact Messages"</h1>
            <p class="text-muted" style="margin-bottom: 1.5rem;">"Incoming inquiries from the contact form."</p>

            <div class="admin-card glass-panel">
                <div class="table-responsive">
                    <table class="admin-table">
                        <tr class="table-header-row">
                            <th class="table-th">"Date"</th>
                            <th class="table-th">"Name"</th>
                            <th class="table-th">"Email"</th>
                            <th class="table-th">"Message"</th>
                        </tr>
                        <Suspense fallback=move || view! { <tr><td colspan="4">"Loading messages..."</td></tr> }>
                            {move || messages_res.get().map(|items| {
                                if items.is_empty() {
                                    view! { <tr><td colspan="4" class="text-muted" style="padding: 1.5rem; text-align: center;">"No messages received yet."</td></tr> }.into_view()
                                } else {
                                    items.into_iter().map(|msg| {
                                        view! {
                                            <tr class="table-row">
                                                <td class="table-td text-muted font-mono" style="white-space: nowrap; font-size: 0.85rem;">
                                                    {msg.created_at.format("%b %d, %Y %H:%M").to_string()}
                                                </td>
                                                <td class="table-td font-medium">{msg.name}</td>
                                                <td class="table-td">
                                                    <a href=format!("mailto:{}", msg.email) class="text-link">{msg.email}</a>
                                                </td>
                                                <td class="table-td" style="max-width: 350px; line-height: 1.5;">{msg.message}</td>
                                            </tr>
                                        }
                                    }).collect_view()
                                }
                            })}
                        </Suspense>
                    </table>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn CommandPalette(
    is_open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
) -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let search_action = create_action(|q: &String| {
        let q = q.clone();
        async move {
            if q.trim().is_empty() {
                Vec::new()
            } else {
                api::search_content(q).await.unwrap_or_default()
            }
        }
    });

    let on_input = move |ev| {
        let val = event_target_value(&ev);
        set_query.set(val.clone());
        search_action.dispatch(val);
    };

    view! {
        {move || if is_open.get() {
            view! {
                <div class="cmd-palette-backdrop animate-fade-in" on:click=move |_| set_open.set(false)>
                    <div class="cmd-palette-modal glass-panel" on:click=move |ev| ev.stop_propagation()>
                        <div class="cmd-palette-search-bar">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="cmd-palette-icon">
                                <circle cx="11" cy="11" r="8"></circle>
                                <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
                            </svg>
                            <input 
                                type="text" 
                                class="cmd-palette-input" 
                                placeholder="Type to search articles, books, projects, skills... (ESC to close)" 
                                autofocus=true 
                                prop:value=query
                                on:input=on_input
                            />
                            <kbd class="cmd-palette-kbd">"ESC"</kbd>
                        </div>
                        <div class="cmd-palette-results">
                            {move || {
                                let res = search_action.value().get();
                                match res {
                                    Some(items) if !items.is_empty() => {
                                        items.into_iter().map(|item| {
                                            let url = match item.doc_type.as_str() {
                                                "Article" => format!("/articles/{}", item.slug),
                                                "Book" => format!("/books/{}", item.slug),
                                                "Book Chapter" => format!("/books/{}", item.slug),
                                                "Project" => format!("/projects/{}", item.slug),
                                                "Skill" => "/about".to_string(),
                                                _ => format!("/articles/{}", item.slug),
                                            };
                                            let doc_type = item.doc_type.clone();
                                            let title = item.title.clone();
                                            let snippet = item.snippet.clone();
                                            view! {
                                                <A 
                                                    href=url 
                                                    class="cmd-palette-item" 
                                                    on:click=move |_| set_open.set(false)
                                                >
                                                    <div class="cmd-palette-item-header">
                                                        <span class="cmd-palette-item-type">{doc_type}</span>
                                                        <span class="cmd-palette-item-title">{title}</span>
                                                    </div>
                                                    <p class="cmd-palette-item-snippet" inner_html=snippet></p>
                                                </A>
                                            }
                                        }).collect_view()
                                    }
                                    Some(_) if !query.get().is_empty() => {
                                        view! {
                                            <div class="cmd-palette-empty">
                                                <p>"No results found for \"" {query.get()} "\""</p>
                                            </div>
                                        }.into_view()
                                    }
                                    _ => {
                                        view! {
                                            <div class="cmd-palette-quick-links">
                                                <span class="cmd-palette-section-title">"Quick Navigation"</span>
                                                <A href="/projects" class="cmd-palette-item" on:click=move |_| set_open.set(false)>
                                                    <span class="cmd-palette-item-title">"🛠 Projects & Architectural Showcases"</span>
                                                </A>
                                                <A href="/experience" class="cmd-palette-item" on:click=move |_| set_open.set(false)>
                                                    <span class="cmd-palette-item-title">"💼 Experience, Career & Credentials"</span>
                                                </A>
                                                <A href="/articles" class="cmd-palette-item" on:click=move |_| set_open.set(false)>
                                                    <span class="cmd-palette-item-title">"📝 Articles & Systems Engineering"</span>
                                                </A>
                                                <A href="/books" class="cmd-palette-item" on:click=move |_| set_open.set(false)>
                                                    <span class="cmd-palette-item-title">"📚 Books & Technical Manuals"</span>
                                                </A>
                                                <A href="/uses" class="cmd-palette-item" on:click=move |_| set_open.set(false)>
                                                    <span class="cmd-palette-item-title">"⚙️ Uses — Hardware, Linux Kernel & Toolchain"</span>
                                                </A>
                                                <A href="/tools" class="cmd-palette-item" on:click=move |_| set_open.set(false)>
                                                    <span class="cmd-palette-item-title">"⚡ Systems Benchmarks & Latency Tools"</span>
                                                </A>
                                                <A href="/resume" class="cmd-palette-item" on:click=move |_| set_open.set(false)>
                                                    <span class="cmd-palette-item-title">"📄 Resume / CV"</span>
                                                </A>
                                                <A href="/contact" class="cmd-palette-item" on:click=move |_| set_open.set(false)>
                                                    <span class="cmd-palette-item-title">"📬 Contact & Direct Dispatch"</span>
                                                </A>
                                            </div>
                                        }.into_view()
                                    }
                                }
                            }}
                        </div>
                    </div>
                </div>
            }.into_view()
        } else {
            view! { <div></div> }.into_view()
        }}
    }
}

#[component]
pub fn ReadingProgressBar() -> impl IntoView {
    let (progress, set_progress) = create_signal(0.0);
    let _ = &set_progress;

    let _ = window_event_listener(ev::scroll, move |_| {
        #[cfg(not(feature = "ssr"))]
        {
            if let Some(win) = web_sys::window() {
                if let Some(doc) = win.document() {
                    if let Some(elem) = doc.document_element() {
                        let scroll_top = elem.scroll_top() as f64;
                        let scroll_height = elem.scroll_height() as f64;
                        let client_height = elem.client_height() as f64;
                        let total = scroll_height - client_height;
                        if total > 0.0 {
                            let pct = (scroll_top / total) * 100.0;
                            set_progress.set(pct.clamp(0.0, 100.0));
                        }
                    }
                }
            }
        }
    });

    view! {
        <div 
            class="reading-progress-bar" 
            style=move || format!("width: {}%;", progress.get())
        ></div>
    }
}

#[component]
pub fn AudioReader(content_selector: &'static str) -> impl IntoView {
    let (is_playing, set_playing) = create_signal(false);
    let (is_paused, set_paused) = create_signal(false);
    let (rate, set_rate) = create_signal(1.0f32);
    let _ = (content_selector, &set_playing, &is_paused, &set_paused);

    let play = move || {
        #[cfg(not(feature = "ssr"))]
        {
            if let Some(win) = web_sys::window() {
                if let Ok(synth) = win.speech_synthesis() {
                    if is_paused.get() {
                        synth.resume();
                        set_paused.set(false);
                        set_playing.set(true);
                        return;
                    }
                    
                    synth.cancel();

                    if let Some(doc) = win.document() {
                        if let Ok(Some(elem)) = doc.query_selector(content_selector) {
                            let text = elem.text_content().unwrap_or_default();
                            if !text.is_empty() {
                                if let Ok(utterance) = web_sys::SpeechSynthesisUtterance::new_with_text(&text) {
                                    utterance.set_rate(rate.get());
                                    synth.speak(&utterance);
                                    set_playing.set(true);
                                    set_paused.set(false);
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    let pause = move || {
        #[cfg(not(feature = "ssr"))]
        {
            if let Some(win) = web_sys::window() {
                if let Ok(synth) = win.speech_synthesis() {
                    synth.pause();
                    set_paused.set(true);
                    set_playing.set(false);
                }
            }
        }
    };

    let stop = move || {
        #[cfg(not(feature = "ssr"))]
        {
            if let Some(win) = web_sys::window() {
                if let Ok(synth) = win.speech_synthesis() {
                    synth.cancel();
                    set_playing.set(false);
                    set_paused.set(false);
                }
            }
        }
    };

    let set_speed = move |new_rate: f32| {
        set_rate.set(new_rate);
        #[cfg(not(feature = "ssr"))]
        {
            if is_playing.get() {
                stop();
                play();
            }
        }
    };

    view! {
        <div class="audio-reader-bar glass-panel" style="margin-bottom: 2rem; padding: 0.75rem 1.25rem; display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 1rem; border-radius: var(--radius-md);">
            <div style="display: flex; align-items: center; gap: 0.75rem;">
                <span style="font-size: 1.2rem;">"🎙️"</span>
                <div>
                    <span style="font-weight: 600; font-size: 0.9rem; color: var(--text-main);">"Listen to this Content"</span>
                    <span class="text-muted" style="display: block; font-size: 0.75rem;">"Web Speech Audio Synthesizer"</span>
                </div>
            </div>

            <div style="display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap;">
                {move || if is_playing.get() {
                    view! {
                        <button class="btn btn-secondary" style="padding: 0.35rem 0.75rem; font-size: 0.85rem;" on:click=move |_| pause()>
                            "⏸ Pause"
                        </button>
                    }.into_view()
                } else {
                    view! {
                        <button class="btn btn-primary" style="padding: 0.35rem 0.75rem; font-size: 0.85rem;" on:click=move |_| play()>
                            "▶ Play"
                        </button>
                    }.into_view()
                }}

                <button class="btn btn-secondary" style="padding: 0.35rem 0.75rem; font-size: 0.85rem;" on:click=move |_| stop()>
                    "⏹ Stop"
                </button>

                <div style="display: flex; gap: 0.25rem; margin-left: 0.5rem;">
                    {[0.8f32, 1.0f32, 1.25f32, 1.5f32, 2.0f32].into_iter().map(|r| {
                        let active = move || (rate.get() - r).abs() < 0.01;
                        view! {
                            <button 
                                class=move || format!("tag-chip {}", if active() { "active" } else { "" })
                                style="padding: 0.2rem 0.5rem; font-size: 0.75rem;"
                                on:click=move |_| set_speed(r)
                            >
                                {format!("{}x", r)}
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn BookmarkButton(slug: String, title: String, doc_type: String, url: String) -> impl IntoView {
    let (is_bookmarked, set_bookmarked) = create_signal(false);
    let s_clone = slug.clone();
    let _ = (&slug, &title, &doc_type, &url, &set_bookmarked, &s_clone);

    create_effect(move |_| {
        #[cfg(not(feature = "ssr"))]
        {
            if let Some(win) = web_sys::window() {
                if let Ok(Some(storage)) = win.local_storage() {
                    if let Ok(Some(data)) = storage.get_item("reading_bookmarks") {
                        if let Ok(items) = serde_json::from_str::<Vec<domain::models::BookmarkItem>>(&data) {
                            if items.iter().any(|i| i.slug == s_clone) {
                                set_bookmarked.set(true);
                            }
                        }
                    }
                }
            }
        }
    });

    let toggle = move |_| {
        #[cfg(not(feature = "ssr"))]
        {
            if let Some(win) = web_sys::window() {
                if let Ok(Some(storage)) = win.local_storage() {
                    let mut items: Vec<domain::models::BookmarkItem> = storage.get_item("reading_bookmarks")
                        .ok()
                        .flatten()
                        .and_then(|d| serde_json::from_str(&d).ok())
                        .unwrap_or_default();

                    let already = items.iter().position(|i| i.slug == slug);
                    if let Some(pos) = already {
                        items.remove(pos);
                        set_bookmarked.set(false);
                    } else {
                        items.push(domain::models::BookmarkItem {
                            slug: slug.clone(),
                            title: title.clone(),
                            doc_type: doc_type.clone(),
                            url: url.clone(),
                            timestamp: "Saved".to_string(),
                        });
                        set_bookmarked.set(true);
                    }

                    if let Ok(json) = serde_json::to_string(&items) {
                        let _ = storage.set_item("reading_bookmarks", &json);
                    }
                }
            }
        }
    };

    view! {
        <button 
            class="btn btn-secondary" 
            style="display: inline-flex; align-items: center; gap: 0.4rem; padding: 0.35rem 0.75rem; font-size: 0.85rem;"
            on:click=toggle
            title="Save for Later"
        >
            {move || if is_bookmarked.get() {
                view! { <span>"🔖 Saved"</span> }.into_view()
            } else {
                view! { <span>"📑 Save for Later"</span> }.into_view()
            }}
        </button>
    }
}

#[component]
pub fn GithubActivityFeed() -> impl IntoView {
    let activity_res = create_resource(|| (), |_| async move { api::get_github_activity().await.unwrap_or_default() });

    view! {
        <section class="page-section container" style="margin-top: 60px;">
            <div class="section-header">
                <span class="section-label">"Open Source"</span>
                <h2 class="section-title">"Live GitHub Activity"</h2>
                <p class="section-subtitle">"Real-time commit telemetry and open source contributions."</p>
            </div>

            <Suspense fallback=move || view! { <div class="skeleton-loader glass-panel" style="height: 160px;"></div> }>
                {move || activity_res.get().map(|events| {
                    view! {
                        <div class="github-activity-grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 1rem;">
                            {events.into_iter().map(|ev| {
                                view! {
                                    <div class="glass-panel" style="padding: 1.25rem; border-radius: var(--radius-md); display: flex; flex-direction: column; justify-content: space-between;">
                                        <div>
                                            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;">
                                                <a href=ev.url target="_blank" rel="noopener" style="font-weight: 700; color: var(--accent); text-decoration: none; font-size: 0.95rem;">
                                                    {ev.repo}
                                                </a>
                                                <span class="tag" style="font-size: 0.7rem; padding: 2px 6px;">{ev.event_type}</span>
                                            </div>
                                            <p style="font-size: 0.85rem; color: var(--text-secondary); line-height: 1.4; margin-bottom: 0.75rem;">
                                                {ev.message}
                                            </p>
                                        </div>
                                        <span style="font-size: 0.75rem; color: var(--text-tertiary); font-family: var(--font-mono);">
                                            {ev.created_at}
                                        </span>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }
                })}
            </Suspense>
        </section>
    }
}

#[component]
pub fn ToolsPage() -> impl IntoView {
    let (concurrency, set_concurrency) = create_signal(64i32);
    let (avg_latency_ms, set_avg_latency_ms) = create_signal(5.0f64);

    let latencies = vec![
        ("L1 Cache Reference", 0.5, "0.5 ns", "0.5 seconds", "Fastest CPU register/cache access"),
        ("Branch Mispredict", 5.0, "5 ns", "5 seconds", "CPU pipeline stall & flush"),
        ("L2 Cache Reference", 7.0, "7 ns", "7 seconds", "14x slower than L1 cache"),
        ("Mutex Lock / Unlock", 25.0, "25 ns", "25 seconds", "OS synchronization overhead"),
        ("Main Memory Reference", 100.0, "100 ns", "1.6 minutes", "DRAM bus latency"),
        ("Snappy/Zstd Compress 1KB", 2_000.0, "2 µs", "33 minutes", "In-memory streaming compression"),
        ("Send 1KB over 1Gbps Network", 10_000.0, "10 µs", "2.7 hours", "Local gigabit network transmission"),
        ("SSD NVMe Random Read", 150_000.0, "150 µs", "1.7 days", "Flash memory controller latency"),
        ("Read 1MB Sequentially Memory", 250_000.0, "250 µs", "2.9 days", "High-bandwidth DDR memory copy"),
        ("Round Trip in Same Datacenter", 500_000.0, "500 µs", "5.8 days", "Switched local datacenter rack RTT"),
        ("Read 1MB Sequentially from SSD", 1_000_000.0, "1 ms", "11.6 days", "Sequential PCIe NVMe disk read"),
        ("Send Packet SF to NY (Cross-US)", 40_000_000.0, "40 ms", "1.3 years", "Speed-of-light fiber optic transit"),
        ("Send Packet CA to Netherlands & back", 150_000_000.0, "150 ms", "4.8 years", "Intercontinental undersea cable"),
    ];

    let calc_qps = move || {
        let conc = concurrency.get() as f64;
        let lat_sec = avg_latency_ms.get() / 1000.0;
        if lat_sec > 0.0 {
            (conc / lat_sec).round() as u64
        } else {
            0
        }
    };

    view! {
        <Title text="Systems Engineering Tools & Benchmarks - Irshad"/>
        <Meta name="description" content="Interactive latency numbers every programmer should know, human-scale time visualizer, and Little's Law throughput calculator."/>
        
        <div class="page-section container animate-fade-in" style="max-width: 1000px; padding-top: 2rem;">
            <div class="section-header" style="text-align: left; margin-bottom: 2.5rem;">
                <span class="section-label">"Systems Benchmarks & Tools"</span>
                <h1 class="page-title" style="margin-bottom: 0.5rem;">"Latency Numbers Every Programmer Should Know"</h1>
                <p class="section-subtitle" style="margin: 0;">
                    "Interactive systems engineering metrics, human-time analogies, and throughput capacity sizing."
                </p>
            </div>

            <div class="glass-panel" style="padding: 2rem; margin-bottom: 2.5rem; border-radius: var(--radius-lg);">
                <div style="margin-bottom: 1.5rem;">
                    <h2 style="font-size: 1.25rem; font-weight: 700; color: var(--text-main); margin-bottom: 0.25rem;">
                        "⏱️ The Human Scale Analogy"
                    </h2>
                    <p style="font-size: 0.9rem; color: var(--text-secondary); margin: 0;">
                        "If 1 CPU cycle (0.5 ns) took exactly 1 second in human perception, how long would other computer operations take?"
                    </p>
                </div>

                <div class="latency-list" style="display: flex; flex-direction: column; gap: 0.85rem;">
                    {latencies.into_iter().map(|(name, ns, raw_str, human_str, desc): (&str, f64, &str, &str, &str)| {
                        let log_val = ns.log10();
                        let min_log = (0.5f64).log10();
                        let max_log = (150_000_000.0f64).log10();
                        let pct = ((log_val - min_log) / (max_log - min_log) * 100.0).clamp(3.0, 100.0);

                        view! {
                            <div style="background: rgba(255, 255, 255, 0.02); border: 1px solid var(--border); border-radius: var(--radius-md); padding: 1rem;">
                                <div style="display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 0.5rem; flex-wrap: wrap; gap: 0.5rem;">
                                    <div>
                                        <span style="font-weight: 600; color: var(--text-main); font-size: 0.95rem;">{name}</span>
                                        <span style="display: block; font-size: 0.8rem; color: var(--text-tertiary);">{desc}</span>
                                    </div>
                                    <div style="text-align: right;">
                                        <span class="tag" style="font-family: var(--font-mono); font-weight: bold; background: rgba(59, 130, 246, 0.15); color: var(--accent);">{raw_str}</span>
                                        <span style="display: block; font-size: 0.85rem; font-weight: 600; color: #f59e0b; margin-top: 2px;">
                                            "≈ " {human_str}
                                        </span>
                                    </div>
                                </div>
                                <div style="width: 100%; height: 6px; background: var(--bg-tertiary); border-radius: 3px; overflow: hidden;">
                                    <div style=format!("width: {}%; height: 100%; background: linear-gradient(90deg, #3b82f6, #8b5cf6, #ef4444); border-radius: 3px;", pct)></div>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>

            <div class="glass-panel" style="padding: 2rem; border-radius: var(--radius-lg);">
                <div class="section-header" style="text-align: left; margin-bottom: 1.5rem;">
                    <span class="section-label">"Capacity Planning"</span>
                    <h2 style="font-size: 1.25rem; font-weight: 700; color: var(--text-main); margin-bottom: 0.25rem;">
                        "⚡ Throughput & Concurrency Sizer (Little's Law)"
                    </h2>
                    <p style="font-size: 0.9rem; color: var(--text-secondary); margin: 0;">
                        "Estimate maximum theoretical requests per second (QPS) based on concurrent worker threads and service latency."
                    </p>
                </div>

                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 1.5rem; margin-bottom: 1.5rem;">
                    <div>
                        <label style="display: block; font-weight: 600; font-size: 0.85rem; margin-bottom: 0.5rem; color: var(--text-secondary);">
                            "Concurrent Worker Tasks / Connections: " <span style="color: var(--accent); font-weight: bold;">{move || concurrency.get()}</span>
                        </label>
                        <input 
                            type="range" 
                            min="1" 
                            max="1024" 
                            step="1" 
                            style="width: 100%;"
                            prop:value=concurrency
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                    set_concurrency.set(val);
                                }
                            }
                        />
                    </div>

                    <div>
                        <label style="display: block; font-weight: 600; font-size: 0.85rem; margin-bottom: 0.5rem; color: var(--text-secondary);">
                            "Average Request Latency (ms): " <span style="color: var(--accent); font-weight: bold;">{move || format!("{:.1} ms", avg_latency_ms.get())}</span>
                        </label>
                        <input 
                            type="range" 
                            min="0.5" 
                            max="500.0" 
                            step="0.5" 
                            style="width: 100%;"
                            prop:value=avg_latency_ms
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                    set_avg_latency_ms.set(val);
                                }
                            }
                        />
                    </div>
                </div>

                <div style="background: rgba(59, 130, 246, 0.1); border: 1px solid rgba(59, 130, 246, 0.3); border-radius: var(--radius-md); padding: 1.5rem; display: flex; justify-content: space-around; align-items: center; flex-wrap: wrap; gap: 1rem; text-align: center;">
                    <div>
                        <span style="display: block; font-size: 0.8rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-tertiary);">"Theoretical Maximum"</span>
                        <span style="font-size: 2.25rem; font-weight: 800; color: var(--accent); font-family: var(--font-mono);">
                            {move || format!("{} QPS", calc_qps())}
                        </span>
                    </div>

                    <div>
                        <span style="display: block; font-size: 0.8rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-tertiary);">"Estimated Saturation"</span>
                        <span style="font-size: 1.1rem; font-weight: 600; color: var(--text-main);">
                            {move || if avg_latency_ms.get() < 2.0 { "⚡ In-Memory / CPU Bound" } else if avg_latency_ms.get() < 20.0 { "💾 Disk / Fast Database I/O" } else { "🌐 Network / Microservice Bound" }}
                        </span>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ClapButton(slug: String) -> impl IntoView {
    let slug_clone = slug.clone();
    let likes_res = create_resource(move || slug_clone.clone(), |s| async move {
        get_article_likes(s).await.unwrap_or(0)
    });

    let (optimistic_likes, set_optimistic_likes) = create_signal(Option::<u64>::None);
    let (is_pulsing, set_pulsing) = create_signal(false);

    let display_count = move || {
        optimistic_likes.get().unwrap_or_else(|| likes_res.get().unwrap_or(0))
    };

    let on_clap = {
        let slug = slug.clone();
        move |_| {
            let current = display_count();
            set_optimistic_likes.set(Some(current + 1));
            set_pulsing.set(true);

            let s = slug.clone();
            spawn_local(async move {
                if let Ok(new_count) = like_article(s).await {
                    set_optimistic_likes.set(Some(new_count));
                }
                set_pulsing.set(false);
            });
        }
    };

    view! {
        <button 
            class=move || format!("clap-btn {}", if is_pulsing.get() { "pulsing" } else { "" })
            on:click=on_clap
            title="Applaud / Clap"
            type="button"
        >
            <span class="clap-icon">"👏"</span>
            <span class="clap-count">{display_count}</span>
        </button>
    }
}

#[component]
pub fn UsesPage() -> impl IntoView {
    view! {
        <Title text="Uses & Rig - Irshad Ali"/>
        <Meta name="description" content="Development workstation, Linux kernel configuration, Kiri container runtime, and systems toolchain used by Irshad Ali."/>
        
        <div class="page-section max-w-4xl animate-fade-in container">
            <header style="margin-bottom: 2.5rem;">
                <div class="tag" style="margin-bottom: 0.75rem;">"Engineering Environment"</div>
                <h1 class="section-title">"Hardware, Kernel & Toolchain"</h1>
                <p class="text-muted" style="font-size: 1.15rem; line-height: 1.7; max-width: 720px;">
                    "A living catalog of the physical workstation, Linux kernel subsystems, pure-Rust container sandboxes, and low-latency developer tooling I rely on daily for high-performance systems engineering."
                </p>
            </header>

            <div class="uses-grid">
                <div class="uses-card">
                    <div class="uses-card-header">
                        <span class="uses-card-icon">"🖥️"</span>
                        <h2 class="uses-card-title">"Workstation Rig & Hardware"</h2>
                    </div>
                    <ul class="uses-item-list">
                        <li class="uses-item">
                            <span class="uses-item-label">"CPU: AMD Ryzen 9 7950X"</span>
                            <span class="uses-item-desc">"16 Cores, 32 Threads, 5.7 GHz Boost, 80MB Cache. High multi-threaded compilation throughput."</span>
                        </li>
                        <li class="uses-item">
                            <span class="uses-item-label">"Memory: 64GB DDR5-6000 ECC"</span>
                            <span class="uses-item-desc">"Dual-channel error-correcting memory for mission-critical compilation and memory leak fuzzing."</span>
                        </li>
                        <li class="uses-item">
                            <span class="uses-item-label">"NVMe Storage: Samsung 990 Pro 2TB"</span>
                            <span class="uses-item-desc">"PCIe 4.0 NVMe running at 7,450 MB/s sequential read, paired with 4TB ZFS mirror array."</span>
                        </li>
                        <li class="uses-item">
                            <span class="uses-item-label">"GPU: NVIDIA RTX 4080 (16GB VRAM)"</span>
                            <span class="uses-item-desc">"Dedicated to local LLM inference (vLLM, Ollama) and CUDA kernel micro-benchmarks."</span>
                        </li>
                        <li class="uses-item">
                            <span class="uses-item-label">"Peripherals: Keychron Q1 Pro & MX Master 3S"</span>
                            <span class="uses-item-desc">"Custom 75% mechanical keyboard with Gateron Oil Kings, dual 27\" 4K Dell UltraSharp displays."</span>
                        </li>
                    </ul>
                </div>

                <div class="uses-card">
                    <div class="uses-card-header">
                        <span class="uses-card-icon">"🐧"</span>
                        <h2 class="uses-card-title">"OS & Kernel Architecture"</h2>
                    </div>
                    <ul class="uses-item-list">
                        <li class="uses-item">
                            <span class="uses-item-label">"OS: Arch Linux / Void Linux (Musl)"</span>
                            <span class="uses-item-desc">"Minimalist rolling distribution with zero bloat and maximum control over shared libraries."</span>
                        </li>
                        <li class="uses-item">
                            <span class="uses-item-label">"Kernel: Linux 6.12+ LTS Custom Sysctl"</span>
                            <span class="uses-item-desc">"Tuned for high file-descriptor limits (fs.file-max=2097152) and aggressive TCP window scaling."</span>
                        </li>
                        <li class="uses-item">
                            <span class="uses-item-label">"Async I/O: io_uring Subsystem"</span>
                            <span class="uses-item-desc">"Ring-buffer zero-copy submission and completion queues for high-throughput disk and socket I/O."</span>
                        </li>
                        <li class="uses-item">
                            <span class="uses-item-label">"Observability: eBPF & bpftrace"</span>
                            <span class="uses-item-desc">"Kernel probes (kprobes), tracepoints, and BCC scripts for microsecond latency profiling."</span>
                        </li>
                        <li class="uses-item">
                            <span class="uses-item-label">"Network Stack: TCP BBRv3 & XDP"</span>
                            <span class="uses-item-desc">"Bottleneck Bandwidth and RTT congestion control with Express Data Path packet filtering."</span>
                        </li>
                    </ul>
                </div>

                <div class="uses-card">
                    <div class="uses-card-header">
                        <span class="uses-card-icon">"📦"</span>
                        <h2 class="uses-card-title">"Container Runtime & Isolation"</h2>
                    </div>
                    <ul class="uses-item-list">
                        <li class="uses-item">
                            <span class="uses-item-label">"Runtime: Kiri (Pure-Rust Engine)"</span>
                            <span class="uses-item-desc">"High-performance pure-Rust container engine (libracoder/kiri) compatible with Docker Engine API."</span>
                        </li>
                        <li class="uses-item">
                            <span class="uses-item-label">"Orchestration: Kiri Compose Spec"</span>
                            <span class="uses-item-desc">"Declarative service definitions running via kiri compose up -d with zero daemon bloat."</span>
                        </li>
                        <li class="uses-item">
                            <span class="uses-item-label">"Sandboxing: cgroups v2 & seccomp-BPF"</span>
                            <span class="uses-item-desc">"Fine-grained memory/CPU quota enforcement and strict syscall allowlists for safe execution."</span>
                        </li>
                    </ul>
                </div>

                <div class="uses-card">
                    <div class="uses-card-header">
                        <span class="uses-card-icon">"🦀"</span>
                        <h2 class="uses-card-title">"Rust Toolchain & Systems Tooling"</h2>
                    </div>
                    <ul class="uses-item-list">
                        <li class="uses-item">
                            <span class="uses-item-label">"Compiler: rustc Nightly + Stable"</span>
                            <span class="uses-item-desc">"Managed via rustup with custom profiles, sccache compiler caching, and mold linker."</span>
                        </li>
                        <li class="uses-item">
                            <span class="uses-item-label">"Profiling: cargo-flamegraph & Miri"</span>
                            <span class="uses-item-desc">"Stacked sampling profiler for CPU bottlenecks and UB interpreter for unsafe code validation."</span>
                        </li>
                        <li class="uses-item">
                            <span class="uses-item-label">"Testing: cargo-nextest & criterion"</span>
                            <span class="uses-item-desc">"Parallel test runner with JUnit reporting and statistical latency benchmarking."</span>
                        </li>
                        <li class="uses-item">
                            <span class="uses-item-label">"Editor: Neovim (v0.10 Lua) + rust-analyzer"</span>
                            <span class="uses-item-desc">"Tree-sitter AST syntax, Telescope fuzzy search, Harpoon project navigation."</span>
                        </li>
                        <li class="uses-item">
                            <span class="uses-item-label">"Database: Stoolap v0.5.1 & Tantivy"</span>
                            <span class="uses-item-desc">"Embedded MVCC SQL engine and pure-Rust inverted index full-text search engine."</span>
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
