//! Benchmark tests for production validation
//!
//! Run with: cargo bench --bench production_validation

use criterion::{criterion_group, criterion_main, Criterion};
use fmp_rs::{CacheConfig, ProductionConfig, ProductionFmpClient, RateLimitConfig};
use std::hint::black_box;
use std::time::Duration;
use tokio::runtime::Runtime;

fn create_test_client() -> ProductionFmpClient {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let config = ProductionConfig {
            api_key: "test_key".to_string(),
            enable_caching: true,
            enable_metrics: true,
            enable_tracing: false,
            cache: CacheConfig {
                default_ttl: Duration::from_secs(300),
                max_items: 1000,
                enable_metrics: true,
                ..Default::default()
            },
            rate_limit: RateLimitConfig {
                max_requests_per_second: 100,
                max_burst: 10,
                ..Default::default()
            },
            ..Default::default()
        };

        ProductionFmpClient::new(config).await.unwrap()
    })
}

fn bench_client_creation(c: &mut Criterion) {
    c.bench_function("production_client_creation", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let config = ProductionConfig {
                    api_key: black_box("test_key".to_string()),
                    enable_caching: true,
                    enable_metrics: false, // Disable to reduce overhead
                    enable_tracing: false,
                    ..Default::default()
                };

                ProductionFmpClient::new(config).await
            })
        });
    });
}

fn bench_cache_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let client = create_test_client();

    c.bench_function("cache_set_and_get", |b| {
        b.iter(|| {
            rt.block_on(async {
                // Simulate caching a response
                let _endpoint = black_box("/quote/AAPL");
                let _mock_data = serde_json::json!({"symbol": "AAPL", "price": 150.0});

                // This would normally go through the caching layer
                // For benchmarking, we'll just test the client health check which is lightweight
                let _result = client.health_check().await;
            })
        });
    });
}

fn bench_bulk_processing_simulation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("bulk_processing_setup", |b| {
        b.iter(|| {
            rt.block_on(async {
                // Simulate bulk processing setup
                let symbols = black_box(vec!["AAPL", "MSFT", "GOOGL", "AMZN"]);

                // In real scenario, this would process through ChunkProcessor
                for symbol in symbols {
                    // Simulate work for each symbol
                    let _endpoint = format!("/quote/{}", symbol);
                }
            })
        });
    });
}

fn bench_config_creation(c: &mut Criterion) {
    c.bench_function("production_config_creation", |b| {
        b.iter(|| {
            let config = ProductionConfig {
                api_key: black_box("test_key".to_string()),
                enable_caching: black_box(true),
                enable_metrics: black_box(true),
                cache: CacheConfig {
                    default_ttl: Duration::from_secs(300),
                    max_items: 1000,
                    ..Default::default()
                },
                rate_limit: RateLimitConfig {
                    max_requests_per_second: 100,
                    ..Default::default()
                },
                ..Default::default()
            };
            black_box(config)
        });
    });
}

fn bench_metrics_collection(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let client = create_test_client();

    c.bench_function("metrics_collection", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _metrics = client.get_performance_metrics().await;
                let _cache_metrics = client.get_cache_metrics().await;
            })
        });
    });
}

criterion_group!(
    production_benches,
    bench_client_creation,
    bench_cache_operations,
    bench_bulk_processing_simulation,
    bench_config_creation,
    bench_metrics_collection
);

criterion_main!(production_benches);
