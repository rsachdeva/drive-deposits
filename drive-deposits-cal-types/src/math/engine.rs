use std::sync::Arc;

use chrono::SecondsFormat;
use serde_json::to_string;
use thiserror::Error;
use tokio::task::{spawn_blocking, JoinSet};
use tracing::{debug, debug_span, info, instrument, Instrument, Span};
use uuid::Uuid;

use drive_deposits_event_source::{
    eb::{
        send_bank_level_event_to_event_bridge, send_portfolio_level_event_to_event_bridge,
        DriveDepositsEventBridge, DriveDepositsEventBridgeError,
    },
    payload_types::{
        Bank as EventSourceBank,
        CalculatePortfolioResponse as EventSourceCalculatePortfolioResponse,
    },
};

use crate::cal_types::{
    Bank, Deposit, NewBank, NewDelta, NewDeposit, PortfolioRequest, PortfolioResponse,
};
use crate::math::outcome::{
    build_outcome_from_banks, build_outcome_from_deposits, build_outcome_from_new_deposit,
    build_outcome_with_dates_from_new_deposit,
};

#[derive(Default, Debug, Error)]
pub enum CalculationHaltError {
    #[default]
    #[error("Internal error All Calculations could not proceed")]
    Internal,

    #[error("Join error all calculations could not proceed: {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error("Drive Deposits SEND_CAL_EVENTS is true but could not send events for processing as desired: {0}")]
    DriveDepositsEventBridgeError(#[from] DriveDepositsEventBridgeError),

    #[error("Drive Deposits SEND_CAL_EVENTS is true but could not serialize events for sending as desired: {0}")]
    EventSourceJsonSerializationError(#[from] serde_json::Error),
}

// same from style not directly using though since more complex with async function

fn build_from_new_deposit(
    new_deposit: NewDeposit,
    new_delta: Arc<NewDelta>,
) -> Result<Deposit, CalculationHaltError> {
    let outcome_with_dates = build_outcome_with_dates_from_new_deposit(&new_deposit);
    let outcome = build_outcome_from_new_deposit(&new_deposit, new_delta.as_ref());
    let deposit = Deposit {
        uuid: Uuid::new_v4(),
        account: new_deposit.account,
        account_type: new_deposit.account_type,
        apy: new_deposit.apy,
        years: new_deposit.years,
        outcome_with_dates,
        outcome,
    };
    Ok(deposit)
}

fn build_from_new_deposits(
    new_deposits: Vec<NewDeposit>,
    new_delta: Arc<NewDelta>,
) -> Result<Vec<Deposit>, CalculationHaltError> {
    let mut deposits = vec![];
    for new_deposit in new_deposits {
        let deposit = build_from_new_deposit(new_deposit, new_delta.clone())?;
        debug!(
            "build_from_new_deposits calling build_from_new_deposit Deposit: {:?}",
            deposit
        );
        deposits.push(deposit);
    }
    Ok(deposits)
}
async fn build_from_new_bank(
    new_bank: NewBank,
    new_delta: Arc<NewDelta>,
    eb: Arc<Option<DriveDepositsEventBridge>>,
) -> Result<Bank, CalculationHaltError> {
    // using spawn blocking for synchronous calculation code
    let bank_with_outcome = spawn_blocking(move || -> Result<Bank, CalculationHaltError> {
        info!(
            "task spawned for actual calculation for new_bank: {:?}",
            new_bank.name
        );
        let deposits = build_from_new_deposits(new_bank.new_deposits, new_delta.clone())?;
        let outcome = build_outcome_from_deposits(&deposits, new_delta.clone().as_ref());
        let bank = Bank {
            uuid: Uuid::new_v4(),
            name: new_bank.name,
            bank_tz: new_bank.bank_tz,
            deposits,
            outcome,
        };
        Ok(bank)
    })
    .await??;

    // this method is executed in joinset spawn task already
    if eb.is_some() {
        debug!("send event to event bridge at the bank level");
        let event_source_bank: EventSourceBank = bank_with_outcome.clone().into();
        let event_source_bank_json = to_string(&event_source_bank)?;
        let eb_access = eb.as_ref().as_ref();

        send_bank_level_event_to_event_bridge(eb_access, event_source_bank_json).await?
    }

    Ok(bank_with_outcome)
}

// The code doesn't need Mutex here because it's using immutable shared state with Arc, and there's no mutable state that needs protection. Here's why:
//
// The new_delta is wrapped in Arc and only shared for reading across multiple tasks
// Each task gets its own clone of the Arc, but they're only reading the data
// The code creates new data structures (deposits, banks, etc.) rather than modifying existing ones
// The JoinSet pattern used creates independent tasks that don't share mutable state
// All mutations happen on locally owned data within each task's scope
// This is an example of Rust's ownership system working with Arc to safely share immutable data across concurrent tasks without needing additional synchronization primitives like Mutex.
// The design follows Rust's principle of "share memory by communicating" through message passing patterns, where each task works with its own data and communicates results back through the JoinSet.
// Arc is still necessary here despite the read-only nature of the data because:
//
// The data needs to be shared across different async tasks that run independently and may outlive the original scope
// The tasks are spawned onto different threads in the tokio runtime, particularly with spawn_blocking and JoinSet
// Rust's lifetime system alone cannot guarantee the data will live long enough across thread boundaries
// Arc provides the thread-safe reference counting needed for the data to remain valid across these concurrent task boundaries
// The code in build_from_new_banks and build_from_new_bank specifically requires Arc because it moves data into multiple spawned tasks that run concurrently on different threads. Simple references, even when cloned, wouldn't be sufficient here as Rust needs static lifetime guarantees for cross-thread data sharing.
async fn build_from_new_banks(
    new_banks: Vec<NewBank>,
    new_delta: Arc<NewDelta>,
    eb: Arc<Option<DriveDepositsEventBridge>>,
) -> Result<Vec<Bank>, CalculationHaltError> {
    let mut banks: Vec<Bank> = Vec::new();
    let mut join_set = JoinSet::new();

    for new_bank in new_banks {
        // Correctly create a new span with the bank name
        let bank_span = debug_span!(parent: &Span::current(), "bank_level_spawned_task_for_processing_all_deposits", bank_name = %new_bank.name);
        let delta_clone = new_delta.clone();
        let eb_clone = eb.clone();
        join_set.spawn(
            async move {
                info!("task spawned for new_bank: {:?}", new_bank.name);
                let bank = build_from_new_bank(new_bank, delta_clone, eb_clone);
                bank.await
            }
            .instrument(bank_span),
        );
    }

    while let Some(res) = join_set.join_next().await {
        let bank = res??;
        banks.push(bank);
    }

    Ok(banks)
}

async fn build_from_portfolio_request(
    portfolio_req: PortfolioRequest,
    eb: Arc<Option<DriveDepositsEventBridge>>,
) -> Result<PortfolioResponse, CalculationHaltError> {
    let uuid = Uuid::new_v4();
    info!("build_from_portfolio_request uuid created: {:?}", uuid);
    let created_at = chrono::Utc::now();
    let created_at_iso8061 = created_at.to_rfc3339_opts(SecondsFormat::Micros, true);
    let eb_clone = eb.clone();
    let new_delta = Arc::new(portfolio_req.new_delta);
    let banks = build_from_new_banks(portfolio_req.new_banks, new_delta.clone(), eb).await?;
    let outcome = build_outcome_from_banks(&banks, new_delta.clone().as_ref());

    let bank_response = PortfolioResponse {
        uuid,
        banks,
        outcome,
        created_at: created_at_iso8061,
    };

    if eb_clone.is_some() {
        debug!("send event to event bridge at the banks level");
        let event_source_response: EventSourceCalculatePortfolioResponse =
            bank_response.clone().into();
        let event_source_response_json = to_string(&event_source_response)?;

        let handle = tokio::spawn(async move {
            send_portfolio_level_event_to_event_bridge(
                eb_clone.as_ref().as_ref(),
                event_source_response_json,
            )
            .await
        });
        handle.await??;
    }

    Ok(bank_response)
}

#[instrument(skip(portfolio_req, eb))]
pub async fn calculate_portfolio(
    portfolio_req: PortfolioRequest,
    eb: Option<DriveDepositsEventBridge>,
) -> Result<PortfolioResponse, CalculationHaltError> {
    debug!(
        "Starting calculation by period per PortfolioRequest overall: {:?}",
        portfolio_req
    );
    info!(
        "Drive Deposits SEND_CAL_EVENTS is {}, so events will {}be sent",
        eb.is_some(),
        if eb.is_some() { "" } else { "not " }
    );

    let eb_access = Arc::new(eb);
    let bank_resp = build_from_portfolio_request(portfolio_req, eb_access).await?;

    Ok(bank_resp)
}
