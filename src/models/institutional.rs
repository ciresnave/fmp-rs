//! Institutional holdings and insider trading models.

use serde::{Deserialize, Serialize};

/// Insider trading transaction (the live `/stable/insider-trading/latest` shape)
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
    #[serde(rename = "url", alias = "link")]
    pub link: Option<String>,
    #[serde(rename = "acquisitionOrDisposition", alias = "acquiredDisposed")]
    pub acquired_disposed: Option<String>,
    pub company_cik: Option<String>,
    pub reporting_cik: Option<String>,
    pub direct_or_indirect: Option<String>,
    pub form_type: Option<String>,
    pub type_of_owner: Option<String>,
}

/// A listed company for a CIK (`/stable/search-cik`)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CikSearchResult {
    pub cik: String,
    pub company_name: String,
    pub symbol: String,
    pub currency: Option<String>,
    pub exchange: Option<String>,
    pub exchange_full_name: Option<String>,
}
