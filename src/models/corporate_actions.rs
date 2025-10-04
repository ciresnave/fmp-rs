//! Models for corporate actions endpoints

use serde::{Deserialize, Serialize};

/// Earnings calendar entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsCalendar {
    /// Earnings date
    pub date: Option<String>,
    /// Stock symbol
    pub symbol: String,
    /// EPS estimate
    pub eps_estimated: Option<f64>,
    /// Actual EPS
    pub eps: Option<f64>,
    /// Time of earnings release
    pub time: Option<String>,
    /// Revenue estimate
    pub revenue_estimated: Option<f64>,
    /// Actual revenue
    pub revenue: Option<f64>,
    /// Fiscal date ending
    pub fiscal_date_ending: Option<String>,
    /// Updated from date
    pub updated_from_date: Option<String>,
}

/// Historical earnings data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalEarnings {
    /// Earnings date
    pub date: Option<String>,
    /// Stock symbol
    pub symbol: String,
    /// Actual EPS
    pub eps: Option<f64>,
    /// EPS estimate
    pub eps_estimated: Option<f64>,
    /// Time of earnings release
    pub time: Option<String>,
    /// Actual revenue
    pub revenue: Option<f64>,
    /// Revenue estimate
    pub revenue_estimated: Option<f64>,
    /// Fiscal date ending
    pub fiscal_date_ending: Option<String>,
}

/// IPO calendar entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpoCalendar {
    /// IPO date
    pub date: Option<String>,
    /// Company name
    pub company: Option<String>,
    /// Stock symbol
    pub symbol: String,
    /// Exchange
    pub exchange: Option<String>,
    /// Number of shares
    pub number_of_shares: Option<i64>,
    /// Price range
    pub price: Option<String>,
    /// Market cap estimate
    pub market_cap: Option<String>,
}

/// Stock split calendar entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockSplitCalendar {
    /// Split date
    pub date: Option<String>,
    /// Stock symbol
    pub symbol: String,
    /// Split ratio (e.g., "2:1")
    pub split: Option<String>,
    /// Company name
    pub label: Option<String>,
    /// Old share price
    pub old_price: Option<f64>,
    /// New share price
    pub new_price: Option<f64>,
}

/// Dividend calendar entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DividendCalendar {
    /// Declaration date
    pub date: Option<String>,
    /// Stock symbol
    pub symbol: String,
    /// Dividend amount
    pub dividend: Option<f64>,
    /// Adjusted dividend
    pub adj_dividend: Option<f64>,
    /// Payment date
    pub payment_date: Option<String>,
    /// Record date
    pub record_date: Option<String>,
    /// Declaration date
    pub declaration_date: Option<String>,
    /// Company name
    pub label: Option<String>,
}

/// Economic calendar event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EconomicCalendar {
    /// Event date
    pub date: Option<String>,
    /// Country
    pub country: Option<String>,
    /// Event name
    pub event: Option<String>,
    /// Currency
    pub currency: Option<String>,
    /// Previous value
    pub previous: Option<String>,
    /// Estimate/forecast
    pub estimate: Option<String>,
    /// Actual value
    pub actual: Option<String>,
    /// Impact level (High/Medium/Low)
    pub impact: Option<String>,
    /// Change in value
    pub change: Option<f64>,
    /// Percentage change
    pub change_percentage: Option<f64>,
}

/// Earnings confirmation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsConfirmation {
    /// Stock symbol
    pub symbol: String,
    /// Exchange
    pub exchange: Option<String>,
    /// Confirmation status
    pub when: Option<String>,
    /// Publication date
    pub publication_date: Option<String>,
    /// Company name
    pub title: Option<String>,
    /// URL
    pub url: Option<String>,
}
