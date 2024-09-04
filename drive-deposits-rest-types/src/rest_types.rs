use std::str::FromStr;

use chrono::NaiveDate;
use chrono_tz::Tz;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use validator::Validate;
use validator::ValidationError;

// Request sections
#[derive(Default, Debug, Deserialize, Validate)]
pub struct CalculatePortfolioRequest {
    #[validate(length(min = 1), nested)]
    pub new_banks: Vec<NewBank>,
    #[validate(nested)]
    pub new_delta: NewDelta,
}

#[derive(Default, Debug, Deserialize, Validate)]
pub struct NewDelta {
    #[validate(length(min = 1), custom(function = "validate_positive_decimal"))]
    pub period: String,
    #[validate(custom(function = "validate_period_unit"))]
    pub period_unit: String,
}

// declarative programming -- functional programming inspired
// #[derive(Default)]: Automatically implements the Default trait for the enum. The Default trait allows you to create a default value for the enum. For enums, the first variant is considered the default unless specified otherwise with the #[default] attribute on one of the variants.
// #[derive(Debug)]: Implements the Debug trait to enable formatting using the {:?} formatter.
// #[derive(Deserialize)]: Derives the Deserialize trait from the serde crate, enabling your enum to be deserialized from formats like JSON.
// #[derive(EnumString)]:Converts strings to enum variants based on their name.auto-derives std::str::FromStr on the enum and std::convert::TryFrom<&str> will be derived as well per docs
// #[derive(Serialize) : since used in response as well
#[derive(Default, Debug, Deserialize, EnumString, Serialize)]
pub enum DeltaPeriodUnit {
    #[default]
    Unspecified = 0,
    Day = 1,
    Week = 2,
    Month = 3,
    Year = 4,
}

fn validate_period_unit(period_unit: &str) -> Result<(), ValidationError> {
    // DeltaPeriodUnit::from_str(period_unit).map_err(|_| {
    //     let mut error = ValidationError::new("invalid_period_unit");
    //     error.message = Some(
    //         format!(
    //             "Incorrect period_unit: {}. Invalid period unit. Must be Day, Week, Month, or Year.\n",
    //             period_unit
    //         )
    //         .into(),
    //     );
    //     error
    // })?;

    // uses FromStr implementation to parse the string into the enum variant: fn from_str(s: &str) -> Result<Self, Self::Err>;
    // converting from strum parse error to Validation error without thisError crate since no custom error defined here
    period_unit.parse::<DeltaPeriodUnit>().map_err(|_| {
        let mut error = ValidationError::new("invalid_period_unit");
        error.message = Some(
            format!(
                "Incorrect period_unit: {}. Must be Day, Week, Month, or Year.\n",
                period_unit
            )
            .into(),
        );
        error
    })?;
    Ok(())
}

#[derive(Default, Debug, Deserialize, Validate, Serialize)]
pub struct NewBank {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(custom(function = "validate_bank_tz"))]
    pub bank_tz: String,
    // requires Serialize trait because it needs to be able to convert the NewDeposit instances into a serializable format for error reporting.
    #[validate(length(min = 1), nested)]
    pub new_deposits: Vec<NewDeposit>,
}

fn validate_bank_tz(bank_tz: &str) -> Result<(), ValidationError> {
    bank_tz.parse::<Tz>().map_err(|e| {
        let mut error = ValidationError::new("invalid_tz");
        error.message = Some(
            format!(
                "Error: {}. Incorrect timezone: {}. Must be a valid timezone.\n",
                e, bank_tz
            )
            .into(),
        );
        error
    })?;
    Ok(())
}
#[derive(Default, Debug, Deserialize, Validate, Serialize)]
pub struct NewDeposit {
    #[validate(length(min = 4))]
    pub account: String,
    #[validate(custom(function = "validate_account_type"))]
    pub account_type: String,
    #[validate(custom(function = "validate_decimal"))]
    pub apy: String,
    #[validate(custom(function = "validate_positive_decimal"))]
    pub years: String,
    #[validate(custom(function = "validate_decimal"))]
    pub amount: String,
    #[validate(custom(function = "validate_iso8601_date"))]
    pub start_date_in_bank_tz: String,
}

fn validate_iso8601_date(date_str: &str) -> Result<(), ValidationError> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|e| {
        let mut error = ValidationError::new("invalid_iso8601_datetime");
        error.message = Some(
            format!(
                "Error: {}. Incorrect date format: {}. Must be in ISO 8601 format YYYY-MM-DD.\n",
                e, date_str
            )
            .into(),
        );
        error
    })?;
    Ok(())
}

#[derive(Default, Deserialize, Debug, EnumString)]
pub enum AccountType {
    #[default]
    Unspecified = 0,
    Checking = 1,
    Savings = 2,
    CertificateOfDeposit = 3,
    BrokerageCertificateOfDeposit = 4,
}

fn validate_account_type(account_type: &str) -> Result<(), ValidationError> {
    AccountType::from_str(account_type).map_err(|e| {
        let mut error = ValidationError::new("invalid_account_type");
        error.message = Some(
            format!(
                "Error: {}. Incorrect account_type: {}. Must be Checking, Savings, CertificateOfDeposit, or BrokerageCertificateOfDeposit.\n",
                e, account_type
            )
            .into(),
        );
        error
    })?;
    Ok(())
}

fn validate_decimal(value: &str) -> Result<(), ValidationError> {
    let v = match value.parse::<Decimal>() {
        Ok(val) => val,
        Err(_) => {
            let mut error = ValidationError::new("invalid_decimal");
            error.message = Some(
                format!(
                    "Incorrect value: {}. Must be a valid decimal number.\n",
                    value
                )
                .into(),
            );
            return Err(error);
        }
    };
    if v.is_sign_negative() {
        let mut error = ValidationError::new("invalid_decimal");
        error.message = Some(
            format!(
                "Incorrect value: {}. Must not be a negative number.\n",
                value
            )
            .into(),
        );
        Err(error)
    } else {
        Ok(())
    }
}

fn validate_positive_decimal(value: &str) -> Result<(), ValidationError> {
    validate_decimal(value)?;
    if value.parse::<Decimal>().unwrap().is_zero() {
        let mut error = ValidationError::new("invalid_positive_decimal");
        error.message = Some(
            format!(
                "Incorrect value: {}. Must be a positive decimal number.\n",
                value
            )
            .into(),
        );
        return Err(error);
    }
    Ok(())
}

// Response sections

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct CalculatePortfolioResponse {
    pub uuid: String,
    pub banks: Vec<Bank>,
    pub outcome: Option<Outcome>,
    pub created_at: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Delta {
    pub period: String,
    pub period_unit: String,
    pub growth: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Maturity {
    pub amount: String,
    pub interest: String,
    pub total: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Bank {
    pub uuid: String,
    pub name: String,
    pub bank_tz: String,
    pub deposits: Vec<Deposit>,
    pub outcome: Option<Outcome>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Deposit {
    pub uuid: String,
    pub account: String,
    pub account_type: String,
    pub apy: String,
    pub years: String,
    pub outcome: Option<Outcome>,
    pub outcome_with_dates: Option<OutcomeWithDates>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Outcome {
    pub delta: Option<Delta>,
    pub maturity: Option<Maturity>,
    pub errors: Vec<ProcessingError>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct OutcomeWithDates {
    pub start_date_in_bank_tz: String,
    pub maturity_date_in_bank_tz: Option<String>,
    pub errors: Vec<ProcessingError>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ProcessingError {
    pub uuid: String,
    pub message: String,
}
