use tonic::Status;
use tracing::{debug, error, info};

use drive_deposits_cal_types::cal_types::PortfolioRequest as CalBankRequest;
use drive_deposits_event_source::eb::DriveDepositsEventBridge;
use drive_deposits_proto_grpc_types::generated::{
    CalculatePortfolioRequest as GrpcCalculatePortfolioRequest,
    CalculatePortfolioResponse as GrpcCalculatePortfolioResponse,
};

use crate::portfolio::grpc_status_handler::CalculationHaltErrorWrapper;
use drive_deposits_cal_types::math::engine::calculate_portfolio;

use super::grpc_status_handler;

pub async fn by_period(
    delta_request: GrpcCalculatePortfolioRequest,
    eb: Option<DriveDepositsEventBridge>,
) -> Result<GrpcCalculatePortfolioResponse, Status> {
    info!("new_banks incoming is : {:?}", delta_request.new_banks);
    grpc_status_handler::bad_request_errors(&delta_request.new_banks)
        .inspect_err(|err| error!("new_banks checking at the grpc level errors : {:?}", err))?;

    // convert grpc CalculatePortfolioRequest to calculator CalculatePortfolioRequest
    let cal_req: CalBankRequest = delta_request.into();
    debug!("Converted from grpc to cal: {:?}", cal_req);

    // process calculation for calculator CalculatePortfolioRequest
    let cal_resp = calculate_portfolio(cal_req, eb)
        .await
        .map_err(CalculationHaltErrorWrapper)?;
    debug!("calculated response: {:?}", cal_resp);

    // convert response fom calculator CalculatePortfolioResponse to grpc CalculatePortfolioResponse
    let grpc_resp: GrpcCalculatePortfolioResponse = cal_resp.into();
    debug!("grpc response: {:?}", grpc_resp);

    Ok(grpc_resp)
}

// use drive_deposits_proto_grpc_types::generated::{
//     Bank, CalculatePortfolioRequest, CalculatePortfolioResponse, Delta, Deposit,
//     Maturity, NewBank, Outcome,
// };

// pub fn build_banks(new_banks: Vec<NewBank>) -> Vec<Bank> {
//     let banks = new_banks
//         .into_iter()
//         .map(|new_bank| -> Bank {
//             Bank {
//                 uuid: "100".to_string(),
//                 name: new_bank.name,
//                 deposits: new_bank
//                     .new_deposits
//                     .into_iter()
//                     .map(|new_deposit| -> Deposit {
//                         Deposit {
//                             uuid: "900".to_string(),
//                             account: new_deposit.account,
//                             account_type: new_deposit.account_type,
//                             apy: new_deposit.apy,
//                             years: new_deposit.years,
//                             outcome: Some(Outcome {
//                                 delta: Some(Delta {
//                                     period: "23.0".to_string(),
//                                     period_unit: 3,
//                                     growth: "23.0".to_string(),
//                                 }),
//                                 maturity: Some(Maturity {
//                                     amount: "23.0".to_string(),
//                                     interest: "23.0".to_string(),
//                                     total: "23.0".to_string(),
//                                 }),
//                                 errors: vec![],
//                             }),
//                             ..Default::default()
//                         }
//                     })
//                     .collect::<Vec<Deposit>>(),
//                 ..Default::default()
//             }
//         })
//         .collect::<Vec<Bank>>();
//     info!("banks is : {:?}", banks);
//     banks
// }
