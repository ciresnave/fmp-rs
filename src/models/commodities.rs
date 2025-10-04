//! Models for commodities endpoints

use serde::{Deserialize, Serialize};

/// Commodity symbol information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommoditySymbol {
    /// Symbol (e.g., "GCUSD", "CLUSD")
    pub symbol: String,
    /// Commodity name
    pub name: Option<String>,
    /// Currency
    pub currency: Option<String>,
    /// Stock exchange
    pub stock_exchange: Option<String>,
    /// Exchange short name
    pub exchange_short_name: Option<String>,
}

/// Commodity quote
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommodityQuote {
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

/// Historical commodity price
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommodityHistorical {
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

/// Intraday commodity price
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommodityIntraday {
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
