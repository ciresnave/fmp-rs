//! Models for cryptocurrency and forex endpoints

use serde::{Deserialize, Serialize};

/// Cryptocurrency information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoSymbol {
    /// Symbol (e.g., "BTCUSD")
    pub symbol: String,
    /// Currency name
    pub name: Option<String>,
    /// Currency code
    pub currency: Option<String>,
    /// Stock exchange
    pub stock_exchange: Option<String>,
    /// Exchange short name
    pub exchange_short_name: Option<String>,
}

/// Cryptocurrency quote
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoQuote {
    /// Symbol
    pub symbol: String,
    /// Name
    pub name: Option<String>,
    /// Current price
    pub price: Option<f64>,
    /// Price change
    pub change: Option<f64>,
    /// Percent change
    pub changes_percentage: Option<f64>,
    /// Day low
    pub day_low: Option<f64>,
    /// Day high
    pub day_high: Option<f64>,
    /// Year low
    pub year_low: Option<f64>,
    /// Year high
    pub year_high: Option<f64>,
    /// Market cap
    pub market_cap: Option<f64>,
    /// Volume
    pub volume: Option<f64>,
    /// Average volume
    pub avg_volume: Option<f64>,
    /// Open price
    pub open: Option<f64>,
    /// Previous close
    pub previous_close: Option<f64>,
    /// Timestamp
    pub timestamp: Option<i64>,
}

/// Historical cryptocurrency price
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoHistorical {
    /// Date
    pub date: String,
    /// Open price
    pub open: f64,
    /// High price
    pub high: f64,
    /// Low price
    pub low: f64,
    /// Close price
    pub close: f64,
    /// Adjusted close
    pub adj_close: Option<f64>,
    /// Volume
    pub volume: f64,
}

/// Intraday cryptocurrency price
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoIntraday {
    /// Date and time
    pub date: String,
    /// Open price
    pub open: f64,
    /// High price
    pub high: f64,
    /// Low price
    pub low: f64,
    /// Close price
    pub close: f64,
    /// Volume
    pub volume: f64,
}

/// Forex pair information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexPair {
    /// Symbol (e.g., "EUR/USD")
    pub symbol: String,
    /// Name
    pub name: Option<String>,
    /// Currency
    pub currency: Option<String>,
    /// Stock exchange
    pub stock_exchange: Option<String>,
    /// Exchange short name
    pub exchange_short_name: Option<String>,
}

/// Forex quote
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexQuote {
    /// Symbol
    pub symbol: String,
    /// Name
    pub name: Option<String>,
    /// Current price/rate
    pub price: Option<f64>,
    /// Price change
    pub change: Option<f64>,
    /// Percent change
    pub changes_percentage: Option<f64>,
    /// Day low
    pub day_low: Option<f64>,
    /// Day high
    pub day_high: Option<f64>,
    /// Year low
    pub year_low: Option<f64>,
    /// Year high
    pub year_high: Option<f64>,
    /// Open price
    pub open: Option<f64>,
    /// Previous close
    pub previous_close: Option<f64>,
    /// Timestamp
    pub timestamp: Option<i64>,
}

/// Historical forex rate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexHistorical {
    /// Date
    pub date: String,
    /// Open rate
    pub open: f64,
    /// High rate
    pub high: f64,
    /// Low rate
    pub low: f64,
    /// Close rate
    pub close: f64,
    /// Adjusted close
    pub adj_close: Option<f64>,
}

/// Intraday forex rate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexIntraday {
    /// Date and time
    pub date: String,
    /// Open rate
    pub open: f64,
    /// High rate
    pub high: f64,
    /// Low rate
    pub low: f64,
    /// Close rate
    pub close: f64,
}
