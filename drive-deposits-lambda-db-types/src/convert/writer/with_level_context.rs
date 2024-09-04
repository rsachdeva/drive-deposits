use thiserror::Error;

#[derive(Debug, Error)]
pub enum WriterErrorLevel {
    #[error("Bank")]
    Bank,
    #[error("Deposit")]
    Deposit,
    #[error("Portfolio")]
    Portfolio,
}

#[derive(Debug, Error)]
pub enum ItemWriterError {
    #[error("Missing data field: {0}")]
    MissingDataField(String),
}

#[derive(Debug, Error)]
pub enum LevelSpecificItemWriterError {
    #[error("{1} level: {0}")]
    Error(#[source] ItemWriterError, WriterErrorLevel),
    #[error("{1} level conversion DecimalError: {0}")]
    DecimalError(#[source] rust_decimal::Error, WriterErrorLevel),
    #[error("{1} level conversion SerdeJsonError: {0}")]
    JsonError(#[source] serde_json::Error, WriterErrorLevel),
}

// pub fn error_with_level<E>(err: E, level: WriterErrorLevel) -> LevelSpecificItemWriterError
// where
//     (E, WriterErrorLevel): Into<LevelSpecificItemWriterError>,
// {
//     (err, level).into()
// }
pub fn error_with_level<E>(err: E, level: WriterErrorLevel) -> LevelSpecificItemWriterError
where
    //means that the type (E, WriterErrorLevel) must implement the From trait for LevelSpecificItemWriterError
    LevelSpecificItemWriterError: From<(E, WriterErrorLevel)>,
{
    LevelSpecificItemWriterError::from((err, level))
}

pub fn error_with_portfolio_level<E>(err: E) -> LevelSpecificItemWriterError
where
    LevelSpecificItemWriterError: From<(E, WriterErrorLevel)>,
{
    error_with_level(err, WriterErrorLevel::Portfolio)
}

pub fn error_with_bank_level<E>(err: E) -> LevelSpecificItemWriterError
where
    LevelSpecificItemWriterError: From<(E, WriterErrorLevel)>,
{
    error_with_level(err, WriterErrorLevel::Bank)
}

pub fn error_with_deposit_level<E>(err: E) -> LevelSpecificItemWriterError
where
    LevelSpecificItemWriterError: From<(E, WriterErrorLevel)>,
{
    error_with_level(err, WriterErrorLevel::Deposit)
}
impl From<(ItemWriterError, WriterErrorLevel)> for LevelSpecificItemWriterError {
    fn from((err, level): (ItemWriterError, WriterErrorLevel)) -> Self {
        Self::Error(err, level)
    }
}

impl From<(rust_decimal::Error, WriterErrorLevel)> for LevelSpecificItemWriterError {
    fn from((err, level): (rust_decimal::Error, WriterErrorLevel)) -> Self {
        Self::DecimalError(err, level)
    }
}

impl From<(serde_json::Error, WriterErrorLevel)> for LevelSpecificItemWriterError {
    fn from((err, level): (serde_json::Error, WriterErrorLevel)) -> Self {
        Self::JsonError(err, level)
    }
}
