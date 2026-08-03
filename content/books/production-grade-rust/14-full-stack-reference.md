---
title: "Full Stack Reference"
description: "The complete 2026 production stack, every crate version, every choice justified:"
order: 14
---

The complete 2026 production stack, every crate version, every choice justified:

Category

Crate

Version

Why

Runtime

`tokio`

1.45

De facto async runtime, full ecosystem support

Web

`axum`

0.8

Tower-based, type-safe extractors, best middleware ecosystem

Middleware

`tower-http`

0.6

Production-grade CORS, tracing, compression, timeouts

Database

`sqlx`

0.8

Compile-time verified SQL, async native, connection pooling

Serialization

`serde`

1.0

Zero-cost abstraction, format-agnostic

Logging

`tracing`

0.1

Structured, correlated, span-based telemetry

Errors (typed)

`thiserror`

2.0

Derive macro for domain error enums

Errors (opaque)

`anyhow`

1.0

Ergonomic context, backtraces, application-level errors

Validation

`garde`

0.22

Compile-time rules, async validation, zero overhead

HTTP Client

`reqwest`

0.12

Async, connection pooling, rustls native

Sessions

`tower-sessions`

0.14

Tower-native, pluggable stores

Passwords

`argon2`

0.5

OWASP-recommended, PHC string format

Email

`lettre`

0.11

SMTP + builder API, async, rustls

Secrets

`secrecy`

0.10

Zeroize on drop, Debug redaction

UUID

`uuid`

1.16

v4 random generation, serde support

Time

`chrono`

0.4

Timezone-aware, SQL mapping

Config

`config`

0.14

Hierarchical, multi-source, env override

Testing

`wiremock`

0.6

HTTP mocking, request recording

Test data

`fake`

3.0

Realistic random data generation

Property tests

`quickcheck`

1.0

Auto-shrinking, arbitrary generators

Test runner

`cargo-nextest`

latest

Parallel test execution, 3x faster

## Architecture Summary

**`System Architecture`**

```text
┌─────────────────────────────────────────────────────┐
│                    Load Balancer                    │
└─────────────────────────┬───────────────────────────┘
                          │
         ┌────────────────┼────────────────┐
         │                │                │
    ┌────▼────┐     ┌────▼────┐     ┌────▼────┐
    │ Axum    │     │ Axum    │     │ Axum    │
    │ Instance│     │ Instance│     │ Instance│
    │  + BG   │     │  + BG   │     │  + BG   │
    │ Worker  │     │ Worker  │     │ Worker  │
    └────┬────┘     └────┬────┘     └────┬────┘
         │                │                │
         └────────────────┼────────────────┘
                          │
              ┌───────────┼───────────┐
              │           │           │
        ┌─────▼─────┐ ┌──▼──┐ ┌─────▼─────┐
        │ Postgres  │ │Redis│ │ Postmark  │
        │ (data +   │ │     │ │ (emails)  │
        │  queue +  │ │     │ │           │
        │  sessions)│ │     │ │           │
        └───────────┘ └─────┘ └───────────┘
```

  
> [!IMPORTANT]  
> You've built a production-grade Rust backend from scratch. Email newsletter API with double opt-in, admin dashboard, session auth, background workers, structured logging, comprehensive testing, and containerized deployment. This is real production architecture — the same patterns used by companies running Rust at scale in 2026.  

## What's Next?

*   **OpenTelemetry** — ship traces to Jaeger or Honeycomb
*   **OAuth 2.0 / OIDC** — "Log in with Google" support
*   **gRPC with tonic** — internal service-to-service communication
*   **Event sourcing** — append-only event logs for audit trails
*   **Feature flags** — progressive rollout with Unleash
*   **Rate limiting** — tower-governor for API protection
*   **GraphQL** — async-graphql for flexible client queries

⚡ Built with Rust 1.96.0 • June 2026 Edition

Inspired by Luca Palmieri's _Zero to Production in Rust_

  
> [!IMPORTANT]  
> To truly understand our stack, we profile the compiled binaries. We use cargo-flamegraph (wrapping \`perf\` on Linux) to visualize CPU cycles. We also utilize Tokio's built-in tokio-console. By compiling with the tokio_unstable flag, we can monitor the state of individual async tasks in real-time, identifying stalled futures or tasks that are blocking the async reactor thread (a cardinal sin in Rust).