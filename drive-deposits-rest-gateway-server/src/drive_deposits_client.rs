use axum::{async_trait, Json};
use mockall::automock;
use std::env::var;
use tracing::{debug, debug_span, error, info};

use app_error::Error as AppError;
use drive_deposits_proto_grpc_types::generated::{
    drive_deposits_service_client::DriveDepositsServiceClient,
    CalculatePortfolioRequest as GrpcCalculatePortfolioRequest,
    CalculatePortfolioResponse as GrpcCalculatePortfolioResponse,
};
use drive_deposits_rest_types::rest_types::{
    CalculatePortfolioRequest as RestCalculatePortfolioRequest,
    CalculatePortfolioResponse as RestCalculatePortfolioResponse,
};
use request_error::ValidateCalculateRequest;
use tonic::transport::Channel;

mod app_error;
mod request_error;

#[automock]
#[async_trait]
pub trait CalculatePortfolioClient {
    async fn calculate_portfolio_request(
        mut self,
        request: tonic::Request<GrpcCalculatePortfolioRequest>,
    ) -> Result<tonic::Response<GrpcCalculatePortfolioResponse>, tonic::Status>;
}

#[async_trait]
impl CalculatePortfolioClient for DriveDepositsServiceClient<Channel> {
    //  Differentiating the trait method name from the method being called on self helps avoid confusion and potential infinite recursion
    async fn calculate_portfolio_request(
        mut self,
        request: tonic::Request<GrpcCalculatePortfolioRequest>,
    ) -> Result<tonic::Response<GrpcCalculatePortfolioResponse>, tonic::Status> {
        // Call the actual gRPC method instead of recursively calling itself: using associated style
        // DriveDepositsServiceClient::calculate_portfolio(&mut self, request).await
        self.calculate_portfolio(request).await
    }
}

// #[debug_handler]
pub async fn calculate_portfolio(
    ValidateCalculateRequest(rest_delta_request): ValidateCalculateRequest,
) -> Result<Json<RestCalculatePortfolioResponse>, AppError> {
    let span = debug_span!("calculate_portfolio");
    span.in_scope(|| {
        debug!(
            "calculate_portfolio request incoming is : {:#?}",
            rest_delta_request
        )
    });

    // for docker compose dns GRPC_SERVER_ADDRESS=http://drive-deposits-grpc-server:50052
    let grpc_server_address =
        var("GRPC_SERVER_ADDRESS").unwrap_or_else(|_| "http://[::]:50052".to_string());
    info!("grpc_server_address is: {}", grpc_server_address);
    let client = DriveDepositsServiceClient::connect(grpc_server_address)
        .await
        .inspect_err(|err| {
            span.in_scope(|| error!("grpc client connection error: {:?}", err));
        })?;
    calculate_portfolio_with_client(rest_delta_request, client).await
}

pub async fn calculate_portfolio_with_client(
    rest_delta_request: RestCalculatePortfolioRequest,
    client: impl CalculatePortfolioClient,
) -> Result<Json<RestCalculatePortfolioResponse>, AppError> {
    let span = debug_span!("calculate_portfolio_with_client");
    span.in_scope(|| {
        debug!(
            "calculate_portfolio request incoming is : {:#?}",
            rest_delta_request
        )
    });

    let grpc_delta_request = span.in_scope(|| rest_delta_request.into());

    let grpc_request = tonic::Request::new(grpc_delta_request);
    let grpc_response = client.calculate_portfolio_request(grpc_request).await?;

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

#[cfg(test)]
mod tests {
    use super::calculate_portfolio_with_client;
    use crate::drive_deposits_client::MockCalculatePortfolioClient;
    use drive_deposits_proto_grpc_types::generated::{
        Bank as GrpcBank, CalculatePortfolioResponse as GrpcCalculatePortfolioResponse,
        Outcome as GrpcOutcome,
    };
    use drive_deposits_rest_types::rest_types::{
        Bank as RestBank, CalculatePortfolioRequest as RestCalculatePortfolioRequest,
        NewDelta as RestNewDelta, Outcome as RestOutcome,
    };
    use pretty_assertions::assert_eq;
    use tracing_subscriber::fmt::format::FmtSpan;

    fn init_test_subscriber() {
        let subscriber = tracing_subscriber::fmt()
            .with_test_writer()
            .with_span_events(FmtSpan::FULL)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }

    #[tokio::test]
    async fn test_calculate_portfolio_with_client() {
        init_test_subscriber();
        let mut mock_client = MockCalculatePortfolioClient::new();

        let test_grpc_bank = GrpcBank {
            name: "name".to_string(),
            uuid: "uuid".to_string(),
            bank_tz: "bank_tz".to_string(),
            deposits: vec![],
            outcome: Some(GrpcOutcome {
                delta: None,
                maturity: None,
                errors: vec![],
            }),
        };
        mock_client
            .expect_calculate_portfolio_request()
            .returning(move |_grpc_request| {
                let grpc_response = GrpcCalculatePortfolioResponse {
                    uuid: "uuid".to_string(),
                    banks: vec![test_grpc_bank.clone()],
                    outcome: None,
                    created_at: "created_at".to_string(),
                };
                Ok(tonic::Response::new(grpc_response))
            });

        let rest_request = RestCalculatePortfolioRequest {
            new_banks: vec![],
            new_delta: RestNewDelta {
                period: "1".to_string(),
                period_unit: "Month".to_string(),
            },
        };
        let result = calculate_portfolio_with_client(rest_request, mock_client).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.0.uuid, "uuid");
        let expected_bank = RestBank {
            name: "name".to_string(),
            uuid: "uuid".to_string(),
            bank_tz: "bank_tz".to_string(),
            deposits: vec![],
            outcome: Some(RestOutcome {
                delta: None,
                maturity: None,
                errors: vec![],
            }),
        };
        let actual_bank_tz = response.0.banks.first().unwrap().bank_tz.clone();
        assert_eq!(actual_bank_tz, expected_bank.bank_tz);
    }
}
