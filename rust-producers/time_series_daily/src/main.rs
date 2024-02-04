use serde::{Serialize, Deserialize};
use serde_with::serde_as;
use serde_nested_with::serde_nested;
use reqwest::Error;
use std::collections::HashMap;
use time::OffsetDateTime;
use time::serde::iso8601;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "Meta Data")]
    pub meta_data: MetaData,
    #[serde(rename = "Time Series (Daily)")]
    pub time_series_daily: TimeSeriesDaily,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    #[serde(rename = "1. Information")]
    pub information: String,
    #[serde(rename = "2. Symbol")]
    pub symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    pub last_refreshed: String,
    #[serde(rename = "4. Output Size")]
    pub output_size: String,
    #[serde(rename = "5. Time Zone")]
    pub time_zone: String,
}

#[serde_nested]
#[derive(Debug, Deserialize)]
pub struct TimeSeriesDaily {
    #[serde_nested(sub = "OffsetDateTime", serde(with = "iso8601"))]
    #[serde(rename = "other")]
    pub price_info_hashmap: Vec<(OffsetDateTime, PriceInfo)>
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)] 
pub struct PriceInfo {
    #[serde(rename = "1. open")]
    pub open: String,
    #[serde(rename = "2. high")]
    pub high: String,
    #[serde(rename = "3. low")]
    pub low: String,
    #[serde(rename = "4. close")]
    pub close: String,
    #[serde(rename = "5. volume")]
    pub volume: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    let request_url = format!(
        "https://www.alphavantage.co/query?function={function}&symbol={symbol}&outputsize={output_size}&apikey={api_key}"
        , function = "TIME_SERIES_DAILY"
        , symbol = "IBM"
        , output_size = "full"
        , api_key = "demo"
    );
    
    println!("{}", request_url);
    let response: reqwest::Response = reqwest::get(&request_url).await?;

    let alpha_response: Root = response.json().await?;
    let content = alpha_response.time_series_daily;
    println!("{:?}", content);
    Ok(())
}
