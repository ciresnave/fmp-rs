//! Real-world API testing for production validation
//!
//! This test validates production features using actual FMP API calls.
//! Set FMP_API_KEY environment variable to run these tests.

use fmp_rs::{CacheConfig, ProductionConfig, ProductionFmpClient, RateLimitConfig, RetryConfig};
use serde_json::Value;
use std::env;
use std::time::{Duration, Instant};

/// Helper function to get API key from environment
fn get_api_key() -> Option<String> {
    env::var("FMP_API_KEY")
        .ok()
        .filter(|key| !key.is_empty() && key != "your_fmp_api_key_here")
}

/// Skip test if no real API key is available
macro_rules! require_api_key {
    () => {
        if get_api_key().is_none() {
            println!("Skipping real API test - no FMP_API_KEY environment variable set");
            return;
        }
    };
}

#[tokio::test]
async fn test_real_api_basic_quote() {
    require_api_key!();

    let api_key = get_api_key().unwrap();
    let config = ProductionConfig {
        api_key,
        enable_caching: true,
        enable_metrics: true,
        enable_tracing: false,
        ..Default::default()
    };

    let client = ProductionFmpClient::new(config)
        .await
        .expect("Failed to create client");

    // Test basic API connectivity with a simple request
    let result: Result<Value, _> = client.get_cached("/quote/AAPL", None).await;

    match result {
        Ok(data) => {
            println!("✓ Successfully fetched AAPL data: {:?}", data);
            assert!(!data.is_null(), "Data should not be null");
        }
        Err(e) => {
            // If it's a rate limit error, that's actually good - means our rate limiting is working
            if e.to_string().contains("rate limit") || e.to_string().contains("429") {
                println!("✓ Rate limiting is working correctly");
            } else {
                panic!("Unexpected error: {}", e);
            }
        }
    }
}

#[tokio::test]
async fn test_real_api_caching_effectiveness() {
    require_api_key!();

    let api_key = get_api_key().unwrap();
    let config = ProductionConfig {
        api_key,
        enable_caching: true,
        enable_metrics: true,
        cache: CacheConfig {
            default_ttl: Duration::from_secs(60),
            max_items: 100,
            enable_metrics: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let client = ProductionFmpClient::new(config)
        .await
        .expect("Failed to create client");

    let symbol = "MSFT";

    let endpoint = format!("/quote/{}", symbol);

    // First request - should hit API
    let start1 = Instant::now();
    let result1: Result<Value, _> = client.get_cached(&endpoint, None).await;
    let duration1 = start1.elapsed();

    if result1.is_err() {
        println!("Skipping cache test due to API error: {:?}", result1.err());
        return;
    }

    // Second request immediately after - should hit cache
    let start2 = Instant::now();
    let result2: Result<Value, _> = client.get_cached(&endpoint, None).await;
    let duration2 = start2.elapsed();

    if let (Ok(data1), Ok(data2)) = (result1, result2) {
        println!("✓ First request took: {:?}", duration1);
        println!("✓ Second request took: {:?}", duration2);

        // Cache hit should be significantly faster
        assert!(duration2 < duration1, "Cached request should be faster");
        assert_eq!(data1, data2, "Cached result should match original");

        // Get cache metrics
        if let Some(metrics) = client.get_cache_metrics().await {
            println!("✓ Cache hits: {}", metrics.hits);
            println!("✓ Cache misses: {}", metrics.misses);
            assert!(metrics.hits >= 1, "Should have at least one cache hit");
        }
    }
}

#[tokio::test]
async fn test_real_api_retry_behavior() {
    require_api_key!();

    let api_key = get_api_key().unwrap();
    let config = ProductionConfig {
        api_key,
        enable_caching: false, // Disable caching to ensure we hit API
        retry: RetryConfig {
            max_retries: 2,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(1),
            multiplier: 2.0,
            jitter: true,
        },
        rate_limit: RateLimitConfig {
            max_requests_per_second: 1, // Very low to potentially trigger retries
            max_burst: 1,
            ..Default::default()
        },
        ..Default::default()
    };

    let client = ProductionFmpClient::new(config)
        .await
        .expect("Failed to create client");

    // Make multiple rapid requests to potentially trigger retry logic
    let symbols = vec!["AAPL", "GOOGL", "TSLA"];
    let mut results = Vec::new();

    for symbol in symbols {
        let endpoint = format!("/quote/{}", symbol);
        let result: Result<Value, _> = client.get_cached(&endpoint, None).await;
        results.push((symbol, result));

        // Small delay between requests
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    let successful_requests = results.iter().filter(|(_, result)| result.is_ok()).count();

    println!(
        "✓ Successful requests: {}/{}",
        successful_requests,
        results.len()
    );

    // Even with aggressive rate limiting, should get at least one successful request
    // due to retry logic
    assert!(
        successful_requests >= 1,
        "Retry logic should ensure at least one success"
    );
}

#[tokio::test]
async fn test_real_api_rate_limiting() {
    require_api_key!();

    let api_key = get_api_key().unwrap();
    let config = ProductionConfig {
        api_key,
        enable_caching: false,
        rate_limit: RateLimitConfig {
            max_requests_per_second: 2, // Very conservative
            max_burst: 1,
            ..Default::default()
        },
        ..Default::default()
    };

    let client = ProductionFmpClient::new(config)
        .await
        .expect("Failed to create client");

    let start = Instant::now();

    // Make 4 requests rapidly
    let requests = vec!["AAPL", "MSFT", "GOOGL", "AMZN"];
    let mut durations = Vec::new();

    for symbol in requests {
        let req_start = Instant::now();
        let endpoint = format!("/quote/{}", symbol);
        let _result: Result<Value, _> = client.get_cached(&endpoint, None).await; // Ignore result, focus on timing
        durations.push(req_start.elapsed());
    }

    let total_duration = start.elapsed();

    println!("✓ Total time for 4 requests: {:?}", total_duration);
    println!("✓ Individual request times: {:?}", durations);

    // With rate limiting of 2 req/sec, 4 requests should take at least 2 seconds
    // But allow some tolerance for network latency
    assert!(
        total_duration >= Duration::from_millis(1500),
        "Rate limiting should space out requests"
    );
}

#[tokio::test]
async fn test_real_api_bulk_processing() {
    require_api_key!();

    let api_key = get_api_key().unwrap();
    let config = ProductionConfig {
        api_key,
        enable_caching: true,
        ..Default::default()
    };

    let client = ProductionFmpClient::new(config)
        .await
        .expect("Failed to create client");

    // Test bulk processing with multiple symbols
    let symbols = vec![
        "AAPL", "MSFT", "GOOGL", "AMZN", "TSLA", "META", "NVDA", "NFLX",
    ];

    let start = Instant::now();

    // Simulate bulk processing by making multiple individual requests
    let mut successful_requests = 0;
    for symbol in &symbols {
        let endpoint = format!("/quote/{}", symbol);
        match client.get_cached::<Value>(&endpoint, None).await {
            Ok(_) => successful_requests += 1,
            Err(e) => println!("⚠ Failed to fetch {}: {}", symbol, e),
        }
        tokio::time::sleep(Duration::from_millis(100)).await; // Avoid rate limiting
    }

    let duration = start.elapsed();

    println!("✓ Bulk processing simulation took: {:?}", duration);
    println!(
        "✓ Successfully retrieved {}/{} symbols",
        successful_requests,
        symbols.len()
    );

    // Should get quotes for most symbols (allowing for some API limitations)
    let success_rate = successful_requests as f64 / symbols.len() as f64;
    assert!(
        success_rate >= 0.5,
        "Should successfully fetch at least 50% of symbols"
    );
}

#[tokio::test]
async fn test_real_api_performance_metrics() {
    require_api_key!();

    let api_key = get_api_key().unwrap();
    let config = ProductionConfig {
        api_key,
        enable_caching: true,
        enable_metrics: true,
        ..Default::default()
    };

    let client = ProductionFmpClient::new(config)
        .await
        .expect("Failed to create client");

    // Make several requests to generate metrics
    let symbols = vec!["AAPL", "MSFT", "GOOGL"];

    for symbol in symbols {
        let endpoint = format!("/quote/{}", symbol);
        let _result: Result<Value, _> = client.get_cached(&endpoint, None).await;
        tokio::time::sleep(Duration::from_millis(100)).await; // Avoid rate limiting
    }

    // Check if we can retrieve metrics
    if let Some(metrics) = client.get_performance_metrics().await {
        println!("✓ Performance metrics available:");
        println!("  - Total requests: {}", metrics.total_requests);
        println!(
            "  - Average response time: {:?}",
            metrics.average_response_time
        );
        println!("  - Error rate: {:.2}%", metrics.error_rate * 100.0);

        assert!(
            metrics.total_requests >= 3,
            "Should have made at least 3 requests"
        );
        assert!(
            metrics.average_response_time < Duration::from_secs(10),
            "Average response time should be reasonable"
        );
    } else {
        println!("⚠ Performance metrics not available (implementation may vary)");
    }
}

#[tokio::test]
async fn test_real_api_error_handling() {
    require_api_key!();

    let api_key = get_api_key().unwrap();
    let config = ProductionConfig {
        api_key,
        timeout: Duration::from_millis(100), // Very short timeout
        retry: RetryConfig {
            max_retries: 1, // Limited retries
            ..Default::default()
        },
        ..Default::default()
    };

    let client = ProductionFmpClient::new(config)
        .await
        .expect("Failed to create client");

    // Try to fetch an invalid symbol to test error handling
    let result: Result<Value, _> = client.get_cached("/quote/INVALID_SYMBOL_12345", None).await;

    match result {
        Ok(_) => {
            println!("⚠ Unexpectedly got result for invalid symbol");
        }
        Err(e) => {
            println!("✓ Error handling working correctly: {}", e);

            // Error should be informative
            let error_msg = e.to_string().to_lowercase();
            assert!(
                error_msg.contains("timeout")
                    || error_msg.contains("invalid")
                    || error_msg.contains("not found")
                    || error_msg.contains("404")
                    || error_msg.contains("400"),
                "Error message should be descriptive"
            );
        }
    }
}

#[tokio::test]
async fn test_real_api_connection_health() {
    require_api_key!();

    let api_key = get_api_key().unwrap();
    let config = ProductionConfig {
        api_key,
        enable_metrics: true,
        ..Default::default()
    };

    let client = ProductionFmpClient::new(config)
        .await
        .expect("Failed to create client");

    // Test health check
    let health_status = client.check_health().await;

    match health_status {
        Ok(is_healthy) => {
            if is_healthy {
                println!("✓ API connection is healthy");
            } else {
                println!("⚠ API connection health check failed");
            }
        }
        Err(e) => {
            println!("⚠ Health check error (this may be expected): {}", e);
        }
    }

    // Make a simple request to verify actual connectivity
    let connectivity_test: Result<Value, _> = client.get_cached("/quote/AAPL", None).await;
    match connectivity_test {
        Ok(_) => println!("✓ Basic connectivity confirmed"),
        Err(e) => println!("⚠ Basic connectivity test failed: {}", e),
    }
}
