---
title: "Real-world patterns"
description: ""
---

patterns" class="section">

<div class="breadcrumb">

Advanced › Real-world patterns



# Real-world patterns

## Pattern 1: CLI with anyhow in main

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
use anyhow::{Context, Result};
use std::{env, fs};

fn main() -> Result<()> {
    let path = env::args().nth(1)
        .context("usage: mytool <path>")?;

    let content = fs::read_to_string(&path)
        .with_context(|| format!("could not read '{}'", path))?;

    println!("Lines: {}", content.lines().count());
    Ok(())
}
```

## Pattern 2: Axum handler error wrapper

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR,
         format!("Error: {}", self.0)).into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(e: E) -> Self { AppError(e.into()) }
}

async fn get_user(Path(id): Path<u64>) -> Result<Json<User>, AppError> {
    let user = fetch_user(id).await
        .context("failed to fetch user")?;
    Ok(Json(user))
}
```

## Pattern 3: Error logging

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
fn run_task() {
    if let Err(e) = do_work() {
        tracing::error!(error = format!("{:?}", e), "task failed");
    }
}
```



> [!TIP] > **Formatting summary** >
`"{}"` → top-level message only  · 
`"{:#}"` → full chain, single line  · 
`"{:?}"` → full chain + backtrace



Next: API cheatsheet →