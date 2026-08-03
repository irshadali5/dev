---
title: "Your first error"
description: ""
---

first-error" class="section">

<div class="breadcrumb">

Getting started › Your first error



# Your first error

There are three ways to create an anyhow error. Let's see each one.

## 1. anyhow! macro — create an error from a message

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
use anyhow::{Result, anyhow};

fn parse_age(s: &str) -> Result<u32> {
    let n: u32 = s.parse()
        .map_err(|_| anyhow!("'{}' is not a valid age", s))?;
    if n > 150 {
        return Err(anyhow!("age {} is unreasonably large", n));
    }
    Ok(n)
}
```

## 2. bail! macro — early return with an error

`bail!` is shorthand for
`return Err(anyhow!(...))`. Use it when you want
to exit a function early with an error message.

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
use anyhow::{Result, bail};

fn check_username(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("username cannot be empty");
    }
    if name.len() > 32 {
        bail!("username '{}' exceeds 32 chars", name);
    }
    Ok(())
}
```

## 3. ensure! macro — assert a condition or error

Like `assert!` but returns an error instead of
panicking. Perfect for validation.

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
use anyhow::{Result, ensure};

fn divide(a: f64, b: f64) -> Result<f64> {
    ensure!(b != 0.0, "cannot divide by zero");
    Ok(a / b)
}
```



> [!TIP] > **Quick rule** > Use `bail!`
for conditional early returns. Use `ensure!`
when you're checking a boolean condition. Use
`anyhow!` when building the error value inline.



Next: The Result type →