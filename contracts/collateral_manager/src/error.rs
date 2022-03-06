use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Asset already on whitelist")]
    AlreadyOnWhitelist {},

    #[error("Asset is not on the whitelist")]
    NotWhitelisted {},





}
