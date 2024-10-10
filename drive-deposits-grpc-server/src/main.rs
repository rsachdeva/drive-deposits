use std::error::Error;

use tracing::{error, info, info_span, Instrument};
use tracing_subscriber::{layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter};

use drive_deposits_grpc_server::service_router::app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    registry()
        .with(
            EnvFilter::try_from_default_env()
                .inspect_err(|err| println!(" error err is {:?}", err))?, // added in .cargo/config.toml.add_directive("drive_deposits_grpc=debug".parse()?),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("going to check RUST_LOG");
    let rust_log = std::env::var("RUST_LOG")?;
    println!("RUST_LOG is {}", rust_log);

    let span = info_span!("main");
    let app = app().instrument(span.clone()).await?;
    let addr = "[::]:50052".parse().unwrap();

    span.in_scope(|| info!("gRPC server running! on {}", addr));
    let serve_result = app.serve(addr).instrument(info_span!("server")).await;

    serve_result.inspect_err(|e| {
        span.in_scope(|| error!("gRPC server error is {:?}", e));
    })?;
    Ok(())
}
