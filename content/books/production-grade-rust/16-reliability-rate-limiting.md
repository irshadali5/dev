---
title: "Reliability & Rate Limiting"
description: "If our API becomes popular, we'll face abuse, bot traffic, and external dependency failures. Let's harden our service."
order: 16
---

If our API becomes popular, we'll face abuse, bot traffic, and external dependency failures. Let's harden our service.

## Rate Limiting with tower-governor

To prevent malicious users from spamming our subscription endpoint, we apply IP-based rate limiting.

**`src/startup.rs`**

```rust
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

let governor_conf = GovernorConfigBuilder::default()
    .per_second(1)
    .burst_size(5)
    .finish()
    .unwrap();

let app = Router::new()
    .route("/subscriptions", post(subscribe))
    .layer(GovernorLayer {
        config: Arc::new(governor_conf),
    });
```

## Circuit Breakers

When our email provider (e.g., Postmark) goes down, we shouldn't waste resources attempting delivery and blocking our workers. The **Circuit Breaker** pattern prevents cascading failures.

We can use crates like `failsafe` to wrap our HTTP clients. If the error rate crosses a threshold, the circuit "opens" and immediately fails fast, allowing the external service time to recover.

  
> [!IMPORTANT]  
> We implement the Token Bucket algorithm for rate limiting, but we go further by injecting Chaos. Inspired by Netflix's Chaos Monkey, we use tower middleware to randomly drop 1% of requests or inject 500ms of latency in our staging environment to ensure our circuit breakers actuate correctly. When clients are rate-limited, they often retry immediately. If 10,000 clients retry exactly 1 second later, it creates a thundering herd. We mandate Exponential Backoff with Full Jitter on all our clients to smooth out traffic spikes.