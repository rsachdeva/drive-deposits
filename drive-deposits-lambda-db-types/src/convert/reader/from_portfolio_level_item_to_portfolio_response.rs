use crate::convert::reader::with_level_context::{
    deserialize_with_portfolio_level, LevelSpecificResponseReaderError,
};
use crate::db_item_types::PortfolioLevelItem;
use crate::query_response_types::PortfolioResponse;

impl TryFrom<PortfolioLevelItem> for PortfolioResponse {
    type Error = LevelSpecificResponseReaderError;

    fn try_from(item: PortfolioLevelItem) -> Result<Self, Self::Error> {
        let portfolio_uuid = item.portfolio_uuid;
        let outcome = deserialize_with_portfolio_level(item.outcome_as_json.as_str())?;
        let created_at = item.created_at;
        Ok(Self {
            portfolio_uuid,
            outcome,
            created_at,
        })
    }
}

// checked with
// get_json_instance_from_str("{\"delta\": {\"period\":\"1\",\"period_unit\":\"Month\",\"growth\":\"34.43\",\"maturity\":{\"amount\":\"11990.50\",\"interest\":\"6210.02\",\"total\":\"18200.52\",\"errors\":[]}", ReaderErrorLevel::Portfolio)?;
