use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use tracing::debug;

use crate::math::individual_calculation_error::Error as IndividualCalculationError;

pub fn maturity_date(
    start_date: NaiveDate,
    years: Decimal,
) -> Result<NaiveDate, IndividualCalculationError> {
    debug!("start_date: {:?}", start_date);
    // round is basically self.round_dp(0) which rounds to the nearest integer
    // follows "Bankers Rounding" rules. e.g. 6.5 -> 6, 7.5 -> 8
    let days = (years * dec!(365.0)).round().to_string();
    debug!("days in years rounded is: {:?}", days);
    // convert string to int64
    let days = days.parse::<i64>()?;
    let date = start_date + chrono::Duration::days(days);
    debug!("maturity_date: {:?}", date);
    Ok(date)
}
