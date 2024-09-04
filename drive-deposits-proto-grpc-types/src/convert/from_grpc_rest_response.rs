use heck::ToUpperCamelCase;
use tracing::{info, info_span};

use drive_deposits_rest_types::rest_types::{
    Bank as RestBank, CalculatePortfolioResponse as RestCalculatePortfolioResponse,
    Delta as RestDelta, Deposit as RestDeposit, Maturity as RestMaturity, Outcome as RestOutcome,
    OutcomeWithDates as RestOutcomeWithDates, ProcessingError as RestProcessingError,
};

use crate::generated::{
    AccountType as GrpcAccountType, Bank as GrpcBank,
    CalculatePortfolioResponse as GrpcCalculatePortfolioResponse, Delta as GrpcDelta,
    Deposit as GrpcDeposit, Maturity as GrpcMaturity, Outcome as GrpcOutcome,
    OutcomeWithDates as GrpcOutcomeWithDates, PeriodUnit as GrpcPeriodUnit,
    ProcessingError as GrpcProcessingError,
};

impl From<GrpcProcessingError> for RestProcessingError {
    fn from(grpc: GrpcProcessingError) -> Self {
        Self {
            uuid: grpc.uuid,
            message: grpc.message,
        }
    }
}

impl From<GrpcDelta> for RestDelta {
    fn from(grpc: GrpcDelta) -> Self {
        Self {
            period: grpc.period,
            period_unit: GrpcPeriodUnit::try_from(grpc.period_unit)
                .unwrap_or_default()
                .as_str_name()
                .to_upper_camel_case(),
            growth: grpc.growth,
        }
    }
}

impl From<GrpcMaturity> for RestMaturity {
    fn from(grpc: GrpcMaturity) -> Self {
        Self {
            amount: grpc.amount,
            interest: grpc.interest,
            total: grpc.total,
        }
    }
}
impl From<GrpcOutcome> for RestOutcome {
    fn from(grpc: GrpcOutcome) -> Self {
        Self {
            delta: grpc.delta.map(|x| x.into()),
            maturity: grpc.maturity.map(|x| x.into()),
            errors: grpc.errors.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<GrpcOutcomeWithDates> for RestOutcomeWithDates {
    fn from(grpc: GrpcOutcomeWithDates) -> Self {
        Self {
            start_date_in_bank_tz: grpc.start_date_in_bank_tz,
            maturity_date_in_bank_tz: grpc.maturity_date_in_bank_tz,
            errors: grpc.errors.into_iter().map(|x| x.into()).collect(),
        }
    }
}
impl From<GrpcDeposit> for RestDeposit {
    fn from(grpc: GrpcDeposit) -> Self {
        Self {
            uuid: grpc.uuid,
            account: grpc.account,
            account_type: GrpcAccountType::try_from(grpc.account_type)
                .unwrap_or_default()
                .as_str_name()
                .to_upper_camel_case(),
            apy: grpc.apy,
            years: grpc.years,
            outcome: grpc.outcome.map(|x| x.into()),
            outcome_with_dates: grpc.outcome_with_dates.map(|x| x.into()),
        }
    }
}
impl From<GrpcBank> for RestBank {
    fn from(grpc: GrpcBank) -> Self {
        Self {
            uuid: grpc.uuid,
            name: grpc.name,
            bank_tz: grpc.bank_tz.to_string(),
            deposits: grpc.deposits.into_iter().map(|x| x.into()).collect(),
            outcome: grpc.outcome.map(|x| x.into()),
        }
    }
}

impl From<GrpcCalculatePortfolioResponse> for RestCalculatePortfolioResponse {
    fn from(grpc: GrpcCalculatePortfolioResponse) -> Self {
        let rest = Self {
            uuid: grpc.uuid,
            banks: grpc.banks.into_iter().map(|x| x.into()).collect(),
            created_at: grpc.created_at,
            outcome: grpc.outcome.map(|x| x.into()),
        };
        info_span!("grpc_rest_response::From::grpc").in_scope(|| {
            info!(
                "In From: grpc response converted to rest response: {:?}",
                rest
            )
        });
        rest
    }
}
