//! Technical Indicators endpoints

use crate::{client::FmpClient, error::Result, models::technical_indicators::*};
use serde::Serialize;

/// Technical Indicators API endpoints
pub struct TechnicalIndicators {
    client: FmpClient,
}

impl TechnicalIndicators {
    pub(crate) fn new(client: FmpClient) -> Self {
        Self { client }
    }

    /// Get Simple Moving Average (SMA)
    ///
    /// Returns SMA values for a symbol over specified period.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol (e.g., "AAPL")
    /// * `period` - Period for calculation (e.g., 20)
    /// * `from` - Start date (optional, format: YYYY-MM-DD)
    /// * `to` - End date (optional, format: YYYY-MM-DD)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let sma = client.technical_indicators().get_sma("AAPL", 20, None, None).await?;
    /// for point in sma.iter().take(5) {
    ///     println!("{}: SMA20 = {:.2}", point.date, point.sma.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_sma(
        &self,
        symbol: &str,
        period: u32,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<SmaData>> {
        #[derive(Serialize)]
        struct Query<'a> {
            period: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/technical_indicator/daily/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    period,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get Exponential Moving Average (EMA)
    ///
    /// Returns EMA values for a symbol over specified period.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `period` - Period for calculation
    /// * `from` - Start date (optional)
    /// * `to` - End date (optional)
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let ema = client.technical_indicators().get_ema("AAPL", 12, None, None).await?;
    /// for point in ema.iter().take(5) {
    ///     println!("{}: EMA12 = {:.2}", point.date, point.ema.unwrap_or(0.0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_ema(
        &self,
        symbol: &str,
        period: u32,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<EmaData>> {
        #[derive(Serialize)]
        struct Query<'a> {
            period: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/technical_indicator/ema/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    period,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get Weighted Moving Average (WMA)
    ///
    /// Returns WMA values for a symbol over specified period.
    ///
    /// # Arguments
    /// * `symbol` - Stock symbol
    /// * `period` - Period for calculation
    /// * `from` - Start date (optional)
    /// * `to` - End date (optional)
    pub async fn get_wma(
        &self,
        symbol: &str,
        period: u32,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<WmaData>> {
        #[derive(Serialize)]
        struct Query<'a> {
            period: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/technical_indicator/wma/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    period,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get Double Exponential Moving Average (DEMA)
    pub async fn get_dema(
        &self,
        symbol: &str,
        period: u32,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<DemaData>> {
        #[derive(Serialize)]
        struct Query<'a> {
            period: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/technical_indicator/dema/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    period,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get Triple Exponential Moving Average (TEMA)
    pub async fn get_tema(
        &self,
        symbol: &str,
        period: u32,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<TemaData>> {
        #[derive(Serialize)]
        struct Query<'a> {
            period: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/technical_indicator/tema/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    period,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get Relative Strength Index (RSI)
    ///
    /// Returns RSI values (0-100 scale) for momentum analysis.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let rsi = client.technical_indicators().get_rsi("AAPL", 14, None, None).await?;
    /// for point in rsi.iter().take(5) {
    ///     let rsi_val = point.rsi.unwrap_or(0.0);
    ///     let signal = if rsi_val > 70.0 { "Overbought" }
    ///                  else if rsi_val < 30.0 { "Oversold" }
    ///                  else { "Neutral" };
    ///     println!("{}: RSI = {:.1} ({})", point.date, rsi_val, signal);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_rsi(
        &self,
        symbol: &str,
        period: u32,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<RsiData>> {
        #[derive(Serialize)]
        struct Query<'a> {
            period: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/technical_indicator/rsi/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    period,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get Average Directional Index (ADX)
    pub async fn get_adx(
        &self,
        symbol: &str,
        period: u32,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<AdxData>> {
        #[derive(Serialize)]
        struct Query<'a> {
            period: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/technical_indicator/adx/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    period,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get standardized technical data
    pub async fn get_standardized(
        &self,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<StandardizedIndicator>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/technical_indicator/standardized/{}", symbol));
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

    /// Get Williams %R indicator
    pub async fn get_williams_r(
        &self,
        symbol: &str,
        period: u32,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<WilliamsRData>> {
        #[derive(Serialize)]
        struct Query<'a> {
            period: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/technical_indicator/williams/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    period,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get Commodity Channel Index (CCI)
    pub async fn get_cci(
        &self,
        symbol: &str,
        period: u32,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<CciData>> {
        #[derive(Serialize)]
        struct Query<'a> {
            period: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/technical_indicator/cci/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    period,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get MACD (Moving Average Convergence Divergence)
    ///
    /// Returns MACD line, signal line, and histogram for trend analysis.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let macd = client.technical_indicators().get_macd("AAPL", 12, 26, 9, None, None).await?;
    /// for point in macd.iter().take(5) {
    ///     if let (Some(macd_val), Some(signal)) = (point.macd, point.signal) {
    ///         let signal_type = if macd_val > signal { "Bullish" } else { "Bearish" };
    ///         println!("{}: MACD={:.3}, Signal={:.3} ({})",
    ///             point.date, macd_val, signal, signal_type);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_macd(
        &self,
        symbol: &str,
        fast_period: u32,
        slow_period: u32,
        signal_period: u32,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<MacdData>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(rename = "fastPeriod")]
            fast_period: u32,
            #[serde(rename = "slowPeriod")]
            slow_period: u32,
            #[serde(rename = "signalPeriod")]
            signal_period: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/technical_indicator/macd/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    fast_period,
                    slow_period,
                    signal_period,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get Stochastic Oscillator
    pub async fn get_stochastic(
        &self,
        symbol: &str,
        k_period: u32,
        d_period: u32,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<StochasticData>> {
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(rename = "kPeriod")]
            k_period: u32,
            #[serde(rename = "dPeriod")]
            d_period: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/technical_indicator/stoch/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    k_period,
                    d_period,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get Bollinger Bands
    ///
    /// Returns upper band, middle band (SMA), and lower band for volatility analysis.
    ///
    /// # Example
    /// ```no_run
    /// # use fmp_rs::FmpClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FmpClient::new()?;
    /// let bb = client.technical_indicators().get_bollinger_bands("AAPL", 20, 2.0, None, None).await?;
    /// for point in bb.iter().take(5) {
    ///     if let (Some(upper), Some(middle), Some(lower)) =
    ///         (point.upper_band, point.middle_band, point.lower_band) {
    ///         println!("{}: Upper={:.2}, Middle={:.2}, Lower={:.2}",
    ///             point.date, upper, middle, lower);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bollinger_bands(
        &self,
        symbol: &str,
        period: u32,
        std_dev: f64,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<BollingerBandsData>> {
        #[derive(Serialize)]
        struct Query<'a> {
            period: u32,
            #[serde(rename = "stdDev")]
            std_dev: f64,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/technical_indicator/bb/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    period,
                    std_dev,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get Average True Range (ATR)
    pub async fn get_atr(
        &self,
        symbol: &str,
        period: u32,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<AtrData>> {
        #[derive(Serialize)]
        struct Query<'a> {
            period: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/technical_indicator/atr/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    period,
                    from,
                    to,
                    apikey: self.client.api_key(),
                },
            )
            .await
    }

    /// Get Aroon Indicator
    pub async fn get_aroon(
        &self,
        symbol: &str,
        period: u32,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<AroonData>> {
        #[derive(Serialize)]
        struct Query<'a> {
            period: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            from: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            to: Option<&'a str>,
            apikey: &'a str,
        }

        let url = self
            .client
            .build_url(&format!("/technical_indicator/aroon/{}", symbol));
        self.client
            .get_with_query(
                &url,
                &Query {
                    period,
                    from,
                    to,
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
    async fn test_get_sma() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_sma("AAPL", 20, None, None)
            .await;
        assert!(result.is_ok());
        let sma_data = result.unwrap();
        assert!(!sma_data.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_ema() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_ema("AAPL", 12, None, None)
            .await;
        assert!(result.is_ok());
        let ema_data = result.unwrap();
        assert!(!ema_data.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_rsi() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_rsi("AAPL", 14, None, None)
            .await;
        assert!(result.is_ok());
        let rsi_data = result.unwrap();
        assert!(!rsi_data.is_empty());
        // RSI should be between 0 and 100
        for point in rsi_data.iter().take(5) {
            if let Some(rsi) = point.rsi {
                assert!(rsi >= 0.0 && rsi <= 100.0);
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_macd() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_macd("AAPL", 12, 26, 9, None, None)
            .await;
        assert!(result.is_ok());
        let macd_data = result.unwrap();
        assert!(!macd_data.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_bollinger_bands() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_bollinger_bands("AAPL", 20, 2.0, None, None)
            .await;
        assert!(result.is_ok());
        let bb_data = result.unwrap();
        assert!(!bb_data.is_empty());
        // Upper band should be >= lower band
        for point in bb_data.iter().take(5) {
            if let (Some(upper), Some(lower)) = (point.upper_band, point.lower_band) {
                assert!(upper >= lower);
            }
        }
    }

    // Edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_various_periods() {
        let client = FmpClient::new().unwrap();
        // Test different common periods
        for period in &[10, 20, 50, 200] {
            let result = client
                .technical_indicators()
                .get_sma("AAPL", *period, None, None)
                .await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_with_date_range() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_rsi("AAPL", 14, Some("2024-01-01"), Some("2024-01-31"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_multiple_indicators() {
        let client = FmpClient::new().unwrap();
        // Test multiple indicators for same symbol
        let sma_result = client
            .technical_indicators()
            .get_sma("MSFT", 20, None, None)
            .await;
        let ema_result = client
            .technical_indicators()
            .get_ema("MSFT", 20, None, None)
            .await;
        let rsi_result = client
            .technical_indicators()
            .get_rsi("MSFT", 14, None, None)
            .await;

        assert!(sma_result.is_ok());
        assert!(ema_result.is_ok());
        assert!(rsi_result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_stochastic_oscillator() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_stochastic("AAPL", 14, 3, None, None)
            .await;
        assert!(result.is_ok());
        let stoch_data = result.unwrap();
        assert!(!stoch_data.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_atr() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_atr("AAPL", 14, None, None)
            .await;
        assert!(result.is_ok());
        let atr_data = result.unwrap();
        assert!(!atr_data.is_empty());
        // ATR should be positive
        for point in atr_data.iter().take(5) {
            if let Some(atr) = point.atr {
                assert!(atr >= 0.0);
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
        let result = client
            .technical_indicators()
            .get_sma("AAPL", 20, None, None)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_invalid_symbol() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_rsi("INVALIDSTOCK123", 14, None, None)
            .await;
        // Should return empty or error
        match result {
            Ok(data) => assert!(data.is_empty()),
            Err(_) => {} // Error is acceptable for invalid symbol
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_extreme_periods() {
        let client = FmpClient::new().unwrap();
        // Test with very small period (should work)
        let result = client
            .technical_indicators()
            .get_sma("AAPL", 1, None, None)
            .await;
        assert!(result.is_ok());

        // Test with very large period (might return less data)
        let result = client
            .technical_indicators()
            .get_sma("AAPL", 500, None, None)
            .await;
        assert!(result.is_ok());
    }

    // Missing endpoint tests - adding comprehensive coverage for all indicators
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_wma() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_wma("AAPL", 20, None, None)
            .await;
        assert!(result.is_ok());
        let wma_data = result.unwrap();
        assert!(!wma_data.is_empty());
        // WMA should have valid values
        for point in wma_data.iter().take(5) {
            if let Some(wma) = point.wma {
                assert!(wma > 0.0); // Stock prices should be positive
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_dema() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_dema("AAPL", 20, None, None)
            .await;
        assert!(result.is_ok());
        let dema_data = result.unwrap();
        assert!(!dema_data.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_tema() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_tema("AAPL", 20, None, None)
            .await;
        assert!(result.is_ok());
        let tema_data = result.unwrap();
        assert!(!tema_data.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_adx() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_adx("AAPL", 14, None, None)
            .await;
        assert!(result.is_ok());
        let adx_data = result.unwrap();
        assert!(!adx_data.is_empty());
        // ADX should be between 0 and 100
        for point in adx_data.iter().take(5) {
            if let Some(adx) = point.adx {
                assert!(adx >= 0.0 && adx <= 100.0);
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_standardized() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_standardized("AAPL", None, None)
            .await;
        assert!(result.is_ok());
        let std_data = result.unwrap();
        assert!(!std_data.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_williams_r() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_williams_r("AAPL", 14, None, None)
            .await;
        assert!(result.is_ok());
        let wr_data = result.unwrap();
        assert!(!wr_data.is_empty());
        // Williams %R should be between -100 and 0
        for point in wr_data.iter().take(5) {
            if let Some(wr) = point.williams_r {
                assert!(wr >= -100.0 && wr <= 0.0);
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_cci() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_cci("AAPL", 20, None, None)
            .await;
        assert!(result.is_ok());
        let cci_data = result.unwrap();
        assert!(!cci_data.is_empty());
        // CCI can range widely but should be reasonable values
        for point in cci_data.iter().take(5) {
            if let Some(cci) = point.cci {
                assert!(cci > -1000.0 && cci < 1000.0); // Reasonable bounds
            }
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_get_aroon() {
        let client = FmpClient::new().unwrap();
        let result = client
            .technical_indicators()
            .get_aroon("AAPL", 14, None, None)
            .await;
        assert!(result.is_ok());
        let aroon_data = result.unwrap();
        assert!(!aroon_data.is_empty());
        // Aroon indicators should be between 0 and 100
        for point in aroon_data.iter().take(5) {
            if let Some(aroon_up) = point.aroon_up {
                assert!(aroon_up >= 0.0 && aroon_up <= 100.0);
            }
            if let Some(aroon_down) = point.aroon_down {
                assert!(aroon_down >= 0.0 && aroon_down <= 100.0);
            }
        }
    }

    // Comprehensive edge case tests
    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_indicator_parameter_validation() {
        let client = FmpClient::new().unwrap();

        // Test MACD with different parameter combinations
        let result = client
            .technical_indicators()
            .get_macd("AAPL", 5, 10, 3, None, None) // Fast < Slow
            .await;
        assert!(result.is_ok());

        // Test Bollinger Bands with different standard deviations
        let result = client
            .technical_indicators()
            .get_bollinger_bands("AAPL", 20, 1.0, None, None)
            .await;
        assert!(result.is_ok());

        let result = client
            .technical_indicators()
            .get_bollinger_bands("AAPL", 20, 3.0, None, None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_indicators_with_date_ranges() {
        let client = FmpClient::new().unwrap();

        // Test various indicators with specific date ranges
        let start_date = "2024-01-01";
        let end_date = "2024-02-01";

        // Test SMA with date range
        let sma_result = client
            .technical_indicators()
            .get_sma("AAPL", 20, Some(start_date), Some(end_date))
            .await;
        assert!(sma_result.is_ok());

        // Test EMA with date range
        let ema_result = client
            .technical_indicators()
            .get_ema("AAPL", 12, Some(start_date), Some(end_date))
            .await;
        assert!(ema_result.is_ok());

        // Test RSI with date range
        let rsi_result = client
            .technical_indicators()
            .get_rsi("AAPL", 14, Some(start_date), Some(end_date))
            .await;
        assert!(rsi_result.is_ok());

        // Test ATR with date range
        let atr_result = client
            .technical_indicators()
            .get_atr("AAPL", 14, Some(start_date), Some(end_date))
            .await;
        assert!(atr_result.is_ok());
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_invalid_date_ranges() {
        let client = FmpClient::new().unwrap();

        // Test invalid date format
        let result = client
            .technical_indicators()
            .get_sma("AAPL", 20, Some("invalid-date"), Some("2024-01-31"))
            .await;
        // Should handle gracefully
        match result {
            Ok(_) => {}  // API might ignore invalid dates
            Err(_) => {} // Or return an error
        }

        // Test end date before start date
        let result = client
            .technical_indicators()
            .get_sma("AAPL", 20, Some("2024-01-31"), Some("2024-01-01"))
            .await;
        match result {
            Ok(data) => assert!(data.is_empty()), // Should return empty
            Err(_) => {}                          // Or return an error
        }
    }

    #[tokio::test]
    #[ignore = "requires FMP API key"]
    async fn test_stochastic_parameter_validation() {
        let client = FmpClient::new().unwrap();

        // Test different k and d periods
        let result = client
            .technical_indicators()
            .get_stochastic("AAPL", 5, 3, None, None)
            .await;
        assert!(result.is_ok());

        let stoch_data = result.unwrap();
        if !stoch_data.is_empty() {
            // Stochastic values should be between 0 and 100
            for point in stoch_data.iter().take(5) {
                if let Some(k) = point.slow_k {
                    assert!(k >= 0.0 && k <= 100.0);
                }
                if let Some(d) = point.slow_d {
                    assert!(d >= 0.0 && d <= 100.0);
                }
            }
        }
    }
}
