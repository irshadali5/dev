---
title: "anyhow + thiserror — the classic combo"
description: ""
---


<div class="breadcrumb">

Advanced › anyhow + thiserror



# anyhow + thiserror — the classic combo

This is the recommended pattern for serious Rust projects:
`thiserror` for defining your domain error
types, `anyhow` for propagating them in
application code.

## Cargo.toml


<span class="code-lang">toml`
