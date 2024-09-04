use drive_deposits_rest_types::rest_types::{Outcome, OutcomeWithDates};
use serde::{Deserialize, Serialize};

pub const DEFAULT_TOP_K: usize = 2;
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ItemParamsRequest {
    pub order: Option<Order>,
    pub top_k: Option<usize>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    #[default]
    #[serde(rename = "desc")]
    Descending,
    #[serde(rename = "asc")]
    Ascending,
}

impl From<&Order> for bool {
    fn from(order: &Order) -> bool {
        match order {
            Order::Ascending => true,
            Order::Descending => false,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct Metadata {
    pub order: Order,
    pub top_k: usize,
    pub sort_criteria_description: String,
}

// REST API response: Calculation results sorted by delta period growth at the portfolios level
// delta period growth is the default order criteria
#[derive(Debug, Default, Serialize)]
pub struct ByLevelForPortfolios {
    pub data: PortfolioData,
    pub metadata: Metadata,
}

#[derive(Debug, Default, Serialize)]
pub struct PortfolioData {
    pub responses: Vec<PortfolioResponse>,
    pub count_responses: usize,
}

#[derive(Debug, Default, Serialize)]
pub struct PortfolioResponse {
    pub portfolio_uuid: String,
    pub outcome: Outcome,
    pub created_at: String,
}

//  REST API response: Calculation results sorted by delta period growth at the banks level for a given portfolio
// delta period growth is the default order criteria
#[derive(Debug, Default, Serialize)]
pub struct ByLevelForBanks {
    pub data: BankData,
    pub metadata: Metadata,
}

#[derive(Debug, Default, Serialize)]
pub struct BankData {
    pub responses: Vec<BankResponse>,
    pub count_responses: usize,
}

#[derive(Debug, Default, Serialize)]
pub struct BankResponse {
    pub bank_uuid: String,
    pub bank_name: String,
    pub portfolio_uuid: String,
    pub bank_tz: String,
    pub outcome: Outcome,
    pub created_at: String,
}

// handles 2 rest api responses at the deposits level:
//  REST API response: Calculation results sorted by delta period growth at the deposits level for all banks for a given portfolio
//  REST API response: Calculation results sorted by maturity date at the deposits level for all banks for a given portfolio
#[derive(Debug, Default, Serialize)]
pub struct ByLevelForDeposits {
    pub data: DepositData,
    pub metadata: Metadata,
}

#[derive(Debug, Default, Serialize)]
pub struct DepositData {
    pub responses: Vec<DepositResponse>,
    pub count_responses: usize,
}

#[derive(Debug, Default, Serialize)]
pub struct DepositResponse {
    pub deposit_uuid: String,
    pub bank_uuid: String,
    pub bank_name: String,
    pub portfolio_uuid: String,
    pub account: String,
    pub account_type: String,
    pub apy: String,
    pub years: String,
    pub deposit_delta_growth: String,
    pub deposit_maturity_date_in_bank_tz: String,
    pub outcome: Outcome,
    pub outcome_with_dates: OutcomeWithDates,
    pub created_at: String,
}
