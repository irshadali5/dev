---
title: "Config Files (via clap + figment / config)"
description: ""
---




## Config Files (via clap + figment / config)

clap alone doesn't read config files — but it pairs well with `figment`
or `config-rs`. The pattern: parse the config path with clap, then load
it yourself.



```rust

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    /// Path to config file
    #[arg(long, default_value = "config.toml")]
    config: PathBuf,

    /// Override port from config
    #[arg(long)]
    port: Option<u16>,
}

fn main() {
    let cli = Cli::parse();
    let file_config = load_toml(&cli.config);

    // CLI takes precedence over file config
    let port = cli.port.unwrap_or(file_config.port);
    println!("Using port: {port}");
}

```



> [!NOTE] > **Typical precedence chain

CLI flags → environment variables → config file → hardcoded defaults.
This is the standard Unix convention and what users expect.

