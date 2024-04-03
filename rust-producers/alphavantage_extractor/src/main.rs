use clap::Parser;
use reqwest::Error;

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
    response_size: String,

    #[arg(short, long, default_value = "demo")]
    api_key: String,
    
    #[arg(short, long, default_value = "./")]
    output_location: String,
}

fn main() -> Result<(), Error> {

    let args = Args::parse();

    //For Dev
    //let response = dev_json::read_json_from_file("earnings.json");
    //println!("{:?}", serde_json::to_string_pretty(&response["annualEarnings"]).unwrap());
    let response =
        alphavantage_request::query_api(&args.function, &args.symbol, &args.response_size, &args.api_key)
            .unwrap();

    let _ = match args.function.as_str() {
        "TIME_SERIES_DAILY" => response_writers::time_series_daily_to_csv(response, &args.output_location),
        "EARNINGS" => response_writers::earnings_to_csv(response, &args.output_location),
        "OVERVIEW" => response_writers::overview_to_csv(response, &args.output_location),
        "INCOME_STATEMENT" => response_writers::income_statement_to_csv(response, &args.output_location),
        "BALANCE_SHEET" => response_writers::balance_sheet_to_csv(response, &args.output_location),
        "CASH_FLOW" => response_writers::cash_flow_to_csv(response, &args.output_location),
        &_ => todo!("Function not implemented"),
    };

    Ok(())
}
