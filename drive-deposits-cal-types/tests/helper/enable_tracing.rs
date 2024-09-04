use anyhow::Result;
use once_cell::sync::OnceCell;
use tracing::{debug, Span};
use tracing_subscriber::{EnvFilter, registry};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

static INIT: OnceCell<()> = OnceCell::new();

pub fn setup_tracing_subscriber() -> Result<()> {
    let rust_log = std::env::var("RUST_LOG")?;
    println!("RUST_LOG is {}", rust_log);
    // can change level to debug to see all debug messages in  tests
    // or to info not to see
    // for test span added test=debug here
    registry()
        .with(EnvFilter::try_from_default_env()?.add_directive("test=debug".parse()?))
        .with(tracing_subscriber::fmt::layer())
        .init();
    debug!("tracing subscriber initialized");
    Ok(())
}

pub fn initialize_test_span(test_name: &str) -> Span {
    INIT.get_or_init(|| {
        setup_tracing_subscriber().expect("Failed to set up tracing subscriber");
    });
    debug!("Running the test: {}", test_name);
    let span = tracing::info_span!("test", test_name = test_name);
    span
}
