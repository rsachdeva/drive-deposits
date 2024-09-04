use chrono::NaiveDate;
use rust_decimal::Decimal;

use drive_deposits_proto_grpc_types::generated::{
    AccountType as GrpcAccountType, CalculatePortfolioRequest as GrpcCalculatePortfolioRequest,
    NewBank as GrpcNewBank, NewDelta as GrpcNewDelta, NewDeposit as GrpcNewDeposit,
    PeriodUnit as GrpcPeriodUnit,
};

use crate::cal_types::{
    NewBank as CalNewBank, NewDelta as CalNewDelta, NewDeposit as CalNewDeposit,
    PortfolioRequest as CalBankRequest,
};

impl From<GrpcNewDeposit> for CalNewDeposit {
    fn from(grpc: GrpcNewDeposit) -> Self {
        Self {
            account: grpc.account,
            account_type: GrpcAccountType::try_from(grpc.account_type).unwrap_or_default(),
            apy: grpc.apy.parse::<Decimal>().unwrap_or_default(),
            years: grpc.years.parse::<Decimal>().unwrap_or_default(),
            amount: grpc.amount.parse::<Decimal>().unwrap_or_default(),
            start_date_in_bank_tz: NaiveDate::parse_from_str(
                &grpc.start_date_in_bank_tz,
                "%Y-%m-%d",
            )
            .unwrap_or_default(),
        }
    }
}

impl From<GrpcNewBank> for CalNewBank {
    fn from(grpc: GrpcNewBank) -> Self {
        Self {
            name: grpc.name,
            bank_tz: grpc.bank_tz.parse().unwrap_or_default(),
            new_deposits: grpc.new_deposits.into_iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<GrpcNewDelta> for CalNewDelta {
    fn from(grpc: GrpcNewDelta) -> Self {
        Self {
            period: grpc.period.parse::<Decimal>().unwrap_or_default(),
            period_unit: GrpcPeriodUnit::try_from(grpc.period_unit).unwrap_or_default(),
        }
    }
}

impl From<GrpcCalculatePortfolioRequest> for CalBankRequest {
    fn from(grpc: GrpcCalculatePortfolioRequest) -> Self {
        Self {
            new_banks: grpc.new_banks.into_iter().map(|x| x.into()).collect(),
            new_delta: grpc.new_delta.unwrap_or_default().into(),
        }
    }
}
