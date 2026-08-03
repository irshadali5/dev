---
title: "Authentication"
description: "Always perform password hashing even when the username doesn't exist. Use a dummy hash to ensure constant-time responses and prevent user enumeration."
order: 11
---

## Password Hashing with Argon2id

**`src/authentication.rs`**

```rust
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<uuid::Uuid, AuthError> {
    let mut user_id = None;
    let mut expected_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
         gZiV/M1gPc22ElAH/Jh1Hw$\
         CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
        .to_string()
    );

    if let Some((stored_id, stored_hash)) = get_stored_credentials(&credentials.username, pool).await? {
        user_id = Some(stored_id);
        expected_hash = stored_hash;
    }

    // Offload CPU-intensive hashing to blocking threadpool
    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_hash, credentials.password)
    })
    .await
    .context("Failed to spawn blocking task")??;

    user_id.ok_or_else(|| AuthError::InvalidCredentials(anyhow::anyhow!("Unknown username")))
}

fn verify_password_hash(
    expected_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<(), AuthError> {
    let expected = PasswordHash::new(expected_hash.expose_secret())
        .context("Failed to parse PHC string")?;

    Argon2::default()
        .verify_password(password_candidate.expose_secret().as_bytes(), &expected)
        .context("Invalid password")
        .map_err(AuthError::InvalidCredentials)
}

fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>, anyhow::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.expose_secret().as_bytes(), &salt)?
    .to_string();
    Ok(Secret::new(hash))
}
```

## Session-Based Auth with tower-sessions

**`src/startup.rs`**

```rust
use tower_sessions::{SessionManagerLayer, Expiry};
use tower_sessions_sqlx_store::PostgresSessionStore;

pub async fn build_router(state: AppState) -> Router {
    let session_store = PostgresSessionStore::new(state.db.clone());
    session_store.migrate().await.unwrap();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(time::Duration::hours(24)));

    Router::new()
        .route("/health_check", get(health_check))
        .route("/login", get(login_form).post(login))
        .route("/admin/dashboard", get(admin_dashboard))
        .route("/admin/password", get(change_password_form).post(change_password))
        .route("/admin/logout", post(logout))
        .layer(session_layer)
        .with_state(state)
}
```

## Protecting Routes

**`src/routes/admin/dashboard.rs`**

```rust
use tower_sessions::Session;

pub async fn admin_dashboard(
    session: Session,
    pool: State<PgPool>,
) -> impl IntoResponse {
    let user_id: Option<uuid::Uuid> = session
        .get("user_id")
        .await
        .unwrap_or(None);

    match user_id {
        Some(id) => {
            let username = get_username(id, &pool).await.unwrap();
            Html(format!("<h1>Welcome, {username}!</h1>"))
        }
        None => {
            Redirect::to("/login").into_response()
        }
    }
}
```

  
> [!WARNING]  
> Always perform password hashing even when the username doesn't exist. Use a dummy hash to ensure constant-time responses and prevent user enumeration.  

  
> [!IMPORTANT]  
> We upgrade from basic sessions to OpenID Connect (OIDC), allowing Single Sign-On (SSO) via Google/GitHub. For API keys, we implement short-lived JWTs (JSON Web Tokens) with seamless background rotation using refresh tokens. For raw password storage, we use argon2id. We dynamically tune the memory cost (e.g., 64MB per hash) and parallelism based on our production server's hardware constraints (using sysinfo at startup) to maximize resistance against ASIC/GPU brute-force attacks while preserving API latency.