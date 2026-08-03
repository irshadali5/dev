---
title: "Default Values"
description: ""
---




## Default Values



```rust

#[derive(Parser)]
struct Cli {
    /// String default
    #[arg(long, default_value = "world")]
    name: String,

    /// Numeric default — note _t suffix means "typed"
    #[arg(long, default_value_t = 8080)]
    port: u16,

    /// Boolean default
    #[arg(long, default_value_t = true)]
    color: bool,

    /// Computed default using a function
    #[arg(long, default_value_t = default_threads())]
    threads: usize,
}

fn default_threads() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

```
