---
title: "Deployment"
description: "We take DevOps to the next level by writing a custom Kubernetes Operator in Rust using kube-rs. Our operator watches for Custom Resource Definitions (CRDs) like EmailCampaign and automatically orchestrates the necessary pods and cronjobs."
order: 13
---

## Multi-Stage Docker Build

**`Dockerfile`**

```dockerfile
# Stage 1: Planner — compute recipe for dependency caching
FROM lukemathwalker/cargo-chef:latest-rust-1.96.0 AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Builder — cache deps, then build binary
FROM lukemathwalker/cargo-chef:latest-rust-1.96.0 AS builder
WORKDIR /app
RUN apt update && apt install -y lld clang && rm -rf /var/lib/apt/lists/*

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin zero2prod

# Stage 3: Runtime — minimal image
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration

ENV APP_ENVIRONMENT=production
ENV RUST_LOG=info

EXPOSE 8000
ENTRYPOINT ["./zero2prod"]
```

```text
$ docker build --tag zero2prod .
$ docker run -p 8000:8000 zero2prod
{"name":"zero2prod","msg":"Starting server on 0.0.0.0:8000","level":"INFO"}
```

## GitHub Actions CI/CD

**`.github/workflows/general.yml`**

```yaml
name: CI/CD Pipeline

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  SQLX_VERSION: 0.8.6
  SQLX_OFFLINE: true

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: password
          POSTGRES_DB: newsletter
        ports:
          - 5432:5432
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: 1.96.0
          components: rustfmt, clippy

      - name: Install sqlx-cli
        run: cargo install sqlx-cli --version ${SQLX_VERSION}

      - name: Run migrations
        run: |
          sudo apt-get install libpq-dev -y
          SKIP_DOCKER=true ./scripts/init_db.sh

      - name: Cargo fmt
        run: cargo fmt --check

      - name: Cargo clippy
        run: cargo clippy -- -D warnings

      - name: Cargo test
        run: cargo nextest run --all-features

      - name: Cargo deny
        run: cargo deny check

  deploy:
    needs: test
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build Docker image
        run: docker build -t zero2prod .
      - name: Deploy
        run: echo "Deploy to your cloud provider here"
```

  
> [!IMPORTANT]  
> We take DevOps to the next level by writing a custom Kubernetes Operator in Rust using kube-rs. Our operator watches for Custom Resource Definitions (CRDs) like EmailCampaign and automatically orchestrates the necessary pods and cronjobs. At the system level, the default system memory allocator (glibc malloc) causes fragmentation in long-running async applications. We swap the global allocator to jemalloc or mimalloc, consistently reducing memory footprint and 99th percentile latency spikes by up to 30%.