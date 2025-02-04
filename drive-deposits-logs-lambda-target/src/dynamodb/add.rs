use aws_sdk_dynamodb::{error::SdkError, operation::put_item::PutItemError, Client};
use drive_deposits_lambda_db_types::{
    convert::writer::with_level_context::LevelSpecificItemWriterError,
    db_item_types::{
        BankLevelItemsWrapper, CalculatePortfolioRestWrapper, DepositLevelItemsWrapper,
        DepositSortCriteria, PortfolioLevelItem,
    },
};
use drive_deposits_rest_types::rest_types::CalculatePortfolioResponse;
use std::collections::HashMap;
use thiserror::Error;
use tokio::try_join;
use tracing::{debug, info, instrument};

#[derive(Error, Debug)]
pub enum AddItemError {
    #[error("AddItemError with LevelSpecificItemWriterError: {0}")]
    LevelSpecificItemWriterError(#[from] LevelSpecificItemWriterError),

    #[error("AddItemError with DynamoDbSdkPutItemError error: {0}")]
    DynamoDbSdkPutItemError(#[from] SdkError<PutItemError>),

    #[error("AddItemError with tokio::task::JoinError error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

#[instrument(skip(client, rest))]
pub async fn add_item(
    client: &Client,
    table: &str,
    rest: CalculatePortfolioResponse,
) -> Result<(), AddItemError> {
    info!("inside add_item");

    let portfolio_level_item = PortfolioLevelItem::try_from(&rest)?;
    let (client_clone, table_clone) = clone_db_connection_details(client, table);
    info!("spawn add_portfolio_level_item");
    let portfolio_level_item_handle = tokio::spawn(async move {
        add_portfolio_level_item(&client_clone, &table_clone, portfolio_level_item).await
    });

    info!("spawn add_bank_level_items");
    let banks_level_items_wrapper = BankLevelItemsWrapper::try_from(&rest)?;
    let (client_clone, table_clone) = clone_db_connection_details(client, table);
    let bank_level_items_handle = tokio::spawn(async move {
        add_bank_level_items(&client_clone, &table_clone, banks_level_items_wrapper).await
    });

    info!("spawn add_deposit_level_items for growth criteria");
    let rest_wrapper_growth_criteria = CalculatePortfolioRestWrapper {
        calculate_portfolio_response: rest.clone(),
        deposit_sort_criteria: DepositSortCriteria::DeltaPeriodGrowth,
    };
    let deposit_level_items_growth_criteria =
        DepositLevelItemsWrapper::try_from(&rest_wrapper_growth_criteria)?;
    let (client_clone, table_clone) = clone_db_connection_details(client, table);
    let deposit_level_items_growth_handle = tokio::spawn(async move {
        add_deposit_level_items(
            &client_clone,
            &table_clone,
            deposit_level_items_growth_criteria,
        )
        .await
    });

    info!("spawn add_deposit_level_items for date criteria");
    let rest_wrapper_date_criteria = CalculatePortfolioRestWrapper {
        calculate_portfolio_response: rest,
        deposit_sort_criteria: DepositSortCriteria::MaturityDate,
    };
    let deposit_level_items_date_criteria =
        DepositLevelItemsWrapper::try_from(&rest_wrapper_date_criteria)?;
    let (client_clone, table_clone) = clone_db_connection_details(client, table);
    let deposit_level_items_date_handle = tokio::spawn(async move {
        add_deposit_level_items(
            &client_clone,
            &table_clone,
            deposit_level_items_date_criteria,
        )
        .await
    });

    info!("try_join! for all the spawned tasks");
    let (portfolio_result, bank_level_result, deposit_growth_result, deposit_date_result) = try_join!(
        portfolio_level_item_handle,
        bank_level_items_handle,
        deposit_level_items_growth_handle,
        deposit_level_items_date_handle
    )?;

    portfolio_result?;
    bank_level_result?;
    deposit_growth_result?;
    deposit_date_result?;

    Ok(())
}

fn clone_db_connection_details(client: &Client, table: &str) -> (Client, String) {
    (client.clone(), table.to_string())
}

pub async fn add_portfolio_level_item(
    client: &Client,
    table: &str,
    item: PortfolioLevelItem,
) -> Result<(), AddItemError> {
    debug!("portfolio level item: {:#?}", item);
    let input_attributes = HashMap::from(item);

    let request = client
        .put_item()
        .table_name(table)
        .set_item(Some(input_attributes));

    let resp = request.send().await?;
    debug!(
        "dynamodb add_portfolio_level_item using HashMap from putItem with set_item response is response: {:#?}",
        resp
    );

    Ok(())
}

pub async fn add_bank_level_items(
    client: &Client,
    table: &str,
    wrapper: BankLevelItemsWrapper,
) -> Result<(), AddItemError> {
    for item in wrapper.items {
        let input_attributes = HashMap::from(item);
        let request = client
            .put_item()
            .table_name(table)
            .set_item(Some(input_attributes));

        let resp = request.send().await?;
        debug!(
            "add_banks_level_item using HashMap from PutItemOutput is: {:#?}",
            resp
        );
    }

    Ok(())
}

pub async fn add_deposit_level_items(
    client: &Client,
    table: &str,
    wrapper: DepositLevelItemsWrapper,
) -> Result<(), AddItemError> {
    debug!("banks deposits level wrapper: {:#?}", wrapper);
    for item in wrapper.deposit_level_items {
        let input_attributes = HashMap::from(item);

        let request = client
            .put_item()
            .table_name(table)
            .set_item(Some(input_attributes));

        let resp = request.send().await?;
        debug!(
            "add_bank_deposits_level_item using HashMap from PutItemOutput is: {:#?}",
            resp
        );
    }
    Ok(())
}
