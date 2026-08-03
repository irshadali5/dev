---
title: "Error Handling"
description: "Use thiserror when callers need to match on error variants (control flow). Use anyhow when callers should just propagate failures (reporting only)."
order: 7
---

## The Two Purposes of Errors

Internal

At the Edge

**Control Flow**

Types, enums, methods

HTTP status codes

**Reporting**

Logs / traces

Response body

## Using \`thiserror\` for Domain Errors

**`src/routes/subscriptions.rs`**

```rust
#[derive(thiserror::Error)]
pub enum SubscribeError {
    #[error("{0}")]
    ValidationError(String),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl axum::response::IntoResponse for SubscribeError {
    fn into_response(self) -> axum::response::Response {
        match self {
            SubscribeError::ValidationError(_) => (
                axum::http::StatusCode::BAD_REQUEST,
                self.to_string(),
            ).into_response(),
            SubscribeError::UnexpectedError(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong",
            ).into_response(),
        }
    }
}
```

## Using \`anyhow\` for Opaque Errors

**`src/email_client.rs`**

```rust
use anyhow::Context;

pub async fn send_email(&self, ...) -> Result<(), anyhow::Error> {
    let response = self.http_client
        .post(&url)
        .header("X-Postmark-Server-Token", self.token.expose_secret())
        .json(&request_body)
        .send()
        .await
        .context("Failed to send HTTP request to email provider")?
        .error_for_status()
        .context("Email provider returned an error status")?;

    Ok(())
}
```

  
> [!NOTE]  
> Use thiserror when callers need to match on error variants (control flow). Use anyhow when callers should just propagate failures (reporting only).  

  
> [!IMPORTANT]  
> We strictly segregate errors: thiserror for domain libraries (enumerable, structured errors) and anyhow for applications (context-rich, opaque errors). By compiling with RUST_BACKTRACE=1 and utilizing std::backtrace::Backtrace::capture(), we embed the exact stack trace inside our custom error types. Our Axum panic handler catches thread panics, records the backtrace to our telemetry system, and safely returns a 500 without crashing the entire Tokio runtime.