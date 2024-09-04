use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use tracing::{debug, instrument};

use crate::cal_types::NewDeposit;

/// Computes the simple math for a deposit.
///
/// Simple math is calculated using the formula:
///
/// I = PRT
///
/// Where:
/// P is the principal amount (initial amount)
/// R is the annual math rate (in decimal form, e.g., 5% is 0.05)
/// T is the time the money is invested for in years
#[instrument]
pub fn compute_interest(deposit: &NewDeposit) -> Decimal {
    // The principal amount
    let principal = deposit.amount;

    // The annual math rate in decimal
    let rate = deposit.apy / dec!(100);

    // The time the money is invested for in years
    let years = deposit.years;

    // Calculate simple math
    let simple = principal * rate * years;

    debug!("simple math overall: {}", simple);

    // Return the calculated simple math
    simple.round_dp(2)
}
