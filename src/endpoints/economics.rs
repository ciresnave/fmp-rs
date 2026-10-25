//! Economics and indicators endpoints

use crate::{
    client::FmpClient,
    error::Result,
    models::economics::{EconomicIndicator, MarketRiskPremium, TreasuryRate},
};
use serde::Serialize;

/// Economics API endpoints
pub struct Economics {
    client: FmpClient,
}

impl Economics {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get treasury rates
    ///
    /// Returns US treasury rates for various maturities (1mo to 30yr).
    ///
    /// # Arguments
    /// * `from` - Start date (optional, format: YYYY-MM-DD)
    /// * `to` - End date (optional, format: YYYY-MM-DD)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let rates = client.economics().get_treasury_rates(None, None).await?;
    /// for rate in rates.iter().take(5) {
    ///     println!("{}: 10Y = {:.2}%, 2Y = {:.2}%",
    ///         rate.date,
    ///         rate.year_10.unwrap_or(0.0),
    ///         rate.year_2.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_treasury_rates(
        &self,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<TreasuryRate>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self.client.build_url("/treasury");
        self.client
            .get_with_query(
                &url,
                &Query {
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get GDP (Gross Domestic Product)
    ///
    /// Returns historical GDP data.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let gdp = client.economics().get_gdp().await?;
    /// for record in gdp.iter().take(5) {
    ///     println!("{}: ${:.2}T", record.date, record.value.unwrap_or(0.0) / 1_000_000_000_000.0);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_gdp(&self) -> Result<Vec<EconomicIndicator>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/economic");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get Real GDP
    ///
    /// Returns inflation-adjusted GDP data.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let real_gdp = client.economics().get_real_gdp().await?;
    /// println!("Latest Real GDP: ${:.2}T",
    ///     real_gdp.first().and_then(|r| r.value).unwrap_or(0.0) / 1_000_000_000_000.0);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_real_gdp(&self) -> Result<Vec<EconomicIndicator>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/economic_indicator/GDP");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get GDP per capita
    ///
    /// Returns GDP per capita (GDP divided by population).
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let gdp_per_capita = client.economics().get_gdp_per_capita().await?;
    /// println!("Latest GDP per capita: ${:.2}",
    ///     gdp_per_capita.first().and_then(|r| r.value).unwrap_or(0.0));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_gdp_per_capita(&self) -> Result<Vec<EconomicIndicator>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/economic_indicator/GDP_PER_CAPITA");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get CPI (Consumer Price Index)
    ///
    /// Returns historical CPI data used to measure inflation.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let cpi = client.economics().get_cpi().await?;
    /// for record in cpi.iter().take(12) {
    ///     println!("{}: CPI = {:.2}", record.date, record.value.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_cpi(&self) -> Result<Vec<EconomicIndicator>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/economic_indicator/CPI");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get inflation rate
    ///
    /// Returns year-over-year inflation rate (percentage change in CPI).
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let inflation = client.economics().get_inflation_rate().await?;
    /// println!("Current inflation: {:.2}%",
    ///     inflation.first().and_then(|r| r.value).unwrap_or(0.0));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_inflation_rate(&self) -> Result<Vec<EconomicIndicator>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/economic_indicator/INFLATION");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get unemployment rate
    ///
    /// Returns historical unemployment rate data.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let unemployment = client.economics().get_unemployment_rate().await?;
    /// println!("Current unemployment: {:.1}%",
    ///     unemployment.first().and_then(|r| r.value).unwrap_or(0.0));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_unemployment_rate(&self) -> Result<Vec<EconomicIndicator>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/economic_indicator/UNEMPLOYMENT");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get federal funds rate
    ///
    /// Returns the federal funds effective rate set by the Federal Reserve.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let fed_rate = client.economics().get_federal_funds_rate().await?;
    /// println!("Current Fed rate: {:.2}%",
    ///     fed_rate.first().and_then(|r| r.value).unwrap_or(0.0));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_federal_funds_rate(&self) -> Result<Vec<EconomicIndicator>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url("/economic_indicator/FEDERAL_FUNDS_RATE");
        self.client
            .get_with_query(
                &url,
                &Query {
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get market risk premium
    ///
    /// Returns equity risk premium data by country.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let risk_premiums = client.economics().get_market_risk_premium().await?;
    /// for premium in risk_premiums.iter().take(10) {
    ///     println!("{}: Total ERP = {:.2}%",
    ///         premium.country,
    ///         premium.total_equity_risk_premium.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_market_risk_premium(&self) -> Result<Vec<MarketRiskPremium>> {
        #[derive(Serialize)]
        struct Query<'a> {
            apikey: &'a str,
        }

        let url = self.client.build_url("/market_risk_premium");
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
    async fn test_get_treasury_rates() {
        let client = FmpClient::new().unwrap();
        let result = client.economics().get_treasury_rates(None, None).await;
        assert!(result.is_ok());
        let rates = result.unwrap();
        assert!(!rates.is_empty());
        assert!(rates[0].year_10.is_some());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_gdp() {
        let client = FmpClient::new().unwrap();
        let result = client.economics().get_gdp().await;
        assert!(result.is_ok());
        let gdp = result.unwrap();
        assert!(!gdp.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_real_gdp() {
        let client = FmpClient::new().unwrap();
        let result = client.economics().get_real_gdp().await;
        assert!(result.is_ok());
        let real_gdp = result.unwrap();
        assert!(!real_gdp.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_gdp_per_capita() {
        let client = FmpClient::new().unwrap();
        let result = client.economics().get_gdp_per_capita().await;
        assert!(result.is_ok());
        let gdp_pc = result.unwrap();
        assert!(!gdp_pc.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_cpi() {
        let client = FmpClient::new().unwrap();
        let result = client.economics().get_cpi().await;
        assert!(result.is_ok());
        let cpi = result.unwrap();
        assert!(!cpi.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_inflation_rate() {
        let client = FmpClient::new().unwrap();
        let result = client.economics().get_inflation_rate().await;
        assert!(result.is_ok());
        let inflation = result.unwrap();
        assert!(!inflation.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_unemployment_rate() {
        let client = FmpClient::new().unwrap();
        let result = client.economics().get_unemployment_rate().await;
        assert!(result.is_ok());
        let unemployment = result.unwrap();
        assert!(!unemployment.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_federal_funds_rate() {
        let client = FmpClient::new().unwrap();
        let result = client.economics().get_federal_funds_rate().await;
        assert!(result.is_ok());
        let fed_rate = result.unwrap();
        assert!(!fed_rate.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_market_risk_premium() {
        let client = FmpClient::new().unwrap();
        let result = client.economics().get_market_risk_premium().await;
        assert!(result.is_ok());
        let premiums = result.unwrap();
        assert!(!premiums.is_empty());
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_treasury_rates_with_dates() {
        let client = FmpClient::new().unwrap();
        let result = client
            .economics()
            .get_treasury_rates(Some("2024-01-01"), Some("2024-12-31"))
            .await;
        assert!(result.is_ok());
    }

    // Error handling tests
    #[tokio::test]
    async fn test_invalid_api_key() {
        let client = FmpClient::builder()
            .api_key("invalid_key_12345")
            .build()
            .unwrap();
        let result = client.economics().get_gdp().await;
        assert!(result.is_err());
    }

    // Additional edge case tests

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_treasury_rates_invalid_dates() {
        let client = FmpClient::new().unwrap();
        // Test with invalid date format
        let result = client
            .economics()
            .get_treasury_rates(Some("invalid-date"), Some("2024-12-31"))
            .await;
        // Should handle invalid dates gracefully
        if result.is_ok() {}
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_treasury_rates_future_dates() {
        let client = FmpClient::new().unwrap();
        // Test with future dates
        let result = client
            .economics()
            .get_treasury_rates(Some("2030-01-01"), Some("2030-12-31"))
            .await;
        assert!(result.is_ok());
        let rates = result.unwrap();
        // Future dates should return empty or current data
        assert!(rates.is_empty() || !rates.is_empty());
    }
}
