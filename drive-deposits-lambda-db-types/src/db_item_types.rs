// Item is used in DynamoDB to format data for write items so could be queries using Single Table Design
// Here we are converting to specific format for write items so could be queries using Single Table Design

// access patterns determine types and the structure of the data

use drive_deposits_rest_types::rest_types::CalculatePortfolioResponse;
use serde::Serialize;

// ResponseLevelItem means deals with responses so RESPONSES as pk has many response items
#[derive(Debug, Default, Serialize)]
pub struct PortfolioLevelItem {
    pub pk_portfolios: String,
    pub sk_format_portfolio_delta_growth: String,
    pub portfolio_uuid: String,
    pub outcome_as_json: String,
    pub created_at: String,
}

#[derive(Debug, Default)]
pub struct BankLevelItemsWrapper {
    pub items: Vec<BankLevelItem>,
}

// BankLevelItem means deals with banks for a specific response uuid so RESPONSE#UUID#{} as pk has many bank items
#[derive(Debug, Default)]
pub struct BankLevelItem {
    pub pk_format_portfolio_uuid: String,
    pub sk_format_bank_delta_growth: String,
    pub bank_uuid: String,
    pub bank_name: String,
    pub portfolio_uuid: String,
    pub bank_tz: String,
    pub outcome_as_json: String,
    pub created_at: String,
}

#[derive(Debug, Default)]
pub struct DepositLevelItemsWrapper {
    pub deposit_level_items: Vec<DepositLevelItem>,
}

// BanksDepositLevelItem means deals with bank deposits for a specific response uuid so RESPONSE#UUID#{} as pk has many bank-deposits items

#[derive(Debug, Default)]
pub enum DepositSortCriteria {
    #[default]
    DeltaPeriodGrowth,
    MaturityDate,
}

#[derive(Debug, Default)]
pub struct CalculatePortfolioRestWrapper {
    pub calculate_portfolio_response: CalculatePortfolioResponse,
    pub deposit_sort_criteria: DepositSortCriteria,
}

#[derive(Debug, Default)]
pub struct DepositLevelItem {
    pub pk_format_portfolio_uuid: String,
    pub sk_format_deposit_sort_criteria: String,
    pub deposit_delta_growth: String,
    pub deposit_maturity_date_in_bank_tz: String,
    pub created_at: String,
    pub bank_uuid: String,
    pub bank_name: String,
    pub portfolio_uuid: String,
    pub deposit_uuid: String,
    pub account: String,
    pub account_type: String,
    pub apy: String,
    pub years: String,
    pub outcome_as_json: String,
    pub outcome_with_dates_as_json: String,
}
