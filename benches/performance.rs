//! Performance benchmarks for FMP client
//!
//! Run with: cargo bench

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use fmp_rs::{FmpClient, ProductionConfig, ProductionFmpClient};
use tokio::runtime::Runtime;

fn bench_basic_client(c: &mut Criterion) {
    c.bench_function("basic_client_creation", |b| {
        b.iter(|| {
            let client = FmpClient::builder().api_key("test_key").build();
            black_box(client)
        })
    });
}

fn bench_production_client(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("production_client_creation", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = ProductionConfig {
                    api_key: "test_key".to_string(),
                    enable_tracing: false,
                    ..Default::default()
                };
                let client = ProductionFmpClient::new(config).await;
                black_box(client)
            })
        })
    });
}

fn bench_cache_operations(c: &mut Criterion) {
    use fmp_rs::cache::CacheKey;
    use fmp_rs::{CacheConfig, IntelligentCache};

    let rt = Runtime::new().unwrap();

    c.bench_function("cache_set_get", |b| {
        b.iter(|| {
            rt.block_on(async {
                let cache: IntelligentCache<String> = IntelligentCache::new(CacheConfig::default());

                let key = CacheKey::new("test_endpoint", "test_params");
                let value = "test_data".to_string();

                cache.set(key.clone(), value, None).await;
                let result = cache.get(&key).await;
                black_box(result)
            })
        })
    });
}

fn bench_bulk_processing(c: &mut Criterion) {
    use fmp_rs::{BulkProcessingConfig, ChunkProcessor};

    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("bulk_processing");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("chunk_processing", size),
            size,
            |b, &size| {
                b.iter(|| {
                    rt.block_on(async {
                        let config = BulkProcessingConfig {
                            chunk_size: 100,
                            ..Default::default()
                        };
                        let mut processor = ChunkProcessor::new(config);

                        let data: Vec<i32> = (0..size).collect();
                        processor.add_items(data).unwrap();

                        let results = processor
                            .process_chunks(|chunk| async move { Ok(chunk.len()) })
                            .await;

                        black_box(results)
                    })
                })
            },
        );
    }
    group.finish();
}

fn bench_string_operations(c: &mut Criterion) {
    c.bench_function("url_building", |b| {
        b.iter(|| {
            let base = "https://financialmodelingprep.com/stable";
            let endpoint = "/test/endpoint";
            let url = format!("{}/{}", base, endpoint.trim_start_matches('/'));
            black_box(url)
        })
    });
}

fn bench_json_operations(c: &mut Criterion) {
    use serde_json;

    let test_json = r#"{"symbol":"AAPL","name":"Apple Inc","price":150.25}"#;

    c.bench_function("json_parsing", |b| {
        b.iter(|| {
            let parsed: serde_json::Value = serde_json::from_str(test_json).unwrap();
            black_box(parsed)
        })
    });
}

criterion_group!(
    benches,
    bench_basic_client,
    bench_production_client,
    bench_cache_operations,
    bench_bulk_processing,
    bench_string_operations,
    bench_json_operations
);
criterion_main!(benches);
