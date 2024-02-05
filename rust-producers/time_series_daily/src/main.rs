use csv::Writer;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let symbol: &str = "IBM";
    
    let request_url = format!(
        "https://www.alphavantage.co/query?function={function}&symbol={symbol}&outputsize={output_size}&apikey={api_key}"
        , function = "TIME_SERIES_DAILY"
        , symbol = "IBM"
        , output_size = "full"
        , api_key = "demo"
    );
    
    println!("{}", request_url);
    let response = 
        reqwest::get(&request_url)
        .await?
        .json::<serde_json::Value>()
        .await?
    ;

    let mut wtr = Writer::from_path("test.csv")
        .expect("File Instance I/O Error");
    
    wtr.write_record([
            "daily_price_date"
            , "symbol"
            , "open"
            , "high"
            , "low"
            , "close"
            , "volume"
        ]
    ).expect("File Header Write Error");

    for key in response["Time Series (Daily)"].as_object().unwrap().keys() {
       let _ = wtr.write_record([
                key
                , symbol
                , &response["Time Series (Daily)"][key]["1. open"].to_string()
                , &response["Time Series (Daily)"][key]["2. high"].to_string()
                , &response["Time Series (Daily)"][key]["3. low"].to_string()
                , &response["Time Series (Daily)"][key]["4. close"].to_string()
                , &response["Time Series (Daily)"][key]["5. volume"].to_string()
            ]
        )
        ;

    }

    Ok(())
}
