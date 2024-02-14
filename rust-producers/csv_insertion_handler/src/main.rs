use std::{env, fs, path::PathBuf};
use sqlx::{postgres::PgPoolOptions, Postgres};
use dotenv::dotenv;

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     #[arg(short, long)]
//     function: String,
// }

mod query_library;

fn append_to_path(p: PathBuf, s: &str) -> PathBuf {
    let mut p = p.into_os_string();
    p.push(s);
    p.into()
}

#[tokio::main]
async fn main()  -> Result<(), sqlx::Error> {
    dotenv().ok();
    let postgres_username: String = std::env::var("postgres_username").expect("postgres_username must be set");
    let postgres_password: String = std::env::var("postgres_password").expect("postgres_password must be set");
    // let args = Args::parse();
    let current_dir = env::current_dir().expect("Current Path Error");

    let csv_path: &str = "stock_csv";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            &format!(
                "postgres://{postgres_username}:{postgres_password}@localhost/SAL"
                , postgres_username = postgres_username
                , postgres_password = postgres_password
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
