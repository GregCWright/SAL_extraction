use csv::Error;
use csv::Writer;
use chrono::prelude::*;

macro_rules! std_json_str {
    ($json:expr) => {
        &format!("{:?}", $json.as_str().unwrap())
    };
}

fn writer_maker(prefix: &str, symbol: &str, execution_time: DateTime<Utc>) -> Writer<std::fs::File> {
    let wtr: Writer<std::fs::File> = Writer::from_path(
        format!("{}_{}_{}.csv"
        , prefix
        , symbol.replace("\"", "")
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
        "execution_time",
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

pub fn overview_to_csv(response: serde_json::Value) -> Result<(), Error> {
    
    let prefix: &str = "overview";
    let symbol: &str = std_json_str!(response["Symbol"]);
    let execution_time = Utc::now();
    let mut wtr: Writer<std::fs::File> = writer_maker(prefix, symbol, execution_time);

    wtr.write_record([
        "symbol",
        "asset_type",
        "name",
        "description",
        "central_index_key",
        "exchange",
        "currency",
        "country",
        "sector",
        "industry",
        "address",
        "fiscal_year_end",
        "latest_quarter",
        "market_capitalization",
        "interest_before_interest_taxes_amortization",
        "price_earnings_ratio",
        "price_earnings_growth_ratio",
        "book_value",
        "dividend_per_share",
        "dividend_yield",
        "earnings_per_share",
        "revenue_per_share_trailing_twelve_months",
        "profit_margin",
        "operating_margin_trailing_twelve_months",
        "return_on_assets_trailing_twelve_months",
        "return_on_equity_trailing_twelve_months",
        "revenue_trailing_twelve_months",
        "gross_profit_trailing_twelve_months",
        "diluted_earnings_per_share_trailing_twelve_months",
        "quarterly_earnings_growth_year_on_year",
        "quarterly_revenue_growth_year_on_year",
        "analyst_target_price",
        "trailing_price_to_earnings_ratio",
        "forward_price_to_earnings_ratio",
        "price_to_sales_ratio_trailing_twelve_months",
        "price_to_book_ratio",
        "enterprise_value_to_revenue_ratio",
        "enterprise_value_to_interest_before_interest_taxes_amortization_ratio",
        "beta",
        "fifty_two_week_high",
        "fifty_two_week_low",
        "fifty_day_moving_average",
        "two_hundred_day_moving_average",
        "shares_outstanding",
        "dividend_date",
        "ex_dividend_date",
        "execution_time"
    ])
    .expect("File Header Write Error (Overview)");

    wtr.write_record([
        std_json_str!(response["Symbol"]),
        std_json_str!(response["AssetType"]),
        std_json_str!(response["Name"]),
        std_json_str!(response["Description"]),
        std_json_str!(response["CIK"]),
        std_json_str!(response["Exchange"]),
        std_json_str!(response["Currency"]),
        std_json_str!(response["Country"]),
        std_json_str!(response["Sector"]),
        std_json_str!(response["Industry"]),
        std_json_str!(response["Address"]),
        std_json_str!(response["FiscalYearEnd"]),
        std_json_str!(response["LatestQuarter"]),
        std_json_str!(response["MarketCapitalization"]),
        std_json_str!(response["EBITDA"]),
        std_json_str!(response["PERatio"]),
        std_json_str!(response["PEGRatio"]),
        std_json_str!(response["BookValue"]),
        std_json_str!(response["DividendPerShare"]),
        std_json_str!(response["DividendYield"]),
        std_json_str!(response["EPS"]),
        std_json_str!(response["RevenuePerShareTTM"]),
        std_json_str!(response["ProfitMargin"]),
        std_json_str!(response["OperatingMarginTTM"]),
        std_json_str!(response["ReturnOnAssetsTTM"]),
        std_json_str!(response["ReturnOnEquityTTM"]),
        std_json_str!(response["RevenueTTM"]),
        std_json_str!(response["GrossProfitTTM"]),
        std_json_str!(response["DilutedEPSTTM"]),
        std_json_str!(response["QuarterlyEarningsGrowthYOY"]),
        std_json_str!(response["QuarterlyRevenueGrowthYOY"]),
        std_json_str!(response["AnalystTargetPrice"]),
        std_json_str!(response["TrailingPE"]),
        std_json_str!(response["ForwardPE"]),
        std_json_str!(response["PriceToSalesRatioTTM"]),
        std_json_str!(response["PriceToBookRatio"]),
        std_json_str!(response["EVToRevenue"]),
        std_json_str!(response["EVToEBITDA"]),
        std_json_str!(response["Beta"]),
        std_json_str!(response["52WeekHigh"]),
        std_json_str!(response["52WeekLow"]),
        std_json_str!(response["50DayMovingAverage"]),
        std_json_str!(response["200DayMovingAverage"]),
        std_json_str!(response["SharesOutstanding"]),
        std_json_str!(response["DividendDate"]),
        std_json_str!(response["ExDividendDate"]),
        &execution_time.to_string(),
    ])
    .expect("CSV Write error (Overview)");

    Ok(())

}

pub fn income_statement_to_csv(response: serde_json::Value) -> Result<(), Error> {
    let prefix_1: &str = "annual_income_statement";
    let prefix_2: &str = "quarterly_income_statement";
    let symbol: &str = std_json_str!(response["symbol"]);
    let execution_time = Utc::now();
    let mut wtr: Writer<std::fs::File> = writer_maker(prefix_1, symbol, execution_time);

    wtr.write_record([
        "fiscal_date_ending",
        "symbol",
        "reported_currency",
        "gross_profit",
        "total_revenue",
        "cost_of_revenue",
        "cost_of_goods_and_services_sold",
        "operating_income",
        "selling_general_and_administrative",
        "research_and_development",
        "operating_expenses",
        "investment_income_net",
        "net_interest_income",
        "interest_income",
        "interest_expense",
        "non_interest_income",
        "other_non_operating_income",
        "depreciation",
        "depreciation_and_amortization",
        "income_before_tax",
        "income_tax_expense",
        "interest_and_dept_expense",
        "net_income_from_continuting_operations",
        "comprehensive_income_net_of_tax",
        "earnings_before_interest_taxes",
        "earnings_befpre_interest_taxes_depreciation_amortization",
        "net_income",
        "execution_time"
    ])
    .expect("File Header Write Error (Annual Income Statement)");

    if let Some(earnings) = response["annualReports"].as_array() {
        for earning in earnings {
            let _ = wtr.write_record([
                std_json_str!(earning["fiscalDateEnding"]),
                symbol,
                std_json_str!(earning["reportedCurrency"]),
                std_json_str!(earning["grossProfit"]),
                std_json_str!(earning["totalRevenue"]),
                std_json_str!(earning["costOfRevenue"]),
                std_json_str!(earning["costofGoodsAndServicesSold"]),
                std_json_str!(earning["operatingIncome"]),
                std_json_str!(earning["sellingGeneralAndAdministrative"]),
                std_json_str!(earning["researchAndDevelopment"]),
                std_json_str!(earning["operatingExpenses"]),
                std_json_str!(earning["investmentIncomeNet"]),
                std_json_str!(earning["netInterestIncome"]),
                std_json_str!(earning["interestIncome"]),
                std_json_str!(earning["interestExpense"]),
                std_json_str!(earning["nonInterestIncome"]),
                std_json_str!(earning["otherNonOperatingIncome"]),
                std_json_str!(earning["depreciation"]),
                std_json_str!(earning["depreciationAndAmortization"]),
                std_json_str!(earning["incomeBeforeTax"]),
                std_json_str!(earning["incomeTaxExpense"]),
                std_json_str!(earning["interestAndDebtExpense"]),
                std_json_str!(earning["netIncomeFromContinuingOperations"]),
                std_json_str!(earning["comprehensiveIncomeNetOfTax"]),
                std_json_str!(earning["ebit"]),
                std_json_str!(earning["ebitda"]),
                std_json_str!(earning["netIncome"]),
                &execution_time.to_string(),
            ]);
        }
    }

    wtr = writer_maker(prefix_2, symbol, execution_time);
    wtr.write_record([
        "fiscal_date_ending",
        "symbol",
        "reported_currency",
        "gross_profit",
        "total_revenue",
        "cost_of_revenue",
        "cost_of_goods_and_services_sold",
        "operating_income",
        "selling_general_and_administrative",
        "research_and_development",
        "operating_expenses",
        "investment_income_net",
        "net_interest_income",
        "interest_income",
        "interest_expense",
        "non_interest_income",
        "other_non_operating_income",
        "depreciation",
        "depreciation_and_amortization",
        "income_before_tax",
        "income_tax_expense",
        "interest_and_dept_expense",
        "net_income_from_continuting_operations",
        "comprehensive_income_net_of_tax",
        "earnings_before_interest_taxes",
        "earnings_befpre_interest_taxes_depreciation_amortization",
        "net_income",
        "execution_time"
    ])
    .expect("File Header Write Error (Quarterly Income Statement)");

    if let Some(earnings) = response["quarterlyReports"].as_array() {
        for earning in earnings {
            let _ = wtr.write_record([
                std_json_str!(earning["fiscalDateEnding"]),
                symbol,
                std_json_str!(earning["reportedCurrency"]),
                std_json_str!(earning["grossProfit"]),
                std_json_str!(earning["totalRevenue"]),
                std_json_str!(earning["costOfRevenue"]),
                std_json_str!(earning["costofGoodsAndServicesSold"]),
                std_json_str!(earning["operatingIncome"]),
                std_json_str!(earning["sellingGeneralAndAdministrative"]),
                std_json_str!(earning["researchAndDevelopment"]),
                std_json_str!(earning["operatingExpenses"]),
                std_json_str!(earning["investmentIncomeNet"]),
                std_json_str!(earning["netInterestIncome"]),
                std_json_str!(earning["interestIncome"]),
                std_json_str!(earning["interestExpense"]),
                std_json_str!(earning["nonInterestIncome"]),
                std_json_str!(earning["otherNonOperatingIncome"]),
                std_json_str!(earning["depreciation"]),
                std_json_str!(earning["depreciationAndAmortization"]),
                std_json_str!(earning["incomeBeforeTax"]),
                std_json_str!(earning["incomeTaxExpense"]),
                std_json_str!(earning["interestAndDebtExpense"]),
                std_json_str!(earning["netIncomeFromContinuingOperations"]),
                std_json_str!(earning["comprehensiveIncomeNetOfTax"]),
                std_json_str!(earning["ebit"]),
                std_json_str!(earning["ebitda"]),
                std_json_str!(earning["netIncome"]),
                &execution_time.to_string(),
            ]);
        }
    }

    Ok(())
}

pub fn balance_sheet_to_csv(response: serde_json::Value) -> Result<(), Error> {
    let prefix_1: &str = "annual_balance_sheet";
    let prefix_2: &str = "quarterly_balance_sheet";
    let symbol: &str = std_json_str!(response["symbol"]);
    let execution_time = Utc::now();
    let mut wtr: Writer<std::fs::File> = writer_maker(prefix_1, symbol, execution_time);

    wtr.write_record([
        "fiscal_date_ending",
        "symbol",
        "reported_currency",
        "total_assets",
        "total_current_assets",
        "cash_and_cash_equivalents_at_carrying_value",
        "cash_and_short_term_investments",
        "inventory",
        "current_net_receivables",
        "total_non_current_assets",
        "property_plant_equipment",
        "accumulated_depreciation_amortization_property_plant_equipment",
        "intangible_assets",
        "intangible_assets_excluding_goodwill",
        "goodwill",
        "investments",
        "long_term_investments",
        "short_term_investments",
        "other_current_assets",
        "other_non_current_assets",
        "total_liabilities",
        "total_current_liabilities",
        "current_accounts_payable",
        "deferred_revenue",
        "current_debt",
        "short_term_debt",
        "total_non_current_liabilities",
        "capital_lease_obligations",
        "long_term_debt",
        "current_long_term_debt",
        "long_term_debt_non_current",
        "short_long_term_debt_total",
        "other_current_liabilities",
        "other_non_current_liabilities",
        "total_shareholder_equity",
        "treasury_stock",
        "retained_earnings",
        "common_stock",
        "common_stock_shares_outstanding",
        "execution_time"
    ])
    .expect("File Header Write Error (Annual Balance Sheet)");

    if let Some(balances) = response["annualReports"].as_array() {
        for balance in balances {
            let _ = wtr.write_record([
                std_json_str!(balance["fiscalDateEnding"]),
                symbol,
                std_json_str!(balance["reportedCurrency"]),
                std_json_str!(balance["totalAssets"]),
                std_json_str!(balance["totalCurrentAssets"]),
                std_json_str!(balance["cashAndCashEquivalentsAtCarryingValue"]),
                std_json_str!(balance["cashAndShortTermInvestments"]),
                std_json_str!(balance["inventory"]),
                std_json_str!(balance["currentNetReceivables"]),
                std_json_str!(balance["totalNonCurrentAssets"]),
                std_json_str!(balance["propertyPlantEquipment"]),
                std_json_str!(balance["accumulatedDepreciationAmortizationPPE"]),
                std_json_str!(balance["intangibleAssets"]),
                std_json_str!(balance["intangibleAssetsExcludingGoodwill"]),
                std_json_str!(balance["goodwill"]),
                std_json_str!(balance["investments"]),
                std_json_str!(balance["longTermInvestments"]),
                std_json_str!(balance["shortTermInvestments"]),
                std_json_str!(balance["otherCurrentAssets"]),
                std_json_str!(balance["otherNonCurrentAssets"]),
                std_json_str!(balance["totalLiabilities"]),
                std_json_str!(balance["totalCurrentLiabilities"]),
                std_json_str!(balance["currentAccountsPayable"]),
                std_json_str!(balance["deferredRevenue"]),
                std_json_str!(balance["currentDebt"]),
                std_json_str!(balance["shortTermDebt"]),
                std_json_str!(balance["totalNonCurrentLiabilities"]),
                std_json_str!(balance["capitalLeaseObligations"]),
                std_json_str!(balance["longTermDebt"]),
                std_json_str!(balance["currentLongTermDebt"]),
                std_json_str!(balance["longTermDebtNoncurrent"]),
                std_json_str!(balance["shortLongTermDebtTotal"]),
                std_json_str!(balance["otherCurrentLiabilities"]),
                std_json_str!(balance["otherNonCurrentLiabilities"]),
                std_json_str!(balance["totalShareholderEquity"]),
                std_json_str!(balance["treasuryStock"]),
                std_json_str!(balance["retainedEarnings"]),
                std_json_str!(balance["commonStock"]),
                std_json_str!(balance["commonStockSharesOutstanding"]),
                &execution_time.to_string(),
            ]);
        }
    }

    wtr = writer_maker(prefix_2, symbol, execution_time);
    wtr.write_record([
        "fiscal_date_ending",
        "symbol",
        "reported_currency",
        "total_assets",
        "total_current_assets",
        "cash_and_cash_equivalents_at_carrying_value",
        "cash_and_short_term_investments",
        "inventory",
        "current_net_receivables",
        "total_non_current_assets",
        "property_plant_equipment",
        "accumulated_depreciation_amortization_property_plant_equipment",
        "intangible_assets",
        "intangible_assets_excluding_goodwill",
        "goodwill",
        "investments",
        "long_term_investments",
        "short_term_investments",
        "other_current_assets",
        "other_non_current_assets",
        "total_liabilities",
        "total_current_liabilities",
        "current_accounts_payable",
        "deferred_revenue",
        "current_debt",
        "short_term_debt",
        "total_non_current_liabilities",
        "capital_lease_obligations",
        "long_term_debt",
        "current_long_term_debt",
        "long_term_debt_non_current",
        "short_long_term_debt_total",
        "other_current_liabilities",
        "other_non_current_liabilities",
        "total_shareholder_equity",
        "treasury_stock",
        "retained_earnings",
        "common_stock",
        "common_stock_shares_outstanding",
        "execution_time"
    ])
    .expect("File Header Write Error (Quarterly Balance Sheet)");

    if let Some(balances) = response["quarterlyReports"].as_array() {
        for balance in balances {
            let _ = wtr.write_record([
                std_json_str!(balance["fiscalDateEnding"]),
                symbol,
                std_json_str!(balance["reportedCurrency"]),
                std_json_str!(balance["totalAssets"]),
                std_json_str!(balance["totalCurrentAssets"]),
                std_json_str!(balance["cashAndCashEquivalentsAtCarryingValue"]),
                std_json_str!(balance["cashAndShortTermInvestments"]),
                std_json_str!(balance["inventory"]),
                std_json_str!(balance["currentNetReceivables"]),
                std_json_str!(balance["totalNonCurrentAssets"]),
                std_json_str!(balance["propertyPlantEquipment"]),
                std_json_str!(balance["accumulatedDepreciationAmortizationPPE"]),
                std_json_str!(balance["intangibleAssets"]),
                std_json_str!(balance["intangibleAssetsExcludingGoodwill"]),
                std_json_str!(balance["goodwill"]),
                std_json_str!(balance["investments"]),
                std_json_str!(balance["longTermInvestments"]),
                std_json_str!(balance["shortTermInvestments"]),
                std_json_str!(balance["otherCurrentAssets"]),
                std_json_str!(balance["otherNonCurrentAssets"]),
                std_json_str!(balance["totalLiabilities"]),
                std_json_str!(balance["totalCurrentLiabilities"]),
                std_json_str!(balance["currentAccountsPayable"]),
                std_json_str!(balance["deferredRevenue"]),
                std_json_str!(balance["currentDebt"]),
                std_json_str!(balance["shortTermDebt"]),
                std_json_str!(balance["totalNonCurrentLiabilities"]),
                std_json_str!(balance["capitalLeaseObligations"]),
                std_json_str!(balance["longTermDebt"]),
                std_json_str!(balance["currentLongTermDebt"]),
                std_json_str!(balance["longTermDebtNoncurrent"]),
                std_json_str!(balance["shortLongTermDebtTotal"]),
                std_json_str!(balance["otherCurrentLiabilities"]),
                std_json_str!(balance["otherNonCurrentLiabilities"]),
                std_json_str!(balance["totalShareholderEquity"]),
                std_json_str!(balance["treasuryStock"]),
                std_json_str!(balance["retainedEarnings"]),
                std_json_str!(balance["commonStock"]),
                std_json_str!(balance["commonStockSharesOutstanding"]),
                &execution_time.to_string(),
            ]);
        }
    }

    Ok(())
}

pub fn cash_flow_to_csv(response: serde_json::Value) -> Result<(), Error> {
    let prefix_1: &str = "annual_cash_flow";
    let prefix_2: &str = "quarterly_cash_flow";
    let symbol: &str = std_json_str!(response["symbol"]);
    let execution_time = Utc::now();
    let mut wtr: Writer<std::fs::File> = writer_maker(prefix_1, symbol, execution_time);

    wtr.write_record([
        "fiscal_date_ending",
        "symbol",
        "reported_currency",
        "operating_cashflow",
        "payments_for_operating_activities",
        "proceeds_from_operating_activities",
        "change_in_operating_liabilities",
        "change_in_operating_assets",
        "depreciation_depletion_and_amortization",
        "captial_expenditures",
        "change_in_receivables",
        "change_in_inventory",
        "profit_loss",
        "cashflow_from_investment",
        "cashflow_from_financing",
        "proceeds_from_repayments_of_short_term_debt",
        "payments_for_repurchase_of_common_stock",
        "payments_for_repurchase_of_equity",
        "payments_for_repurchase_of_preferred_stock",
        "dividend_payout",
        "dividend_payout_common_stock",
        "dividend_payout_preferred_stock",
        "proceeds_from_issuance_of_common_stock",
        "proceeds_from_issuance_of_long_term_debt_and_capital_securities_net",
        "proceeds_from_issuance_of_preferred_stock",
        "proceeds_from_repurchase_of_equity",
        "proceeds_from_sale_of_treasury_stock",
        "change_in_cash_and_cash_equivalents",
        "change_in_exchange_rate",
        "net_income",
        "execution_time"
    ])
    .expect("File Header Write Error (Annual Cash Flow)");

    if let Some(cash_flows) = response["annualReports"].as_array() {
        for cash_flow in cash_flows {
            let _ = wtr.write_record([
                std_json_str!(cash_flow["fiscalDateEnding"]),
                symbol,
                std_json_str!(cash_flow["reportedCurrency"]),
                std_json_str!(cash_flow["operatingCashflow"]),
                std_json_str!(cash_flow["paymentsForOperatingActivities"]),
                std_json_str!(cash_flow["proceedsFromOperatingActivities"]),
                std_json_str!(cash_flow["changeInOperatingLiabilities"]),
                std_json_str!(cash_flow["changeInOperatingAssets"]),
                std_json_str!(cash_flow["depreciationDepletionAndAmortization"]),
                std_json_str!(cash_flow["capitalExpenditures"]),
                std_json_str!(cash_flow["changeInReceivables"]),
                std_json_str!(cash_flow["changeInInventory"]),
                std_json_str!(cash_flow["profitLoss"]),
                std_json_str!(cash_flow["cashflowFromInvestment"]),
                std_json_str!(cash_flow["cashflowFromFinancing"]),
                std_json_str!(cash_flow["proceedsFromRepaymentsOfShortTermDebt"]),
                std_json_str!(cash_flow["paymentsForRepurchaseOfCommonStock"]),
                std_json_str!(cash_flow["paymentsForRepurchaseOfEquity"]),
                std_json_str!(cash_flow["paymentsForRepurchaseOfPreferredStock"]),
                std_json_str!(cash_flow["dividendPayout"]),
                std_json_str!(cash_flow["dividendPayoutCommonStock"]),
                std_json_str!(cash_flow["dividendPayoutPreferredStock"]),
                std_json_str!(cash_flow["proceedsFromIssuanceOfCommonStock"]),
                std_json_str!(cash_flow["proceedsFromIssuanceOfLongTermDebtAndCapitalSecuritiesNet"]),
                std_json_str!(cash_flow["proceedsFromIssuanceOfPreferredStock"]),
                std_json_str!(cash_flow["proceedsFromRepurchaseOfEquity"]),
                std_json_str!(cash_flow["proceedsFromSaleOfTreasuryStock"]),
                std_json_str!(cash_flow["changeInCashAndCashEquivalents"]),
                std_json_str!(cash_flow["changeInExchangeRate"]),
                std_json_str!(cash_flow["netIncome"]),
                &execution_time.to_string(),
            ]);
        }
    }

    wtr = writer_maker(prefix_2, symbol, execution_time);
    wtr.write_record([
        "fiscal_date_ending",
        "symbol",
        "reported_currency",
        "operating_cashflow",
        "payments_for_operating_activities",
        "proceeds_from_operating_activities",
        "change_in_operating_liabilities",
        "change_in_operating_assets",
        "depreciation_depletion_and_amortization",
        "captial_expenditures",
        "change_in_receivables",
        "change_in_inventory",
        "profit_loss",
        "cashflow_from_investment",
        "cashflow_from_financing",
        "proceeds_from_repayments_of_short_term_debt",
        "payments_for_repurchase_of_common_stock",
        "payments_for_repurchase_of_equity",
        "payments_for_repurchase_of_preferred_stock",
        "dividend_payout",
        "dividend_payout_common_stock",
        "dividend_payout_preferred_stock",
        "proceeds_from_issuance_of_common_stock",
        "proceeds_from_issuance_of_long_term_debt_and_capital_securities_net",
        "proceeds_from_issuance_of_preferred_stock",
        "proceeds_from_repurchase_of_equity",
        "proceeds_from_sale_of_treasury_stock",
        "change_in_cash_and_cash_equivalents",
        "change_in_exchange_rate",
        "net_income",
        "execution_time"
    ])
    .expect("File Header Write Error (Quarterly Cash Flow)");

    if let Some(cash_flows) = response["quarterlyReports"].as_array() {
        for cash_flow in cash_flows {
            let _ = wtr.write_record([
                std_json_str!(cash_flow["fiscalDateEnding"]),
                symbol,
                std_json_str!(cash_flow["reportedCurrency"]),
                std_json_str!(cash_flow["operatingCashflow"]),
                std_json_str!(cash_flow["paymentsForOperatingActivities"]),
                std_json_str!(cash_flow["proceedsFromOperatingActivities"]),
                std_json_str!(cash_flow["changeInOperatingLiabilities"]),
                std_json_str!(cash_flow["changeInOperatingAssets"]),
                std_json_str!(cash_flow["depreciationDepletionAndAmortization"]),
                std_json_str!(cash_flow["capitalExpenditures"]),
                std_json_str!(cash_flow["changeInReceivables"]),
                std_json_str!(cash_flow["changeInInventory"]),
                std_json_str!(cash_flow["profitLoss"]),
                std_json_str!(cash_flow["cashflowFromInvestment"]),
                std_json_str!(cash_flow["cashflowFromFinancing"]),
                std_json_str!(cash_flow["proceedsFromRepaymentsOfShortTermDebt"]),
                std_json_str!(cash_flow["paymentsForRepurchaseOfCommonStock"]),
                std_json_str!(cash_flow["paymentsForRepurchaseOfEquity"]),
                std_json_str!(cash_flow["paymentsForRepurchaseOfPreferredStock"]),
                std_json_str!(cash_flow["dividendPayout"]),
                std_json_str!(cash_flow["dividendPayoutCommonStock"]),
                std_json_str!(cash_flow["dividendPayoutPreferredStock"]),
                std_json_str!(cash_flow["proceedsFromIssuanceOfCommonStock"]),
                std_json_str!(cash_flow["proceedsFromIssuanceOfLongTermDebtAndCapitalSecuritiesNet"]),
                std_json_str!(cash_flow["proceedsFromIssuanceOfPreferredStock"]),
                std_json_str!(cash_flow["proceedsFromRepurchaseOfEquity"]),
                std_json_str!(cash_flow["proceedsFromSaleOfTreasuryStock"]),
                std_json_str!(cash_flow["changeInCashAndCashEquivalents"]),
                std_json_str!(cash_flow["changeInExchangeRate"]),
                std_json_str!(cash_flow["netIncome"]),
                &execution_time.to_string(),
            ]);
        }
    }

    Ok(())
}