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
use tracing::debug;

#[derive(Error, Debug)]
pub enum AddItemError {
    #[error("AddItemError with LevelSpecificItemWriterError: {0}")]
    LevelSpecificItemWriterError(#[from] LevelSpecificItemWriterError),

    #[error("AddItemError with DynamoDbSdkPutItemError error: {0}")]
    DynamoDbSdkPutItemError(#[from] SdkError<PutItemError>),
}

pub async fn add_item(
    client: &Client,
    table: &str,
    rest: CalculatePortfolioResponse,
) -> Result<(), AddItemError> {
    let portfolio_level_item = PortfolioLevelItem::try_from(&rest)?;
    add_portfolio_level_item(client, table, portfolio_level_item).await?;

    let banks_level_items_wrapper = BankLevelItemsWrapper::try_from(&rest)?;
    add_bank_level_items(client, table, banks_level_items_wrapper).await?;

    let rest_wrapper_growth_criteria = CalculatePortfolioRestWrapper {
        calculate_portfolio_response: rest.clone(),
        deposit_sort_criteria: DepositSortCriteria::DeltaPeriodGrowth,
    };
    let deposit_level_items_growth_criteria =
        DepositLevelItemsWrapper::try_from(&rest_wrapper_growth_criteria)?;
    add_deposit_level_items(client, table, deposit_level_items_growth_criteria).await?;

    let rest_wrapper_date_criteria = CalculatePortfolioRestWrapper {
        calculate_portfolio_response: rest,
        deposit_sort_criteria: DepositSortCriteria::MaturityDate,
    };
    let deposit_level_items_date_criteria =
        DepositLevelItemsWrapper::try_from(&rest_wrapper_date_criteria)?;
    add_deposit_level_items(client, table, deposit_level_items_date_criteria).await?;

    Ok(())
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
