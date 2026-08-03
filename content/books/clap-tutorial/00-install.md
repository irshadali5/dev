---
title: "Installation"
description: ""
---

install">

<div class="section-eyebrow">

Chapter 1** >

## Installation

Add clap to your `Cargo.toml`. Always enable the `derive` feature — it's
how you'll write clap in 2024.

<div class="code-block">

<div class="code-header">

<span class="code-lang">toml</span>
<span class="code-filename">Cargo.toml</span>

copy

** >

    [dependencies]
    clap = { version = "4", features = ["derive"] }

** >



> [!TIP] > **Feature flags** >

You can also add `"env"` (for reading env vars), `"wrap_help"`
(word-wraps help text), and `"unicode"`. We'll cover these when we need
them.

** >

Verify everything compiles:

<div class="term">

<span class="prompt">\$</span> <span class="cmd">cargo build</span>  
<span class="out"> Compiling clap v4.x.x</span>  
<span class="out"> Compiling your-cli v0.1.0</span>  
<span class="out"> Finished dev [unoptimized + debuginfo] target(s) in
3.2s</span> ** >