use tonic::{async_trait, Request, Response, Status};
use tracing::{debug, error, info_span};

use drive_deposits_event_source::eb::DriveDepositsEventBridge;
use drive_deposits_proto_grpc_types::generated::{
    drive_deposits_service_server::DriveDepositsService, CalculatePortfolioRequest,
    CalculatePortfolioResponse,
};

mod calculate;
mod grpc_status_handler;

pub struct DriveDepositsCalculator {
    pub drive_deposits_eb: Option<DriveDepositsEventBridge>,
}

#[async_trait]
impl DriveDepositsService for DriveDepositsCalculator {
    async fn calculate_portfolio(
        &self,
        request: Request<CalculatePortfolioRequest>,
    ) -> Result<Response<CalculatePortfolioResponse>, Status> {
        info_span!("grpc_calculate_portfolio");
        debug!("calculate_portfolio request incoming is : {:#?}", request);
        let delta_request = request.into_inner();
        let response = calculate::by_period(delta_request, self.drive_deposits_eb.clone())
            .await
            .inspect_err(|err| error!("building response errors : {:?}", err))?;
        Ok(Response::new(response))
    }
}
