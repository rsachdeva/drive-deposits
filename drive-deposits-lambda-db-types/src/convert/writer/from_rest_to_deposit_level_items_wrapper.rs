use crate::convert::writer::with_level_context::{
    error_with_deposit_level, ItemWriterError, LevelSpecificItemWriterError,
};
use crate::db_item_types::{
    CalculatePortfolioRestWrapper, DepositLevelItem, DepositLevelItemsWrapper, DepositSortCriteria,
};
use rust_decimal::Decimal;
use serde_json::to_string;
use tracing::info;

impl TryFrom<&CalculatePortfolioRestWrapper> for DepositLevelItemsWrapper {
    type Error = LevelSpecificItemWriterError;

    fn try_from(rest_wrapper: &CalculatePortfolioRestWrapper) -> Result<Self, Self::Error> {
        info!("Converting CalculatePortfolioRestWrapper to DepositLevelItemsWrapper");
        info!(
            "CalculatePortfolioRestWrapper rest_wrapper: {:#?}",
            rest_wrapper
        );
        let deposit_sort_criteria = &rest_wrapper.deposit_sort_criteria;
        let rest = &rest_wrapper.calculate_portfolio_response;
        let portfolio_uuid = rest.uuid.clone();
        let pk_format_portfolio_uuid = format!("PORTFOLIO#UUID#{}", portfolio_uuid);

        let bank_deposit_level_items_per_bank_iter = rest.banks.iter().map(|bank| {
            let bank_deposit_level_items_per_bank_iter = bank.deposits.iter().map(
                |deposit| -> Result<DepositLevelItem, LevelSpecificItemWriterError> {
                    let created_at = rest.created_at.clone();
                    let outcome = deposit
                        .outcome
                        .as_ref()
                        .ok_or_else(|| error_with_deposit_level(ItemWriterError::MissingDataField("outcome".to_string())))?;
                    let delta = outcome
                        .delta
                        .as_ref()
                        .ok_or_else(|| error_with_deposit_level(ItemWriterError::MissingDataField("delta".to_string())))?;
                    let deposit_delta_growth = delta.growth.clone();
                    let growth_padded = format!("{:020.2}", deposit_delta_growth.parse::<Decimal>().map_err(error_with_deposit_level)?);
                    let outcome_with_dates = deposit
                        .outcome_with_dates
                        .as_ref()
                        .ok_or_else(|| error_with_deposit_level(ItemWriterError::MissingDataField("outcome_with_dates".to_string())))?;

                    let deposit_maturity_date_in_bank_tz = outcome_with_dates
                        .maturity_date_in_bank_tz
                        .as_ref()
                        .ok_or_else(|| error_with_deposit_level(ItemWriterError::MissingDataField("maturity_date_in_bank_tz".to_string())))?
                        .clone();
                    let deposit_uuid = deposit.uuid.clone();
                    let sk_format_deposit_sort_criteria = match deposit_sort_criteria {
                        DepositSortCriteria::DeltaPeriodGrowth => format!(
                            "DEPOSIT#PERIOD#{}#PERIOD_UNIT#{}#GROWTH#{}#PORTFOLIO_CREATED_AT#{}#DEPOSIT_UUID#{}",
                            delta.period, delta.period_unit, growth_padded, created_at, deposit_uuid
                        ),
                        DepositSortCriteria::MaturityDate => format!(
                            "DEPOSIT#MATURITY_DATE#{}#PORTFOLIO_CREATED_AT#{}#DEPOSIT_UUID#{}",
                            deposit_maturity_date_in_bank_tz, rest.created_at, deposit_uuid
                        )
                    };

                    let bank_uuid = bank.uuid.clone();
                    let bank_name = bank.name.clone();
                    let portfolio_uuid = portfolio_uuid.clone();
                    let account = deposit.account.clone();
                    let account_type = deposit.account_type.clone();
                    let apy = deposit.apy.clone();
                    let years = deposit.years.clone();
                    let outcome_as_json = to_string(&deposit.outcome).map_err(error_with_deposit_level)?;
                    let outcome_with_dates_as_json = to_string(&deposit.outcome_with_dates).map_err(error_with_deposit_level)?;

                    Ok(DepositLevelItem {
                        pk_format_portfolio_uuid: pk_format_portfolio_uuid.clone(),
                        sk_format_deposit_sort_criteria,
                        deposit_delta_growth,
                        deposit_maturity_date_in_bank_tz,
                        bank_uuid,
                        bank_name,
                        portfolio_uuid,
                        deposit_uuid,
                        account,
                        account_type,
                        apy,
                        years,
                        outcome_as_json,
                        outcome_with_dates_as_json,
                        created_at,
                    })
                },
            );

            // collect for all bank deposit items for a specific bank
            let bank_deposit_level_items_per_bank = bank_deposit_level_items_per_bank_iter
                .collect::<Result<Vec<DepositLevelItem>, LevelSpecificItemWriterError>>()?;
            Ok(bank_deposit_level_items_per_bank)
        });
        // collect all bank deposit items for all banks
        let deposit_level_items_all_banks = bank_deposit_level_items_per_bank_iter
            .collect::<Result<Vec<Vec<DepositLevelItem>>, LevelSpecificItemWriterError>>()?;
        let deposit_level_items_all_banks_flattened = deposit_level_items_all_banks
            .into_iter()
            .flatten()
            .collect::<Vec<DepositLevelItem>>();
        Ok(DepositLevelItemsWrapper {
            deposit_level_items: deposit_level_items_all_banks_flattened,
        })
    }
}
