---
title: "Downcasting errors"
description: ""
---

downcasting" class="section">

<div class="breadcrumb">

Intermediate › Downcasting errors



# Downcasting errors

Sometimes you want to *recover* from a specific error type. anyhow lets
you "open the box" and inspect what's inside.

## downcast_ref — inspect without consuming

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
use anyhow::Result;
use std::{fs, io};

fn read_or_default(path: &str) -> Result<String> {
    match fs::read_to_string(path) {
        Ok(s) => Ok(s),
        Err(e) => {
            if let Some(io_err) = e.downcast_ref::<io::Error>() {
                if io_err.kind() == io::ErrorKind::NotFound {
                    return Ok(String::new());
                }
            }
            Err(e.into())
        }
    }
}
```

## downcast — consume and get the original type

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
fn handle_error(e: anyhow::Error) {
    match e.downcast::<io::Error>() {
        Ok(io_err) => eprintln!("IO problem: {}", io_err),
        Err(other) => eprintln!("Other error: {}", other),
    }
}
```

## is — quick type check

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
if e.is::<io::Error>() {
    println!("it's an IO error");
}
```



> [!WARNING] > **Context and downcasting** > When you call
`.context()`, the original error is wrapped. You
can still downcast to the original type — anyhow searches the full error
chain.



Next: Custom error types →