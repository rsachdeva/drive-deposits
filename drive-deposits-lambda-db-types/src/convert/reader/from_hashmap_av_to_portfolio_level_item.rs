use crate::convert::reader::with_level_context::{
    get_field_with_portfolio_level, LevelSpecificItemReaderError,
};
use crate::db_item_types::PortfolioLevelItem;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

// #[derive(Error, Debug)]
// pub enum PortfolioLevelItemReaderError {
//     #[error("PortfolioLevelItemReaderError with MissingAttribute error: {0}")]
//     MissingAttribute(String),
//
//     #[error("PortfolioLevelItemReaderError with ConversionToStringError error: {0}")]
//     ConversionToStringError(String),
// }

impl TryFrom<HashMap<String, AttributeValue>> for PortfolioLevelItem {
    type Error = LevelSpecificItemReaderError;

    fn try_from(dynamodb_item: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let pk_portfolios = get_field_with_portfolio_level(&dynamodb_item, "PK")?;
        let sk_response_delta_growth = get_field_with_portfolio_level(&dynamodb_item, "SK")?;
        let portfolio_uuid = get_field_with_portfolio_level(&dynamodb_item, "portfolio_uuid")?;
        let outcome_as_json = get_field_with_portfolio_level(&dynamodb_item, "outcome")?;
        let created_at = get_field_with_portfolio_level(&dynamodb_item, "created_at")?;

        let responses_level_item = PortfolioLevelItem {
            pk_portfolios,
            sk_format_portfolio_delta_growth: sk_response_delta_growth,
            portfolio_uuid,
            outcome_as_json,
            created_at,
        };
        Ok(responses_level_item)
    }
}
