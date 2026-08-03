---
title: "Setup"
description: ""
---

setup" class="section">

<div class="breadcrumb">

Getting started › Setup & Cargo.toml



# Setup

Add anyhow to your `Cargo.toml`. That's the entire setup.

<div class="code-header">

<span class="code-lang">toml`

copy



``` rust
# Cargo.toml
[dependencies]
anyhow = "1"
```

Then in your Rust file, bring the prelude into scope:

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
use anyhow::{Result, Context, anyhow, bail};
```



> [!TIP] > **Tip** > Most of the time, `use
anyhow::Result` is all you need. Add
`Context`, `bail!`, and
`anyhow!` as you need them.



## Feature flags

| Feature   | Description                                   | Default? |
|-----------|-----------------------------------------------|----------|
| std       | Enables std support (almost always want this) | ✅ yes   |
| backtrace | Capture backtraces automatically              | ❌ no    |

Enable backtraces in development by setting the env var
`RUST_BACKTRACE=1`. anyhow captures them
automatically when available — no code changes needed.

Next: Your first error →