---
title: "Required &amp; Optional Arguments"
description: ""
---




## Required & Optional Arguments

The type of your field determines whether the argument is required:

| Field type            | Behavior                           |
|-----------------------|------------------------------------|
| `String`, `u32`, etc. | Required — clap errors if missing  |
| `Option<T>`           | Optional — `None` if not provided  |
| `bool` (flag)         | Optional — `false` if not provided |
| Has `default_value`   | Optional — uses default            |



```rust

#[derive(Parser)]
struct Cli {
    /// Required: must be provided
    name: String,

    /// Optional: fine to omit
    #[arg(short, long)]
    nickname: Option<String>,

    /// Optional with default
    #[arg(short, long, default_value_t = 42)]
    count: u32,
}

```



> [!TIP] > **Making an option required

You can force a named option to be required even if you'd normally use
`Option<T>`: add `#[arg(required = true)]`. But usually just use `T`
(non-option) instead.

