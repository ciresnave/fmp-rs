//! News endpoints

use crate::Result;
use crate::client::FmpClient;
use crate::models::news::{NewsArticle, PressRelease};
use serde::Serialize;

/// News API endpoints
pub struct News {
    client: FmpClient,
}

#[derive(Debug, Clone, Serialize)]
struct NewsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl News {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get general stock news
    ///
    /// # Arguments
    /// * `page` - Page number (optional)
    /// * `limit` - Number of results per page (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let news = client.news().get_stock_news(Some(0), Some(50)).await?;
    /// for article in news {
    ///     println!("{}: {}", article.symbol.unwrap_or_default(), article.title);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_stock_news(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Vec<NewsArticle>> {
        let query = NewsQuery { page, limit };
        self.client.get_with_query("v3/fmp/articles", &query).await
    }

    /// Get news for a specific symbol
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `page` - Page number (optional)
    /// * `limit` - Number of results per page (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let news = client.news().get_symbol_news("AAPL", Some(0), Some(10)).await?;
    /// for article in news {
    ///     println!("{}: {}", article.published_date, article.title);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_symbol_news(
        &self,
        symbol: &str,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Vec<NewsArticle>> {
        let query = NewsQuery { page, limit };
        self.client
            .get_with_query(&format!("v3/stock_news?tickers={}", symbol), &query)
            .await
    }

    /// Get news for multiple symbols
    ///
    /// # Arguments
    /// * `symbols` - List of stock symbols
    /// * `page` - Page number (optional)
    /// * `limit` - Number of results per page (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let symbols = vec!["AAPL", "MSFT", "GOOGL"];
    /// let news = client.news().get_symbols_news(&symbols, Some(0), Some(20)).await?;
    /// for article in news {
    ///     println!("{}: {}", article.symbol.unwrap_or_default(), article.title);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_symbols_news(
        &self,
        symbols: &[&str],
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Vec<NewsArticle>> {
        let tickers = symbols.join(",");
        let query = NewsQuery { page, limit };
        self.client
            .get_with_query(&format!("v3/stock_news?tickers={}", tickers), &query)
            .await
    }

    /// Get cryptocurrency news
    ///
    /// # Arguments
    /// * `page` - Page number (optional)
    /// * `limit` - Number of results per page (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let news = client.news().get_crypto_news(Some(0), Some(20)).await?;
    /// for article in news {
    ///     println!("{}: {}", article.symbol.unwrap_or_default(), article.title);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_crypto_news(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Vec<NewsArticle>> {
        let query = NewsQuery { page, limit };
        self.client.get_with_query("v4/crypto_news", &query).await
    }

    /// Get forex news
    ///
    /// # Arguments
    /// * `page` - Page number (optional)
    /// * `limit` - Number of results per page (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let news = client.news().get_forex_news(Some(0), Some(20)).await?;
    /// for article in news {
    ///     println!("{}: {}", article.symbol.unwrap_or_default(), article.title);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_forex_news(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Vec<NewsArticle>> {
        let query = NewsQuery { page, limit };
        self.client.get_with_query("v4/forex_news", &query).await
    }

    /// Get general news (all types)
    ///
    /// # Arguments
    /// * `page` - Page number (optional)
    /// * `limit` - Number of results per page (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let news = client.news().get_general_news(Some(0), Some(50)).await?;
    /// for article in news {
    ///     println!("{}: {}", article.published_date, article.title);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_general_news(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Vec<NewsArticle>> {
        let query = NewsQuery { page, limit };
        self.client.get_with_query("v4/general_news", &query).await
    }

    /// Get press releases for a symbol
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `page` - Page number (optional)
    /// * `limit` - Number of results per page (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let releases = client.news().get_press_releases("AAPL", Some(0), Some(10)).await?;
    /// for release in releases {
    ///     println!("{}: {}", release.date, release.title);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_press_releases(
        &self,
        symbol: &str,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Vec<PressRelease>> {
        let query = NewsQuery { page, limit };
        self.client
            .get_with_query(&format!("v3/press-releases/{}", symbol), &query)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let news = News::new(client);
        assert!(std::ptr::addr_of!(news.client).is_null() == false);
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_stock_news() {
        let client = FmpClient::new().unwrap();
        let result = client.news().get_stock_news(Some(0), Some(5)).await;
        assert!(result.is_ok());
        let news = result.unwrap();
        assert!(!news.is_empty());
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_symbol_news() {
        let client = FmpClient::new().unwrap();
        let result = client
            .news()
            .get_symbol_news("AAPL", Some(0), Some(5))
            .await;
        assert!(result.is_ok());
        let news = result.unwrap();
        assert!(!news.is_empty());
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_crypto_news() {
        let client = FmpClient::new().unwrap();
        let result = client.news().get_crypto_news(Some(0), Some(5)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_forex_news() {
        let client = FmpClient::new().unwrap();
        let result = client.news().get_forex_news(Some(0), Some(5)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_press_releases() {
        let client = FmpClient::new().unwrap();
        let result = client
            .news()
            .get_press_releases("AAPL", Some(0), Some(5))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_symbols_news() {
        let client = FmpClient::new().unwrap();
        let result = client
            .news()
            .get_symbols_news(&["AAPL", "MSFT"], Some(0), Some(10))
            .await;
        assert!(result.is_ok());
        let news = result.unwrap();
        // Should return news for the specified symbols
        if !news.is_empty() {
            assert!(news.iter().any(|n| {
                n.symbol
                    .as_ref()
                    .map_or(false, |s| s == "AAPL" || s == "MSFT")
            }));
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_general_news() {
        let client = FmpClient::new().unwrap();
        let result = client.news().get_general_news(Some(0), Some(20)).await;
        assert!(result.is_ok());
        let news = result.unwrap();
        assert!(!news.is_empty());
        // General news should have titles and dates
        assert!(news[0].title.len() > 0);
        assert!(news[0].published_date.len() > 0);
    }

    // Edge case tests
    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_symbol_news_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client
            .news()
            .get_symbol_news("INVALID_SYMBOL_12345", Some(0), Some(5))
            .await;
        // Should handle gracefully - either empty result or error
        match result {
            Ok(news) => assert!(news.is_empty()),
            Err(_) => {} // API may return error for invalid symbols
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_news_pagination_limits() {
        let client = FmpClient::new().unwrap();

        // Test large page number
        let result = client.news().get_stock_news(Some(1000), Some(5)).await;
        match result {
            Ok(news) => assert!(news.is_empty()), // Should return empty for high page numbers
            Err(_) => {}                          // API may return error
        }

        // Test large limit
        let result = client.news().get_stock_news(Some(0), Some(1000)).await;
        assert!(result.is_ok()); // Should handle large limits
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_symbols_news_empty_array() {
        let client = FmpClient::new().unwrap();
        let result = client.news().get_symbols_news(&[], Some(0), Some(5)).await;
        // Should handle empty symbols array
        match result {
            Ok(news) => assert!(news.is_empty()),
            Err(_) => {} // API may return error for empty input
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_symbols_news_mixed_valid_invalid() {
        let client = FmpClient::new().unwrap();
        let result = client
            .news()
            .get_symbols_news(&["AAPL", "INVALID_SYM", "MSFT"], Some(0), Some(10))
            .await;
        assert!(result.is_ok());
        let news = result.unwrap();
        // Should return news for valid symbols, may skip invalid ones
        if !news.is_empty() {
            assert!(news.iter().any(|n| {
                n.symbol
                    .as_ref()
                    .map_or(false, |s| s == "AAPL" || s == "MSFT")
            }));
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_news_zero_page_limit() {
        let client = FmpClient::new().unwrap();

        // Test zero limit
        let result = client.news().get_stock_news(Some(0), Some(0)).await;
        match result {
            Ok(news) => assert!(news.is_empty()),
            Err(_) => {} // API may return error for zero limit
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_get_press_releases_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client
            .news()
            .get_press_releases("INVALID_SYMBOL_12345", Some(0), Some(5))
            .await;
        // Should handle invalid symbols gracefully
        match result {
            Ok(releases) => assert!(releases.is_empty()),
            Err(_) => {} // API may return error for invalid symbols
        }
    }

    #[tokio::test]
    #[ignore] // Requires API key
    async fn test_news_data_validation() {
        let client = FmpClient::new().unwrap();
        let result = client.news().get_stock_news(Some(0), Some(5)).await;
        assert!(result.is_ok());
        let news = result.unwrap();

        if !news.is_empty() {
            let article = &news[0];
            // Validate required fields are present
            assert!(!article.title.is_empty());
            assert!(!article.published_date.is_empty());
            // URL should be valid if present
            if !article.url.is_empty() {
                let url = &article.url;
                assert!(url.starts_with("http://") || url.starts_with("https://"));
            }
        }
    }
}
