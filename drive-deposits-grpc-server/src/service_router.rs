use tonic::{transport::server::Router, transport::Server};
use tonic_reflection::server::Builder;
use tracing::{info, info_span, instrument};

use drive_deposits_event_source::eb::create_eb;
use drive_deposits_proto_grpc_types::generated::{
    drive_deposits_service_server::DriveDepositsServiceServer, FILE_DESCRIPTOR_SET,
};

use crate::portfolio::DriveDepositsCalculator;

#[instrument]
pub async fn app() -> Result<Router, Box<dyn std::error::Error>> {
    let server_reflection = Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;
    info!("reflection built using v1alpha since Postman gRPC still uses it as of tonic 0.12.2+");

    let eb = create_eb().await?;
    let delta = DriveDepositsCalculator {
        drive_deposits_eb: eb,
    };

    let builder = Server::builder()
        .trace_fn(|_| info_span!("drivedeposits_server"))
        .add_service(server_reflection)
        .add_service(DriveDepositsServiceServer::new(delta));
    Ok(builder)
}
