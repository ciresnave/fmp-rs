//! Production-grade enhancements for the FMP client.

use crate::error::{Error, Result};
use reqwest::Client;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, info, warn};

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per second
    pub max_requests_per_second: u32,
    /// Maximum burst requests
    pub max_burst: u32,
    /// Backoff multiplier for retries
    pub backoff_multiplier: f64,
    /// Maximum backoff duration
    pub max_backoff: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests_per_second: 300, // FMP free tier limit
            max_burst: 5,
            backoff_multiplier: 2.0,
            max_backoff: Duration::from_secs(60),
        }
    }
}

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Maximum number of connections per host
    pub max_connections_per_host: usize,
    /// Connection idle timeout
    pub idle_timeout: Duration,
    /// Maximum connection lifetime
    pub max_lifetime: Duration,
    /// Enable HTTP/2
    pub http2_prior_knowledge: bool,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections_per_host: 10,
            idle_timeout: Duration::from_secs(90),
            max_lifetime: Duration::from_secs(300),
            http2_prior_knowledge: true,
        }
    }
}

/// Performance monitoring metrics
#[derive(Debug, Clone, Default)]
pub struct Metrics {
    /// Total requests made
    pub total_requests: u64,
    /// Total successful requests
    pub successful_requests: u64,
    /// Total failed requests
    pub failed_requests: u64,
    /// Total retries
    pub total_retries: u64,
    /// Average response time in milliseconds
    pub average_response_time_ms: f64,
    /// Rate limit hits
    pub rate_limit_hits: u64,
}

/// Enhanced retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Initial retry delay
    pub initial_delay: Duration,
    /// Maximum retry delay
    pub max_delay: Duration,
    /// Exponential backoff multiplier
    pub multiplier: f64,
    /// Add jitter to prevent thundering herd
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
            jitter: true,
        }
    }
}

/// Production-grade request executor with retry logic and rate limiting
pub struct ProductionExecutor {
    client: Client,
    rate_limit_config: RateLimitConfig,
    retry_config: RetryConfig,
    metrics: Arc<tokio::sync::Mutex<Metrics>>,
    last_request_time: Arc<tokio::sync::Mutex<Option<Instant>>>,
}

impl ProductionExecutor {
    /// Create a new production executor
    pub fn new(
        pool_config: PoolConfig,
        rate_limit_config: RateLimitConfig,
        retry_config: RetryConfig,
        timeout: Duration,
    ) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .pool_max_idle_per_host(pool_config.max_connections_per_host)
            .pool_idle_timeout(pool_config.idle_timeout)
            .http2_prior_knowledge()
            .user_agent("fmp-rs/1.0.0 (Production)")
            .gzip(true)
            .brotli(true)
            .build()
            .map_err(|e| Error::Custom(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            client,
            rate_limit_config,
            retry_config,
            metrics: Arc::new(tokio::sync::Mutex::new(Metrics::default())),
            last_request_time: Arc::new(tokio::sync::Mutex::new(None)),
        })
    }

    /// Execute a request with retry logic and rate limiting
    pub async fn execute_request<T>(&self, request_builder: reqwest::RequestBuilder) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let start_time = Instant::now();
        let mut attempt = 0;
        let mut delay = self.retry_config.initial_delay;

        loop {
            // Apply rate limiting
            self.apply_rate_limiting().await;

            // Clone the request for retry attempts
            let request = match request_builder.try_clone() {
                Some(req) => req,
                None => {
                    return Err(Error::Custom(
                        "Failed to clone request for retry".to_string(),
                    ));
                }
            };

            // Update metrics
            self.update_request_metrics().await;

            // Execute the request
            match self.execute_single_request::<T>(request).await {
                Ok(result) => {
                    // Update success metrics
                    self.update_success_metrics(start_time.elapsed()).await;
                    if attempt > 0 {
                        info!("Request succeeded after {} retries", attempt);
                    }
                    return Ok(result);
                }
                Err(err) => {
                    attempt += 1;

                    // Check if we should retry
                    if attempt >= self.retry_config.max_retries || !self.should_retry(&err) {
                        self.update_failure_metrics().await;
                        return Err(err);
                    }

                    // Log retry attempt
                    warn!(
                        "Request failed (attempt {}), retrying in {:?}: {}",
                        attempt, delay, err
                    );
                    self.update_retry_metrics().await;

                    // Wait before retrying
                    sleep(delay).await;

                    // Calculate next delay with exponential backoff
                    delay = std::cmp::min(
                        Duration::from_millis(
                            (delay.as_millis() as f64 * self.retry_config.multiplier) as u64,
                        ),
                        self.retry_config.max_delay,
                    );

                    // Add jitter if enabled
                    if self.retry_config.jitter {
                        use rand::Rng;
                        let jitter_ms = rand::thread_rng().gen_range(0..=100);
                        delay += Duration::from_millis(jitter_ms);
                    }
                }
            }
        }
    }

    /// Execute a single request attempt
    async fn execute_single_request<T>(&self, request: reqwest::RequestBuilder) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = request.send().await?;

        // Handle rate limiting
        if response.status().as_u16() == 429 {
            self.update_rate_limit_metrics().await;
            return Err(Error::RateLimitExceeded);
        }

        // Handle other HTTP errors
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::Api {
                status,
                message: error_text,
            });
        }

        // Parse JSON response
        let text = response.text().await?;
        debug!("Response body: {}", text);

        match serde_json::from_str::<T>(&text) {
            Ok(data) => Ok(data),
            Err(e) => {
                warn!(
                    "Failed to parse JSON response: {}. Response was: {}",
                    e, text
                );
                Err(Error::Json(e))
            }
        }
    }

    /// Apply rate limiting logic
    async fn apply_rate_limiting(&self) {
        let mut last_request = self.last_request_time.lock().await;

        if let Some(last_time) = *last_request {
            let min_interval =
                Duration::from_millis(1000 / self.rate_limit_config.max_requests_per_second as u64);
            let elapsed = last_time.elapsed();

            if elapsed < min_interval {
                let sleep_time = min_interval - elapsed;
                debug!("Rate limiting: sleeping for {:?}", sleep_time);
                sleep(sleep_time).await;
            }
        }

        *last_request = Some(Instant::now());
    }

    /// Determine if an error is retryable
    fn should_retry(&self, error: &Error) -> bool {
        match error {
            Error::Http(reqwest_error) => {
                // Retry on connection errors, timeouts, etc.
                reqwest_error.is_timeout()
                    || reqwest_error.is_connect()
                    || reqwest_error.is_request()
            }
            Error::RateLimitExceeded => true,
            Error::Api { status, .. } => {
                // Retry on server errors, not client errors
                *status >= 500
            }
            _ => false,
        }
    }

    /// Update request metrics
    async fn update_request_metrics(&self) {
        let mut metrics = self.metrics.lock().await;
        metrics.total_requests += 1;
    }

    /// Update success metrics
    async fn update_success_metrics(&self, response_time: Duration) {
        let mut metrics = self.metrics.lock().await;
        metrics.successful_requests += 1;

        // Update average response time
        let total_responses = metrics.successful_requests + metrics.failed_requests;
        let old_avg = metrics.average_response_time_ms;
        let new_response_time = response_time.as_millis() as f64;

        metrics.average_response_time_ms =
            (old_avg * (total_responses - 1) as f64 + new_response_time) / total_responses as f64;
    }

    /// Update failure metrics
    async fn update_failure_metrics(&self) {
        let mut metrics = self.metrics.lock().await;
        metrics.failed_requests += 1;
    }

    /// Update retry metrics
    async fn update_retry_metrics(&self) {
        let mut metrics = self.metrics.lock().await;
        metrics.total_retries += 1;
    }

    /// Update rate limit metrics
    async fn update_rate_limit_metrics(&self) {
        let mut metrics = self.metrics.lock().await;
        metrics.rate_limit_hits += 1;
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> Metrics {
        self.metrics.lock().await.clone()
    }

    /// Reset metrics
    pub async fn reset_metrics(&self) {
        let mut metrics = self.metrics.lock().await;
        *metrics = Metrics::default();
    }

    /// Get the underlying HTTP client
    pub fn client(&self) -> &Client {
        &self.client
    }
}
