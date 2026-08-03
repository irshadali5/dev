---
title: "Custom Help &amp; Version"
description: ""
---

help">

<div class="section-eyebrow">

Chapter 20** >

## Custom Help & Version

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

    use clap::Parser;

    #[derive(Parser)]
    #[command(
        name = "myapp",
        version = "1.2.0",             // or env!("CARGO_PKG_VERSION")
        author = "Irshad <me@example.com>",
        about = "Does awesome things",
        long_about = "A longer description shown with --help but not -h",
        after_help = "EXAMPLES:\n  myapp --port 3000\n  myapp list --verbose",
    )]
    struct Cli {
        #[arg(short, long)]
        port: Option<u16>,
    }

** >

### Use `env!("CARGO_PKG_VERSION")` for auto version

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

    #[command(version = env!("CARGO_PKG_VERSION"))]

** >

This reads the version from `Cargo.toml` at compile time. You never need
to update it manually.

### Hiding arguments from help

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

    #[arg(long, hide = true)]   // hidden from --help output
    debug_mode: bool,

** >