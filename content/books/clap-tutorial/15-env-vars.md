---
title: "Environment Variables"
description: ""
---

env-vars">

<div class="section-eyebrow">

Chapter 16** >

## Environment Variables

Add `features = ["env"]` to your Cargo.toml, then use
`#[arg(env = "VAR_NAME")]`. CLI flags take precedence over env vars.

<div class="code-block">

<div class="code-header">

<span class="code-lang">toml</span>
<span class="code-filename">Cargo.toml</span>

copy

** >

    clap = { version = "4", features = ["derive", "env"] }

** >

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

    #[derive(Parser)]
    struct Cli {
        /// Database URL — reads $DATABASE_URL if not set on CLI
        #[arg(long, env = "DATABASE_URL")]
        database_url: String,

        /// API key — hides value in help output
        #[arg(long, env = "API_KEY", hide_env_values = true)]
        api_key: String,
    }

** >

<div class="term">

<span class="prompt">\$</span>
<span class="cmd">DATABASE_URL=postgres://localhost/db ./myapp</span>  
<span class="cm">\# No --database-url needed — reads from env</span>  
  
<span class="prompt">\$</span> <span class="cmd">./myapp --database-url
postgres://prod/db</span>  
<span class="cm">\# CLI flag overrides env var</span> ** >

Help output shows where a value came from:

<div class="term">

<span class="out"> --database-url <DATABASE_URL> [env:
DATABASE_URL=postgres://localhost/db]</span> ** >