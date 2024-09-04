pub mod add;

use std::env;
use std::env::var;

use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use lambda_runtime::tracing::{debug, error, info_span, instrument};
use thiserror::Error;

const LOCALSTACK_ENDPOINT: &str = "http://localhost.localstack.cloud:4566/";

#[derive(Default, Debug, Error)]
pub enum DbError {
    #[default]
    #[error("DynamoDb client error")]
    DynamoDbClientError,

    #[error("Env var error")]
    VarError(#[from] env::VarError),
}

pub struct DriveDepositsDb {
    pub dynamodb_client: Client,
    pub table_name: String,
}

impl DriveDepositsDb {
    fn new(table_name: String, dynamodb_client: Client) -> Self {
        Self {
            dynamodb_client,
            table_name,
        }
    }

    pub async fn handler() -> Result<Self, DbError> {
        let span = info_span!("banks_level_db_handler");
        let table_name = var("DRIVE_DEPOSITS_TABLE_NAME").inspect_err(|err| {
            span.in_scope(|| {
                error!(
                    "var error for DRIVE_DEPOSITS_TABLE_NAME {:?}, so returning",
                    err
                )
            })
        })?;
        span.in_scope(|| debug!("dynamodb drive deposits table_name: {:?}", table_name));
        let mut config_loader = aws_config::defaults(BehaviorVersion::latest());
        if use_localstack() {
            config_loader = config_loader.endpoint_url(LOCALSTACK_ENDPOINT);
        };
        let config = config_loader.load().await;
        let dynamodb_client = Client::new(&config);
        span.in_scope(|| debug!("dynamodb client got!"));
        Ok(Self::new(table_name, dynamodb_client))
    }
}

fn env_var_bool(name: &str) -> bool {
    let env_val = var(name).unwrap_or_else(|_| "false".to_string());
    env_val.eq_ignore_ascii_case("true")
}

#[instrument]
fn use_localstack() -> bool {
    let use_localstack = env_var_bool("USE_LOCALSTACK");
    debug!("use_localstack env variable is: {}", use_localstack);
    use_localstack
}
