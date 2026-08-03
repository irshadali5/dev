---
title: "Required Subcommands &amp; Optional Subcommands"
description: ""
---




## Required Subcommands & Optional Subcommands

By default, subcommands are required. Make them optional with
`Option<Commands>`:



```rust

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

```
