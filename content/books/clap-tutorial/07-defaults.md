---
title: "Default Values"
description: ""
---

defaults">

<div class="section-eyebrow">

Chapter 8** >

## Default Values

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

    #[derive(Parser)]
    struct Cli {
        /// String default
        #[arg(long, default_value = "world")]
        name: String,

        /// Numeric default — note _t suffix means "typed"
        #[arg(long, default_value_t = 8080)]
        port: u16,

        /// Boolean default
        #[arg(long, default_value_t = true)]
        color: bool,

        /// Computed default using a function
        #[arg(long, default_value_t = default_threads())]
        threads: usize,
    }

    fn default_threads() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    }

** >