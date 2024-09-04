use crate::convert::reader::with_level_context::{
    get_field_with_deposit_level, LevelSpecificItemReaderError,
};
use crate::db_item_types::DepositLevelItem;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

impl TryFrom<HashMap<String, AttributeValue>> for DepositLevelItem {
    type Error = LevelSpecificItemReaderError;

    fn try_from(dynamodb_item: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let pk_portfolio_uuid = get_field_with_deposit_level(&dynamodb_item, "PK")?;
        let sk_bank_deposit_delta_growth = get_field_with_deposit_level(&dynamodb_item, "SK")?;
        let deposit_delta_growth =
            get_field_with_deposit_level(&dynamodb_item, "deposit_delta_growth")?;
        let deposit_maturity_date_in_bank_tz =
            get_field_with_deposit_level(&dynamodb_item, "deposit_maturity_date_in_bank_tz")?;
        let bank_uuid = get_field_with_deposit_level(&dynamodb_item, "bank_uuid")?;
        let bank_name = get_field_with_deposit_level(&dynamodb_item, "bank_name")?;
        let portfolio_uuid = get_field_with_deposit_level(&dynamodb_item, "portfolio_uuid")?;
        let deposit_uuid = get_field_with_deposit_level(&dynamodb_item, "deposit_uuid")?;
        let account = get_field_with_deposit_level(&dynamodb_item, "account")?;
        let account_type = get_field_with_deposit_level(&dynamodb_item, "account_type")?;
        let apy = get_field_with_deposit_level(&dynamodb_item, "apy")?;
        let years = get_field_with_deposit_level(&dynamodb_item, "years")?;
        let outcome_as_json = get_field_with_deposit_level(&dynamodb_item, "outcome")?;
        let outcome_with_dates_as_json =
            get_field_with_deposit_level(&dynamodb_item, "outcome_with_dates")?;
        let created_at = get_field_with_deposit_level(&dynamodb_item, "created_at")?;

        Ok(DepositLevelItem {
            pk_format_portfolio_uuid: pk_portfolio_uuid,
            sk_format_deposit_sort_criteria: sk_bank_deposit_delta_growth,
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
    }
}
