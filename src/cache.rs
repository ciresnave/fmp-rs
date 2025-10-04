//! Intelligent caching layer for FMP API responses.

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Default TTL for cache entries
    pub default_ttl: Duration,
    /// Maximum number of cached items
    pub max_items: usize,
    /// Enable cache compression
    pub compress: bool,
    /// Cache hit/miss tracking
    pub enable_metrics: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            default_ttl: Duration::from_secs(300), // 5 minutes default
            max_items: 1000,
            compress: true,
            enable_metrics: true,
        }
    }
}

/// Cache entry with expiration
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    data: T,
    created_at: Instant,
    ttl: Duration,
    access_count: u64,
    last_access: Instant,
}

impl<T> CacheEntry<T> {
    fn new(data: T, ttl: Duration) -> Self {
        let now = Instant::now();
        Self {
            data,
            created_at: now,
            ttl,
            access_count: 0,
            last_access: now,
        }
    }

    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }

    fn access(&mut self) -> &T {
        self.access_count += 1;
        self.last_access = Instant::now();
        &self.data
    }
}

/// Cache key for API requests
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    endpoint: String,
    params: String,
}

impl CacheKey {
    pub fn new(endpoint: &str, params: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            params: params.to_string(),
        }
    }
}

/// Cache metrics
#[derive(Debug, Clone, Default)]
pub struct CacheMetrics {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub expired_items: u64,
    pub total_items: usize,
}

impl CacheMetrics {
    pub fn hit_rate(&self) -> f64 {
        if self.hits + self.misses == 0 {
            0.0
        } else {
            self.hits as f64 / (self.hits + self.misses) as f64
        }
    }
}

/// Intelligent cache with TTL and LRU eviction
pub struct IntelligentCache<T>
where
    T: Clone + Send + Sync,
{
    cache: Arc<RwLock<HashMap<CacheKey, CacheEntry<T>>>>,
    config: CacheConfig,
    metrics: Arc<RwLock<CacheMetrics>>,
}

impl<T> IntelligentCache<T>
where
    T: Clone + Send + Sync,
{
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            config,
            metrics: Arc::new(RwLock::new(CacheMetrics::default())),
        }
    }

    /// Get an item from cache
    pub async fn get(&self, key: &CacheKey) -> Option<T> {
        let mut cache = self.cache.write().await;

        if let Some(entry) = cache.get_mut(key) {
            if entry.is_expired() {
                // Remove expired entry
                cache.remove(key);
                if self.config.enable_metrics {
                    let mut metrics = self.metrics.write().await;
                    metrics.expired_items += 1;
                    metrics.total_items = cache.len();
                }
                return None;
            }

            // Cache hit
            let data = entry.access().clone();
            if self.config.enable_metrics {
                let mut metrics = self.metrics.write().await;
                metrics.hits += 1;
            }
            Some(data)
        } else {
            // Cache miss
            if self.config.enable_metrics {
                let mut metrics = self.metrics.write().await;
                metrics.misses += 1;
            }
            None
        }
    }

    /// Store an item in cache
    pub async fn set(&self, key: CacheKey, value: T, ttl: Option<Duration>) {
        let mut cache = self.cache.write().await;

        // Check if we need to evict items
        if cache.len() >= self.config.max_items {
            self.evict_lru(&mut cache).await;
        }

        let ttl = ttl.unwrap_or(self.config.default_ttl);
        let entry = CacheEntry::new(value, ttl);
        cache.insert(key, entry);

        if self.config.enable_metrics {
            let mut metrics = self.metrics.write().await;
            metrics.total_items = cache.len();
        }
    }

    /// Evict least recently used item
    async fn evict_lru(&self, cache: &mut HashMap<CacheKey, CacheEntry<T>>) {
        if cache.is_empty() {
            return;
        }

        // Find LRU item
        let lru_key = cache
            .iter()
            .min_by_key(|(_, entry)| entry.last_access)
            .map(|(key, _)| key.clone());

        if let Some(key) = lru_key {
            cache.remove(&key);
            if self.config.enable_metrics {
                let mut metrics = self.metrics.write().await;
                metrics.evictions += 1;
            }
        }
    }

    /// Clear expired entries
    pub async fn cleanup_expired(&self) {
        let mut cache = self.cache.write().await;
        let initial_size = cache.len();

        cache.retain(|_, entry| !entry.is_expired());

        if self.config.enable_metrics {
            let mut metrics = self.metrics.write().await;
            metrics.expired_items += (initial_size - cache.len()) as u64;
            metrics.total_items = cache.len();
        }
    }

    /// Get cache metrics
    pub async fn get_metrics(&self) -> CacheMetrics {
        self.metrics.read().await.clone()
    }

    /// Clear all cache entries
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();

        if self.config.enable_metrics {
            let mut metrics = self.metrics.write().await;
            metrics.total_items = 0;
        }
    }

    /// Get cache size
    pub async fn size(&self) -> usize {
        self.cache.read().await.len()
    }
}

/// Smart cache strategy based on endpoint characteristics
pub struct SmartCacheStrategy;

impl SmartCacheStrategy {
    /// Get recommended TTL for different endpoint types
    pub fn get_ttl_for_endpoint(endpoint: &str) -> Duration {
        match endpoint {
            // Real-time data - very short TTL
            path if path.contains("quote") || path.contains("price") => Duration::from_secs(30),

            // Market hours - cache until next day
            path if path.contains("market-hours") => Duration::from_secs(3600 * 12),

            // Company info - longer TTL
            path if path.contains("profile") || path.contains("company") => {
                Duration::from_secs(3600 * 24)
            }

            // Financial statements - medium TTL
            path if path.contains("income-statement") || path.contains("balance-sheet") => {
                Duration::from_secs(3600 * 6)
            }

            // Historical data - longer TTL
            path if path.contains("historical") => Duration::from_secs(3600 * 2),

            // News and events - medium TTL
            path if path.contains("news") || path.contains("calendar") => Duration::from_secs(900),

            // Directory and static data - very long TTL
            path if path.contains("symbols") || path.contains("exchanges") => {
                Duration::from_secs(3600 * 24 * 7)
            }

            // Default TTL
            _ => Duration::from_secs(300),
        }
    }

    /// Determine if an endpoint should be cached
    pub fn should_cache_endpoint(endpoint: &str) -> bool {
        // Don't cache bulk data endpoints
        if endpoint.contains("bulk") {
            return false;
        }

        // Don't cache real-time streaming data
        if endpoint.contains("stream") || endpoint.contains("websocket") {
            return false;
        }

        // Cache everything else
        true
    }

    /// Generate cache key from request details
    pub fn generate_cache_key(endpoint: &str, query_params: Option<&str>) -> CacheKey {
        let params = query_params.unwrap_or("");
        CacheKey::new(endpoint, params)
    }
}

/// Cached API client wrapper
pub struct CachedApiClient<T>
where
    T: Clone + Send + Sync + for<'de> Deserialize<'de> + Serialize,
{
    cache: IntelligentCache<T>,
}

impl<T> CachedApiClient<T>
where
    T: Clone + Send + Sync + for<'de> Deserialize<'de> + Serialize,
{
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: IntelligentCache::new(config),
        }
    }

    /// Get data with caching
    pub async fn get_cached<F, Fut>(&self, key: CacheKey, fetch_fn: F) -> Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        // Try cache first
        if let Some(cached_data) = self.cache.get(&key).await {
            return Ok(cached_data);
        }

        // Fetch from API
        let data = fetch_fn().await?;

        // Cache the result if appropriate
        if SmartCacheStrategy::should_cache_endpoint(&key.endpoint) {
            let ttl = SmartCacheStrategy::get_ttl_for_endpoint(&key.endpoint);
            self.cache.set(key, data.clone(), Some(ttl)).await;
        }

        Ok(data)
    }

    /// Get cache metrics
    pub async fn metrics(&self) -> CacheMetrics {
        self.cache.get_metrics().await
    }

    /// Cleanup expired entries
    pub async fn cleanup(&self) {
        self.cache.cleanup_expired().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let config = CacheConfig::default();
        let cache: IntelligentCache<String> = IntelligentCache::new(config);

        let key = CacheKey::new("test", "params");
        let value = "test_value".to_string();

        // Test miss
        assert!(cache.get(&key).await.is_none());

        // Test set and hit
        cache.set(key.clone(), value.clone(), None).await;
        assert_eq!(cache.get(&key).await.unwrap(), value);
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let config = CacheConfig::default();
        let cache: IntelligentCache<String> = IntelligentCache::new(config);

        let key = CacheKey::new("test", "params");
        let value = "test_value".to_string();

        // Set with short TTL
        cache
            .set(key.clone(), value, Some(Duration::from_millis(100)))
            .await;

        // Should be available immediately
        assert!(cache.get(&key).await.is_some());

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Should be expired
        assert!(cache.get(&key).await.is_none());
    }

    #[test]
    fn test_smart_cache_strategy() {
        // Test TTL recommendations
        assert_eq!(
            SmartCacheStrategy::get_ttl_for_endpoint("/quote"),
            Duration::from_secs(30)
        );
        assert_eq!(
            SmartCacheStrategy::get_ttl_for_endpoint("/profile"),
            Duration::from_secs(3600 * 24)
        );

        // Test caching decisions
        assert!(SmartCacheStrategy::should_cache_endpoint("/quote"));
        assert!(!SmartCacheStrategy::should_cache_endpoint("/bulk-data"));
    }
}
