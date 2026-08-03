---
title: "anyhow + thiserror — the classic combo"
description: ""
---

thiserror" class="section">

<div class="breadcrumb">

Advanced › anyhow + thiserror



# anyhow + thiserror — the classic combo

This is the recommended pattern for serious Rust projects:
`thiserror` for defining your domain error
types, `anyhow` for propagating them in
application code.

## Cargo.toml

<div class="code-header">

<span class="code-lang">toml`

copy



``` rust
[dependencies]
anyhow = "1"
thiserror = "1"
```

## Define domain errors with thiserror

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("connection refused to {host}:{port}")]
    ConnectionRefused { host: String, port: u16 },

    #[error("query failed: {0}")]
    QueryFailed(String),

    #[error("row not found with id {0}")]
    NotFound(u64),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

## Use anyhow in the application layer

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
use anyhow::{Context, Result};

fn get_user_name(db: &Db, id: u64) -> Result<String> {
    let row = db.find_user(id)
        .with_context(|| format!("user {} not found", id))?;
    Ok(row.name)
}

// Still downcast to DbError if needed:
if let Some(db_err) = err.downcast_ref::<DbError>() {
    if matches!(db_err, DbError::NotFound(_)) {
        // Handle 404 case
    }
}
```



> [!NOTE] > **Rule of thumb** > Use

thiserror** > in library crates and domain modules — structured and
matchable. Use

anyhow** > in application code (main, CLI, server handlers) —
convenience over structure.



Next: Libs vs binaries →