use cosmwasm_std::{StdError, StdResult, Uint128, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use steadifi::{AssetInfo, AssetInfoValidated} ;
use cw20::{Cw20ReceiveMsg} ;
use cosmwasm_std{Addr} ;
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMarketingInfo {
    pub project: Option<String>,
    pub description: Option<String>,
    pub marketing: Option<String>,
    pub logo: Option<Logo>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
    pub mint: Option<MinterResponse>,
    pub marketing: Option<InstantiateMarketingInfo>,
}

impl InstantiateMsg {
    pub fn get_cap(&self) -> Option<Uint128> {
        self.mint.as_ref().and_then(|v| v.cap)
    }

    pub fn validate(&self) -> StdResult<()> {
        // Check name, symbol, decimals
        if !is_valid_name(&self.name) {
            return Err(StdError::generic_err(
                "Name is not in the expected format (3-50 UTF-8 bytes)",
            ));
        }
        if !is_valid_symbol(&self.symbol) {
            return Err(StdError::generic_err(
                "Ticker symbol is not in expected format [a-zA-Z\\-]{3,12}",
            ));
        }
        if self.decimals > 18 {
            return Err(StdError::generic_err("Decimals must not exceed 18"));
        }
        Ok(())
    }
}

fn is_valid_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.len() < 3 || bytes.len() > 50 {
        return false;
    }
    true
}

fn is_valid_symbol(symbol: &str) -> bool {
    let bytes = symbol.as_bytes();
    if bytes.len() < 3 || bytes.len() > 12 {
        return false;
    }
    for byte in bytes.iter() {
        if (*byte != 45) && (*byte < 65 || *byte > 90) && (*byte < 97 || *byte > 122) {
            return false;
        }
    }
    true
}
pub struct InstantiateMsg{



}






//////////////////////////////////////////////
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg{
    NativeDeposit{}, // Deposit native tokens as collateral
    NativeSettle{},  // Settle native borrow only available 7 days prior expiry of contract
    NativeWithdraw{} , // Withdraw Native tokens
    NativeLiquidate{}, // Liquidate account

    Receive(Cw20ReceiveMsg), //Exactly same operation for cw20 tokens

    AddSupportedAsset{
        asset_name: String,
        asset_info: AssetInfo,
    },
    RemoveSupportedAsset{
        asset_name: String
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
    Deposit { asset_name: String},
    /// Settle Loan - Only possible 7 days prior to expiry of contract
    Settle { asset_name: String },
    /// Liquidate Under-collaterlaized accounts or accounts that have not settled debt after expiry date
    Liquidate {liquidation_msg: LiquidationMsg},
    /// Withdraw CW20 Tokens
    Withdraw
}


//////////////////////////////////////////////
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum  ReceiveNativeMsg{
    /// Deposit more collateral
    Deposit { asset_name: String},
    /// Settle Loan - Only possible 7 days prior to expiry of contract
    Settle { asset_name: String },
    /// Liquidate Under-collaterlaized accounts or accounts that have not settled debt after expiry date
    Liquidate {liquidation_msg: LiquidationMsg},
}

























//////////////////////////////////////






#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns the current balance of the given address, 0 if unset.
    /// Return type: BalanceResponse.
    Balance { address: String },
    /// Returns metadata on the contract - name, decimals, supply, etc.
    /// Return type: TokenInfoResponse.
    TokenInfo {},
    /// Only with "mintable" extension.
    /// Returns who can mint and the hard cap on maximum tokens after minting.
    /// Return type: MinterResponse.
    Minter {},
    /// Only with "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    /// Return type: AllowanceResponse.
    Allowance { owner: String, spender: String },
    /// Only with "enumerable" extension (and "allowances")
    /// Returns all allowances this owner has approved. Supports pagination.
    /// Return type: AllAllowancesResponse.
    AllAllowances {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "enumerable" extension
    /// Returns all accounts that have balances. Supports pagination.
    /// Return type: AllAccountsResponse.
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "marketing" extension
    /// Returns more metadata on the contract to display in the client:
    /// - description, logo, project url, etc.
    /// Return type: MarketingInfoResponse
    MarketingInfo {},
    /// Only with "marketing" extension
    /// Downloads the mbeded logo data (if stored on chain). Errors if no logo data ftored for this
    /// contract.
    /// Return type: DownloadLogoResponse.
    DownloadLogo {},
}
