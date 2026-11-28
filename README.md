# Developer Portfolio, Blog & Books Platform (Rust + Leptos + Stoolap)

A fast, content-driven personal developer website built with **Rust**, **Leptos (SSR + WASM hydration)**, **Axum**, **Stoolap** (embedded MVCC SQL), and **Tantivy** (full-text search).

## Customizing for Your Own Site

Everything personal lives in `content/` and `.env` — you do **not** need to edit Rust code to deploy your own site:

| File / Directory | Purpose |
| :--- | :--- |
| `content/site_config.ron` | Site title, author, handle, role, email, bio, domain, social links, DB config |
| `content/home.ron` | Hero status/pills, architecture pillars, terminal footer stack & skill bars |
| `content/about.md` | About Me page (Markdown) |
| `content/resume.ron` | Resume summary, timeline (experience, certifications, education), and skills |
| `content/uses.ron` | Hardware, OS, container runtime, and developer toolchain catalog |
| `content/articles/<slug>/` | Blog posts (`meta.ron` + `article.md`), merged with DB-authored articles |
| `content/books/<slug>/` | Multi-chapter technical books (`book.ron` + `*.md` chapters) |
| `content/projects/<slug>.ron` + `.md` | Portfolio projects and case studies |

## Quick Start (Local)

1. Copy `.env.example` to `.env` and set `JWT_SECRET`, `ADMIN_EMAIL`, and `ADMIN_PASSWORD`.
2. Edit the files in `content/` with your own details.
3. Run the app:
   ```bash
   cargo leptos watch
   ```
   Or build and run tests via `cargo xtask`:
   ```bash
   cargo xtask check
   ```

## Deployment & Automation (`cargo xtask`)

All automation lives in Rust (`xtask/`):

- `cargo xtask check` — runs `cargo test --workspace` and checks both `ui --features hydrate` and `app --features ssr`.
- `cargo xtask sync-content` — syncs `content/` into `deploy/content/`.
- `cargo xtask deploy` — loads `.env` (`DOMAIN`, `IP_ADDRESSE`, `USER`, `KEY`), syncs content, uploads `deploy/`, and runs remote setup.
