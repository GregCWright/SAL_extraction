use reqwest::Error;

#[tokio::main]
pub async fn query_api(function: &str, symbol: &str, output_size: &str, api_key: &str) -> Result<serde_json::Value, Error>{
    
    let request_url:String = format!(
        "https://www.alphavantage.co/query?function={function}&symbol={symbol}&outputsize={output_size}&apikey={api_key}"
        , function = function
        , symbol = symbol
        , output_size = output_size
        , api_key = api_key
    );

    let response = 
        reqwest::get(&request_url)
        .await
        .expect("Reqwest Error")
        .json::<serde_json::Value>()
        .await
    ;

    return response
}