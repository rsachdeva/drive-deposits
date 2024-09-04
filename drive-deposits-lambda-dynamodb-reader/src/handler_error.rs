use crate::dynamodb::query::QueryItemError;
use crate::request_error::Error as RequestError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Default, Debug, Error)]
pub enum Error {
    #[default]
    #[error("Internal error")]
    InternalServer,

    #[error("RequestError got: {0}")]
    RequestError(#[from] RequestError),

    #[error("QueryItemError querying DynamoDB: {0}")]
    QueryItemError(#[from] QueryItemError),
}

#[derive(Serialize)]
pub struct SerializableError {
    error: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("IntoResponse for Error is {:?}", self);

        let (status, error_message) = match self {
            Error::RequestError(err) => (
                StatusCode::BAD_REQUEST,
                format!("Bad request error: {}", err),
            ),
            Error::QueryItemError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Query item error: {}", err),
            ),
            Error::InternalServer => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
        };

        error!("Reader app error message is {:?}", error_message);

        let error = SerializableError {
            error: error_message,
        };

        (status, Json(error)).into_response()
    }
}
