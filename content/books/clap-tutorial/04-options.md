---
title: "Options &amp; Flags — Deep Dive"
description: ""
---




## Options & Flags — Deep Dive

### Counting flags (e.g. `-vvv`)

Use `action = ArgAction::Count` to count how many times a flag appears.



```rust

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

```


\$ ./myapp -vvv  
verbosity level: 3 

### Negatable flags (`--flag / --no-flag`)



```rust

#[derive(Parser)]
struct Cli {
    /// Enable colors (default: on). Use --no-color to disable.
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    color: bool,
}

```


\$ ./myapp
--no-color 
