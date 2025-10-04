//! Models for technical indicators endpoints

use serde::{Deserialize, Serialize};

/// Technical indicator data point
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TechnicalIndicator {
    /// Date
    pub date: String,
    /// Indicator value
    #[serde(flatten)]
    pub values: std::collections::HashMap<String, f64>,
}

/// Simple Moving Average (SMA) data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmaData {
    /// Date
    pub date: String,
    /// SMA value
    pub sma: Option<f64>,
}

/// Exponential Moving Average (EMA) data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmaData {
    /// Date
    pub date: String,
    /// EMA value
    pub ema: Option<f64>,
}

/// Weighted Moving Average (WMA) data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WmaData {
    /// Date
    pub date: String,
    /// WMA value
    pub wma: Option<f64>,
}

/// Double Exponential Moving Average (DEMA) data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DemaData {
    /// Date
    pub date: String,
    /// DEMA value
    pub dema: Option<f64>,
}

/// Triple Exponential Moving Average (TEMA) data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemaData {
    /// Date
    pub date: String,
    /// TEMA value
    pub tema: Option<f64>,
}

/// Relative Strength Index (RSI) data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RsiData {
    /// Date
    pub date: String,
    /// RSI value (0-100)
    pub rsi: Option<f64>,
}

/// Average Directional Index (ADX) data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdxData {
    /// Date
    pub date: String,
    /// ADX value
    pub adx: Option<f64>,
}

/// Standardized data structure for technical indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandardizedIndicator {
    /// Date
    pub date: String,
    /// Open price
    pub open: Option<f64>,
    /// High price
    pub high: Option<f64>,
    /// Low price
    pub low: Option<f64>,
    /// Close price
    pub close: Option<f64>,
    /// Volume
    pub volume: Option<f64>,
}

/// Williams %R indicator data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WilliamsRData {
    /// Date
    pub date: String,
    /// Williams %R value (-100 to 0)
    pub williams_r: Option<f64>,
}

/// Commodity Channel Index (CCI) data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CciData {
    /// Date
    pub date: String,
    /// CCI value
    pub cci: Option<f64>,
}

/// MACD (Moving Average Convergence Divergence) data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacdData {
    /// Date
    pub date: String,
    /// MACD line
    pub macd: Option<f64>,
    /// Signal line
    pub signal: Option<f64>,
    /// Histogram
    pub histogram: Option<f64>,
}

/// Stochastic Oscillator data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StochasticData {
    /// Date
    pub date: String,
    /// %K value
    pub slow_k: Option<f64>,
    /// %D value
    pub slow_d: Option<f64>,
}

/// Bollinger Bands data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BollingerBandsData {
    /// Date
    pub date: String,
    /// Upper band
    pub upper_band: Option<f64>,
    /// Middle band (SMA)
    pub middle_band: Option<f64>,
    /// Lower band
    pub lower_band: Option<f64>,
}

/// Average True Range (ATR) data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtrData {
    /// Date
    pub date: String,
    /// ATR value
    pub atr: Option<f64>,
}

/// Aroon Indicator data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AroonData {
    /// Date
    pub date: String,
    /// Aroon Up
    pub aroon_up: Option<f64>,
    /// Aroon Down
    pub aroon_down: Option<f64>,
}
