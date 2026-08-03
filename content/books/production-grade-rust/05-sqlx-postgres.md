---
title: "SQLx & Postgres"
description: "For CI/CD builds without a live database, run cargo sqlx prepare -- --lib to generate .sqlx/ metadata. Set SQLX_OFFLINE=true during builds."
order: 5
---

## Why SQLx?

Feature

SQLx

Diesel

tokio-postgres

Compile-time verification

✅

✅

❌

Async native

✅

❌

✅

Raw SQL queries

✅

❌ (DSL)

✅

Connection pooling

✅

✅

✅

Migrations built-in

✅

✅

❌

## Database Migrations

**`scripts/init_db.sh`**

```bash
#!/usr/bin/env bash
set -eo pipefail

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

if [[ -z "${SKIP_DOCKER}" ]]; then
  docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres:16 \
    postgres -N 1000
fi

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running!"

export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"
sqlx database create
sqlx migrate run

>&2 echo "Database migrated successfully!"
```

**`migrations/001_create_subscriptions.sql`**

```sql
CREATE TABLE subscriptions (
    id            UUID        NOT NULL PRIMARY KEY,
    email         TEXT        NOT NULL UNIQUE,
    name          TEXT        NOT NULL,
    subscribed_at TIMESTAMPTZ NOT NULL,
    status        TEXT        NOT NULL DEFAULT 'pending_confirmation'
);

CREATE TABLE subscription_tokens (
    subscription_token TEXT NOT NULL PRIMARY KEY,
    subscriber_id      UUID NOT NULL REFERENCES subscriptions(id)
);

CREATE TABLE users (
    user_id       UUID PRIMARY KEY,
    username      TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL
);

CREATE TABLE newsletter_issues (
    newsletter_issue_id UUID NOT NULL PRIMARY KEY,
    title               TEXT NOT NULL,
    text_content        TEXT NOT NULL,
    html_content        TEXT NOT NULL,
    published_at        TIMESTAMPTZ NOT NULL
);

CREATE TABLE issue_delivery_queue (
    newsletter_issue_id UUID NOT NULL REFERENCES newsletter_issues(newsletter_issue_id),
    subscriber_email    TEXT NOT NULL,
    PRIMARY KEY (newsletter_issue_id, subscriber_email)
);
```

## Compile-Time Verified Queries

**`src/routes/subscriptions.rs`**

```rust
#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, transaction)
)]
pub async fn insert_subscriber(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    new_subscriber: &NewSubscriber,
) -> Result<uuid::Uuid, sqlx::Error> {
    let subscriber_id = uuid::Uuid::new_v4();

    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at, status)
           VALUES ($1, $2, $3, $4, 'pending_confirmation')"#,
        subscriber_id,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        chrono::Utc::now(),
    )
    .execute(&mut *transaction)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(subscriber_id)
}
```

  
> [!WARNING]  
> For CI/CD builds without a live database, run cargo sqlx prepare -- --lib to generate .sqlx/ metadata. Set SQLX_OFFLINE=true during builds.  

## SQL Transactions

Group related operations atomically:

**`src/routes/subscriptions.rs`**

```rust
pub async fn subscribe(
    Form(form): Form<SubscribeForm>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let new_subscriber = match NewSubscriber::try_from(form) {
        Ok(s) => s,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let mut transaction = match state.db.begin().await {
        Ok(t) => t,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let subscriber_id = match insert_subscriber(&mut transaction, &new_subscriber).await {
        Ok(id) => id,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let token = generate_subscription_token();
    if store_token(&mut transaction, subscriber_id, &token).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if transaction.commit().await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    // Send confirmation email asynchronously...
    send_confirmation_email(&state.email_client, &new_subscriber, &state.base_url, &token).await;

    StatusCode::OK.into_response()
}
```

  
> [!IMPORTANT]  
> Standard SQLx queries buffer all rows in memory. For massive data exports (e.g., millions of subscribers), this causes OOM (Out of Memory) kills. We use sqlx::query(...).fetch(&pool) to return a TryStream, streaming rows lazily to the HTTP response. Furthermore, we utilize the pgvector Postgres extension. By storing embeddings of our newsletter content as vectors, we enable extremely fast nearest-neighbor searches (Cosine Similarity) directly in SQL, laying the groundwork for AI-driven Retrieval-Augmented Generation (RAG) natively within our relational database.  

## Hyper-Expansion: Hyperscale Postgres Partitioning

When our newsletter scales to tracking billions of "email opened" analytics events, a single monolithic Postgres table will suffer catastrophic performance degradation during inserts and vacuums. The solution is **Declarative Table Partitioning**.

We partition the `email_events` table by **range** (specifically, by month). This ensures that active inserts only hit the "hot" partition, keeping indexes small enough to fit entirely in RAM (L1/L2 cache), while older months become immutable "cold" storage.

**`migrations/05_partition_events.sql`**

```sql
-- 1. Create the parent partitioned table
CREATE TABLE email_events (
    event_id UUID NOT NULL,
    subscriber_id UUID NOT NULL,
    campaign_id UUID NOT NULL,
    event_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (event_id, created_at) -- Partition key MUST be in the primary key
) PARTITION BY RANGE (created_at);

-- 2. Create the partitions for the next 3 months
CREATE TABLE email_events_2026_06 PARTITION OF email_events 
    FOR VALUES FROM ('2026-06-01') TO ('2026-07-01');

CREATE TABLE email_events_2026_07 PARTITION OF email_events 
    FOR VALUES FROM ('2026-07-01') TO ('2026-08-01');

CREATE TABLE email_events_2026_08 PARTITION OF email_events 
    FOR VALUES FROM ('2026-08-01') TO ('2026-09-01');

-- 3. Create a default partition for bounds we haven't covered yet
CREATE TABLE email_events_default PARTITION OF email_events DEFAULT;
```

### Architectural Data Flow

```mermaid
flowchart TD A[Axum API] -->|SQLx Insert| B(email_events PARENT) B -->|Date: 2026-06-15| C[(email_events_2026_06)] B -->|Date: 2026-07-02| D[(email_events_2026_07)] B -->|Date: 2026-12-01| E[(email_events_default)] C -.-> F(Fast RAM Cache) E -.-> G(Disk I/O Bound)
```