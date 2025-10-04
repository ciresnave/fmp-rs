//! Models for market hours endpoints

use serde::{Deserialize, Serialize};

/// Market hours for a trading session
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketHours {
    /// Exchange name
    pub exchange: Option<String>,
    /// Stock exchange name
    pub stock_exchange_name: Option<String>,
    /// Stock market (e.g., "NYSE", "NASDAQ")
    pub stock_market: Option<String>,
    /// Current status (e.g., "open", "closed")
    pub is_market_open: Option<bool>,
    /// Market open time
    pub opening_hour: Option<String>,
    /// Market close time
    pub closing_hour: Option<String>,
    /// Current date and time
    pub current_date_time: Option<String>,
    /// Timezone
    pub timezone: Option<String>,
}

/// Extended market hours information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedMarketHours {
    /// Exchange symbol
    pub exchange: Option<String>,
    /// Exchange name
    pub name: Option<String>,
    /// Market is open status
    pub is_open: Option<bool>,
    /// Regular session open time
    pub market_open: Option<String>,
    /// Regular session close time
    pub market_close: Option<String>,
    /// Pre-market open time
    pub pre_market_open: Option<String>,
    /// Pre-market close time
    pub pre_market_close: Option<String>,
    /// After hours open time
    pub after_hours_open: Option<String>,
    /// After hours close time
    pub after_hours_close: Option<String>,
    /// Timezone
    pub timezone: Option<String>,
    /// Current date time
    pub current_date_time: Option<String>,
}

/// Market holiday information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketHoliday {
    /// Holiday date
    pub date: Option<String>,
    /// Holiday year
    pub year: Option<i32>,
    /// Holiday name
    pub holiday: Option<String>,
    /// Exchange
    pub exchange: Option<String>,
    /// Market status on holiday (e.g., "closed", "early_close")
    pub market_status: Option<String>,
    /// Early close time if applicable
    pub early_close_time: Option<String>,
}

/// Exchange trading hours and status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeHours {
    /// Exchange symbol (e.g., "NYSE", "NASDAQ")
    pub exchange: Option<String>,
    /// Full exchange name
    pub exchange_name: Option<String>,
    /// Country
    pub country: Option<String>,
    /// Timezone
    pub timezone: Option<String>,
    /// Regular trading open time
    pub market_open: Option<String>,
    /// Regular trading close time  
    pub market_close: Option<String>,
    /// Pre-market start time
    pub pre_market_start: Option<String>,
    /// Pre-market end time
    pub pre_market_end: Option<String>,
    /// After hours start time
    pub after_hours_start: Option<String>,
    /// After hours end time
    pub after_hours_end: Option<String>,
    /// Current market status
    pub is_open: Option<bool>,
    /// Current date and time
    pub current_time: Option<String>,
    /// Day of week
    pub day_of_week: Option<String>,
}
