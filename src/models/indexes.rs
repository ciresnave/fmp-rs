//! Models for indexes endpoints

use serde::{Deserialize, Serialize};

/// Market index symbol information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexSymbol {
    /// Symbol (e.g., "^GSPC", "^DJI", "^IXIC")
    pub symbol: String,
    /// Index name
    pub name: Option<String>,
    /// Currency
    pub currency: Option<String>,
    /// Stock exchange
    pub stock_exchange: Option<String>,
    /// Exchange short name
    pub exchange_short_name: Option<String>,
}

/// Market index quote
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexQuote {
    /// Symbol
    pub symbol: String,
    /// Name
    pub name: Option<String>,
    /// Current price/level
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

/// Historical index data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexHistorical {
    /// Date
    pub date: String,
    /// Open level
    pub open: f64,
    /// High level
    pub high: f64,
    /// Low level
    pub low: f64,
    /// Close level
    pub close: f64,
    /// Adjusted close
    pub adj_close: Option<f64>,
    /// Volume (if applicable)
    pub volume: Option<f64>,
}

/// Index constituent (component stock)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexConstituent {
    /// Stock symbol
    pub symbol: String,
    /// Company name
    pub name: Option<String>,
    /// Sector
    pub sector: Option<String>,
    /// Sub-sector
    pub sub_sector: Option<String>,
    /// Headquarters location
    pub headquarters_location: Option<String>,
    /// Date first added
    pub date_first_added: Option<String>,
    /// CIK number
    pub cik: Option<String>,
    /// Date founded
    pub founded: Option<String>,
}
