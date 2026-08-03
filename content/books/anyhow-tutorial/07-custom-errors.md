---
title: "Custom error types with anyhow"
description: ""
---

custom-errors" class="section">

<div class="breadcrumb">

Intermediate › Custom error types



# Custom error types with anyhow

Sometimes you need named, structured error variants. You can define your
own error types and use them alongside anyhow.

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
use std::fmt;
use anyhow::{Result, Context};

#[derive(Debug)]
pub enum AppError {
    ConfigMissing,
    InvalidPort(u16),
    DbConnectionFailed(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ConfigMissing => write!(f, "config file not found"),
            AppError::InvalidPort(p) => write!(f, "port {} is not valid", p),
            AppError::DbConnectionFailed(s) => write!(f, "db error: {}", s),
        }
    }
}

impl std::error::Error for AppError {}

// Use it with anyhow — ? converts it automatically
fn connect(port: u16) -> Result<()> {
    if port < 1024 {
        return Err(AppError::InvalidPort(port).into());
    }
    Ok(())
}
```



> [!NOTE] > **Better approach: use thiserror** > Writing
Display + Error manually is tedious. The
`thiserror` crate by the same author
auto-derives them. See the next section.



Next: Error chaining →