//! Models for bulk data endpoints

use serde::{Deserialize, Serialize};

/// Bulk stock prices data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkStockPrice {
    /// Stock symbol
    pub symbol: Option<String>,
    /// Company name
    pub name: Option<String>,
    /// Current price
    pub price: Option<f64>,
    /// Price change
    pub change: Option<f64>,
    /// Percentage change
    pub changes_percentage: Option<f64>,
    /// Day low
    pub day_low: Option<f64>,
    /// Day high
    pub day_high: Option<f64>,
    /// Year low
    pub year_low: Option<f64>,
    /// Year high
    pub year_high: Option<f64>,
    /// Market capitalization
    pub market_cap: Option<f64>,
    /// Price to earnings ratio
    pub pe: Option<f64>,
    /// Volume
    pub volume: Option<i64>,
    /// Average volume
    pub avg_volume: Option<i64>,
    /// Exchange
    pub exchange: Option<String>,
    /// Open price
    pub open: Option<f64>,
    /// Previous close
    pub previous_close: Option<f64>,
    /// Earnings per share
    pub eps: Option<f64>,
    /// Shares outstanding
    pub shares_outstanding: Option<i64>,
    /// Timestamp
    pub timestamp: Option<i64>,
}

/// Bulk financial statements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkFinancialStatement {
    /// Stock symbol
    pub symbol: Option<String>,
    /// Date of the financial statement
    pub date: Option<String>,
    /// Calendar year
    pub calendar_year: Option<String>,
    /// Period (annual, quarterly)
    pub period: Option<String>,
    /// Revenue
    pub revenue: Option<f64>,
    /// Cost of revenue
    pub cost_of_revenue: Option<f64>,
    /// Gross profit
    pub gross_profit: Option<f64>,
    /// Operating income
    pub operating_income: Option<f64>,
    /// Net income
    pub net_income: Option<f64>,
    /// Total assets
    pub total_assets: Option<f64>,
    /// Total liabilities
    pub total_liabilities: Option<f64>,
    /// Total stockholders equity
    pub total_stockholders_equity: Option<f64>,
    /// Operating cash flow
    pub operating_cash_flow: Option<f64>,
    /// Free cash flow
    pub free_cash_flow: Option<f64>,
    /// Total debt
    pub total_debt: Option<f64>,
    /// Cash and cash equivalents
    pub cash_and_cash_equivalents: Option<f64>,
}

/// Bulk ETF holdings data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkEtfHolding {
    /// ETF symbol
    pub etf_symbol: Option<String>,
    /// Holding symbol
    pub asset_symbol: Option<String>,
    /// Asset name
    pub name: Option<String>,
    /// Number of shares held
    pub shares_number: Option<f64>,
    /// Market value of holding
    pub market_value: Option<f64>,
    /// Weight in the ETF
    pub weight_percentage: Option<f64>,
    /// Updated date
    pub updated: Option<String>,
}

/// Bulk insider trading data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkInsiderTrade {
    /// Filing date
    pub filing_date: Option<String>,
    /// Transaction date
    pub transaction_date: Option<String>,
    /// Reporting CIK
    pub reporting_cik: Option<String>,
    /// Transaction type
    pub transaction_type: Option<String>,
    /// Securities owned
    pub securities_owned: Option<f64>,
    /// Company CIK
    pub company_cik: Option<String>,
    /// Company name
    pub company_name: Option<String>,
    /// Securities transacted
    pub securities_transacted: Option<f64>,
    /// Price
    pub price: Option<f64>,
    /// Security name
    pub security_name: Option<String>,
    /// CIK
    pub cik: Option<String>,
    /// Symbol
    pub symbol: Option<String>,
    /// Transaction value
    pub transaction_value: Option<f64>,
}

/// Bulk institutional holdings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkInstitutionalHolding {
    /// Filing date
    pub date_reported: Option<String>,
    /// CIK of the institution
    pub cik: Option<String>,
    /// Institution name
    pub name_of_issuer: Option<String>,
    /// Ticker symbol
    pub ticker_symbol: Option<String>,
    /// CUSIP
    pub cusip: Option<String>,
    /// Value of holding
    pub value: Option<f64>,
    /// Number of shares
    pub shares: Option<f64>,
    /// Put/call indicator
    pub put_call: Option<String>,
    /// Investment discretion
    pub investment_discretion: Option<String>,
    /// Sole voting authority
    pub sole_voting_authority: Option<f64>,
    /// Shared voting authority
    pub shared_voting_authority: Option<f64>,
    /// No voting authority
    pub no_voting_authority: Option<f64>,
}

/// Bulk earnings estimates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkEarningsEstimate {
    /// Symbol
    pub symbol: Option<String>,
    /// Date
    pub date: Option<String>,
    /// Estimated revenue low
    pub estimated_revenue_low: Option<f64>,
    /// Estimated revenue high
    pub estimated_revenue_high: Option<f64>,
    /// Estimated revenue avg
    pub estimated_revenue_avg: Option<f64>,
    /// Estimated EPS avg
    pub estimated_eps_avg: Option<f64>,
    /// Estimated EPS high
    pub estimated_eps_high: Option<f64>,
    /// Estimated EPS low
    pub estimated_eps_low: Option<f64>,
    /// Number of analysts
    pub number_analyst_estimated_revenue: Option<i32>,
    /// Number of EPS analysts
    pub number_analyst_estimated_eps: Option<i32>,
}

/// Bulk download information/metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkDataInfo {
    /// Dataset name
    pub dataset: Option<String>,
    /// Last updated timestamp
    pub last_updated: Option<String>,
    /// File size in bytes
    pub file_size: Option<i64>,
    /// Number of records
    pub record_count: Option<i64>,
    /// Download URL
    pub download_url: Option<String>,
    /// Format (CSV, JSON, etc.)
    pub format: Option<String>,
    /// Compression type
    pub compression: Option<String>,
    /// Schema version
    pub schema_version: Option<String>,
}

/// Bulk historical prices metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkHistoricalPricesMeta {
    /// Date range start
    pub date_from: Option<String>,
    /// Date range end
    pub date_to: Option<String>,
    /// Symbols included count
    pub symbols_count: Option<i32>,
    /// Total data points
    pub total_records: Option<i64>,
    /// File format
    pub file_format: Option<String>,
    /// Estimated download size
    pub estimated_size_mb: Option<f64>,
}
