//! Institutional holdings and insider trading models.

use serde::{Deserialize, Serialize};

/// Institutional holder
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionalHolder {
    pub holder: String,
    pub shares: i64,
    pub date_reported: String,
    pub change: Option<i64>,
    pub change_percent: Option<f64>,
}

/// Institutional portfolio composition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionalPortfolioComposition {
    pub cik: String,
    pub date: String,
    pub filing_date: Option<String>,
    pub accepted_date: Option<String>,
    pub symbol: String,
    pub company_name: Option<String>,
    pub shares: i64,
    pub value: f64,
    pub change: Option<i64>,
    pub change_percent: Option<f64>,
    pub weight_percent: Option<f64>,
}

/// Institutional portfolio date (for RSS feed)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstitutionalPortfolioDate {
    pub cik: String,
    pub year: i32,
    pub quarter: i32,
}

/// Insider trading transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InsiderTrade {
    pub symbol: String,
    pub filing_date: String,
    pub transaction_date: String,
    pub reporting_name: String,
    pub transaction_type: String,
    pub securities_owned: f64,
    pub securities_transacted: f64,
    pub price: f64,
    pub security_name: String,
    pub link: Option<String>,
    pub acquired_disposed: Option<String>,
}

/// Insider trading RSS feed item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InsiderTradingRss {
    pub symbol: String,
    pub filing_date: String,
    pub company_name: Option<String>,
}

/// Fail to Deliver (FTD) data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FailToDeliver {
    pub symbol: String,
    pub date: String,
    pub quantity: i64,
    pub price: f64,
    pub cusip: Option<String>,
}

/// CIK mapping (CIK to company)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CikMapper {
    pub cik: String,
    pub name: String,
}
