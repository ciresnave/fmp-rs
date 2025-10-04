//! ETF and mutual fund models.

use serde::{Deserialize, Serialize};

/// ETF holding
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EtfHolding {
    pub asset: String,
    pub name: String,
    pub shares: i64,
    pub weight_percentage: f64,
    pub market_value: f64,
}

/// ETF information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EtfInfo {
    pub symbol: String,
    pub company_name: String,
    pub inception_date: String,
    pub cusip: String,
    pub isin: String,
    pub description: String,
    pub domicile: String,
    pub etf_company: String,
    pub aum: f64,
    pub nav: f64,
    pub nav_currency: String,
    pub expense_ratio: f64,
    pub holdings_count: i32,
    pub avg_volume: i64,
    pub website: String,
}

/// ETF sector weighting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SectorWeighting {
    pub sector: String,
    pub weight_percentage: String,
}

/// ETF country weighting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CountryWeighting {
    pub country: String,
    pub weight_percentage: String,
}

/// ETF holder (who holds the ETF)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EtfHolder {
    pub asset: String,
    pub name: String,
    pub shares: Option<i64>,
    pub weight_percentage: Option<f64>,
    pub market_value: Option<f64>,
    pub updated: Option<String>,
}

/// ETF search result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EtfSearchResult {
    pub symbol: String,
    pub name: String,
    pub currency: Option<String>,
    pub stock_exchange: Option<String>,
    pub exchange_short_name: Option<String>,
}

/// ETF basic information (from list endpoint)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EtfListItem {
    pub symbol: String,
    pub name: String,
    pub price: Option<f64>,
    pub exchange: Option<String>,
    pub exchange_short_name: Option<String>,
}
