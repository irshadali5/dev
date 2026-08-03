---
title: "Telemetry & Tracing"
description: "Logs are not enough. We need structured, correlated, searchable telemetry data."
order: 6
---

## Structured Logging with \`tracing\`

Logs are not enough. We need **structured, correlated, searchable** telemetry data.

**`src/telemetry.rs`**

```rust
use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber(
    name: String,
    env_filter: String,
    sink: impl std::io::Write + Send + Sync + 'static,
) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
```

## Instrumenting Handlers

**`src/routes/subscriptions.rs`**

```rust
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, state),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    )
)]
pub async fn subscribe(
    Form(form): Form<SubscribeForm>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // All logs inside this function automatically get
    // subscriber_email and subscriber_name fields!
    tracing::info!("Processing subscription");
    // ...
}
```

## Adding Tracing Middleware to Axum

**`src/startup.rs`**

```rust
use tower_http::trace::TraceLayer;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
```

```text
{
  "name": "zero2prod",
  "msg": "[ADDING A NEW SUBSCRIBER - START]",
  "subscriber_email": "ursula@example.com",
  "subscriber_name": "le guin",
  "level": "INFO",
  "target": "zero2prod::routes::subscriptions"
}
{
  "name": "zero2prod",
  "msg": "[ADDING A NEW SUBSCRIBER - END]",
  "elapsed_milliseconds": 42,
  "subscriber_email": "ursula@example.com"
}
```

  
> [!IMPORTANT]  
> Local logging is insufficient for microservices. We upgrade from basic tracing-subscriber to OpenTelemetry (OTLP). We configure an OTLP exporter to send our spans directly to a collector like Jaeger or Honeycomb. Crucially, we implement W3C Trace Context propagation. When our API calls a downstream internal service via Reqwest, we inject the traceparent header, allowing us to visualize the entire request lifecycle across multiple physical machines in a single distributed trace flamegraph.