use clap::Parser;
use reqwest::Error;
use dotenv::dotenv;

mod alphavantage_request;
mod response_writers;
mod dev_json;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    function: String,

    #[arg(short, long)]
    symbol: String,

    #[arg(short, long, default_value = "full")]
    output_size: String,
}

fn main() -> Result<(), Error> {

    dotenv().ok();
    let api_key: String = std::env::var("api_key").expect("api_key must be set");
    let args = Args::parse();

    //For Dev
    //let response = dev_json::read_json_from_file("earnings.json");
    //println!("{:?}", serde_json::to_string_pretty(&response["annualEarnings"]).unwrap());
    let response =
        alphavantage_request::query_api(&args.function, &args.symbol, &args.output_size, &api_key)
            .unwrap();

    let _ = match args.function.as_str() {
        "TIME_SERIES_DAILY" => response_writers::time_series_daily_to_csv(response),
        "EARNINGS" => response_writers::earnings_to_csv(response),
        "OVERVIEW" => response_writers::overview_to_csv(response),
        "INCOME_STATEMENT" => response_writers::income_statement_to_csv(response),
        "BALANCE_SHEET" => response_writers::balance_sheet_to_csv(response),
        &_ => todo!("Function not implemented"),
    };

    Ok(())
}
