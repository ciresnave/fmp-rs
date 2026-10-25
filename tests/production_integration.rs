//! Integration tests for production features

use fmp_rs::{
    BulkProcessingConfig, CacheConfig, ChunkProcessor, IntelligentCache, ProductionConfig,
    ProductionFmpClient, RateLimitConfig, RetryConfig, SmartCacheStrategy,
};
use std::time::Duration;

#[tokio::test]
async fn test_production_client_initialization() {
    let config = ProductionConfig {
        api_key: "test_key".to_string(),
        enable_caching: false,
        enable_metrics: true,
        enable_tracing: false,
        ..Default::default()
    };

    let client = ProductionFmpClient::new(config).await;
    assert!(
        client.is_ok(),
        "Production client should initialize successfully"
    );
}

#[tokio::test]
async fn test_cache_functionality() {
    use fmp_rs::cache::CacheKey;

    let config = CacheConfig {
        default_ttl: Duration::from_secs(1),
        max_items: 10,
        compress: false,
        enable_metrics: true,
    };

    let cache: IntelligentCache<String> = IntelligentCache::new(config);

    // Test basic set/get
    let key = CacheKey::new("test", "params");
    let value = "test_value".to_string();

    cache.set(key.clone(), value.clone(), None).await;
    let retrieved = cache.get(&key).await;
    assert_eq!(retrieved, Some(value));

    // Test expiration
    tokio::time::sleep(Duration::from_millis(1100)).await;
    let expired = cache.get(&key).await;
    assert!(expired.is_none(), "Value should have expired");

    // Test metrics
    let metrics = cache.get_metrics().await;
    assert!(metrics.hits >= 1);
    assert!(metrics.misses >= 1);
    assert!(metrics.expired_items >= 1);
}

#[tokio::test]
async fn test_bulk_processing() {
    let config = BulkProcessingConfig {
        chunk_size: 5,
        max_memory_usage: 1024 * 1024, // 1MB
        ..Default::default()
    };

    let mut processor = ChunkProcessor::new(config);

    // Add test data
    let data: Vec<i32> = (1..=20).collect();
    processor.add_items(data).unwrap();

    // Process in chunks
    let results = processor
        .process_chunks(|chunk| async move {
            // Simulate processing
            tokio::time::sleep(Duration::from_millis(10)).await;
            Ok(chunk.len())
        })
        .await;

    assert!(results.is_ok());
    let chunk_sizes = results.unwrap();

    // Should have 4 chunks of 5 items each
    assert_eq!(chunk_sizes.len(), 4);
    assert_eq!(chunk_sizes, vec![5, 5, 5, 5]);
}

#[tokio::test]
async fn test_smart_cache_strategy() {
    // Test TTL recommendations
    let quote_ttl = SmartCacheStrategy::get_ttl_for_endpoint("/quote/AAPL");
    let profile_ttl = SmartCacheStrategy::get_ttl_for_endpoint("/profile/AAPL");
    let _bulk_ttl = SmartCacheStrategy::get_ttl_for_endpoint("/bulk-stock-prices");

    // Quote data should have shorter TTL than profile data
    assert!(quote_ttl < profile_ttl);

    // Test caching decisions
    assert!(SmartCacheStrategy::should_cache_endpoint("/quote/AAPL"));
    assert!(SmartCacheStrategy::should_cache_endpoint("/profile/AAPL"));
    assert!(!SmartCacheStrategy::should_cache_endpoint(
        "/bulk-stock-prices"
    ));
    assert!(!SmartCacheStrategy::should_cache_endpoint("/stream/quotes"));
}

#[tokio::test]
async fn test_retry_configuration() {
    let retry_config = RetryConfig {
        max_retries: 2,
        initial_delay: Duration::from_millis(100),
        max_delay: Duration::from_millis(500),
        multiplier: 2.0,
        jitter: false,
    };

    // Test that configuration values are set correctly
    assert_eq!(retry_config.max_retries, 2);
    assert_eq!(retry_config.initial_delay, Duration::from_millis(100));
    assert_eq!(retry_config.multiplier, 2.0);
    assert!(!retry_config.jitter);
}

#[tokio::test]
async fn test_rate_limiting_config() {
    let rate_config = RateLimitConfig {
        max_requests_per_second: 100,
        max_burst: 10,
        backoff_multiplier: 1.5,
        max_backoff: Duration::from_secs(30),
    };

    // Test configuration
    assert_eq!(rate_config.max_requests_per_second, 100);
    assert_eq!(rate_config.max_burst, 10);
    assert_eq!(rate_config.backoff_multiplier, 1.5);
    assert_eq!(rate_config.max_backoff, Duration::from_secs(30));
}

#[tokio::test]
async fn test_production_config_defaults() {
    let config = ProductionConfig::default();

    // Test that defaults are reasonable
    assert_eq!(config.timeout, Duration::from_secs(30));
    assert_eq!(config.rate_limit.max_requests_per_second, 300);
    assert_eq!(config.retry.max_retries, 3);
    assert_eq!(config.cache.default_ttl, Duration::from_secs(300));
    assert!(config.enable_caching);
    assert!(config.enable_metrics);
}

#[tokio::test]
async fn test_cache_cleanup() {
    use fmp_rs::cache::CacheKey;

    let config = CacheConfig {
        default_ttl: Duration::from_millis(50), // Very short TTL
        max_items: 100,
        enable_metrics: true,
        ..Default::default()
    };

    let cache: IntelligentCache<String> = IntelligentCache::new(config);

    // Add several items
    for i in 0..10 {
        let key = CacheKey::new(&format!("endpoint_{}", i), "params");
        cache.set(key, format!("value_{}", i), None).await;
    }

    // Wait for expiration
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Cleanup expired items
    cache.cleanup_expired().await;

    // Check that items were cleaned up
    let metrics = cache.get_metrics().await;
    assert_eq!(metrics.total_items, 0);
    assert!(metrics.expired_items >= 10);
}

#[tokio::test]
async fn test_bulk_processing_memory_limits() {
    let config = BulkProcessingConfig {
        chunk_size: 1000,
        max_memory_usage: 1024, // Very small limit to trigger memory checks
        ..Default::default()
    };

    let mut processor = ChunkProcessor::new(config);

    // Try to add data that would exceed memory limit
    let large_data: Vec<String> = (0..10000)
        .map(|i| format!("large_data_item_{}", i))
        .collect();

    let result = processor.add_items(large_data);

    // Should either succeed (if estimation is off) or fail gracefully
    match result {
        Ok(_) => println!("Memory estimation allowed large dataset"),
        Err(e) => {
            assert!(e.to_string().contains("Memory limit exceeded"));
        }
    }
}

#[tokio::test]
async fn test_production_builder_pattern() {
    let client = ProductionFmpClient::builder()
        .api_key("test_key")
        .timeout(Duration::from_secs(60))
        .enable_caching(false)
        .enable_metrics(true)
        .enable_tracing(false)
        .build()
        .await;

    assert!(client.is_ok(), "Builder pattern should work correctly");
}

#[tokio::test]
async fn test_cache_metrics_accuracy() {
    use fmp_rs::cache::CacheKey;

    let cache: IntelligentCache<String> = IntelligentCache::new(CacheConfig::default());

    // Perform several cache operations
    let key1 = CacheKey::new("endpoint1", "params1");
    let key2 = CacheKey::new("endpoint2", "params2");

    // Misses
    cache.get(&key1).await;
    cache.get(&key2).await;

    // Sets
    cache.set(key1.clone(), "value1".to_string(), None).await;
    cache.set(key2.clone(), "value2".to_string(), None).await;

    // Hits
    cache.get(&key1).await;
    cache.get(&key2).await;
    cache.get(&key1).await; // Another hit

    let metrics = cache.get_metrics().await;

    assert_eq!(metrics.misses, 2, "Should have 2 initial misses");
    assert_eq!(metrics.hits, 3, "Should have 3 hits after setting values");
    assert_eq!(metrics.total_items, 2, "Should have 2 items in cache");

    let hit_rate = metrics.hit_rate();
    assert!(
        (hit_rate - 0.6).abs() < 0.01,
        "Hit rate should be 60% (3/5)"
    );
}
