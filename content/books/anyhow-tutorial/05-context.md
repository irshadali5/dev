---
title: "Adding context to errors"
description: ""
---

context" class="section">

<div class="breadcrumb">

Core concepts › Adding context



# Adding context to errors

This is one of anyhow's most powerful features. Instead of just seeing
*"No such file or directory"*, you can add layers of context so errors
read like a story.

## .context() — add a static string

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
use anyhow::{Context, Result};
use std::fs;

fn load_config(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .context("failed to read config file")
}
```

Now instead of just *"No such file or directory (os error 2)"*, the
error message becomes:



> [!CAUTION] > **Error output** > failed to read config file:
No such file or directory (os error 2)



## .with_context() — lazy context

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
fn load_config(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("failed to read config from '{}'", path))
}
```

## Layering context up the call stack

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
fn read_file(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("could not read '{}'", path))
}

fn parse_config(path: &str) -> Result<Config> {
    let raw = read_file(path)
        .context("failed to load configuration")?;
    // parse raw...
}

fn main() -> Result<()> {
    parse_config("app.toml")
        .context("application startup failed")?;
    Ok(())
}

// Error output:
// Error: application startup failed
//
// Caused by:
//     0: failed to load configuration
//     1: could not read 'app.toml'
//     2: No such file or directory (os error 2)
```



> [!TIP] > **Best practice** > Add context at each level
describing what the *function was trying to do*, not what went wrong.
The underlying error already says what went wrong. Your context says why
you were doing it.



Next: Downcasting errors →