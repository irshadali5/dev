---
title: "Installation"
description: ""
---




## Installation

Add clap to your `Cargo.toml`. Always enable the `derive` feature — it's
how you'll write clap in 2024.



```toml

[dependencies]
clap = { version = "4", features = ["derive"] }

```



> [!TIP] > **Feature flags

You can also add `"env"` (for reading env vars), `"wrap_help"`
(word-wraps help text), and `"unicode"`. We'll cover these when we need
them.


Verify everything compiles:


\$ cargo build  
 Compiling clap v4.x.x  
 Compiling your-cli v0.1.0  
 Finished dev [unoptimized + debuginfo] target(s) in
3.2s 
