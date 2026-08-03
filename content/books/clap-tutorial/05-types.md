---
title: "Parsing into Rust Types"
description: ""
---




## Parsing into Rust Types

clap uses the `ValueParser` trait under the hood. For most standard
types, it Just Works™ because they implement `FromStr`.

| Rust type | CLI input | Notes |
|----|----|----|
| `String` | `--name foo` | Any text |
| `u8, u32, i64, f64` | `--port 8080` | Auto-parsed, error if invalid |
| `bool` | flag present/absent | See flags section |
| `PathBuf` | `--input ./foo.txt` | OS path |
| `Option<T>` | omit the flag entirely | None if not provided |
| `Vec<T>` | `--tag a --tag b` | Multiple values |
| Custom enum (ValueEnum) | `--format json` | Covered in ValueEnum section |



```rust

use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    /// Port to listen on
    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    /// Bind address
    #[arg(long, default_value = "127.0.0.1")]
    host: IpAddr,

    /// Config file path
    #[arg(short, long)]
    config: Option<PathBuf>,
}

```



> [!WARNING] > **default_value_t vs default_value

Use `default_value_t = 3000` when your type is not a string (u16, bool,
etc.). Use `default_value = "text"` for strings. Mixing them causes
compile errors.

