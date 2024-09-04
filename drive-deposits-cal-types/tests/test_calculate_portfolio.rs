use tracing::{debug, Instrument};

use drive_deposits_cal_types::cal_types::NewDelta;
use drive_deposits_cal_types::cal_types::PortfolioRequest;
use drive_deposits_cal_types::math::engine::calculate_portfolio;
use drive_deposits_proto_grpc_types::generated::PeriodUnit;
use helper::enable_tracing::initialize_test_span;

mod helper;
#[tokio::test]
async fn test_calculate_by_period_empty_no_new_banks_without_events() {
    let span = initialize_test_span("test_calculate_by_period_empty_no_new_banks");

    let bank_req = PortfolioRequest {
        new_banks: vec![],
        new_delta: NewDelta {
            period: Default::default(),
            period_unit: PeriodUnit::Day,
        },
    };

    // don't have to spawn a task necessarily or even async move since test is async already
    let result = calculate_portfolio(bank_req, None).instrument(span).await;
    debug!("finally result: {:?}", result);
}
