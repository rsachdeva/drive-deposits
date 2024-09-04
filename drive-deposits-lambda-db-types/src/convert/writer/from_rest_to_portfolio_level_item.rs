use crate::convert::writer::with_level_context::{
    error_with_portfolio_level, ItemWriterError, LevelSpecificItemWriterError,
};
use crate::db_item_types::PortfolioLevelItem;
use drive_deposits_rest_types::rest_types::CalculatePortfolioResponse;
use rust_decimal::Decimal;
use serde_json::to_string;

impl TryFrom<&CalculatePortfolioResponse> for PortfolioLevelItem {
    type Error = LevelSpecificItemWriterError;

    fn try_from(rest: &CalculatePortfolioResponse) -> Result<Self, Self::Error> {
        let outcome_as_json = to_string(&rest.outcome).map_err(error_with_portfolio_level)?;
        let outcome = rest.outcome.as_ref().ok_or_else(|| {
            error_with_portfolio_level(ItemWriterError::MissingDataField("outcome".to_string()))
        })?;
        // for different responses
        let pk_portfolios = "PORTFOLIOS".to_string();

        let delta = outcome.delta.as_ref().ok_or_else(|| {
            error_with_portfolio_level(ItemWriterError::MissingDataField("delta".to_string()))
        })?;
        // padding numeric values with leading zeros to ensure correct lexicographical sorting is a common practice in the real world, especially in systems like DynamoDB where sorting is based on string comparison. This approach ensures that numeric values are sorted correctly when stored as strings.
        // the format string {:010.2} means:
        // 0: Pad with leading zeros.
        // 10: The total width of the formatted number, including the decimal point and the digits after it.
        // .2: Two digits after the decimal point.
        // So, for example, the number 367.76 would be formatted as 0000367.76.

        let growth_padded = format!(
            "{:020.2}",
            delta
                .growth
                .parse::<Decimal>()
                .map_err(error_with_portfolio_level)?
        );
        // used with pk_response to sort response_delta_growth overall for response level comparison for
        // different responses overall
        // SK RESPONSE_PERIOD#{}#PERIOD_UNIT#{}#GROWTH#{}CREATED_AT#{} used with RESPONSES to sort responses delta period growth
        let sk_format_portfolio_delta_growth = format!(
            "PORTFOLIO#PERIOD#{}#PERIOD_UNIT#{}#GROWTH#{}#PORTFOLIO_CREATED_AT#{}",
            delta.period, delta.period_unit, growth_padded, rest.created_at
        );
        let created_at = rest.created_at.clone();
        let portfolio_uuid = rest.uuid.clone();

        Ok(Self {
            pk_portfolios,
            sk_format_portfolio_delta_growth,
            portfolio_uuid,
            outcome_as_json,
            created_at,
        })
    }
}
