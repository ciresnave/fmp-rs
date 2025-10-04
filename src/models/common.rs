//! Common types shared across multiple endpoints.

use serde::{Deserialize, Serialize};

/// Represents a stock symbol with basic information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub name: String,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub exchange: Option<String>,
    #[serde(default)]
    pub exchange_short_name: Option<String>,
}

/// Time period for financial data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Period {
    Annual,
    Quarter,
}

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Period::Annual => write!(f, "annual"),
            Period::Quarter => write!(f, "quarter"),
        }
    }
}

/// Chart timeframe for intraday data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Timeframe {
    OneMinute,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    FourHours,
}

impl std::fmt::Display for Timeframe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Timeframe::OneMinute => write!(f, "1min"),
            Timeframe::FiveMinutes => write!(f, "5min"),
            Timeframe::FifteenMinutes => write!(f, "15min"),
            Timeframe::ThirtyMinutes => write!(f, "30min"),
            Timeframe::OneHour => write!(f, "1hour"),
            Timeframe::FourHours => write!(f, "4hour"),
        }
    }
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize)]
pub struct Pagination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl Pagination {
    pub fn new() -> Self {
        Self {
            page: None,
            limit: None,
        }
    }

    pub fn with_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new()
    }
}

/// Date range for filtering
#[derive(Debug, Clone, Serialize)]
pub struct DateRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl DateRange {
    pub fn new() -> Self {
        Self {
            from: None,
            to: None,
        }
    }

    pub fn from(mut self, date: impl Into<String>) -> Self {
        self.from = Some(date.into());
        self
    }

    pub fn to(mut self, date: impl Into<String>) -> Self {
        self.to = Some(date.into());
        self
    }
}

impl Default for DateRange {
    fn default() -> Self {
        Self::new()
    }
}
