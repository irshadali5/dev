# Developer Portfolio, Blog & Books Platform

A fast, content-driven personal publishing platform built in **100% safe Rust** (`unsafe_code = "forbid"`). It combines a developer portfolio, technical blog, multi-chapter digital library, full-text search engine, and admin dashboard into a single self-hostable binary powered by **Leptos (SSR + WASM hydration)**, **Axum**, **Stoolap** (embedded MVCC SQL), and **Tantivy**.

---

## Table of Contents

- [Features](#features)
- [Architecture](#architecture)
- [Technology Stack](#technology-stack)
- [Workspace Structure](#workspace-structure)
- [Content & Customization](#content--customization)
- [Routes & Endpoints](#routes--endpoints)
- [Local Development](#local-development)
- [Environment Variables](#environment-variables)
- [Automation (`cargo xtask`)](#automation-cargo-xtask)
- [Production Deployment](#production-deployment)
- [Security & Performance](#security--performance)

---

## Features

- **Server-Side Rendering + WASM Hydration**: Fast initial HTML delivery via Leptos SSR with interactive client-side hydration and SPA-like navigation.
- **Git-First Content + Database Hybrid**: Author articles, multi-chapter books, portfolio projects, resume timelines, and `/uses` catalogs in Markdown and RON files under `content/`, seamlessly merged with dynamic content created in the `/admin` dashboard.
- **Full-Text Search**: Embedded **Tantivy** search index automatically populated on startup and rebuildable from the admin panel.
- **Technical Writing & Code Highlighting**: Markdown rendering via `pulldown-cmark` with server-side syntax highlighting (`syntect`), reading time, likes, and a built-in Rust code runner tool (`/tools`).
- **Asset Optimization Pipeline**: Automatic image processing and WebP conversion on startup (`content/assets/` -> `/media`).
- **Feeds, SEO & OpenAPI**: Built-in RSS (`/rss.xml`, `/feed`), Atom (`/atom.xml`, `/feed.atom`), JSON Feed (`/feed.json`), `sitemap.xml`, `robots.txt`, `manifest.json`, dynamic OpenGraph image generation (`/api/og`), and a public `/api/v1/*` JSON API with `/api/v1/openapi.json`.
- **Admin Dashboard (`/admin`)**: Protected by Argon2 password hashing and JWT authentication for managing articles, books, projects, portfolio timeline/skills, contact messages, analytics, and search index rebuilds.
- **Observability**: Built-in `/healthz` health check, `/metrics` endpoint, and structured logging with `tracing`.

---

## Architecture

```text
                         Internet (HTTPS / HTTP2)
                                    │
                                    ▼
                     Reverse Proxy (Pingap / Pingora)
                   ACME TLS · Compression · Cache Headers
                                    │
                                    ▼
                        Axum HTTP Server (:3000)
          ┌─────────────────────────┼─────────────────────────┐
          ▼                         ▼                         ▼
    Leptos SSR &              REST API v1 &             Static Assets
   Server Functions         Feeds / SEO / OG          (/pkg, /media, /)
          │                         │                         │
          └─────────────────────────┼─────────────────────────┘
                                    ▼
                     Application & Domain Services
         ┌─────────────┬────────────┼────────────┬─────────────┐
         ▼             ▼            ▼            ▼             ▼
     Portfolio     Articles       Books       Search        Auth &
      Service       Service      Service      Service      Analytics
         │             │            │            │             │
         └─────────────┴────────────┼────────────┴─────────────┘
                                    ▼
                        Infrastructure & Storage
          ┌─────────────────────────┼─────────────────────────┐
          ▼                         ▼                         ▼
   Stoolap (MVCC SQL)        content/ (RON + MD)      Tantivy Index
```

---

## Technology Stack

| Layer | Technology |
| :--- | :--- |
| **Language** | Rust (`unsafe_code = "forbid"`) |
| **Frontend / SSR** | Leptos 0.6 (`ssr` + `hydrate` WASM), `leptos_meta`, `leptos_router` |
| **HTTP Server** | Axum 0.7, Tokio, Tower, `tower-http` |
| **Database** | Stoolap 0.5 (embedded MVCC SQL engine) |
| **Search** | Tantivy 0.22 (embedded full-text search) |
| **Configuration & Metadata** | RON (`ron` 0.8) |
| **Markdown & Highlighting** | `pulldown-cmark`, `syntect` |
| **Serialization & Caching** | `serde`, `serde_json`, `postcard`, `moka` |
| **Authentication** | `argon2`, `jsonwebtoken` |
| **Images & Feeds** | `image`, `webp`, `rss` |
| **Reverse Proxy (Optional)** | Pingap (Cloudflare Pingora-based Rust proxy) |
| **Containerization (Optional)** | Kiri / Docker (`Dockerfile`, `compose.yml`) |

---

## Workspace Structure

The project is organized as a Cargo workspace with single-responsibility crates:

| Crate | Responsibility |
| :--- | :--- |
| `app` | Binary entrypoint (`main.rs`), Axum router, SSR setup, REST API v1, feeds, SEO, health/metrics, and security middleware |
| `ui` | Leptos components, pages, client-side routing, and WASM hydration |
| `api` | Leptos `#[server]` functions bridging `ui` and backend services |
| `domain` | Core domain models, entities, and shared types |
| `database` | Stoolap connection management, SQL migrations, and repositories |
| `config` | RON configuration loader (`site_config.ron`, etc.) |
| `articles` | Article models and service (merges `content/articles/` with DB records) |
| `books` | Multi-chapter book manifest (`book.ron`) and chapter service |
| `portfolio` | Projects, resume timeline, and skills services |
| `search` | Tantivy indexing and full-text query service |
| `markdown` | `pulldown-cmark` parser and `syntect` syntax highlighter |
| `assets` | Startup image processing and WebP optimization pipeline |
| `analytics` | Page view tracking and analytics aggregation |
| `auth` | Argon2 password hashing and JWT token issuance/verification |
| `shared` | Cross-crate shared utilities and helpers |
| `xtask` | Rust-native workspace automation (`check`, `sync-content`, `deploy`, `setup`) |
| `hasher` | Standalone utility binary |

---

## Content & Customization

All personal data, configuration, and static publishing content live in `content/` and `.env` — **no Rust code edits are required** to customize and deploy your own site.

| Path | Purpose |
| :--- | :--- |
| `content/site_config.ron` | Site title, author, handle, role, email, bio, domain, social links, and database path |
| `content/home.ron` | Hero status/pills, architecture pillars, terminal footer stack, and skill bars |
| `content/about.md` | About Me page content (Markdown) |
| `content/resume.ron` | Resume summary, career/education/certification timeline, and skills |
| `content/uses.ron` | Hardware, OS, container runtime, and developer toolchain catalog |
| `content/articles/<slug>/` | Blog posts (`meta.ron` + `article.md`), merged with DB-authored articles |
| `content/books/<slug>/` | Multi-chapter technical books (`book.ron` + ordered `*.md` chapters) |
| `content/projects/<slug>.ron` + `.md` | Portfolio projects, architecture notes, benchmarks, and case studies |
| `content/assets/` | Media assets served at `/media` and optimized on startup |

### Example Content Schemas

**`content/articles/<slug>/meta.ron`**
```ron
(
    slug: "rust-ownership",
    title: "Understanding Rust Ownership",
    description: "A deep dive into how Rust manages memory safely without a garbage collector.",
    published: true,
    tags: ["rust", "memory", "systems"],
    created_at: "2026-07-23",
)
```

**`content/books/<slug>/book.ron`**
```ron
(
    slug: "zero-to-production",
    title: "Zero to Production in Rust",
    subtitle: Some("An Opinionated Guide to Backend Development"),
    description: "Learn how to build production-grade backend services in Rust.",
    author: "Your Name",
    published: true,
    tags: ["Rust", "Backend", "Axum"],
    created_at: "2026-06-01",
    chapters: [
        "00-chapter.md",
        "01-nix.md",
    ],
)
```

---

## Routes & Endpoints

### Public Pages
- `/` — Home
- `/about` — About Me
- `/projects` & `/projects/:slug` — Portfolio projects & case studies
- `/articles`, `/writing`, `/blog` & `/:slug` variants — Technical blog
- `/books`, `/books/:slug` & `/books/:slug/:chapter` — Multi-chapter digital books
- `/resume`, `/experience`, `/career`, `/credentials` — Resume & career timeline
- `/uses` — Hardware & software setup
- `/tools` — Developer utilities (including Rust code runner)
- `/search` — Full-text search
- `/contact` — Contact form
- `/login` — Admin sign-in

### Admin Dashboard (`/admin`)
- `/admin` — Overview
- `/admin/analytics` — Page views & traffic metrics
- `/admin/messages` — Contact form submissions
- `/admin/portfolio` — Timeline & skills management
- `/admin/projects` — Project CRUD
- `/admin/articles` — Article CRUD
- `/admin/books` — Book CRUD
- `/admin/settings` — Site settings & search index rebuild

### Feeds, SEO & Observability
- `GET /rss.xml`, `GET /feed` — RSS 2.0 feed (supports `?tag=...`)
- `GET /atom.xml`, `GET /feed.atom` — Atom feed (supports `?tag=...`)
- `GET /feed.json` — JSON Feed
- `GET /sitemap.xml` — XML Sitemap
- `GET /robots.txt` — Robots directives
- `GET /manifest.json` — Web App Manifest
- `GET /api/og` — Dynamic OpenGraph image endpoint
- `GET /healthz` — Service & database health check
- `GET /metrics` — Runtime & content metrics

### Public REST API (`v1`)
- `GET /api/v1/articles`
- `GET /api/v1/articles/:slug`
- `GET /api/v1/projects`
- `GET /api/v1/books`
- `GET /api/v1/skills`
- `GET /api/v1/stats`
- `GET /api/v1/openapi.json`

*(Leptos internal server functions are mounted under `/api/*fn_name`.)*

---

## Local Development

### Prerequisites

1. **Rust** (stable toolchain)
2. **WASM target**:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. **`cargo-leptos`**:
   ```bash
   cargo install cargo-leptos --locked
   ```

### Setup & Run

1. Copy the example environment file and configure your secrets:
   ```bash
   cp .env.example .env
   ```
   Edit `.env` to set `JWT_SECRET`, `ADMIN_EMAIL`, and `ADMIN_PASSWORD`.

2. Customize the RON and Markdown files in `content/`.

3. Start the local dev server with hot-reloading (serves at `http://127.0.0.1:3000` by default):
   ```bash
   cargo leptos watch
   ```

4. Run the full workspace test suite and verify both `ssr` and `hydrate` targets:
   ```bash
   cargo xtask check
   ```

---

## Environment Variables

| Variable | Description | Example |
| :--- | :--- | :--- |
| `STOOLAP_DB_PATH` | Path to the embedded Stoolap database directory | `data/dev_site.stoolap` |
| `JWT_SECRET` | Secret key used to sign `/admin` JWT tokens | `change-me-to-a-long-random-secret` |
| `ADMIN_EMAIL` | Optional: bootstraps an admin user on startup if absent | `admin@example.com` |
| `ADMIN_PASSWORD` | Optional: password for the bootstrapped admin user | `change-me-strong-password` |
| `DOMAIN` | Deployment domain (used by `cargo xtask deploy` / `setup`) | `example.com` |
| `IP_ADDRESSE` | Target server IP for `cargo xtask deploy` | `203.0.113.10` |
| `USER` | SSH user for `cargo xtask deploy` | `ubuntu` |
| `KEY` | SSH private key path for `cargo xtask deploy` | `~/.ssh/id_ed25519` |

---

## Automation (`cargo xtask`)

Workspace automation is implemented in Rust inside `xtask/`:

- `cargo xtask check` — Runs `cargo test --workspace`, `cargo check -p ui --features hydrate`, and `cargo check -p app --features ssr`.
- `cargo xtask sync-content` — Syncs `content/` (excluding `.search_index`) into `deploy/content/`.
- `cargo xtask deploy [--no-sync] [--use-system-tar]` — Reads `.env`, syncs content, packages `deploy/`, uploads via SSH, and runs server setup.
- `cargo xtask setup [--domain ...] [--user ...] [--target-dir ...] [--bundle-dir ...] [--systemd]` — Server-side installation and `systemd` unit setup.

---

## Production Deployment

### Option 1: `cargo xtask deploy` (Bare-metal / VM + `systemd`)
Configure `DOMAIN`, `IP_ADDRESSE`, `USER`, and `KEY` in `.env`, build the release binary (`cargo leptos build --release`), and run:
```bash
cargo xtask deploy
```

### Option 2: Container + Pingap Reverse Proxy (`Dockerfile` & `compose.yml`)
Compatible with **Kiri** and **Docker Compose**:
```bash
# Build & start web + Pingap reverse proxy
kiri compose up -d
# or
docker compose up -d
```
`pingap.toml` configures automatic Let's Encrypt ACME TLS, HTTP-to-HTTPS redirects, HTTP/2, Brotli/Zstd/Gzip compression, and immutable caching headers for static assets.

---

## Security & Performance

- **Memory Safety**: `#![forbid(unsafe_code)]` enforced at the workspace level (`[workspace.lints.rust]`).
- **Security Headers**: Responses include `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `Referrer-Policy: strict-origin-when-cross-origin`, and `Permissions-Policy: camera=(), microphone=(), geolocation=()`.
- **Authentication**: Passwords hashed with **Argon2**; sessions authenticated via **JWT**.
- **Compression & Graceful Shutdown**: Full `tower-http` response compression and clean `SIGINT`/`SIGTERM` shutdown handling.
