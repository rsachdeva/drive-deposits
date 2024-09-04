use crate::convert::reader::with_level_context::{
    deserialize_with_deposit_level, LevelSpecificResponseReaderError,
};
use crate::db_item_types::DepositLevelItem;
use crate::query_response_types::DepositResponse;

impl TryFrom<DepositLevelItem> for DepositResponse {
    type Error = LevelSpecificResponseReaderError;

    fn try_from(item: DepositLevelItem) -> Result<Self, Self::Error> {
        let bank_uuid = item.bank_uuid;
        let bank_name = item.bank_name;
        let portfolio_uuid = item.portfolio_uuid;
        let deposit_uuid = item.deposit_uuid;
        let account = item.account;
        let account_type = item.account_type;
        let apy = item.apy;
        let years = item.years;
        let outcome = deserialize_with_deposit_level(item.outcome_as_json.as_str())?;
        let outcome_with_dates =
            deserialize_with_deposit_level(item.outcome_with_dates_as_json.as_str())?;

        let created_at = item.created_at;
        let deposit_delta_growth = item.deposit_delta_growth;
        let deposit_maturity_date_in_bank_tz = item.deposit_maturity_date_in_bank_tz;

        Ok(DepositResponse {
            bank_uuid,
            bank_name,
            portfolio_uuid,
            deposit_uuid,
            account,
            account_type,
            apy,
            years,
            deposit_delta_growth,
            deposit_maturity_date_in_bank_tz,
            outcome,
            outcome_with_dates,
            created_at,
        })
    }
}
