use axum::Json;
use tracing::{debug, debug_span, error, info};

use app_error::Error as AppError;
use drive_deposits_proto_grpc_types::generated::drive_deposits_service_client::DriveDepositsServiceClient;
use drive_deposits_rest_types::rest_types::CalculatePortfolioResponse;
use request_error::ValidateCalculateRequest;

mod app_error;

mod request_error;

// #[debug_handler]
pub async fn calculate_portfolio(
    ValidateCalculateRequest(rest_delta_request): ValidateCalculateRequest,
) -> Result<Json<CalculatePortfolioResponse>, AppError> {
    let span = debug_span!("rest_calculate_portfolio");
    span.in_scope(|| {
        debug!(
            "calculate_portfolio request incoming is : {:#?}",
            rest_delta_request
        )
    });

    let mut client = DriveDepositsServiceClient::connect("http://[::1]:50052")
        .await
        .inspect_err(|err| {
            span.in_scope(|| error!("grpc client connection error: {:?}", err));
        })?;
    let grpc_delta_request = span.in_scope(|| rest_delta_request.into());

    let grpc_request = tonic::Request::new(grpc_delta_request);
    let grpc_response = client.calculate_portfolio(grpc_request).await?;

    span.in_scope(|| {
        info!(
            "grpc response in client for rest server acting as gateway is : {:#?}",
            grpc_response
        );
        let grpc_delta_response = grpc_response.into_inner();
        let rest_response = grpc_delta_response.into();
        info!(
            "rest response in client for rest server acting as gateway is : {:#?}",
            rest_response
        );
        Ok(Json(rest_response))
    })
}
