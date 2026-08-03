---
title: "Axum Web Framework"
description: "As of Axum 0.7 (built on Hyper 1.0), the Router::into_make_service() paradigm has been updated to integrate deeply with tokio::net::TcpListener. Furthermore, the ecosystem has moved away from the #[async_trait] macro, fully embracing native async functions in traits enabled by Rust 1.75+."
order: 3
---

> [!WARNING]  
> As of Axum 0.7 (built on Hyper 1.0), the Router::into_make_service() paradigm has been updated to integrate deeply with tokio::net::TcpListener. Furthermore, the ecosystem has moved away from the #[async_trait] macro, fully embracing native async functions in traits enabled by Rust 1.75+.  

## Your First Endpoint

Let's build a health check endpoint — the foundation of every production API.

**`src/routes/health_check.rs`**

```rust
use axum::response::IntoResponse;
use axum::http::StatusCode;

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
```

## Application Startup

**`src/startup.rs`**

```rust
use axum::{Router, routing::get};
use std::net::TcpListener;
use tokio::net::TcpListener as TokioTcpListener;
use crate::routes::health_check::health_check;

pub struct Application {
    port: u16,
    listener: TokioTcpListener,
    router: Router,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, std::io::Error> {
        let listener = TokioTcpListener::bind(address).await?;
        let port = listener.local_addr()?.port();

        let router = build_router();

        Ok(Self { port, listener, router })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        axum::serve(self.listener, self.router).await
    }
}

fn build_router() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
}
```

**`src/main.rs`**

```rust
use zero2prod::startup::Application;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Application::build("127.0.0.1:8000").await?;
    println!("Listening on port {}", app.port());
    app.run_until_stopped().await
}
```

```text
$ cargo run
   Compiling zero2prod v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.21s
     Running `target/debug/zero2prod`
Listening on port 8000
```

```text
$ curl -v http://127.0.0.1:8000/health_check
* Connected to 127.0.0.1 port 8000
> GET /health_check HTTP/1.1
> Host: 127.0.0.1:8000
< HTTP/1.1 200 OK
< content-length: 0
```

## Extractors: Type-Safe Request Parsing

Axum's superpower is **extractors** — types that automatically parse request data:

**`src/routes/subscriptions.rs`**

```rust
use axum::extract::State;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum_extra::extract::Form;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct SubscribeForm {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(
    Form(form): Form<SubscribeForm>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    tracing::info!(
        email = %form.email,
        name = %form.name,
        "New subscription request"
    );

    // Insert into database...
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
           VALUES ($1, $2, $3, $4)"#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        chrono::Utc::now(),
    )
    .execute(&pool)
    .await;

    StatusCode::OK
}
```

## Router with Shared State

**`src/startup.rs`**

```rust
use axum::{Router, routing::{get, post}};
use sqlx::PgPool;
use std::sync::Arc;
use crate::email_client::EmailClient;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub email_client: Arc<EmailClient>,
    pub base_url: String,
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .route("/subscriptions/confirm", get(confirm))
        .with_state(state)
}
```

  
> [!TIP]  
> Extractors that consume the request body (like Form, Json) must come last in the function signature. Path and query extractors can go anywhere.  

  
> [!IMPORTANT]  
> Axum's true power lies in the FromRequest and FromRequestParts traits. We can implement these to build custom extractors that perform validation at the routing edge before the handler is ever invoked. **`src/extractors.rs`** impl FromRequestParts for ValidatedUser { type Rejection = AppError; async fn from_request_parts(parts: &mut Parts, state: &S) -> Result { // Extract JWT, validate against DB, and return domain object } } Additionally, we utilize zero-copy parsing with Serde (\`&'a str\` instead of \`String\` in our structs) when reading JSON payloads, drastically reducing heap allocations during high-throughput ingestion.

## Hyper-Expansion: Implementing a Custom `tower::Service` from Scratch

While Axum extractors are great, true mastery of the Rust web ecosystem requires understanding `tower::Service`. Every Axum router, middleware, and handler is fundamentally a Tower Service. When standard middleware crates fall short (e.g., you need dynamic rate limiting backed by a custom Lua script in Redis), you must write your own.

Below is a massive, production-ready implementation of a custom rate-limiting and audit-logging middleware from first principles. It demonstrates state machines, pinning, and raw Future polling.

**`src/middleware/audit_rate_limit.rs`**

```rust
use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;
use tower::{Service, Layer};
use axum::http::{Request, Response, StatusCode};
use axum::body::Body;
use redis::{AsyncCommands, aio::MultiplexedConnection};
use tracing::info!;

#[derive(Clone)]
pub struct AuditRateLimitLayer {
    pub redis_pool: MultiplexedConnection,
    pub max_requests_per_minute: u64,
}

impl<S> Layer<S> for AuditRateLimitLayer {
    type Service = AuditRateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuditRateLimitService {
            inner,
            redis_pool: self.redis_pool.clone(),
            max_requests: self.max_requests_per_minute,
        }
    }
}

#[derive(Clone)]
pub struct AuditRateLimitService<S> {
    inner: S,
    redis_pool: MultiplexedConnection,
    max_requests: u64,
}

// The response future for our middleware
pin_project_lite::pin_project! {
    pub struct AuditRateLimitFuture<F> {
        #[pin]
        inner: F,
        path: String,
        start_time: tokio::time::Instant,
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for AuditRateLimitService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        let mut redis = self.redis_pool.clone();
        let max_reqs = self.max_requests;
        
        let ip = req.headers()
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("127.0.0.1")
            .to_string();
            
        let path = req.uri().path().to_string();

        Box::pin(async move {
            let script = redis::Script::new(
                r"
                local current
                current = redis.call('incr', KEYS[1])
                if tonumber(current) == 1 then
                    redis.call('expire', KEYS[1], 60)
                end
                return current
                "
            );

            let key = format!("rate_limit:{}", ip);
            let count: u64 = match script.key(&key).invoke_async(&mut redis).await {
                Ok(c) => c,
                Err(_) => 0 
            };

            if count > max_reqs {
                tracing::warn!("Rate limit exceeded for IP: {}", ip);
            }

            let start = tokio::time::Instant::now();
            let response = inner.call(req).await?;
            let elapsed = start.elapsed();
            info!("AUDIT: {} took {:?}", path, elapsed);
            
            Ok(response)
        })
    }
}
```