use crate::convert::reader::with_level_context::{
    get_field_with_bank_level, LevelSpecificItemReaderError,
};
use crate::db_item_types::BankLevelItem;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

impl TryFrom<HashMap<String, AttributeValue>> for BankLevelItem {
    type Error = LevelSpecificItemReaderError;

    fn try_from(dynamodb_item: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let pk_portfolio_uuid = get_field_with_bank_level(&dynamodb_item, "PK")?;
        let sk_bank_delta_growth = get_field_with_bank_level(&dynamodb_item, "SK")?;
        let bank_uuid = get_field_with_bank_level(&dynamodb_item, "bank_uuid")?;
        let bank_name = get_field_with_bank_level(&dynamodb_item, "bank_name")?;
        let portfolio_uuid = get_field_with_bank_level(&dynamodb_item, "portfolio_uuid")?;
        let bank_tz = get_field_with_bank_level(&dynamodb_item, "bank_tz")?;
        let outcome_as_json = get_field_with_bank_level(&dynamodb_item, "outcome")?;
        let created_at = get_field_with_bank_level(&dynamodb_item, "created_at")?;

        Ok(BankLevelItem {
            pk_format_portfolio_uuid: pk_portfolio_uuid,
            sk_format_bank_delta_growth: sk_bank_delta_growth,
            bank_uuid,
            bank_name,
            portfolio_uuid,
            bank_tz,
            outcome_as_json,
            created_at,
        })
    }
}
