---
title: "Introduction"
description: "This book teaches you how to build production-grade backend services in Rust. Not toy examples. Not \\"Hello World.\\" Real systems with databases, authentication, background workers, telemetry, and deployment pipelines."
order: 0
---

This book teaches you how to build **production-grade backend services** in Rust. Not toy examples. Not "Hello World." Real systems with databases, authentication, background workers, telemetry, and deployment pipelines.

## What We're Building

An **email newsletter API** — the same project structure used by companies like `Discord`, `Cloudflare`, and `1Password` who run Rust in production. We'll build:

*   A REST API for subscriber management
*   Double opt-in confirmation emails
*   An admin dashboard with session-based auth
*   Background workers for newsletter delivery
*   Full observability with structured logging
*   Docker containerization and CI/CD

## The 2026 Stack

Every crate in this book is the **best-in-class** choice as of June 2026:

axum0.8.3 tokio1.45 sqlx0.8.6 tower0.5.2 tracing0.1.41 serde1.0.219 thiserror2.0.12 anyhow1.0.97 garde0.22 reqwest0.12.15 lettre0.11.14 tower-sessions0.14

  
> [!TIP]  
> Axum is built on tower — the same middleware ecosystem used by hyper and tonic. It offers superior composability, type-safe extractors, and first-class async/await. In 2026, it's the de facto standard for production Rust APIs.  

  
> [!IMPORTANT]  
> In production Rust, we don't just write code; we architect for the CPU. We will leverage zero-cost abstractions where our high-level ergonomic code compiles down to the exact same machine instructions as hand-written, unsafe C. We emphasize mechanical sympathy: structuring our memory layouts to maximize CPU cache line hits (L1/L2) and minimizing pointer chasing.