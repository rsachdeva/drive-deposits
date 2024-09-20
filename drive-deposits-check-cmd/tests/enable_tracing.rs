use anyhow::Result;
use once_cell::sync::OnceCell;
use tracing::{debug, Span};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry;
use tracing_subscriber::util::SubscriberInitExt;

static INIT: OnceCell<()> = OnceCell::new();

pub fn setup_tracing_subscriber() -> Result<()> {
    let rust_log = std::env::var("RUST_LOG")?;
    println!("RUST_LOG is {}", rust_log);
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_test_writer()
        .with_span_events(FmtSpan::FULL);
    registry().with(fmt_layer).try_init()?;
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
