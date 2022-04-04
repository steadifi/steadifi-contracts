use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Api, Decimal, StdResult, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssetInfo {
    FutureAsset {
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
                contract_addr,
                collateralizeable,
                ratio,
                underlying,
                decimals,
            } => Ok(AssetInfo::FutureAsset {
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
                contract_addr,
                ratio,
                collateralizeable,
                decimals,
            } => Ok(NormalAssetInfo::CW20Token {
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
            AssetInfo::FutureAsset { ratio, .. } => ratio.clone(),
            AssetInfo::NormalAsset(normal_asset_info) => match normal_asset_info {
                NormalAssetInfo::NativeToken { ratio, .. } => ratio.clone(),
                NormalAssetInfo::CW20Token { ratio, .. } => ratio.clone(),
            },
        }
    }
}
