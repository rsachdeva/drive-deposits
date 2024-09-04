use chrono::NaiveDate;
use chrono_tz::Tz;
use rust_decimal::Decimal;
use uuid::Uuid;

use drive_deposits_proto_grpc_types::generated::{AccountType, PeriodUnit};

// Request sections
#[derive(Debug)]
pub struct PortfolioRequest {
    pub new_banks: Vec<NewBank>,
    pub new_delta: NewDelta,
}

#[derive(Debug, Clone)]
pub struct NewDelta {
    pub period: Decimal,
    pub period_unit: PeriodUnit,
}

#[derive(Debug)]
pub struct NewBank {
    pub name: String,
    pub bank_tz: Tz,
    pub new_deposits: Vec<NewDeposit>,
}

#[derive(Debug)]
pub struct NewDeposit {
    pub account: String,
    pub account_type: AccountType,
    pub apy: Decimal,
    pub years: Decimal,
    pub amount: Decimal,
    pub start_date_in_bank_tz: NaiveDate,
}

// Response sections
#[derive(Debug, Clone)]
pub struct PortfolioResponse {
    pub uuid: Uuid,
    pub banks: Vec<Bank>,
    pub outcome: Option<Outcome>,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct Delta {
    pub period: Decimal,
    pub period_unit: PeriodUnit,
    pub growth: Decimal,
}

#[derive(Debug, Clone)]
pub struct Maturity {
    pub amount: Decimal,
    pub interest: Decimal,
    pub total: Decimal,
}

#[derive(Debug, Clone)]
pub struct Bank {
    pub uuid: Uuid,
    pub name: String,
    pub bank_tz: Tz,
    pub deposits: Vec<Deposit>,
    pub outcome: Option<Outcome>,
}

#[derive(Debug, Clone)]
pub struct Deposit {
    pub uuid: Uuid,
    pub account: String,
    pub account_type: AccountType,
    pub apy: Decimal,
    pub years: Decimal,
    pub outcome: Option<Outcome>,
    pub outcome_with_dates: Option<OutcomeWithDates>,
}

#[derive(Debug, Clone)]
pub struct Outcome {
    pub delta: Option<Delta>,
    pub maturity: Option<Maturity>,
    pub errors: Vec<ProcessingError>,
}

#[derive(Debug, Clone)]
pub struct OutcomeWithDates {
    pub start_date_in_bank_tz: NaiveDate,
    pub maturity_date_in_bank_tz: Option<NaiveDate>,
    pub errors: Vec<ProcessingError>,
}

#[derive(Debug, Clone)]
pub struct ProcessingError {
    pub uuid: Uuid,
    pub message: String,
}
