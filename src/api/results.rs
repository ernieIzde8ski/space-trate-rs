/*
This file is dedicated to how the api actually returns data.
It is wrapped in an object with one or two of three fields:
"data", "data" and "meta", or "error".

"data" is the key for most replies. "meta" is supplemented
when it tries to return a list, and "error" occurs when
something goes wrong.

For what that "data" field might actually resemble, refer
to api::responses, and in turn api::schemas.
*/

use crate::api::schema;
use serde::{Deserialize, Serialize};

fn get_error_name(code: i32) -> &'static str {
    match code {
        // General error codes
        4000 => "cooldownConflictError",
        4001 => "waypointNoAccessError",
        // Account error codes
        4100 => "tokenEmptyError",
        4101 => "tokenMissingSubjectError",
        4102 => "tokenInvalidSubjectError",
        4103 => "missingTokenRequestError",
        4104 => "invalidTokenRequestError",
        4105 => "invalidTokenSubjectError",
        4106 => "accountNotExistsError",
        4107 => "agentNotExistsError",
        4108 => "accountHasNoAgentError",
        4109 => "registerAgentExistsError",
        // Ship error codes
        4200 => "navigateInTransitError",
        4201 => "navigateInvalidDestinationError",
        4202 => "navigateOutsideSystemError",
        4203 => "navigateInsufficientFuelError",
        4204 => "navigateSameDestinationError",
        4205 => "shipExtractInvalidWaypointError",
        4206 => "shipExtractPermissionError",
        4207 => "shipJumpNoSystemError",
        4208 => "shipJumpSameSystemError",
        4210 => "shipJumpMissingModuleError",
        4211 => "shipJumpNoValidWaypointError",
        4212 => "shipJumpMissingAntimatterError",
        4214 => "shipInTransitError",
        4215 => "shipMissingSensorArraysError",
        4216 => "purchaseShipCreditsError",
        4217 => "shipCargoExceedsLimitError",
        4218 => "shipCargoMissingError",
        4219 => "shipCargoUnitCountError",
        4220 => "shipSurveyVerificationError",
        4221 => "shipSurveyExpirationError",
        4222 => "shipSurveyWaypointTypeError",
        4223 => "shipSurveyOrbitError",
        4224 => "shipSurveyExhaustedError",
        4225 => "shipRefuelDockedError",
        4226 => "shipRefuelInvalidWaypointError",
        4227 => "shipMissingMountsError",
        4228 => "shipCargoFullError",
        4229 => "shipJumpFromGateToGateError",
        4230 => "waypointChartedError",
        4231 => "shipTransferShipNotFound",
        4232 => "shipTransferAgentConflict",
        4233 => "shipTransferSameShipConflict",
        4234 => "shipTransferLocationConflict",
        4235 => "warpInsideSystemError",
        4236 => "shipNotInOrbitError",
        4237 => "shipInvalidRefineryGoodError",
        4238 => "shipInvalidRefineryTypeError",
        4239 => "shipMissingRefineryError",
        4240 => "shipMissingSurveyorError",
        // Contract error codes
        4500 => "acceptContractNotAuthorizedError",
        4501 => "acceptContractConflictError",
        4502 => "fulfillContractDeliveryError",
        4503 => "contractDeadlineError",
        4504 => "contractFulfilledError",
        4505 => "contractNotAcceptedError",
        4506 => "contractNotAuthorizedError",
        4508 => "shipDeliverTermsError",
        4509 => "shipDeliverFulfilledError",
        4510 => "shipDeliverInvalidLocationError",

        // Market error codes
        4600 => "marketTradeInsufficientCreditsError",
        4601 => "marketTradeNoPurchaseError",
        4602 => "marketTradeNotSoldError",
        4603 => "marketNotFoundError",
        4604 => "marketTradeUnitLimitError",

        // Custom error codes
        6000 => "badReplyError",
        _ => "unknownError",
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    message: String,
    code: i32,
    data: Option<schema::Symbolic<Vec<String>>>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.data {
            Some(data) => {
                write!(
                    f,
                    "SpaceTraders API error (code={}): {}\nAdditional info: {}",
                    get_error_name(self.code),
                    self.message,
                    data.symbol.join(" ")
                )
            }
            None => {
                write!(
                    f,
                    "SpaceTraders API error (code={}): {}",
                    get_error_name(self.code),
                    self.message
                )
            }
        }
    }
}
impl std::error::Error for ApiError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResult<T> {
    data: Option<T>,
    meta: Option<schema::Meta>,
    error: Option<ApiError>,
}

impl<T> ApiResult<T> {
    pub fn to_result(self) -> Result<T, Box<dyn std::error::Error>> {
        if let Some(error) = self.error {
            Err(Box::new(error))
        } else if let Some(data) = self.data {
            Ok(data)
        } else {
            Err(Box::new(ApiError {
                message: "Server did not return expected fields 'data' or 'error'".to_owned(),
                code: 6000,
                data: None,
            }))
        }
    }
}
