//! Company information endpoints

use crate::client::FmpClient;
use crate::error::Result;
use crate::models::company::{
    CompanyCoreInfo, CompanyOutlook, CompanyProfile, CompanyRating, Executive, MarketCap,
    ShareFloat, StockPeers,
};
use serde::Serialize;

/// Company information API endpoints
pub struct CompanyInfo {
    client: FmpClient,
}

impl CompanyInfo {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get company profile
    pub async fn get_profile(&self, symbol: &str) -> Result<Vec<CompanyProfile>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/profile");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get company executives
    pub async fn get_executives(&self, symbol: &str) -> Result<Vec<Executive>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/key-executives");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get company market cap
    pub async fn get_market_cap(&self, symbol: &str) -> Result<Vec<MarketCap>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/market-capitalization");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get historical market cap
    pub async fn get_historical_market_cap(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<MarketCap>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/historical-market-capitalization");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get share float
    pub async fn get_share_float(&self, symbol: &str) -> Result<Vec<ShareFloat>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/shares-float");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get stock peers
    pub async fn get_peers(&self, symbol: &str) -> Result<Vec<StockPeers>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/stock-peers");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get comprehensive company outlook
    ///
    /// Returns a comprehensive view of company data including profile, metrics,
    /// ratios, insider trades, executives, and more.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let outlook = client.company_info().get_outlook("AAPL").await?;
    /// println!("Company: {}", outlook.profile.company_name);
    /// if let Some(rating) = &outlook.rating {
    ///     if let Some(r) = rating.first() {
    ///         println!("Rating: {} (Score: {})", r.rating, r.rating_score);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_outlook(&self, symbol: &str) -> Result<CompanyOutlook> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/company-outlook?symbol={}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get company rating
    ///
    /// Returns the company's rating and score based on various financial metrics.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let ratings = client.company_info().get_rating("AAPL").await?;
    /// for rating in ratings {
    ///     println!("Rating: {} (Score: {})", rating.rating, rating.rating_score);
    ///     println!("Recommendation: {}", rating.rating_recommendation);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_rating(&self, symbol: &str) -> Result<Vec<CompanyRating>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url(&format!("/rating/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get historical company rating
    ///
    /// Returns historical ratings for a company over time.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `limit` - Number of results (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let ratings = client.company_info().get_historical_rating("AAPL", Some(10)).await?;
    /// for rating in ratings {
    ///     println!("{}: {} (Score: {})", rating.date, rating.rating, rating.rating_score);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_historical_rating(
        &self,
        symbol: &str,
        limit: Option<u32>,
    ) -> Result<Vec<CompanyRating>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u32>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/historical-rating/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    limit,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get company core information
    ///
    /// Returns basic company information including CIK, symbol, and exchange.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let infos = client.company_info().get_core_info("AAPL").await?;
    /// for info in &infos {
    ///     println!("CIK: {}, Exchange: {}", info.cik, info.exchange);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_core_info(&self, symbol: &str) -> Result<Vec<CompanyCoreInfo>> {
        #[derive(Serialize)]
        struct Query<'a> {
            symbol: &'a str,
            apikey: &'a str,
        }

        let url = self.client.build_url("/company-core-information");
        self.client
            .get_with_query(
                &url,
                &Query {
                    symbol,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let _ = CompanyInfo::new(client);
    }

    // Golden path tests (require API key)
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_profile() {
        let client = FmpClient::new().unwrap();
        let result = client.company_info().get_profile("AAPL").await;
        assert!(result.is_ok());
        let profiles = result.unwrap();
        assert!(!profiles.is_empty());
        assert_eq!(profiles[0].symbol, "AAPL");
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_outlook() {
        let client = FmpClient::new().unwrap();
        let result = client.company_info().get_outlook("AAPL").await;
        assert!(result.is_ok());
        let outlook = result.unwrap();
        assert_eq!(outlook.profile.symbol, "AAPL");
        assert!(outlook.profile.company_name.contains("Apple"));
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_rating() {
        let client = FmpClient::new().unwrap();
        let result = client.company_info().get_rating("AAPL").await;
        assert!(result.is_ok());
        let ratings = result.unwrap();
        assert!(!ratings.is_empty());
        assert_eq!(ratings[0].symbol, "AAPL");
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_historical_rating() {
        let client = FmpClient::new().unwrap();
        let result = client
            .company_info()
            .get_historical_rating("AAPL", Some(5))
            .await;
        assert!(result.is_ok());
        let ratings = result.unwrap();
        assert!(!ratings.is_empty());
        assert!(ratings.len() <= 5);
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_core_info() {
        let client = FmpClient::new().unwrap();
        let result = client.company_info().get_core_info("AAPL").await;
        assert!(result.is_ok());
        let info = result.unwrap();
        assert!(!info.is_empty());
        assert_eq!(info[0].symbol, "AAPL");
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_executives() {
        let client = FmpClient::new().unwrap();
        let result = client.company_info().get_executives("AAPL").await;
        assert!(result.is_ok());
        let execs = result.unwrap();
        assert!(!execs.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_market_cap() {
        let client = FmpClient::new().unwrap();
        let result = client.company_info().get_market_cap("AAPL").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_share_float() {
        let client = FmpClient::new().unwrap();
        let result = client.company_info().get_share_float("AAPL").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_peers() {
        let client = FmpClient::new().unwrap();
        let result = client.company_info().get_peers("AAPL").await;
        assert!(result.is_ok());
        let peers = result.unwrap();
        assert!(!peers.is_empty());
        assert_eq!(peers[0].symbol, "AAPL");
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_profile_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client
            .company_info()
            .get_profile("INVALID_SYMBOL_XYZ123")
            .await;
        // Should either return empty vec or error
        if let Ok(profiles) = result {
            assert!(profiles.is_empty());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_rating_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client
            .company_info()
            .get_rating("INVALID_SYMBOL_XYZ123")
            .await;
        // Should either return empty vec or error
        if let Ok(ratings) = result {
            assert!(ratings.is_empty());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_historical_market_cap_with_date_range() {
        let client = FmpClient::new().unwrap();
        let result = client
            .company_info()
            .get_historical_market_cap("AAPL", Some("2024-01-01"), Some("2024-12-31"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_historical_rating_with_limit() {
        let client = FmpClient::new().unwrap();
        let result = client
            .company_info()
            .get_historical_rating("AAPL", Some(3))
            .await;
        assert!(result.is_ok());
        if let Ok(ratings) = result {
            assert!(ratings.len() <= 3);
        }
    }

    // Test error handling
    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = FmpClient::builder()
            .api_key("invalid_key_12345")
            .build()
            .unwrap();
        let result = client.company_info().get_profile("AAPL").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_empty_symbol() {
        let client = FmpClient::builder().api_key("test_key").build().unwrap();
        let result = client.company_info().get_profile("").await;
        // Should handle gracefully
        assert!(result.is_err() || result.unwrap().is_empty());
    }
}
