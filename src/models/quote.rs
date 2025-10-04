//! Quote data models.

use serde::{Deserialize, Serialize};

/// Full stock quote
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StockQuote {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub changes_percentage: f64,
    pub change: f64,
    pub day_low: f64,
    pub day_high: f64,
    pub year_high: f64,
    pub year_low: f64,
    pub market_cap: Option<i64>,
    pub price_avg50: f64,
    pub price_avg200: f64,
    pub exchange: String,
    pub volume: i64,
    pub avg_volume: i64,
    pub open: f64,
    pub previous_close: f64,
    pub eps: Option<f64>,
    pub pe: Option<f64>,
    pub earnings_announcement: Option<String>,
    pub shares_outstanding: Option<i64>,
    pub timestamp: i64,
}

/// Short-form quote with essential information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShortQuote {
    pub symbol: String,
    pub price: f64,
    pub volume: i64,
}

/// Price change data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PriceChange {
    pub symbol: String,
    #[serde(rename = "1D")]
    pub one_day: Option<f64>,
    #[serde(rename = "5D")]
    pub five_day: Option<f64>,
    #[serde(rename = "1M")]
    pub one_month: Option<f64>,
    #[serde(rename = "3M")]
    pub three_month: Option<f64>,
    #[serde(rename = "6M")]
    pub six_month: Option<f64>,
    #[serde(rename = "ytd")]
    pub year_to_date: Option<f64>,
    #[serde(rename = "1Y")]
    pub one_year: Option<f64>,
    #[serde(rename = "3Y")]
    pub three_year: Option<f64>,
    #[serde(rename = "5Y")]
    pub five_year: Option<f64>,
    #[serde(rename = "10Y")]
    pub ten_year: Option<f64>,
    #[serde(rename = "max")]
    pub max: Option<f64>,
}
