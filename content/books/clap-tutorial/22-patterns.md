---
title: "Real-world Patterns"
description: ""
---



Reference

## Real-world Patterns

### Pattern 1: Git-style tool (global flags + subcommands)



```rust

#[derive(Parser)]
#[command(name = "mytool", version, propagate_version = true)]
struct Cli {
    #[arg(global = true, long, action = ArgAction::Count)]
    verbose: u8,

    #[arg(global = true, long, default_value = "config.toml")]
    config: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

```

### Pattern 2: Server with env + CLI precedence



```rust

#[derive(Parser)]
struct ServerCli {
    #[arg(long, env = "PORT", default_value_t = 8080)]
    port: u16,

    #[arg(long, env = "DATABASE_URL")]
    database_url: String,

    #[arg(long, env = "RUST_LOG", default_value = "info")]
    log_level: String,
}

```

### Pattern 3: Pipe-friendly CLI (stdin or file)



```rust

use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    /// Input file. Reads stdin if omitted.
    input: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let content = match cli.input {
        Some(path) => std::fs::read_to_string(path).unwrap(),
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).unwrap();
            buf
        }
    };
    // process content...
}

```



> [!TIP] > **You're ready

These patterns cover 95% of real CLIs. The official docs at
**docs.rs/clap** are excellent for edge cases — but you now have the
mental model to read them without confusion.

