use reqwest::Error;
use clap::Parser;

mod alphavantage_request;
mod response_writers;

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

    let args = Args::parse();    
    
    let response = alphavantage_request::query_api(
        &args.function
        , &args.symbol
        , &args.output_size
        , "demo"
    ).unwrap();

    let _ =  response_writers::time_series_daily_to_csv(response, &args.symbol);

    Ok(())
}
