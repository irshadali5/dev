---
title: "Built-in Validation"
description: ""
---




## Built-in Validation

Use `#[arg(value_parser = ...)]` to constrain accepted values.



```rust

#[derive(Parser)]
struct Cli {
    /// Port must be 1024–65535
    #[arg(long, value_parser = clap::value_parser!(u16).range(1024..=65535))]
    port: u16,

    /// Number of workers: 1–16
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=16))]
    workers: u8,
}

```


\$ ./server --port
80  
<span class="err">error: invalid value '80' for '--port <PORT>': 80 is
not in 1024..=65535 
