//! Historical price chart models.

use serde::{Deserialize, Serialize};

/// Historical price data (EOD - End of Day)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalPrice {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub adj_close: f64,
    pub volume: i64,
    pub unadjusted_volume: i64,
    pub change: f64,
    pub change_percent: f64,
    pub vwap: f64,
    pub label: String,
    pub change_over_time: f64,
}

/// Intraday price data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntradayPrice {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
}

/// Historical dividend data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalDividend {
    pub date: String,
    pub label: String,
    pub adj_dividend: f64,
    pub dividend: f64,
    pub record_date: Option<String>,
    pub payment_date: Option<String>,
    pub declaration_date: Option<String>,
}

/// Stock split data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StockSplit {
    pub date: String,
    pub label: String,
    pub numerator: f64,
    pub denominator: f64,
}
