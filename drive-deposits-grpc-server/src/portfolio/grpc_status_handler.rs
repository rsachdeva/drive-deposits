use tonic::{Code, Status};
use tonic_types::{BadRequest, Help, LocalizedMessage, StatusExt};

use drive_deposits_cal_types::math::engine::CalculationHaltError;
use drive_deposits_proto_grpc_types::generated::NewBank;

pub fn bad_request_errors(new_banks: &[NewBank]) -> Result<(), Status> {
    let mut bad_request = BadRequest::new(vec![]);
    if new_banks.is_empty() {
        bad_request.add_violation("new_banks", "name_banks cannot be empty");
    } else if new_banks.len() > 500 {
        bad_request.add_violation(
            "new_banks",
            "too many banks provided; must be less than upper limit of 500",
        );
    }

    if !bad_request.is_empty() {
        let help = Help::with_link("check your banks list", "https://drinnovations.us");
        let localized_message = LocalizedMessage::new("en-US", "overall validate your banks list");
        let status = Status::with_error_details_vec(
            Code::InvalidArgument,
            "request contains invalid arguments",
            vec![bad_request.into(), help.into(), localized_message.into()],
        );
        return Err(status);
    }
    Ok(())
}

pub struct CalculationHaltErrorWrapper(pub CalculationHaltError);
impl From<CalculationHaltErrorWrapper> for Status {
    fn from(wrapper: CalculationHaltErrorWrapper) -> Self {
        match wrapper.0 {
            CalculationHaltError::Internal => {
                Status::internal("All Calculations could not proceed")
            }
            CalculationHaltError::Join(e) => Status::internal(format!(
                "Join error all calculations could not proceed: {}",
                e
            )),
            CalculationHaltError::DriveDepositsEventBridgeError(e) => {
                Status::internal(format!(
                    "Drive Deposits SEND_CAL_EVENTS is true but could not send events for processing as desired: {}",
                    e
                ))
            }
            CalculationHaltError::EventSourceJsonSerializationError(e) => {
                Status::internal(format!(
                    "Drive Deposits SEND_CAL_EVENTS is true but could not serialize events for sending as desired: {}",
                    e
                ))
            }
        }
    }
}
