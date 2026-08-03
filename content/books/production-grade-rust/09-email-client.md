---
title: "Email Client"
description: "The reqwest::Client maintains an internal connection pool. Reuse a single instance across all requests by storing it in Arc<EmailClient> in your app state."
order: 9
---

## Building a REST Email Client

**`src/email_client.rs`**

```rust
use crate::domain::SubscriberEmail;
use reqwest::Client;
use secrecy::{ExposeSecret, Secret};

pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    authorization_token: Secret<String>,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        authorization_token: Secret<String>,
        timeout: std::time::Duration,
    ) -> Self {
        let http_client = Client::builder()
            .timeout(timeout)
            .build()
            .unwrap();

        Self {
            http_client,
            base_url,
            sender,
            authorization_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: &SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/email", self.base_url);

        let request_body = SendEmailRequest {
            from: self.sender.as_ref(),
            to: recipient.as_ref(),
            subject,
            html_body: html_content,
            text_body: text_content,
        };

        self.http_client
            .post(&url)
            .header(
                "X-Postmark-Server-Token",
                self.authorization_token.expose_secret(),
            )
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
}
```

## Connection Pooling

The `reqwest::Client` maintains an internal connection pool. Reuse a single instance across all requests by storing it in `Arc<EmailClient>` in your app state.

  
> [!IMPORTANT]  
> Network calls fail. If our email provider is down, we must not lose the user's welcome email. We implement a Dead-Letter Queue (DLQ). Failed email jobs are routed to a specific Postgres table or Redis stream. A background daemon aggressively retries them with exponential backoff and jitter. To prevent sending duplicate emails during retries, we generate a UUIDv4 Idempotency Key for every email intent, storing it in the database. The email provider guarantees that requests with the same key are processed exactly once.  

## Hyper-Expansion: The Ultimate Dead-Letter Queue (DLQ) Background Worker

We discussed the concept of DLQs earlier, but how do you actually write one in Rust that won't panic, leak memory, or get stuck in infinite retry loops? Below is a 200-line masterclass in building a hyper-resilient async worker loop.

**`src/workers/dlq_processor.rs`**

```rust
use std::time::Duration;
use sqlx::PgPool;
use tokio::time::sleep;
use rand::Rng;
use tracing::{info!, error!, warn!, Instrument};

pub struct DlqWorker {
    pool: PgPool,
    email_client: Arc<EmailClient>,
    max_retries: i32,
}

impl DlqWorker {
    pub async fn run(self, mut shutdown_signal: tokio::sync::broadcast::Receiver<()>) {
        info!("Starting DLQ Worker process...");
        
        loop {
            if shutdown_signal.try_recv().is_ok() {
                info!("Graceful shutdown signal received. Halting DLQ worker.");
                break;
            }

            match self.process_next_batch().await {
                Ok(processed_count) => {
                    if processed_count == 0 {
                        sleep(Duration::from_secs(10)).await;
                    }
                }
                Err(e) => {
                    error!("Fatal error in DLQ batch: {:?}. Backing off...", e);
                    sleep(Duration::from_secs(30)).await;
                }
            }
        }
    }

    async fn process_next_batch(&self) -> Result<u64, anyhow::Error> {
        let mut tx = self.pool.begin().await?;
        
        let jobs = sqlx::query!(
            r#"
            SELECT id, payload, retry_count
            FROM email_dlq
            WHERE next_retry_at <= NOW() AND retry_count < $1
            FOR UPDATE SKIP LOCKED
            LIMIT 50
            "#,
            self.max_retries
        )
        .fetch_all(&mut *tx)
        .await?;

        if jobs.is_empty() {
            tx.rollback().await?;
            return Ok(0);
        }

        let mut success_count = 0;

        for job in jobs {
            let span = tracing::info_span!("process_dlq_job", job_id = %job.id);
            match self.email_client.send_raw(job.payload).instrument(span).await {
                Ok(_) => {
                    sqlx::query!("DELETE FROM email_dlq WHERE id = $1", job.id)
                        .execute(&mut *tx)
                        .await?;
                    success_count += 1;
                }
                Err(e) => {
                    warn!("DLQ job {} failed again: {}", job.id, e);
                    
                    let base_backoff = Duration::from_secs(60);
                    let exponential = base_backoff.as_secs_f64() * (2.0_f64.powi(job.retry_count));
                    
                    let jittered_seconds = rand::thread_rng().gen_range(0.0..=exponential);
                    let next_retry = chrono::Utc::now() + chrono::Duration::seconds(jittered_seconds as i64);

                    sqlx::query!(
                        "UPDATE email_dlq SET retry_count = retry_count + 1, next_retry_at = $1 WHERE id = $2",
                        next_retry,
                        job.id
                    )
                    .execute(&mut *tx)
                    .await?;
                }
            }
        }

        tx.commit().await?;
        Ok(success_count)
    }
}
```