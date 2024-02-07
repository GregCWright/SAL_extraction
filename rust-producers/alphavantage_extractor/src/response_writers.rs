use csv::Error;
use csv::Writer;
use chrono::prelude::*;

macro_rules! std_json_str {
    ($json:expr) => {
        $json.as_str().unwrap()
    };
}

fn writer_maker(prefix: &str, symbol: &str, execution_time: DateTime<Utc>) -> Writer<std::fs::File> {
    let wtr: Writer<std::fs::File> = Writer::from_path(
        format!("{}_{}_{}.csv"
        , prefix
        , symbol
        , execution_time.format("%Y-%m-%d")
    ))
    .expect("File Instance I/O Error");

    wtr
}

pub fn time_series_daily_to_csv(response: serde_json::Value) -> Result<(), Error> {
    
    let prefix: &str = "time_series_daily";
    let symbol: &str = std_json_str!(response["Meta Data"]["2. Symbol"]);
    let execution_time = Utc::now();
    let mut wtr: Writer<std::fs::File> = writer_maker(prefix, symbol, execution_time);

    wtr.write_record([
        "daily_price_date",
        "symbol",
        "open",
        "high",
        "low",
        "close",
        "volume",
        "execution_time"
    ])
    .expect("File Header Write Error");

    for key in response["Time Series (Daily)"].as_object().unwrap().keys() {
        let _ = wtr.write_record([
            key,
            symbol,
            std_json_str!(response["Time Series (Daily)"][key]["1. open"]),
            std_json_str!(response["Time Series (Daily)"][key]["2. high"]),
            std_json_str!(response["Time Series (Daily)"][key]["3. low"]),
            std_json_str!(response["Time Series (Daily)"][key]["4. close"]),
            std_json_str!(response["Time Series (Daily)"][key]["5. volume"]),
            &execution_time.to_string()
        ]);
    }

    Ok(())
}

pub fn earnings_to_csv(response: serde_json::Value) -> Result<(), Error> {
    let prefix_1: &str = "annual_earnings";
    let prefix_2: &str = "quarterly_earnings";
    let symbol: &str = std_json_str!(response["symbol"]);
    let execution_time = Utc::now();
    let mut wtr: Writer<std::fs::File> = writer_maker(prefix_1, symbol, execution_time);

    wtr.write_record([
        "fiscal_date_ending",
        "symbol",
        "reported_earnings_per_share",
        "execution_time"
    ])
    .expect("File Header Write Error (Annual Earnings)");

    if let Some(earnings) = response["annualEarnings"].as_array() {
        for earning in earnings {
            let _ = wtr.write_record([
                std_json_str!(earning["fiscalDateEnding"]),
                symbol,
                std_json_str!(earning["reportedEPS"]),
                &execution_time.to_string(),
            ]);
        }
    }

    wtr = writer_maker(prefix_2, symbol, execution_time);
    wtr.write_record([
        "fiscal_date_ending",
        "symbol",
        "reported_date",
        "reported_earnings_per_share",
        "estimated_earnings_per_share",
        "surprise",
        "surprise_percentage",
        "execuation_time",
    ])
    .expect("File Header Write Error (Quarterly Earnings)");

    if let Some(earnings) = response["quarterlyEarnings"].as_array() {
        for earning in earnings {
            let _ = wtr.write_record([
                std_json_str!(earning["fiscalDateEnding"]),
                symbol,
                std_json_str!(earning["reportedDate"]),
                std_json_str!(earning["reportedEPS"]),
                std_json_str!(earning["estimatedEPS"]),
                std_json_str!(earning["surprise"]),
                std_json_str!(earning["surprisePercentage"]),
                &execution_time.to_string(),
            ]);
        }
    }

    Ok(())
}
