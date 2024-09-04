use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;
use tracing::{debug, instrument};

use crate::cal_types::NewDeposit;

/// Computes the compound math for a deposit.
///
/// Compound math is calculated using the formula:
///
/// A = P(1 + r/n)^(nt)
///
/// Where:
/// P is the principal amount (initial deposit)
/// r is the annual math rate (in decimal form, e.g., 5% is 0.05)
/// n is the number of times that the math is compounded per year (In this case it is 1)
/// t is the number of years the money is invested for
///
/// The order of operations is as follows:
/// First compute `r/n` (math rate divided by number of times compounded per year)
/// Second, add 1 to the result
/// Third, raise the resulting sum to the power of `n*t` (n times t)
/// Finally, multiply the result with P (principal). So the P times happens last in the calculation.
#[instrument]
pub fn compute_interest(deposit: &NewDeposit) -> Decimal {
    debug!("Calculating compound math for deposit: {:?}", deposit);
    let principal = deposit.amount;
    let rate = deposit.apy / dec!(100);
    let years = deposit.years;

    //Assuming the math is compounded annually
    let n = dec!(1);

    let total_amount = principal * ((dec!(1) + rate / n).powd(n * years));

    // Subtract the principal to get only the math
    let interest = total_amount - principal;

    debug!("Compound math overall: {}", interest);
    interest.round_dp(2)
}
