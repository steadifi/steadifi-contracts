use crate::mars_protocol_math::Decimal;
use cosmwasm_std::{Addr, Api, StdResult, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssetInfo {
    FutureAsset {
        asset_name: String,
        contract_addr: Addr,
        collateralizeable: bool,
        ratio: Decimal,
        underlying: NormalAssetInfoUnvalidated,
        decimals: Uint128,
    },
    NormalAsset(NormalAssetInfo),
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NormalAssetInfo {
    CW20Token {
        asset_name: String,
        contract_addr: Addr,
        collateralizeable: bool,
        ratio: Decimal,
        decimals: Uint128,
    },
    NativeToken {
        denom: String,
        collateralizeable: bool,
        ratio: Decimal,
        decimals: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssetInfoUnvalidated {
    FutureAsset {
        asset_name: String,
        contract_addr: String,
        collateralizeable: bool,
        ratio: Decimal,
        underlying: NormalAssetInfoUnvalidated,
        decimals: Uint128,
    },
    NormalAsset(NormalAssetInfoUnvalidated),
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NormalAssetInfoUnvalidated {
    CW20Token {
        asset_name: String,
        contract_addr: String,
        collateralizeable: bool,
        ratio: Decimal,
        decimals: Uint128,
    },
    NativeToken {
        denom: String,
        collateralizeable: bool,
        ratio: Decimal,
        decimals: Uint128,
    },
}

impl AssetInfoUnvalidated {
    pub fn to_validated(self, api: &dyn Api) -> StdResult<AssetInfo> {
        match self {
            AssetInfoUnvalidated::FutureAsset {
                asset_name,
                contract_addr,
                collateralizeable,
                ratio,
                underlying,
                decimals,
            } => Ok(AssetInfo::FutureAsset {
                asset_name,
                contract_addr: api.addr_validate(contract_addr.as_str())?,
                collateralizeable,
                ratio,
                underlying,
                decimals, //TODO: Add some validation here
            }),
            AssetInfoUnvalidated::NormalAsset(normal_asset_info) => {
                Ok(AssetInfo::NormalAsset(normal_asset_info.to_validated(api)?))
            }
        }
    }
}

impl NormalAssetInfoUnvalidated {
    pub fn to_validated(self, api: &dyn Api) -> StdResult<NormalAssetInfo> {
        match self {
            NormalAssetInfoUnvalidated::CW20Token {
                asset_name,
                contract_addr,
                ratio,
                collateralizeable,
                decimals,
            } => Ok(NormalAssetInfo::CW20Token {
                asset_name,
                contract_addr: api.addr_validate(contract_addr.as_str())?,
                ratio,
                collateralizeable,
                decimals, //TODO: add some validation here
            }),

            NormalAssetInfoUnvalidated::NativeToken {
                denom,
                ratio,
                collateralizeable,
                decimals,
            } => Ok(NormalAssetInfo::NativeToken {
                denom,
                ratio,
                collateralizeable,
                decimals, //TODO: add some validation here
            }),
        }
    }
}

impl AssetInfo {
    pub fn get_ratio(&self) -> Decimal {
        match self {
            AssetInfo::FutureAsset { ratio, .. } => *ratio,
            AssetInfo::NormalAsset(normal_asset_info) => match normal_asset_info {
                NormalAssetInfo::NativeToken { ratio, .. } => *ratio,
                NormalAssetInfo::CW20Token { ratio, .. } => *ratio,
            },
        }
    }
    pub fn get_name(&self) -> String {
        match self {
            AssetInfo::FutureAsset { asset_name, .. } => asset_name.clone(),
            AssetInfo::NormalAsset(normal_asset_info) => match normal_asset_info {
                NormalAssetInfo::NativeToken { denom, .. } => denom.clone(),
                NormalAssetInfo::CW20Token { asset_name, .. } => asset_name.clone(),
            },
        }
    }
    pub fn get_decimals(&self) -> Uint128 {
        match self {
            AssetInfo::FutureAsset { decimals, .. } => *decimals,
            AssetInfo::NormalAsset(normal_asset_info) => match normal_asset_info {
                NormalAssetInfo::NativeToken { decimals, .. } => *decimals,
                NormalAssetInfo::CW20Token { decimals, .. } => *decimals,
            },
        }
    }
}
