use rust_decimal::Decimal;
use thiserror::Error;
use uuid::Uuid;

use crate::cal_types::{Bank, Deposit, ProcessingError};

#[derive(Default, Debug, Error)]
pub enum AccumulatorError {
    #[default]
    #[error("Missing outcome in deposit")]
    MissingOutcome,
    #[error("Missing delta in deposit")]
    MissingDelta,
    #[error("Missing maturity in deposit")]
    MissingMaturity,
}

impl From<AccumulatorError> for ProcessingError {
    fn from(error: AccumulatorError) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            message: error.to_string(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Accumulator {
    pub(crate) growth: Decimal,
    pub(crate) amount: Decimal,
    pub(crate) interest: Decimal,
    pub(crate) total: Decimal,
}

pub fn accumulate_deposits(deposits: &[Deposit]) -> Result<Accumulator, AccumulatorError> {
    deposits
        .iter()
        .try_fold(Accumulator::default(), |acc, deposit| {
            let deposit = deposit
                .outcome
                .as_ref()
                .ok_or(AccumulatorError::MissingOutcome)?;
            let deposit_maturity = deposit
                .maturity
                .as_ref()
                .ok_or(AccumulatorError::MissingMaturity)?;
            let deposit_delta = deposit
                .delta
                .as_ref()
                .ok_or(AccumulatorError::MissingDelta)?;

            let deposit_growth = deposit_delta.growth;
            let deposit_amount = deposit_maturity.amount;
            let deposit_interest = deposit_maturity.interest;
            let deposit_total = deposit_maturity.total;

            Ok(Accumulator {
                growth: acc.growth + deposit_growth,
                amount: acc.amount + deposit_amount,
                interest: acc.interest + deposit_interest,
                total: acc.total + deposit_total,
            })
        })
}

pub fn accumulate_banks(banks: &[Bank]) -> Result<Accumulator, AccumulatorError> {
    banks.iter().try_fold(Accumulator::default(), |acc, bank| {
        let bank = bank
            .outcome
            .as_ref()
            .ok_or(AccumulatorError::MissingOutcome)?;
        let bank_maturity = bank
            .maturity
            .as_ref()
            .ok_or(AccumulatorError::MissingMaturity)?;
        let bank_delta = bank.delta.as_ref().ok_or(AccumulatorError::MissingDelta)?;

        let bank_growth = bank_delta.growth;
        let bank_amount = bank_maturity.amount;
        let bank_interest = bank_maturity.interest;
        let bank_total = bank_maturity.total;
        Ok(Accumulator {
            growth: acc.growth + bank_growth,
            amount: acc.amount + bank_amount,
            interest: acc.interest + bank_interest,
            total: acc.total + bank_total,
        })
    })
}
