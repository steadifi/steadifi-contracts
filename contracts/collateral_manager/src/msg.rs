use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use steadifi::AssetInfoUnvalidated;

///////////////////////////////////////////////
// Instantiate messages
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {}

//////////////////////////////////////////////
// Execute messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    NativeDeposit {}, // Deposit native tokens as collateral
    NativeSettle {},  // Settle native borrow only available 7 days prior expiry of contract
    NativeWithdraw {
        coin_denom: String,
        amount: Uint128,
    }, // Withdraw Native tokens
    NativeLiquidate {}, // Liquidate account
    Receive(Cw20ReceiveMsg), //Exactly same operations but for cw20 tokens
    AddSupportedAsset {
        asset_name: String,
        asset_info_unvalidated: AssetInfoUnvalidated,
    },
    RemoveSupportedAsset {
        asset_name: String,
    },
    UpdateAdmin {
        new_admin: String,
    },
}

//////////////////////////////////////////////
//If sending a cw20 token to the collateral manager a message needs to come as well
// to specify what this cw20 token is for
// Thi shook message is in the Binary of the Cw20ReceiveMsg
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    /// Deposit more collateral
    Deposit { asset_name: String },
    /// Settle Loan - Only possible 7 days prior to expiry of contract
    Settle { asset_name: String },
    /// Liquidate Under-collaterlaized accounts or accounts that have not settled debt after expiry date
    Liquidate {},
    /// Withdraw CW20 Tokens
    Withdraw { asset_name: String, amount: Uint128 },
}

//////////////////////////////////////////////
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveNativeMsg {
    /// Deposit more collateral
    Deposit { asset_name: String },
    /// Settle Loan - Only possible 7 days prior to expiry of contract
    Settle { asset_name: String },
    /// Liquidate Under-collaterlaized accounts or accounts that have not settled debt after expiry date
    Liquidate {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns the current balance of the given address, and asset_name. Default is zero.
    /// Return Type: Balance response
    Balance { address: String, asset_name: String },
    /// Returns registered information of asset_name
    /// Return type: AssetInfoValidated
    AssetInfo { asset_name: String },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
/// amount of collateral and borrow. At most one of these two can be non-zero.
pub struct BalanceResponse {
    pub collateral: Uint128,
    pub borrow: Uint128,
}
