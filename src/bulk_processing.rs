//! Memory-optimized bulk data processing for large datasets.

use crate::error::{Error, Result};
use futures::Stream;
// Serde imports removed - not needed in this module
use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

/// Configuration for bulk data processing
#[derive(Debug, Clone)]
pub struct BulkProcessingConfig {
    /// Chunk size for processing large datasets
    pub chunk_size: usize,
    /// Maximum memory usage in bytes
    pub max_memory_usage: usize,
    /// Number of concurrent processing tasks
    pub concurrency: usize,
    /// Enable compression for temporary storage
    pub enable_compression: bool,
    /// Progress reporting interval
    pub progress_interval: usize,
}

impl Default for BulkProcessingConfig {
    fn default() -> Self {
        Self {
            chunk_size: 1000,
            max_memory_usage: 100 * 1024 * 1024, // 100MB
            concurrency: 4,
            enable_compression: true,
            progress_interval: 10000, // Report progress every 10k items
        }
    }
}

/// Progress information for bulk operations
#[derive(Debug, Clone)]
pub struct BulkProgress {
    pub processed_items: usize,
    pub total_items: Option<usize>,
    pub processing_rate: f64, // items per second
    pub estimated_completion: Option<std::time::Duration>,
    pub memory_usage: usize,
}

/// Memory-aware chunk processor
pub struct ChunkProcessor<T> {
    config: BulkProcessingConfig,
    buffer: VecDeque<T>,
    processed_count: usize,
    start_time: std::time::Instant,
    last_progress_report: usize,
}

impl<T> ChunkProcessor<T>
where
    T: Clone + Send + Sync,
{
    pub fn new(config: BulkProcessingConfig) -> Self {
        Self {
            config,
            buffer: VecDeque::new(),
            processed_count: 0,
            start_time: std::time::Instant::now(),
            last_progress_report: 0,
        }
    }

    /// Add items to the processing buffer
    pub fn add_items(&mut self, items: Vec<T>) -> Result<()> {
        // Check memory usage
        let estimated_memory = self.estimate_memory_usage(&items);
        if estimated_memory > self.config.max_memory_usage {
            return Err(Error::Custom(
                "Memory limit exceeded. Consider reducing chunk size.".to_string(),
            ));
        }

        self.buffer.extend(items);
        Ok(())
    }

    /// Process items in chunks with a callback function
    pub async fn process_chunks<F, Fut, R>(&mut self, mut processor: F) -> Result<Vec<R>>
    where
        F: FnMut(Vec<T>) -> Fut,
        Fut: std::future::Future<Output = Result<R>>,
        R: Send,
    {
        let mut results = Vec::new();

        while !self.buffer.is_empty() {
            // Take a chunk from the buffer
            let chunk_size = std::cmp::min(self.config.chunk_size, self.buffer.len());
            let chunk: Vec<T> = self.buffer.drain(..chunk_size).collect();

            // Process the chunk
            match processor(chunk).await {
                Ok(result) => {
                    results.push(result);
                    self.processed_count += chunk_size;

                    // Report progress if needed
                    if self.processed_count - self.last_progress_report
                        >= self.config.progress_interval
                    {
                        self.report_progress();
                        self.last_progress_report = self.processed_count;
                    }
                }
                Err(e) => {
                    warn!("Chunk processing failed: {}", e);
                    return Err(e);
                }
            }
        }

        info!("Completed processing {} items", self.processed_count);
        Ok(results)
    }

    /// Estimate memory usage of items
    fn estimate_memory_usage(&self, items: &[T]) -> usize {
        // Simple estimation: assume each item takes roughly 1KB
        // In production, this could be more sophisticated
        std::mem::size_of::<T>() * (self.buffer.len() + items.len())
            + 1024 * (self.buffer.len() + items.len())
    }

    /// Report processing progress
    fn report_progress(&self) {
        let elapsed = self.start_time.elapsed();
        let rate = self.processed_count as f64 / elapsed.as_secs_f64();

        debug!(
            "Processed {} items, rate: {:.2} items/sec, elapsed: {:?}",
            self.processed_count, rate, elapsed
        );
    }

    /// Get current progress
    pub fn get_progress(&self) -> BulkProgress {
        let elapsed = self.start_time.elapsed();
        let rate = if elapsed.as_secs_f64() > 0.0 {
            self.processed_count as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };

        BulkProgress {
            processed_items: self.processed_count,
            total_items: None, // Would need to be set externally
            processing_rate: rate,
            estimated_completion: None, // Would need total_items to calculate
            memory_usage: self.estimate_memory_usage(&[]),
        }
    }
}

/// Streaming data processor for very large datasets
pub struct StreamingProcessor<T> {
    sender: mpsc::UnboundedSender<T>,
    receiver: mpsc::UnboundedReceiver<T>,
    config: BulkProcessingConfig,
}

impl<T> StreamingProcessor<T>
where
    T: Send + 'static,
{
    pub fn new(config: BulkProcessingConfig) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        Self {
            sender,
            receiver,
            config,
        }
    }

    /// Get a sender for streaming data
    pub fn get_sender(&self) -> mpsc::UnboundedSender<T> {
        self.sender.clone()
    }

    /// Process streaming data with backpressure
    pub async fn process_stream<F, Fut, R>(&mut self, mut processor: F) -> Result<Vec<R>>
    where
        F: FnMut(Vec<T>) -> Fut,
        Fut: std::future::Future<Output = Result<R>>,
        R: Send,
    {
        let mut results = Vec::new();
        let mut buffer = Vec::with_capacity(self.config.chunk_size);

        while let Some(item) = self.receiver.recv().await {
            buffer.push(item);

            // Process when chunk is full
            if buffer.len() >= self.config.chunk_size {
                let chunk = std::mem::take(&mut buffer);
                match processor(chunk).await {
                    Ok(result) => results.push(result),
                    Err(e) => return Err(e),
                }
            }
        }

        // Process remaining items
        if !buffer.is_empty() {
            match processor(buffer).await {
                Ok(result) => results.push(result),
                Err(e) => return Err(e),
            }
        }

        Ok(results)
    }
}

/// Async stream adapter for bulk data
pub struct BulkDataStream<T> {
    data: VecDeque<T>,
    chunk_size: usize,
}

impl<T> BulkDataStream<T> {
    pub fn new(data: Vec<T>, chunk_size: usize) -> Self {
        Self {
            data: VecDeque::from(data),
            chunk_size,
        }
    }
}

impl<T> Stream for BulkDataStream<T>
where
    T: Clone + Unpin,
{
    type Item = Vec<T>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.data.is_empty() {
            return Poll::Ready(None);
        }

        let chunk_size = std::cmp::min(self.chunk_size, self.data.len());
        let chunk: Vec<T> = self.data.drain(..chunk_size).collect();

        Poll::Ready(Some(chunk))
    }
}

/// Memory-efficient data aggregator
pub struct DataAggregator<T> {
    config: BulkProcessingConfig,
    temp_storage: Vec<Vec<T>>,
    total_items: usize,
}

impl<T> DataAggregator<T>
where
    T: Clone + Send + Sync,
{
    pub fn new(config: BulkProcessingConfig) -> Self {
        Self {
            config,
            temp_storage: Vec::new(),
            total_items: 0,
        }
    }

    /// Add a batch of items
    pub fn add_batch(&mut self, batch: Vec<T>) -> Result<()> {
        let batch_size = batch.len();

        // Check if adding this batch would exceed memory limits
        let estimated_new_memory =
            self.estimate_total_memory() + self.estimate_batch_memory(&batch);

        if estimated_new_memory > self.config.max_memory_usage {
            // Flush to disk or compress if enabled
            if self.config.enable_compression {
                self.compress_oldest_batch()?;
            } else {
                return Err(Error::Custom(
                    "Memory limit exceeded and compression disabled".to_string(),
                ));
            }
        }

        self.temp_storage.push(batch);
        self.total_items += batch_size;
        Ok(())
    }

    /// Get all aggregated data
    pub fn get_all_data(&mut self) -> Vec<T> {
        let mut all_data = Vec::with_capacity(self.total_items);

        for batch in self.temp_storage.drain(..) {
            all_data.extend(batch);
        }

        self.total_items = 0;
        all_data
    }

    /// Get data in chunks for memory-efficient processing
    pub fn drain_chunks(&mut self, chunk_size: usize) -> Vec<Vec<T>> {
        let mut chunks = Vec::new();
        let mut current_chunk = Vec::with_capacity(chunk_size);

        for batch in self.temp_storage.drain(..) {
            for item in batch {
                current_chunk.push(item);

                if current_chunk.len() >= chunk_size {
                    chunks.push(std::mem::take(&mut current_chunk));
                    current_chunk = Vec::with_capacity(chunk_size);
                }
            }
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        self.total_items = 0;
        chunks
    }

    /// Estimate memory usage of a batch
    fn estimate_batch_memory(&self, batch: &[T]) -> usize {
        std::mem::size_of_val(batch) + 1024 * batch.len() // Rough overhead estimation
    }

    /// Estimate total memory usage
    fn estimate_total_memory(&self) -> usize {
        self.temp_storage
            .iter()
            .map(|batch| self.estimate_batch_memory(batch))
            .sum()
    }

    /// Compress oldest batch to save memory
    fn compress_oldest_batch(&mut self) -> Result<()> {
        if self.temp_storage.is_empty() {
            return Ok(());
        }

        // In a real implementation, this would compress the data
        // For now, we'll just remove the oldest batch
        warn!("Memory limit reached, removing oldest batch");
        if !self.temp_storage.is_empty() {
            let removed_batch = self.temp_storage.remove(0);
            self.total_items -= removed_batch.len();
        }

        Ok(())
    }

    /// Get current statistics
    pub fn get_stats(&self) -> (usize, usize, usize) {
        (
            self.total_items,
            self.temp_storage.len(),
            self.estimate_total_memory(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chunk_processor() {
        let config = BulkProcessingConfig {
            chunk_size: 3,
            ..Default::default()
        };

        let mut processor = ChunkProcessor::new(config);

        // Add test data
        let items = vec![1, 2, 3, 4, 5, 6, 7];
        processor.add_items(items).unwrap();

        // Process chunks
        let results = processor
            .process_chunks(|chunk| async move {
                Ok(chunk.len()) // Just return chunk size
            })
            .await
            .unwrap();

        assert_eq!(results, vec![3, 3, 1]); // 3 chunks: [1,2,3], [4,5,6], [7]
    }

    #[test]
    fn test_data_aggregator() {
        let config = BulkProcessingConfig::default();
        let mut aggregator = DataAggregator::new(config);

        // Add batches
        aggregator.add_batch(vec![1, 2, 3]).unwrap();
        aggregator.add_batch(vec![4, 5]).unwrap();

        // Get all data
        let all_data = aggregator.get_all_data();
        assert_eq!(all_data, vec![1, 2, 3, 4, 5]);
    }

    #[tokio::test]
    async fn test_bulk_data_stream() {
        use futures::StreamExt;

        let data = vec![1, 2, 3, 4, 5, 6, 7];
        let stream = BulkDataStream::new(data, 3);

        let chunks: Vec<Vec<i32>> = stream.collect().await;
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
    }
}
