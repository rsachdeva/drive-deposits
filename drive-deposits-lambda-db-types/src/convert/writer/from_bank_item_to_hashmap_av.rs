use crate::db_item_types::BankLevelItem;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

impl From<BankLevelItem> for HashMap<String, AttributeValue> {
    fn from(item: BankLevelItem) -> Self {
        let pk_portfolio_uuid_av = AttributeValue::S(item.pk_format_portfolio_uuid);
        let created_at_av = AttributeValue::S(item.created_at);
        let sk_bank_delta_growth_av = AttributeValue::S(item.sk_format_bank_delta_growth);
        let bank_uuid_av = AttributeValue::S(item.bank_uuid);
        let bank_name_av = AttributeValue::S(item.bank_name);
        let portfolio_uuid_av = AttributeValue::S(item.portfolio_uuid);
        let bank_tz_av = AttributeValue::S(item.bank_tz);
        let outcome_as_json_av = AttributeValue::S(item.outcome_as_json);

        HashMap::from([
            ("PK".to_string(), pk_portfolio_uuid_av),
            ("SK".to_string(), sk_bank_delta_growth_av),
            ("bank_uuid".to_string(), bank_uuid_av),
            ("bank_name".to_string(), bank_name_av),
            ("portfolio_uuid".to_string(), portfolio_uuid_av),
            ("bank_tz".to_string(), bank_tz_av),
            ("outcome".to_string(), outcome_as_json_av),
            ("created_at".to_string(), created_at_av),
        ])
    }
}
