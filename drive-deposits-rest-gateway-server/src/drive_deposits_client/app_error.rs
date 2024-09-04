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
    #[error("Tonic transport error: {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("Tonic status error:  {0}")]
    Status(#[from] tonic::Status),
}

#[derive(Serialize)]
pub struct SerializableError {
    error: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("IntoResponse for Error is {:?}", self);

        let (status, error_message) = match self {
            Error::InternalServer => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            Error::Transport(err) => (
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Connect Server Error {:?}", err),
            ),
            Error::Status(err) => (
                StatusCode::BAD_GATEWAY,
                format!("gRPC Server Error {:?}", err),
            ),
        };

        error!("{}", error_message);

        let error = SerializableError {
            error: error_message,
        };

        (status, Json(error)).into_response()
    }
}
