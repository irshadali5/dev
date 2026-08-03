---
title: "Project Structure"
description: "We split our project into a library crate (lib.rs) and a thin binary (main.rs). This allows integration tests to import the library directly, enabling true black-box testing."
order: 2
---

## Creating the Project

**`terminal`**

```bash
cargo new zero2prod
cd zero2prod

# Initialize git
git init
git add .
git commit -m "Initial project skeleton"
```

## The Cargo Manifest

**`Cargo.toml`**

```toml
[package]
name = "zero2prod"
version = "0.1.0"
edition = "2024"
rust-version = "1.96"
authors = ["Your Name <you@example.com>"]

[lib]
path = "src/lib.rs"

[[bin]]
path = "src/main.rs"
name = "zero2prod"

[dependencies]
# Web framework
axum = { version = "0.8", features = ["macros"] }
axum-extra = { version = "0.10", features = ["typed-header", "form"] }
tower = { version = "0.5", features = ["util", "timeout"] }
tower-http = { version = "0.6", features = ["trace", "cors", "set-header", "timeout"] }

# Async runtime
tokio = { version = "1.45", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Database
sqlx = { version = "0.8", features = [
    "runtime-tokio",
    "tls-rustls",
    "postgres",
    "uuid",
    "chrono",
    "migrate",
] }

# Telemetry
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tracing-bunyan-formatter = "0.3"
tracing-log = "0.2"

# Error handling
thiserror = "2.0"
anyhow = "1.0"

# Utilities
uuid = { version = "1.16", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
secrecy = { version = "0.10", features = ["serde"] }
base64 = "0.22"
config = "0.14"

# Validation
garde = { version = "0.22", features = ["derive", "email", "pattern"] }

# HTTP client
reqwest = { version = "0.12", default-features = false, features = [
    "json",
    "rustls-tls",
] }

# Authentication
argon2 = "0.5"
hmac = "0.12"
sha2 = "0.10"
tower-sessions = "0.14"
tower-sessions-sqlx-store = { version = "0.14", features = ["postgres"] }

# Email
lettre = { version = "0.11", features = ["tokio1-rustls-tls"] }

[dev-dependencies]
wiremock = "0.6"
fake = { version = "3.0", features = ["derive", "uuid", "chrono"] }
quickcheck = "1.0"
quickcheck_macros = "1.0"
claim = "0.5"
once_cell = "1.20"
linkify = "0.10"
```

## Project Layout

zero2prod/
├── .cargo/
│   └── config.toml
├── configuration/
│   ├── base.yaml
│   ├── local.yaml
│   └── production.yaml
├── migrations/
│   └── 20260630_create_subscriptions.sql
├── scripts/
│   ├── init_db.sh
│   └── init_redis.sh
├── src/
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── subscriber_email.rs
│   │   └── subscriber_name.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── health_check.rs
│   │   ├── subscriptions.rs
│   │   └── newsletters.rs
│   ├── configuration.rs
│   ├── email_client.rs
│   ├── startup.rs
│   ├── telemetry.rs
│   ├── lib.rs
│   └── main.rs
├── tests/
│   └── api/
│       ├── main.rs
│       ├── helpers.rs
│       ├── health_check.rs
│       └── subscriptions.rs
├── Cargo.toml
├── Dockerfile
├── spec.yaml
└── .sqlx/

  
> [!NOTE]  
> We split our project into a library crate (lib.rs) and a thin binary (main.rs). This allows integration tests to import the library directly, enabling true black-box testing.  

  
> [!IMPORTANT]  
> As the project grows, a flat structure becomes unmaintainable. We apply Hexagonal Architecture via Cargo Workspaces. core/: Contains pure business logic. Zero dependencies on external crates like SQLx or Axum. adapters/: Implementations of the core traits (e.g., a Postgres adapter for the repository trait). api/: The Axum HTTP layer that glues adapters to the core. This decoupling allows us to swap databases or transport layers (e.g., from HTTP to gRPC) without touching the core domain logic, and makes unit testing incredibly fast using mock adapters.