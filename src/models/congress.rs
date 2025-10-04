//! Models for congressional and insider trading endpoints

use serde::{Deserialize, Serialize};

/// Congressional trading data (House and Senate)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CongressionalTrade {
    /// Representative/Senator name
    pub representative: Option<String>,
    /// Trading date
    pub transaction_date: Option<String>,
    /// Stock symbol
    pub ticker: Option<String>,
    /// Company name
    pub company: Option<String>,
    /// Transaction type (Purchase, Sale, Exchange)
    pub transaction: Option<String>,
    /// Transaction amount range
    pub amount: Option<String>,
    /// Political party (Democrat, Republican, Independent)
    pub party: Option<String>,
    /// State represented
    pub state: Option<String>,
    /// House or Senate
    pub chamber: Option<String>,
    /// Asset type (Stock, Bond, Option, etc.)
    pub asset_type: Option<String>,
    /// Filing date
    pub filing_date: Option<String>,
    /// Disclosure period
    pub disclosure_period: Option<String>,
    /// Transaction ID
    pub transaction_id: Option<String>,
    /// Owner (Self, Spouse, Child)
    pub owner: Option<String>,
}

/// Insider trading transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsiderTrade {
    /// Company symbol
    pub symbol: Option<String>,
    /// Company name
    pub company_name: Option<String>,
    /// Filing date
    pub filing_date: Option<String>,
    /// Transaction date
    pub transaction_date: Option<String>,
    /// Insider name
    pub reporting_name: Option<String>,
    /// Insider title/position
    pub title: Option<String>,
    /// Transaction type (P=Purchase, S=Sale, A=Award, etc.)
    pub transaction_type: Option<String>,
    /// Number of securities transacted
    pub securities_transacted: Option<f64>,
    /// Price per security
    pub price: Option<f64>,
    /// Total transaction value
    pub transaction_value: Option<f64>,
    /// Securities owned after transaction
    pub securities_owned: Option<f64>,
    /// Form type (4, 5, etc.)
    pub form_type: Option<String>,
    /// CIK (Central Index Key)
    pub cik: Option<String>,
    /// Accession number
    pub accession_number: Option<String>,
    /// Is the insider a director
    pub is_director: Option<bool>,
    /// Is the insider an officer
    pub is_officer: Option<bool>,
    /// Is the insider a 10% owner
    pub is_ten_percent_owner: Option<bool>,
    /// Direct or indirect ownership
    pub ownership_type: Option<String>,
}

/// Insider trading statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsiderStats {
    /// Company symbol
    pub symbol: Option<String>,
    /// Company name
    pub company_name: Option<String>,
    /// Time period
    pub period: Option<String>,
    /// Total insider purchases
    pub total_purchases: Option<i32>,
    /// Total insider sales
    pub total_sales: Option<i32>,
    /// Total purchase value
    pub total_purchase_value: Option<f64>,
    /// Total sale value
    pub total_sale_value: Option<f64>,
    /// Net insider activity (purchases - sales)
    pub net_activity: Option<f64>,
    /// Number of insiders buying
    pub insiders_buying: Option<i32>,
    /// Number of insiders selling
    pub insiders_selling: Option<i32>,
    /// Average purchase size
    pub avg_purchase_size: Option<f64>,
    /// Average sale size
    pub avg_sale_size: Option<f64>,
    /// Insider sentiment (positive/negative/neutral)
    pub insider_sentiment: Option<String>,
}

/// Insider roster information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsiderRoster {
    /// Company symbol
    pub symbol: Option<String>,
    /// Company name
    pub company_name: Option<String>,
    /// Insider name
    pub name: Option<String>,
    /// Position/title
    pub title: Option<String>,
    /// CIK
    pub cik: Option<String>,
    /// Is director
    pub is_director: Option<bool>,
    /// Is officer
    pub is_officer: Option<bool>,
    /// Is 10% owner
    pub is_ten_percent_owner: Option<bool>,
    /// Date became insider
    pub date_appointed: Option<String>,
    /// Total shares owned
    pub shares_owned: Option<f64>,
    /// Ownership percentage
    pub ownership_percent: Option<f64>,
    /// Last transaction date
    pub last_transaction_date: Option<String>,
    /// Most recent transaction type
    pub last_transaction_type: Option<String>,
}

/// Senate/House disclosure summary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CongressionalSummary {
    /// Representative name
    pub representative: Option<String>,
    /// Political party
    pub party: Option<String>,
    /// State
    pub state: Option<String>,
    /// Chamber (House/Senate)
    pub chamber: Option<String>,
    /// Filing year
    pub year: Option<i32>,
    /// Total number of trades
    pub total_trades: Option<i32>,
    /// Total estimated transaction value
    pub total_value: Option<String>,
    /// Number of buy transactions
    pub buy_transactions: Option<i32>,
    /// Number of sell transactions
    pub sell_transactions: Option<i32>,
    /// Most traded sector
    pub top_sector: Option<String>,
    /// Most traded stock
    pub top_stock: Option<String>,
    /// First filing date
    pub first_filing: Option<String>,
    /// Last filing date
    pub last_filing: Option<String>,
}

/// Insider ownership summary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsiderOwnership {
    /// Company symbol
    pub symbol: Option<String>,
    /// Company name
    pub company_name: Option<String>,
    /// Total insider ownership percentage
    pub insider_ownership_percent: Option<f64>,
    /// Number of insider owners
    pub number_of_insiders: Option<i32>,
    /// Total insider owned shares
    pub total_insider_shares: Option<f64>,
    /// Market value of insider holdings
    pub market_value: Option<f64>,
    /// Last updated date
    pub last_updated: Option<String>,
    /// Institutional ownership percentage
    pub institutional_ownership_percent: Option<f64>,
    /// Float percentage
    pub float_percentage: Option<f64>,
    /// Insider buying activity (last 6 months)
    pub recent_insider_buying: Option<i32>,
    /// Insider selling activity (last 6 months)
    pub recent_insider_selling: Option<i32>,
}
