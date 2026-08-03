---
title: "The ? operator"
description: ""
---


<div class="breadcrumb">

Core concepts › ? operator



# The ? operator

The `?` operator is how errors propagate in
Rust. When you put `?` after a
`Result`, it either unwraps the
`Ok` value or returns the
`Err` early from the current function.

## How it works with anyhow

The key insight: `?` automatically calls
`From::from()` on the error. anyhow::Error
implements `From<E>` for any E that implements
`std::error::Error`. So any standard error type
converts automatically.

<div class="comparison">

<div class="cmp-box bad">

<div class="cmp-label">

❌ without anyhow



```
fn read_number(path: &str)
    -> Result<u32, Box<dyn std::Error>>
{
    let s = fs::read_to_string(path)?;
    let n = s.trim().parse::<u32>()?;
    Ok(n)
}
```



<div class="cmp-box good">

<div class="cmp-label">

✅ with anyhow



```
use anyhow::Result;

fn read_number(path: &str) -> Result<u32> {
    let s = fs::read_to_string(path)?;
    let n = s.trim().parse::<u32>()?;
    Ok(n)
}
```





## Multiple error types — where anyhow shines


<span class="code-lang">rust`
