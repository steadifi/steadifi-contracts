use cosmwasm_std::StdError;
pub use cw_controllers::AdminError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    ControllerError(#[from] AdminError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Asset already supported")]
    AssetAlreadySupported {},

    #[error("Asset is not supported")]
    AssetNotSupported {},

    #[error("Asset is not supported as collateral")]
    AssetNotCollaterlizeable {},

    #[error("You have amount = 0 of this asset")]
    AssetIsZero {},
}
