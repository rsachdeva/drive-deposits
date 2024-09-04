use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::query::builders::QueryFluentBuilder;
use aws_sdk_dynamodb::operation::query::QueryError;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use drive_deposits_lambda_db_types::convert::reader::with_level_context::LevelSpecificResponseReaderError;
use drive_deposits_lambda_db_types::{
    convert::reader::with_level_context::LevelSpecificItemReaderError,
    db_item_types::{BankLevelItem, PortfolioLevelItem},
    db_item_types::{DepositLevelItem, DepositSortCriteria},
    query_response_types::{
        BankData, BankResponse, ByLevelForBanks, ByLevelForPortfolios, ItemParamsRequest, Metadata,
        PortfolioData, PortfolioResponse, DEFAULT_TOP_K,
    },
    query_response_types::{ByLevelForDeposits, DepositData, DepositResponse},
};
use thiserror::Error;
use tracing::{debug, error, info};

#[derive(Error, Debug)]
pub enum QueryItemError {
    #[error("QueryItemError with DynamoDbSdkError error: {0}")]
    DynamoDbSdkError(#[from] SdkError<QueryError>),

    #[error("QueryItemError with QueryNoItemFound error: {0}")]
    QueryNoItemFound(String),

    #[error("QueryItemError with item reader error: {0}")]
    LevelSpecificItemReaderError(#[from] LevelSpecificItemReaderError),

    #[error("QueryItemError with response reader error: {0}")]
    LevelSpecificResponseReaderError(#[from] LevelSpecificResponseReaderError),
}

pub async fn query_portfolios(
    client: &Client,
    table: &str,
    query_item_params: ItemParamsRequest,
) -> Result<ByLevelForPortfolios, QueryItemError> {
    info!("in query_portfolios");
    info!("query_item_params is {:?}", query_item_params);
    let order = query_item_params.order.unwrap_or_default();
    info!("taking order for response: {:?}", order);
    let top_k = query_item_params.top_k.unwrap_or(DEFAULT_TOP_K);
    info!("taking top_k number k items for response: {:?}", top_k);

    let pk = "PK".to_string();
    let forward = bool::from(&order);
    let request = client
        .query()
        .table_name(table)
        .key_condition_expression("#partitionKeyName = :partitionKeyValue".to_string())
        .expression_attribute_names("#partitionKeyName".to_string(), pk)
        .expression_attribute_values(
            ":partitionKeyValue".to_string(),
            AttributeValue::S("PORTFOLIOS".to_string()),
        )
        .scan_index_forward(forward);
    let query_resp = request.send().await?;
    let sort_criteria_description =
        "Calculation results sorted by delta period growth for the overall calculation at the portfolios level"
            .to_string();
    let metadata = Metadata {
        order,
        top_k,
        sort_criteria_description,
    };
    if query_resp.count == 0 {
        return Ok(ByLevelForPortfolios {
            data: PortfolioData {
                responses: vec![],
                count_responses: 0,
            },
            metadata,
        });
    }

    let db_items = query_resp.items.ok_or_else(|| {
        QueryItemError::QueryNoItemFound("In query_portfolios_level_item".to_string())
    })?;
    debug!("db_items is {:?}", db_items);
    let portfolio_level_items = db_items
        .into_iter()
        .take(top_k)
        .map(PortfolioLevelItem::try_from)
        .collect::<Result<Vec<PortfolioLevelItem>, LevelSpecificItemReaderError>>()?;
    let portfolio_responses = portfolio_level_items
        .into_iter()
        .map(PortfolioResponse::try_from)
        .inspect(|responses_level| info!("Ordered responses_level: {:?}", responses_level))
        .collect::<Result<Vec<PortfolioResponse>, LevelSpecificResponseReaderError>>()?;
    let count_responses = portfolio_responses.len();
    let by_level_for_portfolios = ByLevelForPortfolios {
        data: PortfolioData {
            responses: portfolio_responses,
            count_responses,
        },
        metadata,
    };

    Ok(by_level_for_portfolios)
}

pub async fn query_banks(
    client: &Client,
    table: &str,
    query_item_params: ItemParamsRequest,
    pk_portfolio_uuid: String,
) -> Result<ByLevelForBanks, QueryItemError> {
    info!("in query_banks");
    info!("query_item_params is {:?}", query_item_params);
    let order = query_item_params.order.unwrap_or_default();
    info!("taking order for response: {:?}", order);
    let top_k = query_item_params.top_k.unwrap_or(DEFAULT_TOP_K);
    info!("taking top_k number k items for response: {:?}", top_k);

    let pk = "PK".to_string();
    let sk = "SK".to_string();
    let partition_key_value = format!("PORTFOLIO#UUID#{}", pk_portfolio_uuid);
    let sort_key_value = "BANK#PERIOD#".to_string();
    let forward = bool::from(&order);
    let query_params = QueryParams {
        table: table.to_string(),
        pk,
        sk,
        partition_key_value,
        sort_key_value,
        forward,
    };
    let request = build_begins_with_request_for_query(client, query_params);
    let query_resp = request.send().await?;
    let sort_criteria_description = format!(
        "Calculation results sorted by delta period growth at the banks level for portfolio {}",
        pk_portfolio_uuid
    );
    let metadata = Metadata {
        order,
        top_k,
        sort_criteria_description,
    };
    info!("query_resp.count is {:?}", query_resp.count);
    if query_resp.count == 0 {
        return Ok(ByLevelForBanks {
            data: BankData {
                responses: vec![],
                count_responses: 0,
            },
            metadata,
        });
    }

    let db_items = query_resp
        .items
        .ok_or_else(|| QueryItemError::QueryNoItemFound("In query_banks_level_item".to_string()))?;
    debug!("db_items is {:?}", db_items);
    let bank_level_item_iter = db_items.into_iter().take(top_k).map(|hashmap_av| {
        let bank_level_item = BankLevelItem::try_from(hashmap_av)?;
        Ok(bank_level_item)
    });
    let bank_level_items = bank_level_item_iter
        .collect::<Result<Vec<BankLevelItem>, LevelSpecificItemReaderError>>()?;
    info!("bank_level_items is {:?}", bank_level_items);
    let bank_response_iter = bank_level_items.into_iter().map(|item| {
        let bank_level_rest = BankResponse::try_from(item)
            .inspect_err(|err| error!("reader bank level rest error is {:?}", err))?;
        Ok(bank_level_rest)
    });
    let bank_responses = bank_response_iter
        .collect::<Result<Vec<BankResponse>, LevelSpecificResponseReaderError>>()?;
    info!("responses is {:?}", bank_responses);
    let count_responses = bank_responses.len();
    let by_level_for_banks = ByLevelForBanks {
        data: BankData {
            responses: bank_responses,
            count_responses,
        },
        metadata,
    };

    Ok(by_level_for_banks)
}

pub async fn query_deposits(
    client: &Client,
    table: &str,
    query_item_params: ItemParamsRequest,
    pk_portfolio_uuid: String,
    sort_criteria: DepositSortCriteria,
) -> Result<ByLevelForDeposits, QueryItemError> {
    info!("in query_deposits");
    info!("query_item_params is {:?}", query_item_params);
    let order = query_item_params.order.unwrap_or_default();
    info!("taking order for response: {:?}", order);
    let top_k = query_item_params.top_k.unwrap_or(DEFAULT_TOP_K);
    info!("taking top_k number k items for response: {:?}", top_k);

    let pk = "PK".to_string();
    let sk = "SK".to_string();
    let partition_key_value = format!("PORTFOLIO#UUID#{}", pk_portfolio_uuid);
    let (sort_key_value, sort_criteria_description) = match sort_criteria {
        DepositSortCriteria::DeltaPeriodGrowth => ("DEPOSIT#PERIOD#".to_string(), format!(
            "Calculation results sorted by delta period growth at the deposits level for portfolio {}",
            pk_portfolio_uuid
        )),
        DepositSortCriteria::MaturityDate => ("DEPOSIT#MATURITY_DATE#".to_string(), format!(
            "Calculation results sorted by maturity date at the deposits level for portfolio {}",
            pk_portfolio_uuid
        )),
    };
    let forward = bool::from(&order);
    let query_params = QueryParams {
        table: table.to_string(),
        pk,
        sk,
        partition_key_value,
        sort_key_value,
        forward,
    };
    let request = build_begins_with_request_for_query(client, query_params);
    let query_resp = request.send().await?;
    let metadata = Metadata {
        order,
        top_k,
        sort_criteria_description,
    };
    info!("query_resp.count is {:?}", query_resp.count);
    if query_resp.count == 0 {
        return Ok(ByLevelForDeposits {
            data: DepositData {
                responses: vec![],
                count_responses: 0,
            },
            metadata,
        });
    }

    let db_items = query_resp.items.ok_or_else(|| {
        QueryItemError::QueryNoItemFound("In query_bank_deposits_level_item".to_string())
    })?;
    debug!("db_items is {:?}", db_items);
    let deposit_level_item_iter = db_items.into_iter().take(top_k).map(|hashmap_av| {
        let bank_deposit_level_item = DepositLevelItem::try_from(hashmap_av)?;
        Ok(bank_deposit_level_item)
    });
    let deposit_level_items = deposit_level_item_iter
        .collect::<Result<Vec<DepositLevelItem>, LevelSpecificItemReaderError>>()?;
    // info!("bank_deposit_level_items is {:?}", bank_deposit_level_items);

    let deposit_response_iter = deposit_level_items.into_iter().map(|item| {
        let bank_deposit_level_rest = DepositResponse::try_from(item)
            .inspect_err(|err| error!("reader bank deposit level rest error is {:?}", err))?;
        Ok(bank_deposit_level_rest)
    });
    let deposit_responses = deposit_response_iter
        .collect::<Result<Vec<DepositResponse>, LevelSpecificResponseReaderError>>()?;
    let count_responses = deposit_responses.len();
    let by_bank_deposits_level_rest = ByLevelForDeposits {
        data: DepositData {
            responses: deposit_responses,
            count_responses,
        },
        metadata,
    };

    Ok(by_bank_deposits_level_rest)
}

struct QueryParams {
    table: String,
    pk: String,
    sk: String,
    partition_key_value: String,
    sort_key_value: String,
    forward: bool,
}

fn build_begins_with_request_for_query(client: &Client, params: QueryParams) -> QueryFluentBuilder {
    client
        .query()
        .table_name(params.table)
        .key_condition_expression(
            "#partitionKeyName = :partitionKeyValue AND begins_with(#sortKeyName, :sortKeyValue)"
                .to_string(),
        )
        .expression_attribute_names("#partitionKeyName".to_string(), params.pk)
        .expression_attribute_values(
            ":partitionKeyValue".to_string(),
            AttributeValue::S(params.partition_key_value),
        )
        .expression_attribute_names("#sortKeyName".to_string(), params.sk)
        .expression_attribute_values(
            ":sortKeyValue".to_string(),
            AttributeValue::S(params.sort_key_value),
        )
        .scan_index_forward(params.forward)
}
