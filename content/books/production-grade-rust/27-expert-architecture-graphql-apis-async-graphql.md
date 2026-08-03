---
title: "Expert Architecture: GraphQL APIs (`async-graphql`)"
description: "REST and OpenAPI are great, but for highly relational data (like campaigns, issues, subscribers, and analytics), GraphQL prevents over-fetching. We use async-graphql to bind our Rust structs directly to a GraphQL schema."
order: 27
---

REST and OpenAPI are great, but for highly relational data (like campaigns, issues, subscribers, and analytics), GraphQL prevents over-fetching. We use `async-graphql` to bind our Rust structs directly to a GraphQL schema.

**`src/graphql/schema.rs`**

```rust
use async_graphql::{Object, Context, Schema, EmptyMutation, EmptySubscription};
use sqlx::PgPool;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn subscriber(&self, ctx: &Context<'_>, id: Uuid) -> async_graphql::Result<Subscriber> {
        let pool = ctx.data::<PgPool>()?;
        let sub = sqlx::query_as!(
            Subscriber,
            "SELECT id, email, name, status FROM subscribers WHERE id = $1",
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(sub)
    }
}

// DataLoader implementation omitted for brevity
```