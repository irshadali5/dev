use moka::sync::Cache;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::OnceLock;
use std::time::Duration;

pub struct MemoryCache;

static STRING_CACHE: OnceLock<Cache<String, String>> = OnceLock::new();
static BINARY_CACHE: OnceLock<Cache<String, Vec<u8>>> = OnceLock::new();

impl MemoryCache {
    fn string_cache() -> &'static Cache<String, String> {
        STRING_CACHE.get_or_init(|| {
            Cache::builder()
                .time_to_live(Duration::from_secs(300))
                .max_capacity(1000)
                .build()
        })
    }

    fn binary_cache() -> &'static Cache<String, Vec<u8>> {
        BINARY_CACHE.get_or_init(|| {
            Cache::builder()
                .time_to_live(Duration::from_secs(300))
                .max_capacity(2000)
                .build()
        })
    }

    pub fn get(key: &str) -> Option<String> {
        Self::string_cache().get(key)
    }

    pub fn insert(key: String, value: String) {
        Self::string_cache().insert(key, value);
    }

    /// Fast, compact zero-copy binary serialization using Postcard
    pub fn get_postcard<T: DeserializeOwned>(key: &str) -> Option<T> {
        let bytes = Self::binary_cache().get(key)?;
        postcard::from_bytes(&bytes).ok()
    }

    /// Fast, compact zero-copy binary serialization using Postcard
    pub fn insert_postcard<T: Serialize>(key: String, value: &T) {
        if let Ok(bytes) = postcard::to_allocvec(value) {
            Self::binary_cache().insert(key, bytes);
        }
    }

    pub fn invalidate(key: &str) {
        Self::string_cache().invalidate(key);
        Self::binary_cache().invalidate(key);
    }

    pub fn invalidate_all() {
        Self::string_cache().invalidate_all();
        Self::binary_cache().invalidate_all();
    }
}

pub struct RateLimiter;

static RATE_LIMIT_CACHE: OnceLock<Cache<String, (u32, std::time::Instant)>> = OnceLock::new();

impl RateLimiter {
    fn cache() -> &'static Cache<String, (u32, std::time::Instant)> {
        RATE_LIMIT_CACHE.get_or_init(|| {
            Cache::builder()
                .time_to_live(Duration::from_secs(600))
                .max_capacity(10_000)
                .build()
        })
    }

    /// Fixed-window in-memory rate limiter backed by `moka`.
    /// Returns `true` if the request is allowed, `false` if the limit is exceeded.
    pub fn check(key: &str, max_requests: u32, window_secs: u64) -> bool {
        let now = std::time::Instant::now();
        let window = Duration::from_secs(window_secs);
        let cache = Self::cache();

        if let Some((count, window_start)) = cache.get(key) {
            if now.duration_since(window_start) < window {
                if count >= max_requests {
                    return false;
                }
                cache.insert(key.to_string(), (count + 1, window_start));
                return true;
            }
        }

        cache.insert(key.to_string(), (1, now));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
    struct BenchmarkPayload {
        id: u64,
        title: String,
        tags: Vec<String>,
    }

    #[test]
    fn test_postcard_cache_roundtrip() {
        let item = BenchmarkPayload {
            id: 101,
            title: "Zero Copy High Performance Serialization".to_string(),
            tags: vec!["rust".to_string(), "postcard".to_string(), "systems".to_string()],
        };

        MemoryCache::insert_postcard("test_bench_item".to_string(), &item);
        let retrieved: Option<BenchmarkPayload> = MemoryCache::get_postcard("test_bench_item");
        assert_eq!(retrieved, Some(item));
    }

    #[test]
    fn test_rate_limiter() {
        let key = "test_rl_user_1";
        assert!(RateLimiter::check(key, 2, 60));
        assert!(RateLimiter::check(key, 2, 60));
        assert!(!RateLimiter::check(key, 2, 60));
    }
}
