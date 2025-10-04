//! Models for mutual funds endpoints

use serde::{Deserialize, Serialize};

/// Mutual fund information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutualFund {
    /// Fund symbol
    pub symbol: Option<String>,
    /// Fund name
    pub name: Option<String>,
    /// Fund family/company
    pub fund_family: Option<String>,
    /// Fund type
    pub fund_type: Option<String>,
    /// Investment category
    pub category: Option<String>,
    /// Investment objective
    pub investment_objective: Option<String>,
    /// Net asset value
    pub nav: Option<f64>,
    /// Previous NAV
    pub previous_nav: Option<f64>,
    /// NAV change
    pub change: Option<f64>,
    /// Percentage change
    pub change_percent: Option<f64>,
    /// Total assets (AUM)
    pub total_assets: Option<f64>,
    /// Expense ratio
    pub expense_ratio: Option<f64>,
    /// Minimum investment
    pub minimum_investment: Option<f64>,
    /// Yield
    pub yield_: Option<f64>,
    /// Inception date
    pub inception_date: Option<String>,
    /// Manager name
    pub manager_name: Option<String>,
    /// Currency
    pub currency: Option<String>,
    /// Exchange
    pub exchange: Option<String>,
    /// Last updated
    pub last_updated: Option<String>,
}

/// Mutual fund historical price data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutualFundHistorical {
    /// Date
    pub date: Option<String>,
    /// Net Asset Value (NAV)
    pub nav: Option<f64>,
    /// Adjusted NAV
    pub adj_nav: Option<f64>,
    /// Change from previous day
    pub change: Option<f64>,
    /// Percentage change
    pub change_percent: Option<f64>,
    /// Trading volume
    pub volume: Option<i64>,
    /// Symbol
    pub symbol: Option<String>,
}

/// Mutual fund holdings information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutualFundHolding {
    /// Holding symbol
    pub symbol: Option<String>,
    /// Security name
    pub name: Option<String>,
    /// Sector
    pub sector: Option<String>,
    /// Industry
    pub industry: Option<String>,
    /// Country
    pub country: Option<String>,
    /// Weight in portfolio (percentage)
    pub weight_percentage: Option<f64>,
    /// Market value
    pub market_value: Option<f64>,
    /// Number of shares held
    pub shares_held: Option<i64>,
    /// Asset type (stock, bond, etc.)
    pub asset_type: Option<String>,
    /// CUSIP identifier
    pub cusip: Option<String>,
    /// ISIN identifier
    pub isin: Option<String>,
    /// Date of holdings data
    pub date: Option<String>,
}

/// Mutual fund performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutualFundPerformance {
    /// Fund symbol
    pub symbol: Option<String>,
    /// Fund name
    pub name: Option<String>,
    /// 1 day return
    pub one_day: Option<f64>,
    /// 1 week return
    pub one_week: Option<f64>,
    /// 1 month return
    pub one_month: Option<f64>,
    /// 3 months return
    pub three_months: Option<f64>,
    /// 6 months return
    pub six_months: Option<f64>,
    /// Year to date return
    pub ytd: Option<f64>,
    /// 1 year return
    pub one_year: Option<f64>,
    /// 3 years return (annualized)
    pub three_years: Option<f64>,
    /// 5 years return (annualized)
    pub five_years: Option<f64>,
    /// 10 years return (annualized)
    pub ten_years: Option<f64>,
    /// Since inception return (annualized)
    pub since_inception: Option<f64>,
    /// Standard deviation (risk measure)
    pub standard_deviation: Option<f64>,
    /// Sharpe ratio
    pub sharpe_ratio: Option<f64>,
    /// Alpha
    pub alpha: Option<f64>,
    /// Beta
    pub beta: Option<f64>,
    /// R-squared
    pub r_squared: Option<f64>,
    /// Tracking error
    pub tracking_error: Option<f64>,
    /// Information ratio
    pub information_ratio: Option<f64>,
    /// Benchmark symbol
    pub benchmark: Option<String>,
    /// Performance date
    pub date: Option<String>,
}

/// Mutual fund sector allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutualFundSector {
    /// Sector name
    pub sector: Option<String>,
    /// Weight percentage in fund
    pub weight_percentage: Option<f64>,
    /// Market value
    pub market_value: Option<f64>,
    /// Date
    pub date: Option<String>,
}

/// Mutual fund country allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutualFundCountry {
    /// Country name
    pub country: Option<String>,
    /// Weight percentage in fund
    pub weight_percentage: Option<f64>,
    /// Market value
    pub market_value: Option<f64>,
    /// Date
    pub date: Option<String>,
}
