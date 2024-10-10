use std::env::var;

use aws_config::BehaviorVersion;
use aws_sdk_eventbridge::error::SdkError;
use aws_sdk_eventbridge::operation::list_rules::ListRulesError;
use aws_sdk_eventbridge::operation::put_events::{PutEventsError, PutEventsOutput};
use aws_sdk_eventbridge::types::builders::PutEventsRequestEntryBuilder;
use aws_sdk_eventbridge::Client;
use thiserror::Error;
use tracing::{debug, error, instrument};

const LOCALSTACK_ENDPOINT: &str = "http://localhost.localstack.cloud:4566/";

#[derive(Debug, Error)]
pub enum DriveDepositsEventBridgeError {
    #[error("Missing EventBridge")]
    MissingEventBridge,
    #[error("No rule found {0}")]
    NoRuleErrorForEventBridgeEventBus(String),
    #[error("Failed to send event to event bridge {0}")]
    PuEventsSdkError(#[from] SdkError<PutEventsError>),
    #[error("Failed to list event rules {0}")]
    ListRulesSdkError(#[from] SdkError<ListRulesError>),
    #[error("Failed Entry Count Error is {0}")]
    FailedEntryCountError(String),
}

#[derive(Debug, Clone)]
pub struct DriveDepositsEventBridge {
    pub(crate) eb_client: Client,
    pub(crate) bus_name: String,
}

impl DriveDepositsEventBridge {
    pub async fn new(bus_name: String) -> Self {
        // let config = aws_config::load_from_env().await;
        let mut config_loader = aws_config::defaults(BehaviorVersion::latest());
        if use_localstack() {
            config_loader = config_loader.endpoint_url(LOCALSTACK_ENDPOINT);
        };
        let config = config_loader.load().await;

        let client = aws_sdk_eventbridge::Client::new(&config);
        Self {
            eb_client: client,
            bus_name,
        }
    }
}

fn env_var_bool(name: &str) -> bool {
    let env_val = var(name).unwrap_or_else(|_| "false".to_string());
    env_val.eq_ignore_ascii_case("true")
}

#[instrument]
fn use_localstack() -> bool {
    let use_local = env_var_bool("USE_LOCALSTACK");
    debug!("use_localstack based on USE_LOCALSTACK: {}", use_local);
    use_local
}

#[instrument]
fn should_send_events() -> bool {
    env_var_bool("SEND_CAL_EVENTS")
}

#[instrument]
pub async fn create_eb() -> Result<Option<DriveDepositsEventBridge>, DriveDepositsEventBridgeError>
{
    let send_events = should_send_events();
    debug!("send_events based on SEND_CAL_EVENTS: {}", send_events);
    if !send_events {
        return Ok(None);
    }
    let eb = DriveDepositsEventBridge::new("DriveDepositsEventBus".to_string()).await;
    check_rules_exist_for_bus_name(&eb.eb_client, eb.bus_name.as_str())
        .await
        .inspect_err(|err| {
            error!("check rules exist for bus name err is {}", err);
        })?;
    Ok(Some(eb))
}

pub async fn check_rules_exist_for_bus_name(
    client: &Client,
    bus_name: &str,
) -> Result<(), DriveDepositsEventBridgeError> {
    let list_rules = client
        .list_rules()
        .event_bus_name(bus_name)
        .send()
        .await
        .inspect_err(|err| {
            error!("list_rules send err is {:?}", err);
        })?;
    // more checks to be sure
    let rules = list_rules.rules.unwrap_or_default();
    if rules.is_empty() {
        error!(
            "No rules found error in EventBridge for event bus {}",
            bus_name
        );
        return Err(
            DriveDepositsEventBridgeError::NoRuleErrorForEventBridgeEventBus(bus_name.to_string()),
        );
    }
    Ok(())
}

pub async fn send_bank_level_event_to_event_bridge(
    eb: Option<&DriveDepositsEventBridge>,
    json_payload: String,
) -> Result<(), DriveDepositsEventBridgeError> {
    let level = "bank-level";
    let put_events_output = send_event_to_event_bridge(eb, json_payload, level).await?;
    puts_event_failed_entry_count(put_events_output, level)?;

    Ok(())
}

pub async fn send_portfolio_level_event_to_event_bridge(
    eb: Option<&DriveDepositsEventBridge>,
    json_payload: String,
) -> Result<(), DriveDepositsEventBridgeError> {
    let level = "portfolio-level";
    let put_events_output = send_event_to_event_bridge(eb, json_payload, level).await?;
    puts_event_failed_entry_count(put_events_output, level)?;
    Ok(())
}

pub fn puts_event_failed_entry_count(
    put_events_output: PutEventsOutput,
    level: &str,
) -> Result<(), DriveDepositsEventBridgeError> {
    debug!(
        "At {} detail-type, PutEventsOutput is {:?}",
        level, put_events_output
    );
    if put_events_output.failed_entry_count > 0 {
        return Err(DriveDepositsEventBridgeError::FailedEntryCountError(
            format!(
                "{} for sending events to event bridge detail type {}",
                put_events_output.failed_entry_count.to_string(),
                level
            ),
        ));
    }
    Ok(())
}

async fn send_event_to_event_bridge(
    eb: Option<&DriveDepositsEventBridge>,
    json_payload: String,
    detail_type: &str,
) -> Result<PutEventsOutput, DriveDepositsEventBridgeError> {
    let bridge = eb.ok_or_else(|| DriveDepositsEventBridgeError::MissingEventBridge)?;
    let bus_name = &bridge.bus_name;
    let request = PutEventsRequestEntryBuilder::default()
        .set_source(Some(String::from("drive-deposits")))
        .set_detail_type(Some(detail_type.into()))
        .set_detail(Some(json_payload))
        .set_event_bus_name(Some(bus_name.into()))
        .build();
    debug!("request being sent to event bridge: {:?}", request);
    let aws_eb_client = &bridge.eb_client;
    let request = aws_eb_client.put_events().entries(request);

    let put_events_output = request.send().await.inspect_err(|err| {
        error!("put_events_output send err is {}", err);
    })?;
    debug!(
        "Received response from EventBridge: {:?}",
        put_events_output
    );
    // put events output is useful example :
    // 2024-07-24T23:39:57.059295Z  INFO drive_deposits_check_cmd{json_request_file_path="examples/data/portfolio_request_valid.json"}:process_input{file_path="examples/data/portfolio_request_valid.json"}:calculate_by_period: drive_deposits_event_source::eb: PuEventsOutput is JMD: PutEventsOutput { failed_entry_count: 1, entries: Some([PutEventsResultEntry { event_id: None, error_code: Some("MalformedDetail"), error_message: Some("Detail is malformed.") }]), _request_id: Some("15fa9ddd-fd7f-4640-ba96-531715b211e5") }
    Ok(put_events_output)
}
