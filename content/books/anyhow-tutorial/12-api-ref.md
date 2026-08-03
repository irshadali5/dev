---
title: "API cheatsheet"
description: ""
---

api-ref" class="section">

<div class="breadcrumb">

Reference › API cheatsheet



# API cheatsheet

## Macros

| Macro | What it does |
|----|----|
| anyhow!(msg) | Creates an anyhow::Error from a message or format string |
| bail!(msg) | Equivalent to return Err(anyhow!(msg)) |
| ensure!(cond, msg) | Returns Err(anyhow!(msg)) if cond is false |

## Error methods

| Method                  | What it does                                |
|-------------------------|---------------------------------------------|
| .context(msg)           | Wraps error with a static context message   |
| .with_context(\|\| ...) | Same, but lazy — closure only runs on error |
| .downcast::<T>()      | Consume error, returns Result<T, Error>   |
| .downcast_ref::<T>()  | Borrow error as &T if that's the inner type |
| .downcast_mut::<T>()  | Borrow error as &mut T                      |
| .is::<T>()            | Returns true if inner type is T             |
| .chain()                | Iterator over the error chain (all causes)  |
| .root_cause()           | The innermost error in the chain            |
| .backtrace()            | Optional backtrace (if captured)            |

## Imports quick-reference

<div class="code-header">

<span class="code-lang">rust`

copy



``` rust
// Covers 80% of use cases
use anyhow::Result;

// Add Context for .context() and .with_context()
use anyhow::{Context, Result};

// Full import
use anyhow::{Context, Result, Error, anyhow, bail, ensure};
```

## Format strings

| Format  | Output                                              |
|---------|-----------------------------------------------------|
| "{}"    | Just the top-level message                          |
| "{:#}"  | Full chain, colon-separated, single line            |
| "{:?}"  | Full chain, multi-line, with backtrace if available |
| "{:#?}" | Pretty-printed debug, good for logging              |



> [!TIP] > **Done!** > You now know everything you need to use
anyhow in real Rust projects. The most important things: always add
.context(), know when NOT to use it in libraries, and combine with
thiserror for domain errors.