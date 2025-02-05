use aws_lambda_events::event::eventbridge::EventBridgeEvent;
use drive_deposits_logs_lambda_target::dynamodb::add::add_item;
use drive_deposits_logs_lambda_target::dynamodb::DriveDepositsDb;
use drive_deposits_rest_types::rest_types::CalculatePortfolioResponse;
use lambda_runtime::{
    run, service_fn,
    tracing::{debug, error, info_span, init_default_subscriber, instrument, Instrument},
    Error, LambdaEvent,
};
use serde_json::from_value;

#[instrument(skip(db_handler, event))]
async fn banks_level_handler(
    db_handler: &DriveDepositsDb,
    event: LambdaEvent<EventBridgeEvent>,
) -> Result<(), Error> {
    // Extract some useful information from the request
    let span = info_span!("banks-level-handler-event-bridge-event");
    span.in_scope(|| debug!("inside banks_level_handler"));
    // debug!("EventBridgeEvent event bridge event: {}", event);
    let payload = event.payload;
    debug!("payload.detail_type: {:?}", payload.detail_type);
    debug!("payload.source: {:?}", payload.source);
    debug!("db_handler table name is: {:?}", db_handler.table_name);
    let payload_detail = payload.detail;
    // debug!("payload.detail is  {:#?}", payload_detail);
    debug!("event_target_response is  ------------ ");
    let event_target_response: CalculatePortfolioResponse = from_value(payload_detail)
        .inspect_err(|err| error!("Failed to deserialize payload_detail: {:?}", err))?;
    debug!("event_target_response is  {:#?}", event_target_response);

    add_item(
        &db_handler.dynamodb_client,
        db_handler.table_name.as_str(),
        event_target_response,
    )
    .instrument(span)
    .await?;

    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("before init tracing subscriber");
    init_default_subscriber();

    let span = info_span!("by_level_lambda_writer_main");
    span.in_scope(|| debug!("after init_default_subscriber"));

    let db_handler = DriveDepositsDb::handler().await?;

    // service_fn returns ServiceFn that implements FnMut so cannot pass ownership of db_handler
    // closure is `FnOnce` if it moves the variable `db_handler` out of its environment
    run(service_fn(|event| banks_level_handler(&db_handler, event)))
        .instrument(span)
        .await?;

    Ok(())
}
