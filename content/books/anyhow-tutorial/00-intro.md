---
title: "anyhow — Rust error handling made easy"
description: ""
---


<div class="hero-bar">

# anyhow — Rust error handling made easy

<div class="subtitle">

A complete guide from zero to production-ready error handling





If you've hit Rust's error handling and felt confused by the jungle of
`Box<dyn Error>`, `impl
Error`, and custom enums — **anyhow is the crate that
clears all of that up for application code.



> [!NOTE] > **What is anyhow? A crate by David Tolnay that
gives you a single `anyhow::Error` type that can
hold *any* error, plus helpers to add human-readable context as errors
bubble up. Zero boilerplate.



## Why does Rust error handling feel hard?

In Rust, every function that can fail returns
`Result<T, E>`. The problem in real
applications is that you call many functions which return *different* E
types — `io::Error`,
`serde_json::Error`,
`reqwest::Error`... and they don't automatically
compose.

Without a crate like anyhow, you'd need to manually convert each error
type, write From implementations, or use `Box<dyn
std::error::Error>` everywhere. anyhow wraps all of that
automatically.

## What you'll learn

<div class="toc-mini">

In this guide

<span onclick="nav('setup',null)">→ Setup: adding anyhow to your
project` <span onclick="nav('first-error',null)">→ Creating and
returning your first error`
<span onclick="nav('result-type',null)">→ anyhow::Result vs
std::result::Result` <span onclick="nav('question-mark',null)">→
The ? operator and automatic conversion`
<span onclick="nav('context',null)">→ Adding context to errors with
.context()` <span onclick="nav('downcasting',null)">→ Recovering
specific errors via downcasting`
<span onclick="nav('thiserror',null)">→ Pairing with thiserror for
custom error types` <span onclick="nav('libs-vs-bins',null)">→
When NOT to use anyhow (library crates)`
<span onclick="nav('patterns',null)">→ Production patterns and
real-world usage`



Start: Setup & Cargo.toml →
