//! Models for economics and indicators endpoints

use serde::{Deserialize, Serialize};

/// Treasury rate data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreasuryRate {
    /// Date
    pub date: String,
    /// 1 month rate
    pub month_1: Option<f64>,
    /// 2 month rate
    pub month_2: Option<f64>,
    /// 3 month rate
    pub month_3: Option<f64>,
    /// 6 month rate
    pub month_6: Option<f64>,
    /// 1 year rate
    pub year_1: Option<f64>,
    /// 2 year rate
    pub year_2: Option<f64>,
    /// 3 year rate
    pub year_3: Option<f64>,
    /// 5 year rate
    pub year_5: Option<f64>,
    /// 7 year rate
    pub year_7: Option<f64>,
    /// 10 year rate
    pub year_10: Option<f64>,
    /// 20 year rate
    pub year_20: Option<f64>,
    /// 30 year rate
    pub year_30: Option<f64>,
}

/// Economic indicator data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EconomicIndicator {
    /// Date
    pub date: String,
    /// Indicator value
    pub value: Option<f64>,
}

/// Market risk premium data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketRiskPremium {
    /// Country
    pub country: String,
    /// Continent
    pub continent: Option<String>,
    /// Total equity risk premium
    pub total_equity_risk_premium: Option<f64>,
    /// Country risk premium
    pub country_risk_premium: Option<f64>,
}
