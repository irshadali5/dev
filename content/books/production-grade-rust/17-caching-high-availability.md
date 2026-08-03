---
title: "Caching & High Availability"
description: "As traffic scales, querying the Postgres database for every request becomes a bottleneck. We introduce caching to serve reads lightning-fast."
order: 17
---

As traffic scales, querying the Postgres database for every request becomes a bottleneck. We introduce caching to serve reads lightning-fast.

## In-Memory Caching with moka

For configuration or static newsletter templates, an in-memory cache is ideal.

**`src/cache.rs`**

```rust
use moka::future::Cache;

let cache: Cache<String, String> = Cache::builder()
    .max_capacity(10_000)
    .time_to_live(Duration::from_secs(60 * 5))
    .build();

cache.insert("template_1".to_string(), "<html>...</html>".to_string()).await;
```

## Distributed Caching with Redis

When scaling horizontally across multiple Kubernetes pods, in-memory caches aren't shared. We use **Redis** (via the `redis` or `fred` crates) for distributed caching of user sessions and rate-limit quotas.

  
> [!IMPORTANT]  
> For ultimate performance, serve a slightly stale cached response while spawning an async background task to fetch the fresh data from the DB. This completely hides database latency from the end-user.  

  
> [!IMPORTANT]  
> A "Cache Stampede" occurs when a highly-requested cache key expires, and 1,000 concurrent requests immediately query the database to rebuild it, crashing the DB. We implement the Singleflight pattern. Using a shared DashMap of active futures, if 1,000 requests ask for the same expired key simultaneously, 999 of them will await the single database query initiated by the 1st request. The database sees 1 query, and all 1,000 requests are satisfied instantaneously. This is a critical pattern for hyperscale architectures.