use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{debug_handler, Json, Router};
use drive_deposits_lambda_db_types::db_item_types::DepositSortCriteria;
use drive_deposits_lambda_db_types::query_response_types::{
    ByLevelForBanks, ByLevelForDeposits, ByLevelForPortfolios, ItemParamsRequest,
};
use drive_deposits_lambda_dynamodb_reader::dynamodb::query::{
    query_banks, query_deposits, query_portfolios,
};
use drive_deposits_lambda_dynamodb_reader::dynamodb::DriveDepositsDb;
use drive_deposits_lambda_dynamodb_reader::handler_error::Error as HandlerError;
use drive_deposits_lambda_dynamodb_reader::request_error::Error as RequestError;
use lambda_http::{
    http::StatusCode,
    run, service_fn,
    tracing::{debug, error, info_span, init_default_subscriber},
    Error as LambdaHttpError, Request, Service,
};
use serde_json::{json, Value};
use std::env::set_var;
use std::sync::Arc;
use tracing::{info, Instrument, Span};
use uuid::Uuid;

async fn root() -> Json<Value> {
    Json(
        json!({ "msg": "Use on of the the routes /by-level-for-portfolios/delta-growth; /by-level-for-banks/delta-growth/:pk_portfolio_uuid; /by-level-for-deposits/delta-growth/:pk_portfolio_uuid; /by-bank-deposits-level/maturity-date/:pk_portfolio_uuid" }),
    )
}

async fn health_check() -> (StatusCode, String) {
    let health = true;
    match health {
        true => (StatusCode::OK, "Healthy!".to_string()),
        false => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Not healthy!".to_string(),
        ),
    }
}

// #[debug_handler]
// using just Query<ItemParamsRequest> instead of Option<Query<ItemParamsRequest>> would be sufficient and more straightforward.
// Since Axum always provides a Some value with an empty ItemParamsRequest when no query parameters are passed, wrapping it in an Option doesn't add any extra functionality.
async fn by_level_query_portfolios_delta_growth(
    Query(query_item_params): Query<ItemParamsRequest>,
    State(db_handler): State<Arc<DriveDepositsDb>>,
) -> Result<Json<ByLevelForPortfolios>, HandlerError> {
    let span = info_span!("handler by_level_query_portfolios_delta_growth");
    span.in_scope(|| {
        debug!(
            "inside by_portfolios_level query_item_params is {:?}",
            query_item_params
        );
    });

    let portfolios = query_portfolios(
        &db_handler.dynamodb_client,
        db_handler.table_name.as_str(),
        query_item_params,
    )
    .instrument(span)
    .await?;

    Ok(Json(portfolios))
}

// #[debug_handler]
async fn by_level_query_banks_delta_growth(
    Path(pk_portfolio_uuid): Path<String>,
    Query(query_item_params): Query<ItemParamsRequest>,
    State(db_handler): State<Arc<DriveDepositsDb>>,
) -> Result<Json<ByLevelForBanks>, HandlerError> {
    let span = info_span!("handler by_level_query_banks_delta_growth");

    span.in_scope(|| {
        debug!(
            "pk_portfolio_uuid in banks level reader is: {:?}",
            pk_portfolio_uuid
        );
        debug!(
            "inside by_banks_level query_item_params is {:?}",
            query_item_params
        )
    });

    validate_uuid(pk_portfolio_uuid.as_str())?;

    let banks = query_banks(
        &db_handler.dynamodb_client,
        db_handler.table_name.as_str(),
        query_item_params,
        pk_portfolio_uuid,
    )
    .instrument(span)
    .await?;

    Ok(Json(banks))
}

#[debug_handler]
async fn by_level_query_deposits_delta_growth(
    Path(pk_portfolio_uuid): Path<String>,
    Query(query_item_params): Query<ItemParamsRequest>,
    State(db_handler): State<Arc<DriveDepositsDb>>,
) -> Result<Json<ByLevelForDeposits>, HandlerError> {
    let span = info_span!("handler by_level_query_deposits_delta_growth");

    by_level_query_deposits(
        Path(pk_portfolio_uuid),
        Query(query_item_params),
        State(db_handler),
        DepositSortCriteria::DeltaPeriodGrowth,
    )
    .instrument(span)
    .await
}

#[debug_handler]
async fn by_level_query_deposits_maturity_date(
    Path(pk_portfolio_uuid): Path<String>,
    Query(query_item_params): Query<ItemParamsRequest>,
    State(db_handler): State<Arc<DriveDepositsDb>>,
) -> Result<Json<ByLevelForDeposits>, HandlerError> {
    let span = info_span!("handler by_level_query_deposits_maturity_date");

    by_level_query_deposits(
        Path(pk_portfolio_uuid),
        Query(query_item_params),
        State(db_handler),
        DepositSortCriteria::MaturityDate,
    )
    .instrument(span)
    .await
}

async fn by_level_query_deposits(
    Path(pk_portfolio_uuid): Path<String>,
    Query(query_item_params): Query<ItemParamsRequest>,
    State(db_handler): State<Arc<DriveDepositsDb>>,
    deposit_sort_criteria: DepositSortCriteria,
) -> Result<Json<ByLevelForDeposits>, HandlerError> {
    debug!(
        "pk_portfolio_uuid in bank deposits level reader is: {:?}",
        pk_portfolio_uuid
    );
    debug!(
        "inside by_deposits_level query_item_params is {:?}",
        query_item_params
    );
    debug!(
        "deposit_sort_criteria in bank deposits level reader is: {:?}",
        deposit_sort_criteria
    );

    validate_uuid(pk_portfolio_uuid.as_str())?;

    let deposits = query_deposits(
        &db_handler.dynamodb_client,
        db_handler.table_name.as_str(),
        query_item_params,
        pk_portfolio_uuid,
        deposit_sort_criteria,
    )
    .instrument(Span::current())
    .await?;

    Ok(Json(deposits))
}

fn validate_uuid(uuid: &str) -> Result<(), RequestError> {
    if Uuid::try_parse(uuid).is_err() {
        return Err(RequestError::InvalidUuid(uuid.to_string()));
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), LambdaHttpError> {
    println!("before init tracing subscriber");
    init_default_subscriber();

    let span = info_span!("by_level_lambda_reader_main");
    span.in_scope(|| debug!("after init_default_subscriber"));

    // add AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH: for cargo lambda invoke also
    // If you use API Gateway stages, the Rust Runtime will include the stage name
    // as part of the path that your application receives.
    // Setting the following environment variable, you can remove the stage from the path.
    // This variable only applies to API Gateway stages,
    // you can remove it if you don't use them.
    // i.e with: `GET /test-stage/todo/id/123` without: `GET /todo/id/123`
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");

    let db_handler = Arc::new(
        DriveDepositsDb::handler()
            .await
            .inspect_err(|err| error!("DriveDepositsDb::handler error is {}", err))?,
    );

    let mut app_router: Router<()> = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route(
            "/by-level-for-portfolios/delta-growth",
            get(by_level_query_portfolios_delta_growth),
        )
        .route(
            "/portfolios/:pk_portfolio_uuid/by-level-for-banks/delta-growth",
            get(by_level_query_banks_delta_growth),
        )
        .route(
            "/portfolios/:pk_portfolio_uuid/by-level-for-deposits/delta-growth",
            get(by_level_query_deposits_delta_growth),
        )
        .route(
            "/portfolios/:pk_portfolio_uuid/by-level-for-deposits/maturity-date",
            get(by_level_query_deposits_maturity_date),
        )
        .with_state(db_handler);

    run(service_fn(|event: Request| {
        info!("api gateway event: {:?}", event);
        app_router.call(event)
    }))
    .await?;

    Ok(())
}
