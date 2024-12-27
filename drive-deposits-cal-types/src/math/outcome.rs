use crate::math::accumulator::Accumulator;
use crate::{
    cal_types::{
        Bank, Delta, Deposit, Maturity, NewDelta, NewDeposit, Outcome, OutcomeWithDates,
        ProcessingError,
    },
    math::{
        accumulator::{accumulate_banks, accumulate_deposits},
        compound_interest::compute_interest as compute_compound_interest,
        growth::compute as compute_growth,
        maturity_date::maturity_date,
        simple_interest::compute_interest as compute_simple_interest,
        total::compute as compute_total,
    },
};
use drive_deposits_proto_grpc_types::generated::AccountType;
use rust_decimal::Decimal;
use tracing::debug;
use uuid::Uuid;

fn outcome_with_growth(
    new_deposit: &NewDeposit,
    new_delta: &NewDelta,
    interest: Decimal,
) -> Option<Outcome> {
    let total = compute_total(new_deposit.amount, interest);
    let growth = compute_growth(new_deposit, interest, new_delta);
    debug!("outcome_with_growth growth: {:?}", growth);
    let outcome = growth.map_or_else(
        |err| {
            Some(Outcome {
                delta: None,
                maturity: None,
                errors: vec![err.into()],
            })
        },
        |growth| {
            Some(Outcome {
                delta: Some(Delta {
                    period: new_delta.period,
                    period_unit: new_delta.period_unit,
                    growth,
                }),
                maturity: Some(Maturity {
                    amount: new_deposit.amount,
                    interest,
                    total,
                }),
                errors: vec![],
            })
        },
    );
    debug!("outcome_with_growth outcome: {:?}", outcome);
    outcome
}
pub fn build_outcome_from_new_deposit(
    new_deposit: &NewDeposit,
    new_delta: &NewDelta,
) -> Option<Outcome> {
    // at deposit level
    // to resume work on this function, we need to implement the logic
    match new_deposit.account_type {
        AccountType::Checking | AccountType::Savings | AccountType::CertificateOfDeposit => {
            let interest = compute_compound_interest(new_deposit);
            outcome_with_growth(new_deposit, new_delta, interest)
        }
        AccountType::BrokerageCertificateOfDeposit => {
            let interest = compute_simple_interest(new_deposit);
            outcome_with_growth(new_deposit, new_delta, interest)
        }
        _ => {
            Some(Outcome {
                delta: None,
                maturity: None,
                errors: vec![ProcessingError {
                    uuid: Uuid::new_v4(),
                    message: format!(
                        "Unspecified account type...Error calculating outcome for account type {:?} deposit: {:?}",
                        new_deposit.account_type, new_deposit.account
                    ),
                }],
            })
        }
    }
}

pub fn build_outcome_with_dates_from_new_deposit(
    new_deposit: &NewDeposit,
) -> Option<OutcomeWithDates> {
    maturity_date(new_deposit.start_date_in_bank_tz, new_deposit.years).map_or_else(
        |err| {
            Some(OutcomeWithDates {
                start_date_in_bank_tz: new_deposit.start_date_in_bank_tz,
                maturity_date_in_bank_tz: None,
                errors: vec![err.into()],
            })
        },
        |calculated_maturity_date| {
            Some(OutcomeWithDates {
                start_date_in_bank_tz: new_deposit.start_date_in_bank_tz,
                maturity_date_in_bank_tz: Some(calculated_maturity_date),
                errors: vec![],
            })
        },
    )
}

fn outcome_from_accumulator(accumulator: Accumulator, new_delta: &NewDelta) -> Option<Outcome> {
    Some(Outcome {
        delta: Some(Delta {
            period: new_delta.period,
            period_unit: new_delta.period_unit,
            growth: accumulator.growth,
        }),
        maturity: Some(Maturity {
            amount: accumulator.amount,
            interest: accumulator.interest,
            total: accumulator.total,
        }),
        errors: vec![],
    })
}
pub fn build_outcome_from_deposits(deposits: &[Deposit], new_delta: &NewDelta) -> Option<Outcome> {
    accumulate_deposits(deposits).map_or_else(
        |err| {
            Some(Outcome {
                delta: None,
                maturity: None,
                errors: vec![err.into()],
            })
        },
        |accumulator| outcome_from_accumulator(accumulator, new_delta),
    )
}

pub fn build_outcome_from_banks(banks: &[Bank], new_delta: &NewDelta) -> Option<Outcome> {
    accumulate_banks(banks).map_or_else(
        |err| {
            Some(Outcome {
                delta: None,
                maturity: None,
                errors: vec![err.into()],
            })
        },
        |accumulator| outcome_from_accumulator(accumulator, new_delta),
    )
}
