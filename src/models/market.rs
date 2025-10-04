//! Market performance models.

use serde::{Deserialize, Serialize};

/// Sector performance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SectorPerformance {
    pub sector: String,
    pub changes_percentage: String,
}

/// Biggest gainer/loser
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StockMover {
    pub symbol: String,
    pub name: String,
    pub change: f64,
    pub price: f64,
    pub changes_percentage: String,
}
