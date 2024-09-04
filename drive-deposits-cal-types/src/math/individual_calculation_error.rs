use std::num::ParseIntError;

use thiserror::Error;
use uuid::Uuid;

use crate::cal_types::ProcessingError;

#[derive(Default, Debug, Error)]
pub enum Error {
    #[default]
    #[error("Internal error Individual Calculation is incomplete")]
    Internal,

    #[error("ParseInt error Individual Calculation is incomplete: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("ZeroYears error Individual Calculation is incomplete: {0}")]
    ZeroYears(String),
}

impl From<Error> for ProcessingError {
    fn from(error: Error) -> Self {
        match error {
            Error::ParseInt(e) => ProcessingError {
                uuid: Uuid::new_v4(),
                message: e.to_string(),
            },
            _ => ProcessingError {
                uuid: Uuid::new_v4(),
                message: error.to_string(),
            },
        }
    }
}
