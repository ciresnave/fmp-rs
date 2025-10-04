//! Models for market performance endpoints

use serde::{Deserialize, Serialize};

/// Sector performance data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectorPerformance {
    /// Sector name
    pub sector: String,
    /// Performance change percentage
    pub changes_percentage: Option<String>,
}

/// Historical sector performance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectorHistorical {
    /// Date
    pub date: String,
    /// Technology sector performance
    #[serde(rename = "technologyChangesPercentage")]
    pub technology_changes_percentage: Option<f64>,
    /// Healthcare sector performance
    #[serde(rename = "healthcareChangesPercentage")]
    pub healthcare_changes_percentage: Option<f64>,
    /// Financial sector performance
    #[serde(rename = "financialServicesChangesPercentage")]
    pub financial_services_changes_percentage: Option<f64>,
    /// Consumer Cyclical sector performance
    #[serde(rename = "consumerCyclicalChangesPercentage")]
    pub consumer_cyclical_changes_percentage: Option<f64>,
    /// Industrials sector performance
    #[serde(rename = "industrialsChangesPercentage")]
    pub industrials_changes_percentage: Option<f64>,
    /// Energy sector performance
    #[serde(rename = "energyChangesPercentage")]
    pub energy_changes_percentage: Option<f64>,
    /// Basic Materials sector performance
    #[serde(rename = "basicMaterialsChangesPercentage")]
    pub basic_materials_changes_percentage: Option<f64>,
    /// Consumer Defensive sector performance
    #[serde(rename = "consumerDefensiveChangesPercentage")]
    pub consumer_defensive_changes_percentage: Option<f64>,
    /// Real Estate sector performance
    #[serde(rename = "realEstateChangesPercentage")]
    pub real_estate_changes_percentage: Option<f64>,
    /// Communication Services sector performance
    #[serde(rename = "communicationServicesChangesPercentage")]
    pub communication_services_changes_percentage: Option<f64>,
    /// Utilities sector performance
    #[serde(rename = "utilitiesChangesPercentage")]
    pub utilities_changes_percentage: Option<f64>,
}

/// Market hours information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketHours {
    /// Stock exchange name
    pub stock_exchange_name: Option<String>,
    /// Stock market hours
    pub stock_market_hours: Option<MarketHourDetails>,
    /// Stock market holidays
    #[serde(default)]
    pub stock_market_holidays: Vec<Holiday>,
    /// Whether market is open
    pub is_the_stock_market_open: Option<bool>,
    /// Whether it's a holiday
    pub is_the_stock_market_holiday: Option<bool>,
}

/// Market hour details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketHourDetails {
    /// Opening time
    pub open_time: Option<String>,
    /// Closing time
    pub close_time: Option<String>,
}

/// Holiday information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Holiday {
    /// Year
    pub year: Option<i32>,
    /// Holiday name
    pub holiday: Option<String>,
    /// Holiday date
    pub date: Option<String>,
}

/// Stock gainer, loser, or most active
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockMover {
    /// Stock symbol
    pub symbol: String,
    /// Company name
    pub name: Option<String>,
    /// Current price
    pub price: Option<f64>,
    /// Price change
    pub change: Option<f64>,
    /// Percentage change
    pub changes_percentage: Option<f64>,
    /// Trading volume
    pub volume: Option<i64>,
}

/// Pre/Post market trading data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrePostMarket {
    /// Stock symbol
    pub symbol: String,
    /// Pre-market price
    pub pre_market_price: Option<f64>,
    /// Pre-market change
    pub pre_market_change: Option<f64>,
    /// Pre-market change percentage
    pub pre_market_change_percentage: Option<f64>,
    /// After-hours price
    pub after_hours_price: Option<f64>,
    /// After-hours change
    pub after_hours_change: Option<f64>,
    /// After-hours change percentage
    pub after_hours_change_percentage: Option<f64>,
}
