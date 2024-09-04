use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ReaderErrorLevel {
    #[error("Bank")]
    Bank,
    #[error("Deposit")]
    Deposit,
    #[error("Portfolio")]
    Portfolio,
}

#[derive(Debug, Error, Clone)]
pub enum ItemReaderError {
    #[error("Missing attribute: {0}")]
    MissingAttribute(String),
    #[error("Conversion to string error for attribute: {0}")]
    ConversionToStringError(String),
}

#[derive(Debug, Error, Clone)]
pub enum LevelSpecificItemReaderError {
    #[error("{level} level: {source}")]
    Error {
        #[source]
        source: ItemReaderError,
        level: ReaderErrorLevel,
    },
}

pub fn get_field_value_with_level(
    dynamodb_item: &HashMap<String, AttributeValue>,
    field_name: &str,
    level: ReaderErrorLevel,
) -> Result<String, LevelSpecificItemReaderError> {
    let field_value = dynamodb_item
        .get(field_name)
        .ok_or_else(|| LevelSpecificItemReaderError::Error {
            source: ItemReaderError::MissingAttribute(field_name.to_string()),
            level: level.clone(),
        })?
        .as_s()
        .map_err(|_av| LevelSpecificItemReaderError::Error {
            source: ItemReaderError::ConversionToStringError(field_name.to_string()),
            level,
        })?
        .to_owned();
    Ok(field_value)
}

pub fn get_field_with_portfolio_level(
    dynamodb_item: &HashMap<String, AttributeValue>,
    field_name: &str,
) -> Result<String, LevelSpecificItemReaderError> {
    get_field_value_with_level(dynamodb_item, field_name, ReaderErrorLevel::Portfolio)
}

pub fn get_field_with_bank_level(
    dynamodb_item: &HashMap<String, AttributeValue>,
    field_name: &str,
) -> Result<String, LevelSpecificItemReaderError> {
    get_field_value_with_level(dynamodb_item, field_name, ReaderErrorLevel::Bank)
}

pub fn get_field_with_deposit_level(
    dynamodb_item: &HashMap<String, AttributeValue>,
    field_name: &str,
) -> Result<String, LevelSpecificItemReaderError> {
    get_field_value_with_level(dynamodb_item, field_name, ReaderErrorLevel::Deposit)
}

#[derive(Debug, Error)]
pub enum LevelSpecificResponseReaderError {
    #[error("{level} level conversion SerdeJsonError: {source}")]
    JsonError {
        source: serde_json::Error,
        level: ReaderErrorLevel,
    },
}

pub fn deserialize_with_error_level<S>(
    json_str: &str,
    level: ReaderErrorLevel,
) -> Result<S, LevelSpecificResponseReaderError>
where
    S: serde::de::DeserializeOwned,
{
    serde_json::from_str(json_str)
        .map_err(|source| LevelSpecificResponseReaderError::JsonError { source, level })
}

pub fn deserialize_with_portfolio_level<T>(
    json_str: &str,
) -> Result<T, LevelSpecificResponseReaderError>
where
    T: serde::de::DeserializeOwned,
{
    deserialize_with_error_level(json_str, ReaderErrorLevel::Portfolio)
}

pub fn deserialize_with_bank_level<T>(json_str: &str) -> Result<T, LevelSpecificResponseReaderError>
where
    T: serde::de::DeserializeOwned,
{
    deserialize_with_error_level(json_str, ReaderErrorLevel::Bank)
}

pub fn deserialize_with_deposit_level<T>(
    json_str: &str,
) -> Result<T, LevelSpecificResponseReaderError>
where
    T: serde::de::DeserializeOwned,
{
    deserialize_with_error_level(json_str, ReaderErrorLevel::Deposit)
}
