use crate::convert::writer::with_level_context::{
    error_with_bank_level, ItemWriterError, LevelSpecificItemWriterError,
};
use crate::db_item_types::{BankLevelItem, BankLevelItemsWrapper};
use drive_deposits_rest_types::rest_types::CalculatePortfolioResponse;
use rust_decimal::Decimal;
use serde_json::to_string;

impl TryFrom<&CalculatePortfolioResponse> for BankLevelItemsWrapper {
    type Error = LevelSpecificItemWriterError;

    fn try_from(rest: &CalculatePortfolioResponse) -> Result<Self, Self::Error> {
        // for a specific response
        let portfolio_uuid = rest.uuid.clone();
        let pk_portfolio_uuid = format!("PORTFOLIO#UUID#{}", portfolio_uuid);
        let created_at = rest.created_at.clone();

        let items = rest
            .banks
            .iter()
            .map(|bank| {
                let outcome = bank.outcome.as_ref().ok_or_else(|| {
                    error_with_bank_level(ItemWriterError::MissingDataField("outcome".to_string()))
                })?;
                let delta = outcome.delta.as_ref().ok_or_else(|| {
                    error_with_bank_level(ItemWriterError::MissingDataField("delta".to_string()))
                })?;
                let growth_padded = format!(
                    "{:020.2}",
                    delta
                        .growth
                        .parse::<Decimal>()
                        .map_err(error_with_bank_level)?
                );
                let bank_uuid = bank.uuid.clone();
                let sk_format_bank_delta_growth = format!(
                    "BANK#PERIOD#{}#PERIOD_UNIT#{}#GROWTH#{}#PORTFOLIO_CREATED_AT#{}BANK_UUID#{}",
                    delta.period, delta.period_unit, growth_padded, created_at, bank_uuid
                );
                let bank_name = bank.name.clone();
                let portfolio_uuid = portfolio_uuid.clone();
                let bank_tz = bank.bank_tz.clone();
                let outcome_as_json = to_string(&bank.outcome).map_err(error_with_bank_level)?;
                Ok(BankLevelItem {
                    pk_format_portfolio_uuid: pk_portfolio_uuid.clone(),
                    sk_format_bank_delta_growth,
                    bank_uuid,
                    bank_name,
                    portfolio_uuid,
                    bank_tz,
                    outcome_as_json,
                    created_at: created_at.clone(),
                })
            })
            .collect::<Result<Vec<BankLevelItem>, LevelSpecificItemWriterError>>()?;
        Ok(Self { items })
    }
}
