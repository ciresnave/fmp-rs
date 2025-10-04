# Script to generate endpoint stub files
endpoints = [
    ("stock_directory", "StockDirectory"),
    ("company_info", "CompanyInfo"),
    ("financials", "Financials"),
    ("charts", "Charts"),
    ("economics", "Economics"),
    ("corporate_actions", "CorporateActions"),
    ("news", "News"),
    ("analyst", "Analyst"),
    ("market_performance", "MarketPerformance"),
    ("etf", "Etf"),
    ("sec_filings", "SecFilings"),
    ("insider_trades", "InsiderTrades"),
    ("indexes", "Indexes"),
    ("commodities", "Commodities"),
    ("dcf", "Dcf"),
    ("forex", "Forex"),
    ("crypto", "Crypto"),
    ("technical_indicators", "TechnicalIndicators"),
    ("institutional", "Institutional"),
    ("congress", "Congress"),
    ("esg", "Esg"),
    ("market_hours", "MarketHours"),
    ("transcripts", "Transcripts"),
    ("bulk", "Bulk"),
]

template = """//! {title} endpoints

use crate::client::FmpClient;

/// {title} API endpoints
pub struct {struct_name} {{
    client: FmpClient,
}}

impl {struct_name} {{
    pub(crate) fn new(client: FmpClient) -> Self {{
        Self {{ client }}
    }}

    // TODO: Add endpoint methods here
    // Example:
    // pub async fn some_method(&self) -> Result<ResponseType> {{
    //     todo!("Implement {title} endpoint")
    // }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_new() {{
        let client = FmpClient::builder()
            .api_key("test_key")
            .build()
            .unwrap();
        let _ = {struct_name}::new(client);
    }}
}}
"""

import os

os.makedirs("src/endpoints", exist_ok=True)

for filename, struct_name in endpoints:
    title = struct_name
    content = template.format(title=title, struct_name=struct_name)

    with open(f"src/endpoints/{filename}.rs", "w") as f:
        f.write(content)

    print(f"Created src/endpoints/{filename}.rs")

print("\nAll endpoint files created!")
