# Multi-stage production container for dev-site
# Compatible with Kiri (Rust-based lightweight container runtime: https://runkiri.com)
#
# Build with Kiri:
#   kiri build -t dev-site .
#   (or with Docker CLI context: docker --context kiri build -t dev-site .)
#
# Run with Kiri:
#   kiri run -p 3000:3000 dev-site

FROM rust:1.80-slim-bullseye AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev sqlite3 curl

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos --locked

COPY . .

RUN cargo leptos build --release

FROM debian:bullseye-slim AS runner

WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates sqlite3 libssl1.1 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/app /app/server
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/content /app/content
COPY --from=builder /app/data /app/data
COPY --from=builder /app/dev_site.db /app/dev_site.db

ENV LEPTOS_SITE_ROOT=/app/site
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV LEPTOS_ENV=PROD
ENV DATABASE_URL=sqlite:/app/dev_site.db

EXPOSE 3000

CMD ["/app/server"]
