use crate::convert::reader::with_level_context::{
    deserialize_with_bank_level, LevelSpecificResponseReaderError,
};
use crate::db_item_types::BankLevelItem;
use crate::query_response_types::BankResponse;
use tracing::info;

impl TryFrom<BankLevelItem> for BankResponse {
    type Error = LevelSpecificResponseReaderError;

    fn try_from(item: BankLevelItem) -> Result<Self, Self::Error> {
        let bank_uuid = item.bank_uuid;
        let bank_name = item.bank_name;
        let portfolio_uuid = item.portfolio_uuid;
        let bank_tz = item.bank_tz;
        info!(
            "item.outcome_as_json.as_str() is {}",
            item.outcome_as_json.as_str()
        );
        let outcome = deserialize_with_bank_level(item.outcome_as_json.as_str())?;
        let created_at = item.created_at;
        Ok(BankResponse {
            bank_uuid,
            bank_name,
            portfolio_uuid,
            bank_tz,
            outcome,
            created_at,
        })
    }
}
