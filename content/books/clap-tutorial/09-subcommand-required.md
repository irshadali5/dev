---
title: "Required Subcommands &amp; Optional Subcommands"
description: ""
---

subcommand-required">

<div class="section-eyebrow">

Chapter 10** >

## Required Subcommands & Optional Subcommands

By default, subcommands are required. Make them optional with
`Option<Commands>`:

<div class="code-block">

<div class="code-header">

<span class="code-lang">rust</span>

copy

** >

    #[derive(Parser)]
    struct Cli {
        /// Optional: running with no subcommand shows status
        #[command(subcommand)]
        command: Option<Commands>,
    }

    fn main() {
        let cli = Cli::parse();
        match cli.command {
            Some(cmd) => run_command(cmd),
            None      => show_status(),
        }
    }

** >