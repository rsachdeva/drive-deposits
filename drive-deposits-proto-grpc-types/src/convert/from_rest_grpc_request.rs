use heck::ToShoutySnakeCase;
use tracing::{debug, info, info_span};

use drive_deposits_rest_types::rest_types::{
    CalculatePortfolioRequest as RestCalculatePortfolioRequest, NewBank as RestNewBank,
    NewDelta as RestNewDelta, NewDeposit as RestNewDeposit,
};

use crate::generated::{
    AccountType as GrpcAccountType, CalculatePortfolioRequest as GrpcCalculatePortfolioRequest,
    NewBank as GrpcNewBank, NewDelta as GrpcNewDelta, NewDeposit as GrpcNewDeposit,
    PeriodUnit as GrpcPeriodUnit,
};

impl From<RestNewDeposit> for GrpcNewDeposit {
    fn from(rest: RestNewDeposit) -> Self {
        debug!(
            "rest.account_type.to_shouty_snake_case() is {}",
            rest.account_type.to_shouty_snake_case()
        );
        GrpcNewDeposit {
            account: rest.account,
            account_type: GrpcAccountType::from_str_name(&rest.account_type.to_shouty_snake_case())
                .unwrap_or_default() as i32,
            apy: rest.apy,
            years: rest.years,
            amount: rest.amount,
            start_date_in_bank_tz: rest.start_date_in_bank_tz.to_string(),
        }
    }
}

impl From<RestNewBank> for GrpcNewBank {
    fn from(rest: RestNewBank) -> Self {
        Self {
            name: rest.name,
            bank_tz: rest.bank_tz.to_string(),
            new_deposits: rest.new_deposits.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<RestNewDelta> for GrpcNewDelta {
    fn from(rest: RestNewDelta) -> Self {
        Self {
            period: rest.period,
            period_unit: GrpcPeriodUnit::from_str_name(&rest.period_unit.to_shouty_snake_case())
                .unwrap_or_default() as i32,
        }
    }
}
impl From<RestCalculatePortfolioRequest> for GrpcCalculatePortfolioRequest {
    fn from(rest: RestCalculatePortfolioRequest) -> Self {
        // 44 |         let grpc_new_delta = rest.new_delta.into();
        //    |                                             ^^^^ the trait `From<drive_deposits_io_types::io_types::NewDelta>` is not implemented for `generated::NewDelta`, which is required by `drive_deposits_io_types::io_types::NewDelta: Into<_>`
        let grpc = Self {
            new_banks: rest.new_banks.into_iter().map(|x| x.into()).collect(),
            new_delta: Some(rest.new_delta.into()),
        };
        info_span!("rest_grpc_request::From::rest")
            .in_scope(|| info!("rest request converted to grpc request: {:?}", grpc));
        grpc
    }
}
