//! Company information models.

use serde::{Deserialize, Serialize};

/// Comprehensive company profile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompanyProfile {
    pub symbol: String,
    pub price: f64,
    pub beta: f64,
    pub vol_avg: i64,
    pub mkt_cap: i64,
    pub last_div: f64,
    pub range: String,
    pub changes: f64,
    pub company_name: String,
    pub currency: String,
    pub cik: String,
    pub isin: String,
    pub cusip: String,
    pub exchange: String,
    pub exchange_short_name: String,
    pub industry: String,
    pub website: String,
    pub description: String,
    pub ceo: String,
    pub sector: String,
    pub country: String,
    pub full_time_employees: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub dcf_diff: f64,
    pub dcf: f64,
    pub image: String,
    pub ipo_date: String,
    pub default_image: bool,
    pub is_etf: bool,
    pub is_actively_trading: bool,
    pub is_adr: bool,
    pub is_fund: bool,
}

/// Company key executives
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Executive {
    pub title: String,
    pub name: String,
    pub pay: Option<i64>,
    pub currency_pay: Option<String>,
    pub gender: Option<String>,
    pub year_born: Option<i32>,
    pub title_since: Option<String>,
}

/// Company market capitalization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MarketCap {
    pub symbol: String,
    pub date: String,
    pub market_cap: i64,
}

/// Company share float
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShareFloat {
    pub symbol: String,
    pub free_float: f64,
    pub float_shares: i64,
    pub outstanding_shares: i64,
    pub date: String,
}

/// Search result for company
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompanySearchResult {
    pub symbol: String,
    pub name: String,
    pub currency: String,
    pub stock_exchange: String,
    pub exchange_short_name: String,
}

/// Stock peers (similar companies)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StockPeers {
    pub symbol: String,
    #[serde(rename = "peersList")]
    pub peers: Vec<String>,
}

/// Company rating
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompanyRating {
    pub symbol: String,
    pub date: String,
    pub rating: String,
    pub rating_score: i32,
    pub rating_recommendation: String,
    pub rating_details_dcf_score: i32,
    pub rating_details_dcf_recommendation: String,
    pub rating_details_roe_score: i32,
    pub rating_details_roe_recommendation: String,
    pub rating_details_roa_score: i32,
    pub rating_details_roa_recommendation: String,
    pub rating_details_de_score: i32,
    pub rating_details_de_recommendation: String,
    pub rating_details_pe_score: i32,
    pub rating_details_pe_recommendation: String,
    pub rating_details_pb_score: i32,
    pub rating_details_pb_recommendation: String,
}

/// Company outlook (comprehensive company data)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompanyOutlook {
    pub profile: CompanyProfile,
    pub metrics: Option<serde_json::Value>,
    pub ratios: Option<Vec<serde_json::Value>>,
    pub insider_trades: Option<Vec<serde_json::Value>>,
    pub key_executives: Option<Vec<Executive>>,
    pub split_history: Option<Vec<serde_json::Value>>,
    pub stock_dividend: Option<Vec<serde_json::Value>>,
    pub stock_news: Option<Vec<serde_json::Value>>,
    pub rating: Option<Vec<CompanyRating>>,
}

/// Company core information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompanyCoreInfo {
    pub cik: String,
    pub symbol: String,
    pub exchange: String,
}

/// Stock screener result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StockScreenerResult {
    pub symbol: String,
    pub company_name: String,
    pub market_cap: Option<f64>,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub beta: Option<f64>,
    pub price: Option<f64>,
    pub last_annual_dividend: Option<f64>,
    pub volume: Option<i64>,
    pub exchange: Option<String>,
    pub exchange_short_name: Option<String>,
    pub country: Option<String>,
    pub is_etf: Option<bool>,
    pub is_actively_trading: Option<bool>,
}

/// Available traded symbol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TradableSymbol {
    pub symbol: String,
    pub name: String,
    pub price: Option<f64>,
    pub exchange: Option<String>,
    pub exchange_short_name: Option<String>,
    #[serde(rename = "type")]
    pub symbol_type: Option<String>,
}
