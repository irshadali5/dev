---
title: "anyhow::Result&lt;T&gt;"
description: ""
---

result-type" class="section">

<div class="breadcrumb">

Core concepts › The Result type



# anyhow::Result<T>

`anyhow::Result<T>` is just a type alias. It
fixes the error type to `anyhow::Error` so you
don't have to write it every time.

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
// What anyhow::Result is defined as internally:
pub type Result<T, E = Error> = std::result::Result<T, E>;

// So these are identical:
fn foo() -> Result<String>                         // anyhow::Result
fn foo() -> std::result::Result<String, anyhow::Error>
```

## anyhow::Error vs std

| anyhow::Error                              | Box<dyn std::error::Error> |
|--------------------------------------------|------------------------------|
| Ergonomic API with .context(), .downcast() | Manual conversions needed    |
| Backtrace support built in                 | No built-in backtrace        |
| 1 pointer wide (optimized)                 | 2 pointers (fat pointer)     |
| Only usable in applications                | Can be used in libraries     |



> [!WARNING] > **Important** >
`anyhow::Error` does NOT implement
`std::error::Error` itself — you can't return it
from a library. It's designed for application-level error reporting, not
library APIs.



## Using it in main

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
use anyhow::Result;

fn main() -> Result<()> {
    let config = load_config("config.toml")?;
    run(config)?;
    Ok(())
}
```

Next: ? operator →