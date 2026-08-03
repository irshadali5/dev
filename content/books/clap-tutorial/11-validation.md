---
title: "Built-in Validation"
description: ""
---

validation">

<div class="section-eyebrow">

Chapter 12** >

## Built-in Validation

Use `#[arg(value_parser = ...)]` to constrain accepted values.

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

    #[derive(Parser)]
    struct Cli {
        /// Port must be 1024–65535
        #[arg(long, value_parser = clap::value_parser!(u16).range(1024..=65535))]
        port: u16,

        /// Number of workers: 1–16
        #[arg(long, value_parser = clap::value_parser!(u8).range(1..=16))]
        workers: u8,
    }

** >

<div class="term">

<span class="prompt">\$</span> <span class="cmd">./server --port
80</span>  
<span class="err">error: invalid value '80' for '--port <PORT>': 80 is
not in 1024..=65535</span> ** >