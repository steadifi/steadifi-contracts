use cosmwasm_std::{StdError, Uint128};
pub use cw_controllers::AdminError;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    ControllerError(#[from] AdminError),

    #[error("{0}")]
    UTF8Error(#[from] Utf8Error),

    #[error("Asset already supported")]
    AssetAlreadySupported {},

    #[error("Asset is not supported")]
    AssetNotSupported {},

    #[error("Asset is not supported as collateral")]
    AssetNotCollaterlizeable {},

    #[error("You have amount = 0 of this asset")]
    AssetIsZero {},

    #[error(
        "Current balance for native asset {:?} is {:?} which is less than withdraw amount {:?}",
        coin_denom,
        current_amount,
        withdraw_amount
    )]
    NotEnoughAsset {
        coin_denom: String,
        current_amount: Uint128,
        withdraw_amount: Uint128,
    },

    #[error("You do not have anough collateral to withdraw this asset")]
    NotEnoughTotalCollateral {},
}
