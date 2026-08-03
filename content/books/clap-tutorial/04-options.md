---
title: "Options &amp; Flags — Deep Dive"
description: ""
---

options">

<div class="section-eyebrow">

Chapter 5** >

## Options & Flags — Deep Dive

### Counting flags (e.g. `-vvv`)

Use `action = ArgAction::Count` to count how many times a flag appears.

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

    use clap::{Parser, ArgAction};

    #[derive(Parser)]
    struct Cli {
        /// Verbosity (-v = info, -vv = debug, -vvv = trace)
        #[arg(short, long, action = ArgAction::Count)]
        verbose: u8,
    }

    fn main() {
        let args = Cli::parse();
        println!("verbosity level: {}", args.verbose);
    }

** >

<div class="term">

<span class="prompt">\$</span> <span class="cmd">./myapp -vvv</span>  
<span class="out">verbosity level: 3</span> ** >

### Negatable flags (`--flag / --no-flag`)

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

    #[derive(Parser)]
    struct Cli {
        /// Enable colors (default: on). Use --no-color to disable.
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        color: bool,
    }

** >

<div class="term">

<span class="prompt">\$</span> <span class="cmd">./myapp
--no-color</span> ** >