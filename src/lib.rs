//! # FMP-RS
//!
//! An ergonomic Rust wrapper for the Financial Modeling Prep (FMP) API.
//!
//! This library provides a comprehensive, type-safe interface to the FMP API,
//! with support for mocking and testing without making actual API requests.
//!
//! ## Example
//!
//! ```no_run
//! use fmp_rs::FmpClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = FmpClient::new()?;
//!     let quotes = client.quote().get_quote("AAPL").await?;
//!     for quote in &quotes {
//!         println!("Apple stock price: ${}", quote.price);
//!     }
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod endpoints;
pub mod error;
pub mod models;

// Production-grade enhancements
pub mod bulk_processing;
pub mod cache;
pub mod production;
pub mod production_client;

#[cfg(test)]
mod tests;

pub use client::{FmpClient, FmpClientBuilder};
pub use error::{Error, Result};

// Re-export production features
pub use bulk_processing::{BulkProcessingConfig, BulkProgress, ChunkProcessor, DataAggregator};
pub use cache::{CacheConfig, CacheMetrics, IntelligentCache, SmartCacheStrategy};
pub use production::{Metrics, PoolConfig, ProductionExecutor, RateLimitConfig, RetryConfig};
pub use production_client::{ProductionClientBuilder, ProductionConfig, ProductionFmpClient};

// Re-export commonly used types
pub use endpoints::*;
pub use models::common::*;
