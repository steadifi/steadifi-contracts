use cosmwasm_std::StdError;
use cw_controllers::AdminError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    ControllerError(#[from] AdminError),

    #[error(display = "No oracle registered for asset {}"), _0]
    OracleNotFound(String),

    #[error("Oracle for Native asset returned an error")]
    NativePriceNotFound {}
}
