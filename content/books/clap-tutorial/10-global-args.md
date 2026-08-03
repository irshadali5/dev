---
title: "Global Arguments"
description: ""
---




## Global Arguments

A **global argument** is available to all subcommands. Add
`#[arg(global = true)]` to the top-level struct.



```rust

#[derive(Parser)]
struct Cli {
    /// Available in ALL subcommands
    #[arg(global = true, short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

```


\$ ./myapp --verbose
list ← global flag before subcommand  
\$ ./myapp list
--verbose ← global flag after subcommand (both
work!) 
