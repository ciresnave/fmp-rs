//! Example demonstrating production features of fmp-rs

use fmp_rs::{CacheConfig, ProductionConfig, ProductionFmpClient, RateLimitConfig, RetryConfig};
use std::time::Duration;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for production monitoring
    tracing_subscriber::fmt::init();

    // Configure production client with advanced features
    let config = ProductionConfig {
        api_key: std::env::var("FMP_API_KEY").unwrap_or_else(|_| "demo".to_string()),
        timeout: Duration::from_secs(30),

        // Rate limiting configuration
        rate_limit: RateLimitConfig {
            max_requests_per_second: 300,
            max_burst: 5,
            backoff_multiplier: 2.0,
            max_backoff: Duration::from_secs(60),
        },

        // Retry configuration
        retry: RetryConfig {
            max_retries: 3,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
            jitter: true,
        },

        // Cache configuration
        cache: CacheConfig {
            default_ttl: Duration::from_secs(300),
            max_items: 1000,
            compress: true,
            enable_metrics: true,
        },

        enable_caching: true,
        enable_metrics: true,
        enable_tracing: true,

        ..Default::default()
    };

    info!("Creating production FMP client...");
    let client = ProductionFmpClient::new(config).await?;

    // Health check
    info!("Performing health check...");
    let is_healthy = client.health_check().await?;
    info!(
        "API health status: {}",
        if is_healthy { "OK" } else { "Failed" }
    );

    // Example API calls with production features
    info!("Fetching AAPL quote with caching and retry logic...");
    match client.quote().get_quote("AAPL").await {
        Ok(quote_data) => {
            info!(
                "Successfully fetched AAPL quote: {} bytes",
                serde_json::to_string(&quote_data)?.len()
            );
        }
        Err(e) => {
            info!("Failed to fetch quote: {}", e);
        }
    }

    // Demonstrate bulk processing
    info!("Testing bulk data processing...");
    let test_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let processed_results = client
        .process_bulk_data(test_data, |chunk| {
            Box::pin(async move {
                // Simulate processing
                tokio::time::sleep(Duration::from_millis(100)).await;
                Ok(chunk.len())
            })
        })
        .await?;

    info!(
        "Bulk processing completed: processed {} chunks",
        processed_results.len()
    );

    // Get performance metrics
    if let Some(metrics) = client.get_metrics().await {
        info!("Performance Metrics:");
        info!("  Total requests: {}", metrics.total_requests);
        info!("  Successful requests: {}", metrics.successful_requests);
        info!("  Failed requests: {}", metrics.failed_requests);
        info!(
            "  Average response time: {:.2}ms",
            metrics.average_response_time_ms
        );
        info!("  Rate limit hits: {}", metrics.rate_limit_hits);
        info!("  Total retries: {}", metrics.total_retries);
    }

    // Get cache metrics
    if let Some(cache_metrics) = client.get_cache_metrics().await {
        info!("Cache Metrics:");
        info!("  Cache hits: {}", cache_metrics.hits);
        info!("  Cache misses: {}", cache_metrics.misses);
        info!("  Hit rate: {:.2}%", cache_metrics.hit_rate() * 100.0);
        info!("  Total items: {}", cache_metrics.total_items);
        info!("  Evictions: {}", cache_metrics.evictions);
    }

    // Cleanup cache
    client.cleanup_cache().await;
    info!("Cache cleanup completed");

    info!("Production example completed successfully!");
    Ok(())
}
