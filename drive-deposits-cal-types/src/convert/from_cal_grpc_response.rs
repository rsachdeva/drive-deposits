use drive_deposits_proto_grpc_types::generated::{
    Bank as GrpcBank, CalculatePortfolioResponse as GrpcCalculatePortfolioResponse,
    Delta as GrpcDelta, Deposit as GrpcDeposit, Maturity as GrpcMaturity, Outcome as GrpcOutcome,
    OutcomeWithDates as GrpcOutcomeWithDates, ProcessingError as GrpcProcessingError,
};

use crate::cal_types::{
    Bank as CalBank, Delta as CalDelta, Deposit as CalDeposit, Maturity as CalMaturity,
    Outcome as CalOutcome, OutcomeWithDates as CalOutcomeWithDates,
    PortfolioResponse as CalBankResponse, ProcessingError as cal_ProcessingError,
};

impl From<cal_ProcessingError> for GrpcProcessingError {
    fn from(cal: cal_ProcessingError) -> Self {
        Self {
            uuid: cal.uuid.to_string(),
            message: cal.message,
        }
    }
}

impl From<CalMaturity> for GrpcMaturity {
    fn from(cal: CalMaturity) -> Self {
        Self {
            amount: cal.amount.to_string(),
            interest: cal.interest.to_string(),
            total: cal.total.to_string(),
        }
    }
}
impl From<CalDelta> for GrpcDelta {
    fn from(cal: CalDelta) -> Self {
        Self {
            period: cal.period.to_string(),
            period_unit: cal.period_unit as i32,
            growth: cal.growth.to_string(),
        }
    }
}

impl From<CalOutcomeWithDates> for GrpcOutcomeWithDates {
    fn from(cal: CalOutcomeWithDates) -> Self {
        Self {
            start_date_in_bank_tz: cal.start_date_in_bank_tz.to_string(),
            maturity_date_in_bank_tz: cal.maturity_date_in_bank_tz.map(|x| x.to_string()),
            errors: cal.errors.into_iter().map(|x| x.into()).collect(),
        }
    }
}
impl From<CalOutcome> for GrpcOutcome {
    fn from(cal: CalOutcome) -> Self {
        Self {
            delta: cal.delta.map(|x| x.into()),
            maturity: cal.maturity.map(|x| x.into()),
            errors: cal.errors.into_iter().map(|x| x.into()).collect(),
        }
    }
}
// Orphan rule helps not to implement very generic From<Option<NaiveDate>> for String unless wrap in our own types using new type pattern
// impl From<Option<NaiveDate>> for String {
//     fn from(date: Option<NaiveDate>) -> Self {
//         date.map(|date| date.to_string())
//             .unwrap_or("None".to_string())
//     }
// }
impl From<CalDeposit> for GrpcDeposit {
    fn from(cal: CalDeposit) -> Self {
        Self {
            uuid: cal.uuid.to_string(),
            account: cal.account,
            account_type: cal.account_type as i32,
            apy: cal.apy.to_string(),
            years: cal.years.to_string(),
            outcome: cal.outcome.map(|cal_outcome| cal_outcome.into()),
            outcome_with_dates: cal
                .outcome_with_dates
                .map(|cal_outcome_with_dates| cal_outcome_with_dates.into()),
        }
    }
}

impl From<CalBank> for GrpcBank {
    fn from(cal: CalBank) -> Self {
        Self {
            uuid: cal.uuid.to_string(),
            name: cal.name,
            bank_tz: cal.bank_tz.to_string(),
            deposits: cal.deposits.into_iter().map(|x| x.into()).collect(),
            outcome: cal.outcome.map(|x| x.into()),
        }
    }
}

impl From<CalBankResponse> for GrpcCalculatePortfolioResponse {
    fn from(cal: CalBankResponse) -> Self {
        Self {
            banks: cal.banks.into_iter().map(|x| x.into()).collect(),
            uuid: cal.uuid.to_string(),
            outcome: cal.outcome.map(|x| x.into()),
            created_at: cal.created_at,
        }
    }
}
