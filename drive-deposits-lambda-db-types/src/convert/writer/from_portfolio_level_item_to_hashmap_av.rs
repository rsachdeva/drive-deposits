use crate::db_item_types::PortfolioLevelItem;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

impl From<PortfolioLevelItem> for HashMap<String, AttributeValue> {
    fn from(item: PortfolioLevelItem) -> Self {
        let pk_portfolios_av = AttributeValue::S(item.pk_portfolios);
        let sk_growth_av = AttributeValue::S(item.sk_format_portfolio_delta_growth);
        let outcome_as_json_av = AttributeValue::S(item.outcome_as_json);
        let portfolio_uuid_av = AttributeValue::S(item.portfolio_uuid);
        let created_at_av = AttributeValue::S(item.created_at);

        HashMap::from([
            ("PK".to_string(), pk_portfolios_av),
            ("SK".to_string(), sk_growth_av),
            ("portfolio_uuid".to_string(), portfolio_uuid_av),
            ("outcome".to_string(), outcome_as_json_av),
            ("created_at".to_string(), created_at_av),
        ])
    }
}
