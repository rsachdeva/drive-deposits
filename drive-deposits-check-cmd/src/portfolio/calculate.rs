use thiserror::Error;
use tracing::{debug, instrument};
use validator::Validate;

use drive_deposits_cal_types::cal_types::PortfolioRequest as CalBankRequest;
use drive_deposits_cal_types::math::engine::calculate_portfolio;
use drive_deposits_event_source::eb::create_eb;
use drive_deposits_proto_grpc_types::generated::{
    CalculatePortfolioRequest as GrpcCalculatePortfolioRequest,
    CalculatePortfolioResponse as GrpcCalculatePortfolioResponse,
};
use drive_deposits_rest_types::rest_types::{
    CalculatePortfolioRequest as RestCalculatePortfolioRequest,
    CalculatePortfolioResponse as RestCalculatePortfolioResponse,
};

#[derive(Default, Debug, Error)]
pub enum Error {
    #[default]
    #[error("Internal server error")]
    InternalServer,

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Validation errors: {0}")]
    Validation(#[from] validator::ValidationErrors),

    #[error("Calculation halt error could not progress, so had to halt: errors: {0}")]
    CalculationHalt(#[from] drive_deposits_cal_types::math::engine::CalculationHaltError),

    #[error("Drive Deposits EventBridge error: {0}")]
    DriveDepositsEventBridgeError(
        #[from] drive_deposits_event_source::eb::DriveDepositsEventBridgeError,
    ),
}

#[instrument]
pub async fn process_input(file_path: String) -> Result<String, Error> {
    let data = std::fs::read_to_string(file_path)?;

    // deserialize file into RestCalculatePortfolioRequest request and validate input data
    let rest_req: RestCalculatePortfolioRequest = serde_json::from_str(&data)?;
    debug!("Deserialized rest: {:?}", rest_req);
    validate(&rest_req)?;
    debug!("Validated rest: {:?}", rest_req);

    // convert from rest CalculatePortfolioRequest to grpc CalculatePortfolioRequest
    let grpc_req: GrpcCalculatePortfolioRequest = rest_req.into();
    debug!("Converted from rest to grpc: {:?}", grpc_req);

    // convert grpc CalculatePortfolioRequest to calculator CalculatePortfolioRequest
    let cal_req: CalBankRequest = grpc_req.into();
    debug!("Converted from grpc to cal: {:?}", cal_req);

    let drive_deposits_eb = create_eb().await?;

    // process calculation for calculator CalculatePortfolioRequest
    let cal_resp = calculate_portfolio(cal_req, drive_deposits_eb).await?;
    debug!("calculated response: {:?}", cal_resp);

    // convert response fom calculator CalculatePortfolioResponse to grpc CalculatePortfolioResponse
    let grpc_resp: GrpcCalculatePortfolioResponse = cal_resp.into();
    debug!("grpc response: {:?}", grpc_resp);
    // convert response from grpc CalculatePortfolioResponse to rest CalculatePortfolioResponse
    let rest_resp: RestCalculatePortfolioResponse = grpc_resp.into();

    debug!("rest response: {:?}", rest_resp);

    let json_resp = serde_json::to_string(&rest_resp)?;
    // pretty print serialized json
    let pretty_json_resp = serde_json::to_string_pretty(&rest_resp)?;
    debug!("pretty serialized json response: {}", pretty_json_resp);
    Ok(json_resp)
}

fn validate(rest: &RestCalculatePortfolioRequest) -> Result<(), Error> {
    rest.validate()?;
    Ok(())
}
