---
title: "Error chaining"
description: ""
---

chaining" class="section">

<div class="breadcrumb">

Intermediate › Error chaining



# Error chaining

## Iterating the error chain

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
fn print_chain(e: &anyhow::Error) {
    eprintln!("Error: {}", e);
    for cause in e.chain().skip(1) {
        eprintln!("Caused by: {}", cause);
    }
}
```

## The {:?} debug format

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
if let Err(e) = run() {
    eprintln!("{:?}", e);
    // Error: application startup failed
    //
    // Caused by:
    //     0: failed to read config
    //     1: No such file or directory (os error 2)
    //
    // Stack backtrace (if RUST_BACKTRACE=1):
    //    0: ...
}
```



> [!TIP] > **Tip** > In production CLIs, use
`{:#}` for a single-line chain, or
`{:?}` for the full multi-line debug view with
backtrace.



Next: anyhow + thiserror →