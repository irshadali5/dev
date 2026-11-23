# Master Blueprint & Gap Analysis: Full-Fledged Personal Developer Web Platform
**Project**: `irshadali.org` | **Stack**: Pure Rust (Leptos SSR + Axum + Tokio + SQLite/PostgreSQL + Tantivy)

---

## 1. Executive Summary & Architectural Audit

The current codebase establishes a solid Rust workspace architecture with separate domain, database, API, UI, and feature crates (`portfolio`, `books`, `articles`, `search`, `analytics`, `auth`, `assets`, `markdown`). However, to transition from an initial prototype into a **world-class, full-fledged personal developer platform and publishing engine**, significant architectural, functional, content-pipeline, UI/UX, and operational gaps must be resolved.

This document details:
1. **What is currently lacking / incomplete in the codebase** across all architectural layers.
2. **Concrete technical suggestions & feature specifications** to achieve 100% full-fledged capability.
3. **A step-by-step, 1-by-1 implementation roadmap** with exact file modifications and crate responsibilities.

---

## 2. Exhaustive Gap Analysis: What Is Lacking & What To Add

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           CURRENT GAPS OVERVIEW                             │
├──────────────────────┬──────────────────────┬───────────────────────────────┤
│ Layer / Domain       │ Current State        │ Full-Fledged Target State     │
├──────────────────────┼──────────────────────┼───────────────────────────────┤
│ Dynamic Content      │ Hardcoded / Minimal  │ Git-backed + DB Hybrid CMS    │
│ Search Engine        │ In-Memory Tantivy    │ Persistent + Snippets + Tags  │
│ Articles & Books     │ Static / Broken Nav  │ TOC, Read Time, Series, Math  │
│ Admin CMS Studio     │ Minimal Forms        │ Live MD Preview, Media Mgmt   │
│ Interactive Demos    │ None                 │ WASM Playground, Arch Viewer  │
│ SEO & Feeds          │ Hardcoded Mock XML   │ Dynamic RSS/Atom, JSON-LD, OG │
│ Performance & Cache  │ Raw DB queries       │ Moka Cache + ETag + Lazy SSR  │
│ Community & Reach    │ Logged-only Contact  │ Stored Forms, Newsletter, E-mail│
└──────────────────────┴──────────────────────┴───────────────────────────────┘
```

---

### Category 1: Routing, Navigation & Content Consistency Bugs

#### 1.1 Inconsistent Article URLs and Routing
* **Current Gap**: 
  - In `ui/src/lib.rs`, `ArticlesPage` links to `<A href=format!("/writing/{}", slug)>` and the back link points to `/writing`, while the Leptos route definition is registered as `<Route path="articles/:slug" view=ArticlePage/>`. This results in `404 Not Found` upon clicking any article.
  - No slug redirect or alias support exists between `/articles/:slug`, `/writing/:slug`, and `/blog/:slug`.
* **Suggestion to Add**:
  - Unify routing to `/articles` and `/articles/:slug`, with secondary route aliases or 301 redirects for legacy `/writing` and `/blog` paths.
  - Add tag filtering route: `/articles/tag/:tag` and category route `/articles/category/:cat`.

#### 1.2 Book Engine Disconnect (DB vs. Filesystem)
* **Current Gap**:
  - `BooksPage` fetches items from SQLite via `get_books()`, but `BookIndexPage` and `BookChapterPage` read directly from `content/books/{slug}/book.ron` and local markdown files.
  - If a book is added via Admin UI into the database, it lacks corresponding chapter files on disk; conversely, rich multi-chapter books in `content/books/` (e.g., `production-grade-rust`, `anyhow-tutorial`, `clap-tutorial`, `zero-to-production`) may not render if their database metadata is missing.
* **Suggestion to Add**:
  - Create a unified `BookSyncService` that scans `content/books/` during startup or admin trigger, synchronizing filesystem book manifests (`book.ron`) and chapter lists with SQLite/PostgreSQL metadata.
  - Allow dual-mode book rendering: Local Markdown Git-CMS for technical manuals, and DB entries for reading list / review recommendations.

---

### Category 2: Markdown, Technical Typography & Reading Experience

#### 2.1 Missing Server-Side Syntax Highlighting & Code UX
* **Current Gap**:
  - Markdown parsing (`markdown/src/parser.rs` and `api/src/lib.rs`) uses basic `pulldown_cmark` without server-side syntax highlighting, relying on client-side CDN Highlight.js script tags. This causes Layout Shift (CLS) and flash of un-highlighted code.
  - No code block toolbar: Missing **One-Click Copy**, Language Badge (e.g., `Rust`, `SQL`, `TOML`), Line Numbers, and Diff highlighting (`+` / `-` colored rows).
* **Suggestion to Add**:
  - Integrate `syntect` or server-side highlighted tokenization into `markdown::MarkdownParser` for instant, zero-JS highlighted HTML output.
  - Add client-side Web Component or Leptos action for interactive code copy with toast feedback (`"Copied to clipboard!"`).

#### 2.2 Mathematical Formulas (LaTeX / KaTeX) & Diagram Support
* **Current Gap**:
  - No support for technical math formulas ($O(N \log N)$, $\sum_{i=1}^n x_i$) or architecture diagrams.
* **Suggestion to Add**:
  - Extend `markdown` parser to recognize `$...$` (inline) and `$$...$$` (block) math, converting them via KaTeX SSR or client-side KaTeX rendering.
  - Integrate Mermaid.js client-side loader for fenced ` ```mermaid ` code blocks to render interactive architecture graphs and state machines.

#### 2.3 Article / Chapter Reading Enhancements
* **Current Gap**:
  - Missing Table of Contents (TOC) with active scrollspy.
  - Missing Estimated Reading Time (e.g., `5 min read` computed at ~200 words/min).
  - Missing Article Series navigation (e.g., "Part 3 of Building Distributed Systems").
  - Missing Previous / Next Chapter floating navigation footer in books and tutorials.
* **Suggestion to Add**:
  - AST-based heading extractor in `markdown` parser that generates a structured TOC JSON `Vec<{ level: u8, title: String, anchor: String }>`.
  - Floating sticky TOC sidebar with intersection observer highlight.
  - Reading progress bar fixed at the top viewport.

---

### Category 3: Search Engine & Full-Text Discovery

#### 3.1 Persistent Disk-backed Index & Unified Document Types
* **Current Gap**:
  - `search/src/lib.rs` uses `Index::create_in_ram()`, rebuilding the index from scratch on every server boot. If articles or books are updated at runtime, the in-memory index gets out of sync.
  - Only articles are indexed; books, chapters, project cases, notes, and skills are completely omitted from the search index.
  - `snippet` generation is currently a placeholder (`let snippet = title.clone()`).
* **Suggestion to Add**:
  - Switch Tantivy to a persistent directory storage (`content/.search_index` or configurable path).
  - Index all entity types: `Article`, `BookChapter`, `Project`, `Skill`, `Talk`.
  - Use Tantivy `SnippetGenerator` to provide rich contextual highlights matching user search terms.
  - Add search modal overlay (`Cmd+K` / `Ctrl+K` keyboard shortcut) for instant global search across the entire site.

---

### Category 4: Portfolio, Projects & Interactive Developer Showcase

#### 4.1 Missing Project Case Studies & Deep Dives
* **Current Gap**:
  - `projects` table only has basic fields (`slug`, `title`, `description`, `content_html`, `github_url`, `live_url`).
  - Lacks structured architecture breakdown, tech stack taxonomy badges, screenshots gallery, benchmarks, metrics, and lessons learned.
* **Suggestion to Add**:
  - Expand project schema to support `featured: bool`, `architecture_diagram: Option<String>`, `metrics: Option<String>`, `tech_stack: Vec<String>`, `screenshots: Vec<String>`.
  - Add dedicated Project Detail page (`/projects/:slug`) rendering comprehensive engineering case studies with system architecture diagrams.

#### 4.2 Interactive Rust WASM Code Playground
* **Current Gap**:
  - No live interactive code execution on the personal website to demonstrate systems code or algorithms.
* **Suggestion to Add**:
  - Add an embedded Rust WebAssembly Playground component where visitors can edit and run sandboxed Rust / SQL / WebAssembly snippets directly in the browser or via a server-side execution runner.

#### 4.3 Interactive Resume & Dynamic PDF Generation
* **Current Gap**:
  - "Download Resume" button on hero section has `href="#"`.
* **Suggestion to Add**:
  - Build a dedicated `/resume` page pulling live data from `timeline_items` and `skills` repositories.
  - Add a printable CSS stylesheet (`@media print`) and an automated PDF exporter route (`/resume.pdf` or headless Chromium / `typst` backend).

---

### Category 5: Admin CMS & Content Management Studio

#### 5.1 Admin CRUD Incompleteness
* **Current Gap**:
  - Admin sections only have "Add" and "Delete" actions. No "Edit / Update" capability exists for articles, projects, books, skills, or timeline entries without raw DB manipulation.
  - No markdown editor with side-by-side live preview in the admin panel.
  - `AdminOverview` displays mock hardcoded metrics ("4", "12", "1,492", "24ms") rather than live server telemetry.
* **Suggestion to Add**:
  - Implement full CRUD: Add `UpdateArticle`, `UpdateProject`, `UpdateTimelineItem`, `UpdateSkill`, `UpdateBook` server functions and editing modal / page views.
  - Add a rich Split-Pane Markdown / RON editor with auto-save drafts in `localStorage`.
  - Live Admin Telemetry: Real CPU/Memory usage (via `sysinfo`), actual database record counts, and true 7-day / 30-day unique visitor metrics.

#### 5.2 Media & Asset Manager
* **Current Gap**:
  - No file upload interface in Admin UI; images must be copied manually via terminal into `content/` folders.
* **Suggestion to Add**:
  - Axum multipart file upload endpoint (`/api/upload`) with automated WebP/AVIF generation via `assets::AssetOptimizer`.
  - Media Library gallery in Admin panel allowing one-click markdown image snippet copying (`![alt](/assets/uploads/img.webp)`).

---

### Category 6: Community, Communication & Engagement

#### 6.1 Persistent Contact Messages & Notification Dispatcher
* **Current Gap**:
  - `submit_contact_form` in `api/src/lib.rs` simply logs submissions with `println!`. If server restarts, inbound messages are lost.
* **Suggestion to Add**:
  - Create `contact_messages` table in database (`id`, `name`, `email`, `message`, `read`, `created_at`).
  - Store all submissions in DB and display in Admin Panel (`/admin/messages`) with unread badge.
  - Optional webhook / email dispatcher (e.g. SMTP / SendGrid / Telegram Bot notification upon new message).

#### 6.2 Newsletter Subscription System
* **Current Gap**:
  - No newsletter capture mechanism exists.
* **Suggestion to Add**:
  - Create `subscribers` table (`id`, `email`, `verified`, `token`, `created_at`).
  - Add newsletter signup widget in footer and article sidebars.
  - Add double opt-in verification flow and RSS-to-Email blast helper in Admin.

#### 6.3 Comments & Social Reactions
* **Current Gap**:
  - Readers cannot leave feedback, ask questions, or clap/react to articles.
* **Suggestion to Add**:
  - Light-weight privacy-first comment system (stored in SQLite with admin moderation) OR Giscus / GitHub Discussions integration.
  - Article clap / like counter button with server-fn atomic increment.

---

### Category 7: SEO, Feeds, Metadata & OpenGraph

#### 7.1 Dynamic Sitemap & Multi-Feed Engine (RSS / Atom / JSON Feed)
* **Current Gap**:
  - `app/src/server.rs` has hardcoded strings for `rss_handler` (single dummy item) and `sitemap_handler` (3 static URLs).
* **Suggestion to Add**:
  - Dynamically query database + content files to build valid XML sitemap including all articles, books, chapters, and projects with `<lastmod>` timestamps.
  - Generate full RSS 2.0 (`/rss.xml`), Atom (`/atom.xml`), and JSON Feed (`/feed.json`) containing complete article contents.
  - Dynamic `robots.txt` generator.

#### 7.2 Dynamic OpenGraph (OG) Card Generator
* **Current Gap**:
  - Sharing links on Twitter/LinkedIn/Discord shows generic or missing preview cards.
* **Suggestion to Add**:
  - Server endpoint (`/api/og?title=...&category=...`) that dynamically generates branded SVG/PNG preview banners for social media crawlers.
  - Inject accurate `og:title`, `og:description`, `og:image`, `twitter:card` meta tags into every page via `leptos_meta`.

---

### Category 8: Performance, Caching & Resilience

#### 8.1 In-Memory Caching (Moka) & HTTP Cache Headers
* **Current Gap**:
  - Every page request and server function hits SQLite directly without in-memory caching.
* **Suggestion to Add**:
  - Implement `moka::future::Cache` for high-frequency queries: article lists, book chapters, search index lookups, and rendered markdown HTML.
  - Add HTTP caching headers (`Cache-Control: public, max-age=3600, stale-while-revalidate=86400`, `ETag`) for static assets and immutable book content.

#### 8.2 Configuration System & Multi-Database Backend (SQLite + Neon Postgres)
* **Current Gap**:
  - `config/src/lib.rs` and `shared/src/lib.rs` are empty boilerplate.
  - Database is hardcoded to SQLite; `plan.md` outlines cloud Neon PostgreSQL support.
* **Suggestion to Add**:
  - Implement RON-based configuration loader in `config` crate (`config/app.ron`, `config/database.ron`, `config/security.ron`).
  - Abstract repository traits in `domain` or `database` to allow zero-downtime switching between local SQLite (for offline development) and Neon PostgreSQL (for cloud deployment).

---

## 3. Step-by-Step Implementation Roadmap (1-by-1 Plan)

Here is the exact sequential roadmap to build out every missing component step-by-step:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                       1-BY-1 EXECUTION ROADMAP                              │
├───────┬──────────────────────────────────┬──────────────────────────────────┤
│ Step  │ Task Name                        │ Crates / Files Affected          │
├───────┼──────────────────────────────────┼──────────────────────────────────┤
│ 01    │ Fix Routing & Navigation Links   │ ui/src/lib.rs, app/src/server.rs │
│ 02    │ Server-Side Markdown & Syntect   │ markdown/, ui/src/lib.rs, css    │
│ 03    │ Dynamic Sitemap & Multi-Feed     │ app/src/server.rs, api/src/lib.rs│
│ 04    │ Persistent Tantivy Search Index  │ search/src/lib.rs, api/src/lib.rs│
│ 05    │ Book Manifest & Chapter Sync     │ books/, api/src/lib.rs, ui/      │
│ 06    │ Interactive Tech Showcase & Code │ ui/src/lib.rs, portfolio/        │
│ 07    │ Full Admin CRUD & Live Telemetry │ ui/src/lib.rs, api/, database/   │
│ 08    │ Contact Messages & Newsletter    │ database/, api/, ui/src/lib.rs   │
│ 09    │ Caching Layer (Moka) & Config    │ config/, shared/, app/src/main.rs│
│ 10    │ Social OG Generator & SEO Polish │ app/src/server.rs, ui/src/lib.rs │
└───────┴──────────────────────────────────┴──────────────────────────────────┘
```

---

### Detailed 1-by-1 Execution Breakdown

#### Step 1: Fix Routing, Broken Links & Navigation Consistency
1. In `ui/src/lib.rs`, correct the article links in `ArticlesPage` and `ArticlePage` from `/writing/:slug` to `/articles/:slug`.
2. Add route redirects in `app/src/server.rs` or Axum router for `/writing` $\to$ `/articles`.
3. Add secondary navigation tabs for:
   - `/articles` (Articles & Tech Essays)
   - `/books` (Interactive Books & Guides)
   - `/projects` (Portfolio & Case Studies)
   - `/about` (Biography, Tech Stack & Philosophy)
   - `/resume` (Interactive CV & Printable Resume)
   - `/search` (Global Instant Search)

#### Step 2: Upgrade Markdown Engine & Code Syntax Highlighting
1. Update `markdown` crate to support:
   - Heading ID generation & Anchor Link injection (`#heading-id`).
   - AST extraction of Table of Contents (`TocItem { level, title, id }`).
   - Server-side syntax highlighting with clean CSS themes (Atom One Dark / Dracula).
   - KaTeX math block recognition.
   - Reading time estimator (`words.count() / 200`).
2. Add copy button and language pill to code block headers in `ui` and `app/style/main.css`.

#### Step 3: Dynamic RSS, Atom, JSON Feeds & Sitemap
1. Refactor `app/src/server.rs`:
   - Replace dummy `rss_handler` with dynamic SQLx query fetching all published articles and books.
   - Replace static `sitemap_handler` with an XML generator iterating over all valid routes, articles, books, chapters, and projects.
   - Add `/feed.json` endpoint supporting JSON Feed v1.1 format.
   - Add `/robots.txt` dynamic handler.

#### Step 4: Persistent Full-Text Search with Tantivy
1. Update `search/src/lib.rs`:
   - Store search index on disk (`content/.search_index`).
   - Index `Article`, `BookChapter`, `Project`, `Skill`, and `TimelineItem`.
   - Implement real Snippet Generation with `<mark>` highlight tags.
2. In `ui/src/lib.rs`, add search keyboard shortcut listener (`Cmd+K` / `Ctrl+K`) opening a quick-search palette.

#### Step 5: Unify Books Engine & Reading Experience
1. Refactor `books/src/service.rs` to automatically discover all books in `content/books/` and synchronize their metadata into the reading list.
2. Enhance `BookChapterPage` in `ui/src/lib.rs`:
   - Add Chapter Sidebar with completion checkboxes.
   - Add Next Chapter / Previous Chapter navigation buttons.
   - Add sticky reading progress indicator.

#### Step 6: Case Studies, Interactive Showcase & Resume
1. Expand `Project` model and database schema with `featured`, `tech_stack`, `architecture_svg`, `metrics`, and `lessons_learned`.
2. Create `ProjectDetailPage` (`/projects/:slug`) in `ui/src/lib.rs` for full engineering case studies.
3. Build the dedicated `/resume` page with clean, professional formatting and print stylesheet (`@media print`).

#### Step 7: Complete Admin CMS Studio & System Health Dashboard
1. Implement Edit/Update server functions in `api/src/lib.rs` and repositories in `database/`:
   - `UpdateArticle`, `UpdateProject`, `UpdateSkill`, `UpdateTimelineItem`, `UpdateBook`.
2. Add Edit modals and split-pane Markdown editor with live preview in `AdminArticles` and `AdminProjects`.
3. Replace hardcoded dashboard in `AdminOverview` with live system telemetry:
   - Real SQLite connection pool status.
   - Real database row counts.
   - Real Tantivy index document counts.
   - Server uptime and host memory stats.

#### Step 8: Persistent Contact Dispatcher & Newsletter Module
1. Add migration for `contact_messages` table (`id`, `name`, `email`, `message`, `is_read`, `created_at`).
2. Add migration for `subscribers` table (`id`, `email`, `token`, `confirmed`, `created_at`).
3. Update `submit_contact_form` in `api/src/lib.rs` to insert into `contact_messages`.
4. Add `/admin/messages` view to read and manage incoming inquiries.
5. Add `SubscribeNewsletter` server function and UI component.

#### Step 9: In-Memory Caching (Moka) & RON Configuration
1. Initialize `moka` cache in Axum state for fast retrieval of:
   - Processed article HTML.
   - Search query results.
   - Aggregated portfolio timeline and skill trees.
2. Populate `config/src/lib.rs` to parse `config/*.ron` configuration files for environment customization.

#### Step 11: Global Keyboard Command Palette (Cmd+K / Ctrl+K)
1. Floating spotlight command palette with keyboard navigation (`↑`/`↓`/`Enter`/`ESC`).
2. Global `keydown` shortcut listener and debounced multi-entity search integration.

#### Step 12: Interactive Tag Filter Chips
1. Dynamic extraction of unique tags from loaded articles and projects.
2. Reactive client-side filtering without page reload.

#### Step 13: Reading Progress Bar & Scrollspy
1. Real-time top viewport reading progress bar ($0 \to 100\%$) for long-form tutorials and book chapters.

#### Step 14: Theme Persistence & OS Sync
1. `localStorage` caching with automatic `prefers-color-scheme` OS detection and zero-FOUC inline script.

#### Step 15: Rust Playground Code Runner
1. Interactive "▶ Run" button on Rust code blocks.
2. Server function proxying execution to the official Rust Playground API with real-time terminal output drawer.

#### Step 16: KaTeX Math, Mermaid Diagrams & Production Deployment
1. Integrated KaTeX for LaTeX expressions and Mermaid for architecture flowcharts.
2. Multi-stage production `Dockerfile`, Kiri `compose.yml` (`docker-compose.yml`), and auto-SSL `pingap.toml` (Pingap pure-Rust reverse proxy).

#### Step 17: Production System Health (`/healthz`) & Prometheus Telemetry (`/metrics`)
1. `/healthz` endpoint returning live SQLite connection pool metrics, index status, and system uptime.
2. `/metrics` endpoint returning Prometheus exposition format telemetry for monitoring.

#### Step 18: Interactive "Listen to Content" Web Speech Synthesizer (Audio Reader)
1. Browser-native speech synthesizer (`SpeechSynthesis`) on `ArticlePage` and `BookChapterPage`.
2. Play, pause, stop controls and playback speed rate selectors ($0.8\times, 1.0\times, 1.25\times, 1.5\times, 2.0\times$).

#### Step 19: Systems Engineering Tools & Latency Benchmark Visualizer (`/tools`)
1. Interactive "Latency Numbers Every Programmer Should Know" comparison with logarithmic bars and human-scale time analogies.
2. Little's Law throughput and concurrency capacity sizing calculator ($Q = \frac{\text{Workers}}{\text{Latency}}$).

#### Step 20: Reading Bookmarks & Recent History
1. Local storage bookmark manager (`BookmarkButton`) on articles and book chapters.
2. Quick-access bookmarks integration in `Cmd+K` command palette.

#### Step 21: Live GitHub Activity Stream & Open Source Metrics Service
1. Server-side cached GitHub event aggregator (`GetGithubActivity`) with fallback resilience.
2. Real-time open-source commit telemetry feed displayed on `HomePage`.

#### Step 22: Automated CI/CD Testing & Release Automation
1. GitHub Actions workflow (`.github/workflows/ci.yml`) validating workspace compilation, tests, and release packaging.

---

## 4. Verification & Quality Checklist (Status: 100% Completed Across All 22 Phases)

- [x] **Routing & Navigation**: All internal navigation links resolve without 404 errors (`/articles`, `/books`, `/projects`, `/about`, `/contact`, `/tools`, `/resume`, `/search`, `/blog`, `/writing`).
- [x] **Server-Side Markdown & Syntect**: SSR Code syntax highlighting renders with language badges, copy buttons, callouts, and header anchors.
- [x] **Multi-Feeds & Sitemap**: Dynamic `/rss.xml`, `/atom.xml`, `/feed.json`, and `/sitemap.xml` automatically index 100% of published articles, books, chapters, and projects from DB and filesystem.
- [x] **Persistent Search Engine**: Tantivy disk-based index with query parser, SnippetGenerator, and search palette across articles, books, projects, and skills.
- [x] **Unified Books Engine**: Local filesystem technical manuals (`production-grade-rust`, `anyhow-tutorial`, `clap-tutorial`, `zero-to-production`) seamlessly unified with DB reading lists, chapter navigation, and TOCs.
- [x] **Project Case Studies & Architecture Showcase**: Dynamic `/projects/:slug` deep-dive case studies with tech stack pills, GitHub/Demo links, and architecture breakdowns.
- [x] **Full Admin CMS Studio**: CRUD update actions (`UpdateArticle`, `UpdateProject`, `UpdateBook`), inbound contact inbox, live analytics telemetry, and index rebuild settings.
- [x] **Contact Dispatcher & Newsletter Module**: SQLite `contact_messages` and `newsletter_subscribers` tables with persistent storage and interactive UI subscription forms.
- [x] **In-Memory Cache & Site Config**: `moka` cache and `SiteConfig` loader from `content/site_config.ron`.
- [x] **Dynamic OpenGraph Generator & SEO**: `/api/og` SVG card generation, Twitter cards, RSS feed autodiscovery, and structured meta tags.
- [x] **Global Command Palette (`Cmd+K`)**: Instant spotlight overlay across all pages with keyboard shortcuts and search suggestions.
- [x] **Interactive Tag Filters**: Client-side reactive tag filter chips on `/articles` and `/projects`.
- [x] **Reading Progress Bar**: Dynamic top-fixed scroll progress indicator for articles and book chapters.
- [x] **Persistent Theme Preference**: `localStorage` persistence with zero-FOUC initialization and OS preference sync.
- [x] **Interactive Rust Playground**: "▶ Run" button on Rust code blocks executing against the Rust Playground API with terminal drawers.
- [x] **KaTeX Math & Mermaid Diagrams**: Formula rendering and interactive flowchart diagrams.
- [x] **Production Deployment**: Production `Dockerfile`, Kiri `compose.yml` (`docker-compose.yml`), and `pingap.toml` (Pingap reverse proxy).
- [x] **System Health & Prometheus Metrics**: Live `/healthz` and `/metrics` telemetry endpoints.
- [x] **Interactive Voice Audio Reader**: In-browser speech synthesizer for articles and book chapters.
- [x] **Systems Engineering Tools & Latency Benchmark Visualizer**: Dedicated `/tools` route with human-scale latency analogies and Little's Law throughput calculator.
- [x] **Reading Bookmarks**: Local storage bookmark manager on articles and book chapters with `Cmd+K` palette integration.
- [x] **Live GitHub Activity Feed**: Real-time cached open source commit stream on the homepage.
- [x] **Automated CI/CD Pipeline**: GitHub Actions workflow (`.github/workflows/ci.yml`) for workspace checks and tests.
- [x] **Compilation & Test Suite**: `cargo test --workspace` passes cleanly across all 12 crates in the workspace with 0 errors and 0 warnings.
