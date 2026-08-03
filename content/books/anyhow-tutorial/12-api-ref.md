---
title: "API cheatsheet"
description: ""
---


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


<span class="code-lang">rust`
