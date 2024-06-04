// Note: I know this is a fucking awful way of doing this but it's 10PM and I need a pint.

pub fn time_series_daily_queries(file_path: String) -> (String, String) {
    let queries: (String, String) = (
        "create table if not exists landing.temp_time_series_daily(daily_price_date text, symbol text, open text, high text, low text, close text, volume text, execution_time text)".to_string()
        , format!("copy landing.temp_time_series_daily(daily_price_date,symbol,open,high,low,close,volume,execution_time) from '{}' delimiter ',' csv header"
        , file_path )
    );

    queries
}

pub fn overview_queries(file_path: String) -> (String, String) {
    let queries: (String, String) = (
        "create table if not exists landing.temp_overview(symbol text, asset_type text, name text, description text, central_index_key text, exchange text, currency text, country text, sector text, industry text, address text, fiscal_year_end text, latest_quarter text, market_capitalization text, interest_before_interest_taxes_amortization text, price_earnings_ratio text, price_earnings_growth_ratio text, book_value text, dividend_per_share text, dividend_yield text, earnings_per_share text, revenue_per_share_trailing_twelve_months text, profit_margin text, operating_margin_trailing_twelve_months text, return_on_assets_trailing_twelve_months text, return_on_equity_trailing_twelve_months text, revenue_trailing_twelve_months text, gross_profit_trailing_twelve_months text, diluted_earnings_per_share_trailing_twelve_months text, quarterly_earnings_growth_year_on_year text, quarterly_revenue_growth_year_on_year text, analyst_target_price text, trailing_price_to_earnings_ratio text, forward_price_to_earnings_ratio text, price_to_sales_ratio_trailing_twelve_months text, price_to_book_ratio text, enterprise_value_to_revenue_ratio text, enterprise_value_to_interest_before_interest_taxes_amortization_ratio text, beta text, fifty_two_week_high text, fifty_two_week_low text, fifty_day_moving_average text, two_hundred_day_moving_average text, shares_outstanding text, dividend_date text, ex_dividend_date text, execution_time text)".to_string()
        , format!("copy landing.temp_overview(symbol,asset_type,name,description,central_index_key,exchange,currency,country,sector,industry,address,fiscal_year_end,latest_quarter,market_capitalization,interest_before_interest_taxes_amortization,price_earnings_ratio,price_earnings_growth_ratio,book_value,dividend_per_share,dividend_yield,earnings_per_share,revenue_per_share_trailing_twelve_months,profit_margin,operating_margin_trailing_twelve_months,return_on_assets_trailing_twelve_months,return_on_equity_trailing_twelve_months,revenue_trailing_twelve_months,gross_profit_trailing_twelve_months,diluted_earnings_per_share_trailing_twelve_months,quarterly_earnings_growth_year_on_year,quarterly_revenue_growth_year_on_year,analyst_target_price,trailing_price_to_earnings_ratio,forward_price_to_earnings_ratio,price_to_sales_ratio_trailing_twelve_months,price_to_book_ratio,enterprise_value_to_revenue_ratio,enterprise_value_to_interest_before_interest_taxes_amortization_ratio,beta,fifty_two_week_high,fifty_two_week_low,fifty_day_moving_average,two_hundred_day_moving_average,shares_outstanding,dividend_date,ex_dividend_date,execution_time) from '{}' delimiter ',' csv header"
        , file_path )
    );

    queries
}

pub fn annual_earnings_queries(file_path: String) -> (String, String) {
    let queries: (String, String) = (
        "create table if not exists landing.temp_annual_earnings(fiscal_date_ending text, symbol text, reported_earnings_per_share text, execution_time text)".to_string()
        , format!("copy landing.temp_annual_earnings(fiscal_date_ending,symbol,reported_earnings_per_share,execution_time) from '{}' delimiter ',' csv header"
        , file_path )
    );

    queries
}

pub fn annual_income_statement_queries(file_path: String) -> (String, String) {
    let queries: (String, String) = (
        "create table if not exists landing.temp_annual_income_statement(fiscal_date_ending text, symbol text, reported_currency text, gross_profit text, total_revenue text, cost_of_revenue text, cost_of_goods_and_services_sold text, operating_income text, selling_general_and_administrative text, research_and_development text, operating_expenses text, investment_income_net text, net_interest_income text, interest_income text, interest_expense text, non_interest_income text, other_non_operating_income text, depreciation text, depreciation_and_amortization text, income_before_tax text, income_tax_expense text, interest_and_dept_expense text, net_income_from_continuting_operations text, comprehensive_income_net_of_tax text, earnings_before_interest_taxes text, earnings_before_interest_taxes_depreciation_amortization text, net_income text, execution_time text)".to_string()
        , format!("copy landing.temp_annual_income_statement(fiscal_date_ending,symbol,reported_currency,gross_profit,total_revenue,cost_of_revenue,cost_of_goods_and_services_sold,operating_income,selling_general_and_administrative,research_and_development,operating_expenses,investment_income_net,net_interest_income,interest_income,interest_expense,non_interest_income,other_non_operating_income,depreciation,depreciation_and_amortization,income_before_tax,income_tax_expense,interest_and_dept_expense,net_income_from_continuting_operations,comprehensive_income_net_of_tax,earnings_before_interest_taxes,earnings_before_interest_taxes_depreciation_amortization,net_income,execution_time) from '{}' delimiter ',' csv header"
        , file_path )
    );

    queries
}

pub fn annual_balance_sheet_queries(file_path: String) -> (String, String) {
    let queries: (String, String) = (
        "create table if not exists landing.temp_annual_balance_sheet(fiscal_date_ending text, symbol text, reported_currency text, total_assets text, total_current_assets text, cash_and_cash_equivalents_at_carrying_value text, cash_and_short_term_investments text, inventory text, current_net_receivables text, total_non_current_assets text, property_plant_equipment text, accumulated_depreciation_amortization_property_plant_equipment text, intangible_assets text, intangible_assets_excluding_goodwill text, goodwill text, investments text, long_term_investments text, short_term_investments text, other_current_assets text, other_non_current_assets text, total_liabilities text, total_current_liabilities text, current_accounts_payable text, deferred_revenue text, current_debt text, short_term_debt text, total_non_current_liabilities text, capital_lease_obligations text, long_term_debt text, current_long_term_debt text, long_term_debt_non_current text, short_long_term_debt_total text, other_current_liabilities text, other_non_current_liabilities text, total_shareholder_equity text, treasury_stock text, retained_earnings text, common_stock text, common_stock_shares_outstanding text, execution_time text)".to_string()
        , format!("copy landing.temp_annual_balance_sheet(fiscal_date_ending,symbol,reported_currency,total_assets,total_current_assets,cash_and_cash_equivalents_at_carrying_value,cash_and_short_term_investments,inventory,current_net_receivables,total_non_current_assets,property_plant_equipment,accumulated_depreciation_amortization_property_plant_equipment,intangible_assets,intangible_assets_excluding_goodwill,goodwill,investments,long_term_investments,short_term_investments,other_current_assets,other_non_current_assets,total_liabilities,total_current_liabilities,current_accounts_payable,deferred_revenue,current_debt,short_term_debt,total_non_current_liabilities,capital_lease_obligations,long_term_debt,current_long_term_debt,long_term_debt_non_current,short_long_term_debt_total,other_current_liabilities,other_non_current_liabilities,total_shareholder_equity,treasury_stock,retained_earnings,common_stock,common_stock_shares_outstanding,execution_time) from '{}' delimiter ',' csv header"
        , file_path )
    );

    queries
}

pub fn annual_cash_flow_queries(file_path: String) -> (String, String) {
    let queries: (String, String) = (
        "create table if not exists landing.temp_annual_cash_flow(fiscal_date_ending text, symbol text, reported_currency text, operating_cashflow text, payments_for_operating_activities text, proceeds_from_operating_activities text, change_in_operating_liabilities text, change_in_operating_assets text, depreciation_depletion_and_amortization text, captial_expenditures text, change_in_receivables text, change_in_inventory text, profit_loss text, cashflow_from_investment text, cashflow_from_financing text, proceeds_from_repayments_of_short_term_debt text, payments_for_repurchase_of_common_stock text, payments_for_repurchase_of_equity text, payments_for_repurchase_of_preferred_stock text, dividend_payout text, dividend_payout_common_stock text, dividend_payout_preferred_stock text, proceeds_from_issuance_of_common_stock text, proceeds_from_issuance_of_long_term_debt_and_capital_securities_net text, proceeds_from_issuance_of_preferred_stock text, proceeds_from_repurchase_of_equity text, proceeds_from_sale_of_treasury_stock text, change_in_cash_and_cash_equivalents text, change_in_exchange_rate text, net_income text, execution_time text)".to_string()
        , format!("copy landing.temp_annual_cash_flow(fiscal_date_ending,symbol,reported_currency,operating_cashflow,payments_for_operating_activities,proceeds_from_operating_activities,change_in_operating_liabilities,change_in_operating_assets,depreciation_depletion_and_amortization,captial_expenditures,change_in_receivables,change_in_inventory,profit_loss,cashflow_from_investment,cashflow_from_financing,proceeds_from_repayments_of_short_term_debt,payments_for_repurchase_of_common_stock,payments_for_repurchase_of_equity,payments_for_repurchase_of_preferred_stock,dividend_payout,dividend_payout_common_stock,dividend_payout_preferred_stock,proceeds_from_issuance_of_common_stock,proceeds_from_issuance_of_long_term_debt_and_capital_securities_net,proceeds_from_issuance_of_preferred_stock,proceeds_from_repurchase_of_equity,proceeds_from_sale_of_treasury_stock,change_in_cash_and_cash_equivalents,change_in_exchange_rate,net_income,execution_time) from '{}' delimiter ',' csv header"
        , file_path )
    );

    queries
}

pub fn quarterly_earnings_queries(file_path: String) -> (String, String) {
    let queries: (String, String) = (
        "create table if not exists landing.temp_quarterly_earnings(fiscal_date_ending text, symbol text, reported_date text, reported_earnings_per_share text, estimated_earnings_per_share text, surprise text, surprise_percentage text, execution_time text)".to_string()
        , format!("copy landing.temp_quarterly_earnings(fiscal_date_ending,symbol,reported_date,reported_earnings_per_share,estimated_earnings_per_share,surprise,surprise_percentage,execution_time) from '{}' delimiter ',' csv header"
        , file_path )
    );

    queries
}

pub fn quarterly_income_statement_queries(file_path: String) -> (String, String) {
    let queries: (String, String) = (
        "create table if not exists landing.temp_quarterly_income_statement(fiscal_date_ending text, symbol text, reported_currency text, gross_profit text, total_revenue text, cost_of_revenue text, cost_of_goods_and_services_sold text, operating_income text, selling_general_and_administrative text, research_and_development text, operating_expenses text, investment_income_net text, net_interest_income text, interest_income text, interest_expense text, non_interest_income text, other_non_operating_income text, depreciation text, depreciation_and_amortization text, income_before_tax text, income_tax_expense text, interest_and_dept_expense text, net_income_from_continuting_operations text, comprehensive_income_net_of_tax text, earnings_before_interest_taxes text, earnings_before_interest_taxes_depreciation_amortization text, net_income text, execution_time text)".to_string()
        , format!("copy landing.temp_quarterly_income_statement(fiscal_date_ending,symbol,reported_currency,gross_profit,total_revenue,cost_of_revenue,cost_of_goods_and_services_sold,operating_income,selling_general_and_administrative,research_and_development,operating_expenses,investment_income_net,net_interest_income,interest_income,interest_expense,non_interest_income,other_non_operating_income,depreciation,depreciation_and_amortization,income_before_tax,income_tax_expense,interest_and_dept_expense,net_income_from_continuting_operations,comprehensive_income_net_of_tax,earnings_before_interest_taxes,earnings_before_interest_taxes_depreciation_amortization,net_income,execution_time) from '{}' delimiter ',' csv header"
        , file_path )
    );

    queries
}

pub fn quarterly_balance_sheet_queries(file_path: String) -> (String, String) {
    let queries: (String, String) = (
        "create table if not exists landing.temp_quarterly_balance_sheet(fiscal_date_ending text, symbol text, reported_currency text, total_assets text, total_current_assets text, cash_and_cash_equivalents_at_carrying_value text, cash_and_short_term_investments text, inventory text, current_net_receivables text, total_non_current_assets text, property_plant_equipment text, accumulated_depreciation_amortization_property_plant_equipment text, intangible_assets text, intangible_assets_excluding_goodwill text, goodwill text, investments text, long_term_investments text, short_term_investments text, other_current_assets text, other_non_current_assets text, total_liabilities text, total_current_liabilities text, current_accounts_payable text, deferred_revenue text, current_debt text, short_term_debt text, total_non_current_liabilities text, capital_lease_obligations text, long_term_debt text, current_long_term_debt text, long_term_debt_non_current text, short_long_term_debt_total text, other_current_liabilities text, other_non_current_liabilities text, total_shareholder_equity text, treasury_stock text, retained_earnings text, common_stock text, common_stock_shares_outstanding text, execution_time text)".to_string()
        , format!("copy landing.temp_quarterly_balance_sheet(fiscal_date_ending,symbol,reported_currency,total_assets,total_current_assets,cash_and_cash_equivalents_at_carrying_value,cash_and_short_term_investments,inventory,current_net_receivables,total_non_current_assets,property_plant_equipment,accumulated_depreciation_amortization_property_plant_equipment,intangible_assets,intangible_assets_excluding_goodwill,goodwill,investments,long_term_investments,short_term_investments,other_current_assets,other_non_current_assets,total_liabilities,total_current_liabilities,current_accounts_payable,deferred_revenue,current_debt,short_term_debt,total_non_current_liabilities,capital_lease_obligations,long_term_debt,current_long_term_debt,long_term_debt_non_current,short_long_term_debt_total,other_current_liabilities,other_non_current_liabilities,total_shareholder_equity,treasury_stock,retained_earnings,common_stock,common_stock_shares_outstanding,execution_time) from '{}' delimiter ',' csv header"
        , file_path )
    );

    queries
}

pub fn quarterly_cash_flow_queries(file_path: String) -> (String, String) {
    let queries: (String, String) = (
        "create table if not exists landing.temp_quarterly_cash_flow(fiscal_date_ending text, symbol text, reported_currency text, operating_cashflow text, payments_for_operating_activities text, proceeds_from_operating_activities text, change_in_operating_liabilities text, change_in_operating_assets text, depreciation_depletion_and_amortization text, captial_expenditures text, change_in_receivables text, change_in_inventory text, profit_loss text, cashflow_from_investment text, cashflow_from_financing text, proceeds_from_repayments_of_short_term_debt text, payments_for_repurchase_of_common_stock text, payments_for_repurchase_of_equity text, payments_for_repurchase_of_preferred_stock text, dividend_payout text, dividend_payout_common_stock text, dividend_payout_preferred_stock text, proceeds_from_issuance_of_common_stock text, proceeds_from_issuance_of_long_term_debt_and_capital_securities_net text, proceeds_from_issuance_of_preferred_stock text, proceeds_from_repurchase_of_equity text, proceeds_from_sale_of_treasury_stock text, change_in_cash_and_cash_equivalents text, change_in_exchange_rate text, net_income text, execution_time text)".to_string()
        , format!("copy landing.temp_quarterly_cash_flow(fiscal_date_ending,symbol,reported_currency,operating_cashflow,payments_for_operating_activities,proceeds_from_operating_activities,change_in_operating_liabilities,change_in_operating_assets,depreciation_depletion_and_amortization,captial_expenditures,change_in_receivables,change_in_inventory,profit_loss,cashflow_from_investment,cashflow_from_financing,proceeds_from_repayments_of_short_term_debt,payments_for_repurchase_of_common_stock,payments_for_repurchase_of_equity,payments_for_repurchase_of_preferred_stock,dividend_payout,dividend_payout_common_stock,dividend_payout_preferred_stock,proceeds_from_issuance_of_common_stock,proceeds_from_issuance_of_long_term_debt_and_capital_securities_net,proceeds_from_issuance_of_preferred_stock,proceeds_from_repurchase_of_equity,proceeds_from_sale_of_treasury_stock,change_in_cash_and_cash_equivalents,change_in_exchange_rate,net_income,execution_time) from '{}' delimiter ',' csv header"
        , file_path )
    );

    queries
}
