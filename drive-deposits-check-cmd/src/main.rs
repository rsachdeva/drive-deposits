use anyhow::Result;
use clap::Parser;
use tracing::{debug, info, info_span, Instrument};
use tracing_subscriber::{layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter};

use drive_deposits_check_cmd::portfolio::calculate::process_input;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// The `drive-deposits-cal-types` command performs comprehensive local calculations. It mirrors the calculations done by the REST gateway, which forwards requests to the gRPC deposits service for identical processing. This dual functionality allows `drive-deposits-cal-types` to serve as a reliable tool for verifying calculations from REST and gRPC submissions.
///
/// The data types used in this command are consistent with those in the actual services, ensuring a dependable method for rapid local verification of calculations.
/// Use this command to verify calculations locally.
///
/// To run with a sample input file: see the "Hybrid Integration Testing Tool" section in the README.md file.
///
struct Args {
    /// Input json file path
    #[arg(required(true))]
    json_request_file_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    registry()
        .with(EnvFilter::try_from_default_env()?)
        // added in .cargo/config.tom .add_directive("drive_deposits_local=debug".parse()?))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();
    debug!("args: {:?}", args);
    let span = info_span!(
        "drive_deposits_check_cmd",
        json_request_file_path = args.json_request_file_path.as_str()
    );
    let response = process_input(args.json_request_file_path)
        .instrument(span)
        .await?;
    info!("response after processing request locally is: {}", response);
    Ok(())
}
