use crate::db_item_types::DepositLevelItem;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;

impl From<DepositLevelItem> for HashMap<String, AttributeValue> {
    fn from(item: DepositLevelItem) -> Self {
        let pk_portfolio_uuid_av = AttributeValue::S(item.pk_format_portfolio_uuid);
        let sk_deposit_sort_criteria = AttributeValue::S(item.sk_format_deposit_sort_criteria);
        let deposit_delta_growth = AttributeValue::S(item.deposit_delta_growth);
        let deposit_maturity_date_in_bank_tz_av =
            AttributeValue::S(item.deposit_maturity_date_in_bank_tz);
        let created_at_av = AttributeValue::S(item.created_at);
        let bank_uuid_av = AttributeValue::S(item.bank_uuid);
        let bank_name_av = AttributeValue::S(item.bank_name);
        let portfolio_uuid_av = AttributeValue::S(item.portfolio_uuid);
        let deposit_uuid_av = AttributeValue::S(item.deposit_uuid);
        let account_av = AttributeValue::S(item.account);
        let account_type_av = AttributeValue::S(item.account_type);
        let apy_av = AttributeValue::S(item.apy);
        let years_av = AttributeValue::S(item.years);
        let outcome_as_json_av = AttributeValue::S(item.outcome_as_json);
        let outcome_with_dates_as_json_av = AttributeValue::S(item.outcome_with_dates_as_json);

        HashMap::from([
            ("PK".to_string(), pk_portfolio_uuid_av),
            ("SK".to_string(), sk_deposit_sort_criteria),
            ("deposit_delta_growth".to_string(), deposit_delta_growth),
            (
                "deposit_maturity_date_in_bank_tz".to_string(),
                deposit_maturity_date_in_bank_tz_av,
            ),
            ("bank_uuid".to_string(), bank_uuid_av),
            ("bank_name".to_string(), bank_name_av),
            ("portfolio_uuid".to_string(), portfolio_uuid_av),
            ("deposit_uuid".to_string(), deposit_uuid_av),
            ("account".to_string(), account_av),
            ("account_type".to_string(), account_type_av),
            ("apy".to_string(), apy_av),
            ("years".to_string(), years_av),
            ("outcome".to_string(), outcome_as_json_av),
            (
                "outcome_with_dates".to_string(),
                outcome_with_dates_as_json_av,
            ),
            ("created_at".to_string(), created_at_av),
        ])
    }
}
