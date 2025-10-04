//! Models for SEC filings endpoints

use serde::{Deserialize, Serialize};

/// SEC Filing information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecFiling {
    /// Stock symbol
    pub symbol: String,
    /// CIK number
    pub cik: Option<String>,
    /// Accepted date
    pub accepted_date: Option<String>,
    /// Filing date
    pub filing_date: Option<String>,
    /// Report date
    pub report_date: Option<String>,
    /// Form type (e.g., "10-K", "10-Q", "8-K")
    #[serde(rename = "type")]
    pub filing_type: Option<String>,
    /// Document link
    pub link: Option<String>,
    /// Final link
    pub final_link: Option<String>,
}

/// RSS Feed item for SEC filings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecFilingRss {
    /// Filing title
    pub title: Option<String>,
    /// Publication date
    pub date: Option<String>,
    /// Link to filing
    pub link: Option<String>,
    /// Stock symbol
    pub symbol: Option<String>,
    /// CIK number
    pub cik: Option<String>,
    /// Form type
    pub form_type: Option<String>,
}

/// Form 13F filing data (institutional holdings)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form13F {
    /// Filing date
    pub date: Option<String>,
    /// Date submitted
    pub date_submitted: Option<String>,
    /// CIK number
    pub cik: Option<String>,
    /// Cusip
    pub cusip: Option<String>,
    /// Stock symbol
    pub symbol: Option<String>,
    /// Company name
    pub name_of_issuer: Option<String>,
    /// Number of shares
    pub shares: Option<i64>,
    /// Market value
    pub market_value: Option<f64>,
    /// Share price calculation
    pub share_price_calc: Option<f64>,
}

/// Insider ownership from Form 3, 4, 5
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsiderOwnership {
    /// Filing date
    pub filing_date: Option<String>,
    /// Transaction date
    pub transaction_date: Option<String>,
    /// Stock symbol
    pub symbol: String,
    /// CIK number
    pub cik: Option<String>,
    /// Reporting owner name
    pub reporting_name: Option<String>,
    /// Transaction type
    pub transaction_type: Option<String>,
    /// Securities owned
    pub securities_owned: Option<i64>,
    /// Securities transacted
    pub securities_transacted: Option<i64>,
    /// Transaction price per share
    pub price: Option<f64>,
    /// Link to filing
    pub link: Option<String>,
}

/// Form 4 filing (Statement of Changes in Beneficial Ownership)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form4 {
    /// Accepted date
    pub accepted_date: Option<String>,
    /// Filing date
    pub filing_date: Option<String>,
    /// Transaction date
    pub transaction_date: Option<String>,
    /// Stock symbol
    pub symbol: String,
    /// CIK number
    pub cik: Option<String>,
    /// Reporting name
    pub reporting_name: Option<String>,
    /// Transaction type
    pub transaction_type: Option<String>,
    /// Securities transacted
    pub securities_transacted: Option<f64>,
    /// Transaction price
    pub price: Option<f64>,
    /// Securities owned
    pub securities_owned: Option<f64>,
    /// Link to filing
    pub link: Option<String>,
}

/// CIK search result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CikSearch {
    /// CIK number
    pub cik: String,
    /// Company name
    pub name: Option<String>,
}

/// Company name by CIK
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyName {
    /// CIK number
    pub cik: String,
    /// Company name
    pub name: String,
}
