use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use tracing::debug;

use drive_deposits_proto_grpc_types::generated::PeriodUnit;

use crate::cal_types::{NewDelta, NewDeposit};
use crate::math::individual_calculation_error::Error as IndividualCalculationError;

pub fn compute(
    deposit: &NewDeposit,
    interest: Decimal,
    delta: &NewDelta,
) -> Result<Decimal, IndividualCalculationError> {
    if deposit.years.is_zero() {
        return Err(IndividualCalculationError::ZeroYears(
            "cannot calculate growth for zero years".to_string(),
        ));
    }
    let interest_per_smallest_unit_day = interest / (dec!(365.0) * deposit.years);
    debug!(
        "interest_per_smallest_unit_day: {}",
        interest_per_smallest_unit_day
    );
    debug!("delta.period_unit is: {:?}", delta.period_unit);
    debug!("delta.period: {}", delta.period);
    let period_in_smallest_unit_days = match delta.period_unit {
        PeriodUnit::Day => delta.period,
        PeriodUnit::Week => delta.period * dec!(7.0),
        PeriodUnit::Month => delta.period * dec!(30.0),
        PeriodUnit::Year => delta.period * dec!(365.0),
        _ => delta.period,
    };
    let delta = (interest_per_smallest_unit_day * period_in_smallest_unit_days).round_dp(2);
    debug!("delta: {}", delta);
    Ok(delta)
}
