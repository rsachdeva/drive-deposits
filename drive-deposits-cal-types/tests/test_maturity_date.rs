use rust_decimal_macros::dec;

use drive_deposits_cal_types::math::maturity_date::maturity_date;

use crate::helper::enable_tracing::initialize_test_span;
use crate::helper::test_data::naive_date_2023_11_23;

mod helper;
#[test]
fn test_maturity_date_after_one_and_a_half_years() {
    initialize_test_span("test_maturity_date_after_5_days").in_scope(|| {
        let start_date = naive_date_2023_11_23();
        let years = dec!(1.5);
        let maturity = maturity_date(start_date, years);
        assert!(maturity.is_ok());
        let maturity = maturity.unwrap().to_string();
        assert_eq!(maturity, "2025-05-24");
    });
}

#[test]
fn test_maturity_date_after_five_years() {
    initialize_test_span("test_maturity_date_after_5_days").in_scope(|| {
        let start_date = naive_date_2023_11_23();
        let years = dec!(5);
        let maturity = maturity_date(start_date, years);
        assert!(maturity.is_ok());
        let maturity = maturity.unwrap().to_string();
        assert_eq!(maturity, "2028-11-21");
    });
}
