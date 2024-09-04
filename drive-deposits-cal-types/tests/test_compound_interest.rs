use pretty_assertions::assert_eq;
use rust_decimal_macros::dec;
use tracing::instrument;

use drive_deposits_cal_types::cal_types::NewDelta;
use drive_deposits_cal_types::cal_types::NewDeposit;
use drive_deposits_cal_types::math::{
    compound_interest::compute_interest, growth::compute as compute_growth,
};
use drive_deposits_proto_grpc_types::generated::{AccountType, PeriodUnit};
use helper::enable_tracing::initialize_test_span;
use helper::test_data::naive_date_2023_11_23;

mod helper;

#[test]
fn test_compound_interest_calculation_delta_years() {
    initialize_test_span("test_compound_interest_calculation_delta_years").in_scope(|| {
        let deposit = NewDeposit {
            account: "test_account".to_string(),
            account_type: AccountType::BrokerageCertificateOfDeposit,
            apy: dec!(5.0),
            years: dec!(2.0),
            amount: dec!(1000.0),
            start_date_in_bank_tz: naive_date_2023_11_23(),
        };
        let delta = NewDelta {
            period: dec!(1.0),
            period_unit: PeriodUnit::Year,
        };
        let interest = compute_interest(&deposit);
        let delta_interest = compute_growth(&deposit, interest, &delta).unwrap();
        assert_eq!(interest, dec!(102.50));
        assert_eq!(delta_interest, dec!(51.25));
    });
}

#[test]
fn test_compound_interest_calculation_delta_several_years() {
    initialize_test_span("test_compound_interest_calculation_delta_several_years").in_scope(|| {
        let deposit = NewDeposit {
            account: "test_account".to_string(),
            account_type: AccountType::BrokerageCertificateOfDeposit,
            apy: dec!(7.0),
            years: dec!(5.0),
            amount: dec!(5000.0),
            start_date_in_bank_tz: naive_date_2023_11_23(),
        };
        let delta = NewDelta {
            period: dec!(15),
            period_unit: PeriodUnit::Day,
        };
        let interest = compute_interest(&deposit);
        let delta_interest = compute_growth(&deposit, interest, &delta).unwrap();
        assert_eq!(interest, dec!(2012.76));
        assert_eq!(delta_interest, dec!(16.54));
    });
}

#[test]
fn test_compound_interest_calculation_delta_mid_level_decimal_interest() {
    initialize_test_span("test_compound_interest_calculation_delta_mid_level_decimal_interest")
        .in_scope(|| {
            let deposit = NewDeposit {
                account: "test_account".to_string(),
                account_type: AccountType::CertificateOfDeposit,
                apy: dec!(3.4),
                years: dec!(5.0),
                amount: dec!(10000.0),
                start_date_in_bank_tz: naive_date_2023_11_23(),
            };
            let delta = NewDelta {
                period: dec!(15),
                period_unit: PeriodUnit::Day,
            };
            let interest = compute_interest(&deposit);
            let delta_interest = compute_growth(&deposit, interest, &delta).unwrap();
            assert_eq!(interest, dec!(1819.60));
            assert_eq!(delta_interest, dec!(14.96));
        });
}

#[test]
fn test_compound_interest_calculation_delta_months() {
    initialize_test_span("test_compound_interest_calculation_delta_months").in_scope(|| {
        let deposit = NewDeposit {
            account: "test_account".to_string(),
            account_type: AccountType::CertificateOfDeposit,
            apy: dec!(5.0),
            years: dec!(2.0),
            amount: dec!(1000.0),
            start_date_in_bank_tz: naive_date_2023_11_23(),
        };
        let delta = NewDelta {
            period: dec!(1.0),
            period_unit: PeriodUnit::Month,
        };
        let interest = compute_interest(&deposit);
        let delta_interest = compute_growth(&deposit, interest, &delta).unwrap();
        assert_eq!(interest, dec!(102.50));
        assert_eq!(delta_interest, dec!(4.21));
    });
}

#[test]
#[instrument]
fn test_compound_interest_calculation_delta_weeks() {
    initialize_test_span("test_compound_interest_calculation_delta_weeks").in_scope(|| {
        let deposit = NewDeposit {
            account: "test_account".to_string(),
            account_type: AccountType::BrokerageCertificateOfDeposit,
            apy: dec!(5.0),
            years: dec!(2.0),
            amount: dec!(1000.0),
            start_date_in_bank_tz: naive_date_2023_11_23(),
        };
        let delta = NewDelta {
            period: dec!(2.0),
            period_unit: PeriodUnit::Week,
        };
        let interest = compute_interest(&deposit);
        let delta_interest = compute_growth(&deposit, interest, &delta).unwrap();
        assert_eq!(interest, dec!(102.50));
        assert_eq!(delta_interest, dec!(1.97));
    });
}

#[test]
fn test_compound_interest_calculation_delta_days() {
    initialize_test_span("test_compound_interest_calculation_delta_days").in_scope(|| {
        let deposit = NewDeposit {
            account: "test_account".to_string(),
            account_type: AccountType::BrokerageCertificateOfDeposit,
            apy: dec!(5.0),
            years: dec!(2.0),
            amount: dec!(1000.0),
            start_date_in_bank_tz: naive_date_2023_11_23(),
        };
        let delta = NewDelta {
            period: dec!(30.0),
            period_unit: PeriodUnit::Day,
        };
        let interest = compute_interest(&deposit);
        let delta_interest = compute_growth(&deposit, interest, &delta).unwrap();
        assert_eq!(interest, dec!(102.50));
        assert_eq!(delta_interest, dec!(4.21));
    });
}

#[test]
#[should_panic]
fn test_growth_with_compound_interest_zero_years() {
    initialize_test_span("test_growth_with_compound_interest_zero_years").in_scope(|| {
        let deposit = NewDeposit {
            account: "test_account".to_string(),
            account_type: AccountType::BrokerageCertificateOfDeposit,
            apy: dec!(5.0),
            years: dec!(0.0),
            amount: dec!(1000.0),
            start_date_in_bank_tz: naive_date_2023_11_23(),
        };
        let delta = NewDelta {
            period: dec!(30.0),
            period_unit: PeriodUnit::Day,
        };
        let interest = compute_interest(&deposit);
        compute_growth(&deposit, interest, &delta).unwrap();
    });
}
