use csv::Writer;
use reqwest::Error;

mod alphavantage_request;

fn main() -> Result<(), Error> {

    let symbol = "IBM";

    let response = alphavantage_request::query_api(
        "TIME_SERIES_DAILY"
        , "IBM"
        , "full"
        , "demo"
    ).unwrap();

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
