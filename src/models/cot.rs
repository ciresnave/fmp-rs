//! Models for Commitment of Traders (COT) endpoints
//!
//! FMP publishes the CFTC Commitments of Traders data. The position columns
//! are numerous and follow the CFTC's own naming, so beyond the identifying
//! fields every column is kept in [`CotRecord::fields`] exactly as the API
//! returned it, rather than guessed at as typed struct members.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// One row of a COT response (report or analysis)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CotRecord {
    /// Contract symbol (e.g., "KC" for coffee), when the row carries one
    pub symbol: Option<String>,
    /// Report date, when the row carries one
    pub date: Option<String>,
    /// Every other column, as returned by the API
    #[serde(flatten)]
    pub fields: HashMap<String, serde_json::Value>,
}

impl CotRecord {
    /// A numeric column by its API name; `None` if absent or not numeric
    pub fn number(&self, name: &str) -> Option<f64> {
        self.fields.get(name).and_then(|v| v.as_f64())
    }
}

/// One entry of the COT report list (a contract with available reports)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CotReportListItem {
    /// Contract symbol
    pub symbol: Option<String>,
    /// Contract name
    pub name: Option<String>,
    /// Every other column, as returned by the API
    #[serde(flatten)]
    pub fields: HashMap<String, serde_json::Value>,
}
