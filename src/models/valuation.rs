//! Models for valuation and DCF endpoints

use serde::{Deserialize, Serialize};

/// DCF (Discounted Cash Flow) valuation model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dcf {
    /// Stock symbol
    pub symbol: String,
    /// DCF valuation date
    pub date: String,
    /// Calculated DCF value
    pub dcf: Option<f64>,
    /// Stock price at calculation time
    pub stock_price: Option<f64>,
}

/// Historical DCF valuation over time
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalDcf {
    /// Stock symbol
    pub symbol: String,
    /// Calculation date
    pub date: String,
    /// DCF value
    pub dcf: Option<f64>,
    /// Stock price at that date
    pub stock_price: Option<f64>,
}

/// Advanced DCF model with detailed assumptions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedDcf {
    /// Stock symbol
    pub symbol: String,
    /// Calculation date
    pub date: String,
    /// DCF value
    pub dcf: Option<f64>,
    /// Stock price
    pub stock_price: Option<f64>,
    /// Revenue growth rate
    pub revenue_growth: Option<f64>,
    /// Operating margin
    pub operating_margin: Option<f64>,
    /// Tax rate
    pub tax_rate: Option<f64>,
    /// WACC (Weighted Average Cost of Capital)
    pub wacc: Option<f64>,
    /// Terminal growth rate
    pub terminal_growth_rate: Option<f64>,
    /// Free cash flow projections
    #[serde(default)]
    pub free_cash_flows: Vec<f64>,
}

/// Levered DCF (includes debt in valuation)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeveredDcf {
    /// Stock symbol
    pub symbol: String,
    /// Calculation date
    pub date: String,
    /// Levered DCF value (equity value)
    pub levered_dcf: Option<f64>,
    /// Stock price
    pub stock_price: Option<f64>,
    /// Enterprise value
    pub enterprise_value: Option<f64>,
    /// Total debt
    pub total_debt: Option<f64>,
    /// Cash and equivalents
    pub cash_and_equivalents: Option<f64>,
}

/// Company Enterprise Value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnterpriseValue {
    /// Stock symbol
    pub symbol: String,
    /// Date
    pub date: String,
    /// Stock price
    pub stock_price: Option<f64>,
    /// Number of shares
    pub number_of_shares: Option<f64>,
    /// Market capitalization
    pub market_capitalization: Option<f64>,
    /// Minus cash and cash equivalents
    pub minus_cash_and_cash_equivalents: Option<f64>,
    /// Plus total debt
    pub add_total_debt: Option<f64>,
    /// Enterprise value
    pub enterprise_value: Option<f64>,
}
