---
title: "Global Arguments"
description: ""
---

global-args">

<div class="section-eyebrow">

Chapter 11** >

## Global Arguments

A **global argument** is available to all subcommands. Add
`#[arg(global = true)]` to the top-level struct.

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

    #[derive(Parser)]
    struct Cli {
        /// Available in ALL subcommands
        #[arg(global = true, short, long)]
        verbose: bool,

        #[command(subcommand)]
        command: Commands,
    }

** >

<div class="term">

<span class="prompt">\$</span> <span class="cmd">./myapp --verbose
list</span><span class="out"> ← global flag before subcommand</span>  
<span class="prompt">\$</span> <span class="cmd">./myapp list
--verbose</span><span class="out"> ← global flag after subcommand (both
work!)</span> ** >