use thiserror::Error as thisError;

#[derive(Debug, thisError)]
pub enum Error {
    #[error("Invalid Uuid: {0}")]
    InvalidUuid(String),
}
