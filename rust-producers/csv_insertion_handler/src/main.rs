use std::fs;
use sqlx::postgres::PgPoolOptions;
use clap::Parser;

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     #[arg(short, long)]
//     function: String,
// }

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    username: String,

    #[arg(short, long)]
    password: String,

    #[arg(short, long)]
    host: String,

    #[arg(short, long, default_value = "5432")]
    port: String,

    #[arg(long, default_value = "SAL")]
    database: String,

    #[arg(short, long, default_value = "./")]
    input_location: String,
}

mod query_library;

#[tokio::main]
async fn main()  -> Result<(), sqlx::Error> {
    let args = Args::parse();

    let postgres_username: String = args.username;
    let postgres_password: String = args.password;
    let postgres_host: String = args.host;
    let postgres_port: String = args.port;
    let postgres_database: String = args.database;
    let csv_path: &str = &args.input_location;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            &format!(
                "postgres://{}:{}@{}:{}/{}"
                , postgres_username
                , postgres_password
                , postgres_host
                , postgres_port
                , postgres_database
            )
        ).await?;
    
    let mut query_1: String = "".to_string();
    let mut query_2: String = "".to_string();
    let mut query_3: String = "".to_string();
    let mut query_4: String = "".to_string();

    for file in fs::read_dir(csv_path).unwrap() {

        let file_as_string: String = file
            .unwrap()
            .file_name()
            .into_string()
            .unwrap();

        let file_path = format!("/csv/{}", file_as_string);
        
        if file_as_string.contains("time_series_daily") {(query_1, query_2, query_3, query_4) = query_library::time_series_daily_queries(file_path.clone())}
        if file_as_string.contains("overview") {(query_1, query_2, query_3, query_4) = query_library::overview_queries(file_path.clone())}

        if file_as_string.contains("annual_earnings") {(query_1, query_2, query_3, query_4) = query_library::annual_earnings_queries(file_path.clone())}
        if file_as_string.contains("annual_income_statement") {(query_1, query_2, query_3, query_4) = query_library::annual_income_statement_queries(file_path.clone())}
        if file_as_string.contains("annual_balance_sheet") {(query_1, query_2, query_3, query_4) = query_library::annual_balance_sheet_queries(file_path.clone())}
        if file_as_string.contains("annual_cash_flow") {(query_1, query_2, query_3, query_4) = query_library::annual_cash_flow_queries(file_path.clone())}

        if file_as_string.contains("quarterly_earnings") {(query_1, query_2, query_3, query_4) = query_library::quarterly_earnings_queries(file_path.clone())}
        if file_as_string.contains("quarterly_income_statement") {(query_1, query_2, query_3, query_4) = query_library::quarterly_income_statement_queries(file_path.clone())}
        if file_as_string.contains("quarterly_balance_sheet") {(query_1, query_2, query_3, query_4) = query_library::quarterly_balance_sheet_queries(file_path.clone())}
        if file_as_string.contains("quarterly_cash_flow") {(query_1, query_2, query_3, query_4) = query_library::quarterly_cash_flow_queries(file_path.clone())}

        let _ = sqlx::query(&query_1)
            .execute(&pool).await?;

        let _ = sqlx::query(&query_2)
            .execute(&pool).await?;

        let _ = sqlx::query(&query_3)
            .execute(&pool).await?;

        let _ = sqlx::query(&query_4)
            .execute(&pool).await?;
    }

    Ok(())    
}
