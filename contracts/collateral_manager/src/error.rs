use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Asset already supported")]
    AssetAlreadySupported {},

    #[error("Asset is not supported")]
    AssetNotSupported {},

    #[error("Asset is not supported as collateral")]
    AssetNotCollaterlizeable {},

}
