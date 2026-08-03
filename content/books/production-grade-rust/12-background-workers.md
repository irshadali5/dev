---
title: "Background Workers"
description: "Instead of blocking the HTTP request while sending hundreds of emails, we enqueue delivery tasks and process them in a background worker."
order: 12
---

## Async Newsletter Delivery

Instead of blocking the HTTP request while sending hundreds of emails, we enqueue delivery tasks and process them in a background worker.

**`src/routes/admin/newsletters.rs`**

```rust
pub async fn publish_newsletter(
    Form(form): Form<PublishForm>,
    State(state): State<AppState>,
    user_id: ReqData<UserId>,
) -> impl IntoResponse {
    let mut transaction = state.db.begin().await?;

    // 1. Save newsletter content
    let issue_id = insert_newsletter_issue(
        &mut transaction,
        &form.title,
        &form.text_content,
        &form.html_content,
    ).await?;

    // 2. Enqueue delivery tasks for all confirmed subscribers
    enqueue_delivery_tasks(&mut transaction, issue_id).await?;

    transaction.commit().await?;

    // 3. Return immediately — emails will be sent by background workers
    FlashMessage::info("Newsletter accepted — emails will go out shortly.").send();
    Redirect::to("/admin/newsletters")
}
```

## Worker Loop

**`src/worker.rs`**

```rust
use sqlx::PgPool;
use std::time::Duration;

pub async fn worker_loop(pool: PgPool, email_client: EmailClient) -> Result<(), anyhow::Error> {
    loop {
        match try_execute_task(&pool, &email_client).await {
            Ok(ExecutionOutcome::EmptyQueue) => {
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
            Ok(ExecutionOutcome::TaskCompleted) => {}
            Err(e) => {
                tracing::error!("Worker error: {:?}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

async fn try_execute_task(
    pool: &PgPool,
    email_client: &EmailClient,
) -> Result<ExecutionOutcome, anyhow::Error> {
    let mut transaction = pool.begin().await?;

    // Atomically dequeue one task using SKIP LOCKED
    let task = sqlx::query!(
        r#"SELECT newsletter_issue_id, subscriber_email
           FROM issue_delivery_queue
           FOR UPDATE SKIP LOCKED
           LIMIT 1"#
    )
    .fetch_optional(&mut *transaction)
    .await?;

    let Some(task) = task else {
        return Ok(ExecutionOutcome::EmptyQueue);
    };

    // Fetch issue content and send email...
    let issue = get_issue(pool, task.newsletter_issue_id).await?;
    email_client.send_email(...).await;

    // Remove from queue
    sqlx::query!(
        "DELETE FROM issue_delivery_queue WHERE newsletter_issue_id = $1 AND subscriber_email = $2",
        task.newsletter_issue_id,
        task.subscriber_email,
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(ExecutionOutcome::TaskCompleted)
}

pub enum ExecutionOutcome {
    TaskCompleted,
    EmptyQueue,
}
```

## Running API + Worker Concurrently

**`src/main.rs`**

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Failed to read config");

    let application = Application::build(configuration.clone()).await?;
    let app_task = tokio::spawn(application.run_until_stopped());
    let worker_task = tokio::spawn(run_worker_until_stopped(configuration));

    tokio::select! {
        o = app_task => report_exit("API", o),
        o = worker_task => report_exit("Background worker", o),
    };

    Ok(())
}
```

  
> [!TIP]  
> FOR UPDATE SKIP LOCKED makes Postgres act as a concurrency-safe queue. Multiple workers can poll simultaneously without duplicates — each grabs an uncontested row.  

  
> [!IMPORTANT]  
> Simple \`tokio::spawn\` background tasks are lost if the pod crashes. We migrate to Redis Streams for durable background jobs. We use Redis Consumer Groups. When a worker pulls a job, it remains in the "Pending" state. If the worker crashes before ACKing the job, a watchdog re-assigns the stalled job to another worker. Combined with our Idempotency Keys, we upgrade from At-Least-Once to true Exactly-Once Delivery semantics across our distributed worker fleet.