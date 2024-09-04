use std::error::Error;

use tracing::{debug, debug_span, info, Instrument};
use tracing_subscriber::{layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter};

use drive_deposits_rest_gateway_server::service_router::router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    registry()
        .with(
            EnvFilter::try_from_default_env()?
                // added in .cargo/config.toml .add_directive("drive_deposits_rest_grpc_gateway=debug".parse()?)
                .add_directive("axum::rejection=trace".parse()?)
                .add_directive("tower_http=debug".parse()?),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let rust_log = std::env::var("RUST_LOG")?;
    println!("RUST_LOG is {}", rust_log);

    let span = debug_span!("main");
    tracing::dispatcher::get_default(|dispatch| -> Option<()> {
        let metadata = span.metadata()?;
        if dispatch.enabled(metadata) {
            println!("Span event is emitted");
        } else {
            println!("Span event is not emitted");
        }
        None
    });
    span.in_scope(|| debug!("router is being set up first up"));
    let app_router = router().instrument(span.clone()).await;

    let span = tracing::span!(tracing::Level::INFO, "server");
    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    let listening_on = listener.local_addr()?;
    span.in_scope(|| info!("listening on {}", listening_on));
    axum::serve(listener, app_router).await?;
    Ok(())
}
