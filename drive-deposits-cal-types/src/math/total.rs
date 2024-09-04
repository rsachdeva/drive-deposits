use rust_decimal::Decimal;

pub fn compute(initial_amount: Decimal, interest: Decimal) -> Decimal {
    (initial_amount + interest).round_dp(2)
}
