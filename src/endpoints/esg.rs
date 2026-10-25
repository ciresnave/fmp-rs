//! ESG and Social endpoints

use crate::{client::FmpClient, error::Result, models::esg::*};
use serde::Serialize;

/// ESG and Social API endpoints
pub struct Esg {
    client: FmpClient,
}

impl Esg {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get ESG scores for a company
    ///
    /// Returns Environmental, Social, and Governance scores and ratings.
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
    /// let esg_scores = client.esg().get_esg_score("AAPL").await?;
    /// for score in esg_scores.iter().take(3) {
    ///     println!("{}: ESG Score = {:.1}",
    ///         score.company_name.as_deref().unwrap_or("N/A"),
    ///         score.esg_score.unwrap_or(0.0));
    ///     println!("  E: {:.1}, S: {:.1}, G: {:.1}",
    ///         score.environment_score.unwrap_or(0.0),
    ///         score.social_score.unwrap_or(0.0),
    ///         score.governance_score.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_esg_score(&self, symbol: &str) -> Result<Vec<EsgScore>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url(&format!("/esg-score/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get ESG risk ratings for a company
    ///
    /// Returns ESG risk assessment and industry rankings.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let esg_risk = client.esg().get_esg_risk_rating("TSLA").await?;
    /// for risk in esg_risk.iter().take(3) {
    ///     println!("{}: Risk Score = {:.1} ({})",
    ///         risk.company_name.as_deref().unwrap_or("N/A"),
    ///         risk.esg_risk_score.unwrap_or(0.0),
    ///         risk.esg_risk_level.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_esg_risk_rating(&self, symbol: &str) -> Result<Vec<EsgRiskRating>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url(&format!(
            "/esg-environmental-social-governance-data-ratings/{}",
            symbol
        ));
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get ESG benchmark data by industry
    ///
    /// Returns industry ESG benchmark scores for comparison.
    ///
    /// # Arguments
    /// * `year` - Year for benchmark data (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let benchmarks = client.esg().get_esg_sector_benchmark(Some("2023")).await?;
    /// for benchmark in benchmarks.iter().take(5) {
    ///     println!("{}: ESG Benchmark = {:.1}",
    ///         benchmark.industry.as_deref().unwrap_or("N/A"),
    ///         benchmark.esg_benchmark.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_esg_sector_benchmark(&self, year: Option<&str>) -> Result<Vec<EsgBenchmark>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            year: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/esg-sector-benchmark");
        self.client
            .get_with_query(
                &url,
                &Query {
                    year,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get Congressional trading data
    ///
    /// Returns stock transactions by US Congress members.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (optional, filters by specific stock)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let congress_trades = client.esg().get_senate_trading(None).await?;
    /// for trade in congress_trades.iter().take(10) {
    ///     println!("{}: {} {} {} ({})",
    ///         trade.transaction_date.as_deref().unwrap_or("N/A"),
    ///         trade.representative.as_deref().unwrap_or("N/A"),
    ///         trade.transaction.as_deref().unwrap_or("N/A"),
    ///         trade.ticker.as_deref().unwrap_or("N/A"),
    ///         trade.amount.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_senate_trading(
        &self,
        symbol: Option<&str>,
    ) -> Result<Vec<CongressionalTrading>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            symbol: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/senate-trading");
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

    /// Get House of Representatives trading data
    ///
    /// Returns stock transactions by US House members.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let house_trades = client.esg().get_house_disclosure(None).await?;
    /// for trade in house_trades.iter().take(10) {
    ///     println!("{}: {} {} {} ({})",
    ///         trade.transaction_date.as_deref().unwrap_or("N/A"),
    ///         trade.representative.as_deref().unwrap_or("N/A"),
    ///         trade.transaction.as_deref().unwrap_or("N/A"),
    ///         trade.ticker.as_deref().unwrap_or("N/A"),
    ///         trade.amount.as_deref().unwrap_or("N/A"));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_house_disclosure(
        &self,
        symbol: Option<&str>,
    ) -> Result<Vec<CongressionalTrading>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            symbol: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/senate-disclosure");
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

    /// Get social sentiment data
    ///
    /// Returns social media sentiment analysis for stocks.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let sentiment = client.esg().get_social_sentiment("GME").await?;
    /// for data in sentiment.iter().take(5) {
    ///     println!("{}: Sentiment = {} (Twitter: {:.2}, StockTwits: {:.2})",
    ///         data.date,
    ///         data.general_sentiment.as_deref().unwrap_or("N/A"),
    ///         data.twitter_sentiment.unwrap_or(0.0),
    ///         data.stocktwits_sentiment.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_social_sentiment(&self, symbol: &str) -> Result<Vec<SocialSentiment>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/social-sentiment/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Golden path tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_esg_score() {
        let client = FmpClient::new().unwrap();
        let result = client.esg().get_esg_score("AAPL").await;
        assert!(result.is_ok());
        let esg_scores = result.unwrap();
        assert!(!esg_scores.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_esg_risk_rating() {
        let client = FmpClient::new().unwrap();
        let result = client.esg().get_esg_risk_rating("TSLA").await;
        assert!(result.is_ok());
        let risk_ratings = result.unwrap();
        assert!(!risk_ratings.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_esg_sector_benchmark() {
        let client = FmpClient::new().unwrap();
        let result = client.esg().get_esg_sector_benchmark(None).await;
        assert!(result.is_ok());
        let benchmarks = result.unwrap();
        assert!(!benchmarks.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_senate_trading() {
        let client = FmpClient::new().unwrap();
        let result = client.esg().get_senate_trading(None).await;
        assert!(result.is_ok());
        let _trades = result.unwrap();
        // May be empty if no recent trades
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_social_sentiment() {
        let client = FmpClient::new().unwrap();
        let result = client.esg().get_social_sentiment("AAPL").await;
        assert!(result.is_ok());
        let _sentiment = result.unwrap();
        // Sentiment data may be limited
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_esg_with_year_filter() {
        let client = FmpClient::new().unwrap();
        let result = client.esg().get_esg_sector_benchmark(Some("2023")).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_senate_trading_with_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client.esg().get_senate_trading(Some("AAPL")).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_multiple_esg_symbols() {
        let client = FmpClient::new().unwrap();
        // Test ESG scores for different companies
        for symbol in &["AAPL", "TSLA", "MSFT"] {
            let result = client.esg().get_esg_score(symbol).await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_social_sentiment_validation() {
        let client = FmpClient::new().unwrap();
        let result = client.esg().get_social_sentiment("GME").await;
        assert!(result.is_ok());
        let sentiment = result.unwrap();

        // Validate sentiment scores are in expected ranges
        for data in sentiment.iter().take(3) {
            if let Some(twitter_sentiment) = data.twitter_sentiment {
                assert!((-1.0..=1.0).contains(&twitter_sentiment));
            }
            if let Some(stocktwits_sentiment) = data.stocktwits_sentiment {
                assert!((-1.0..=1.0).contains(&stocktwits_sentiment));
            }
        }
    }

    // Error handling tests
    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = FmpClient::builder()
            .api_key("invalid_key_12345")
            .build()
            .unwrap();
        let result = client.esg().get_esg_score("AAPL").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client.esg().get_esg_score("INVALIDSTOCK123").await;
        // Should return empty or error
        if let Ok(data) = result {
            assert!(data.is_empty())
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_house_disclosure() {
        let client = FmpClient::new().unwrap();
        let result = client.esg().get_house_disclosure(None).await;
        assert!(result.is_ok());
        let _trades = result.unwrap();
    }
}
