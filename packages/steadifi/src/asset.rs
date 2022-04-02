use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Api, Decimal, StdResult, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssetInfoValidated {
    FutureAsset {
        contract_addr: Addr,
        collateralizeable: bool,
        ratio: Decimal,
        underlying: NormalAssetInfo,
        decimals: Uint128,
    },
    NormalAsset(NormalAssetInfoValidated),
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NormalAssetInfoValidated {
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
pub enum AssetInfo {
    FutureAsset {
        contract_addr: String,
        collateralizeable: bool,
        ratio: Decimal,
        underlying: NormalAssetInfo,
        decimals: Uint128,
    },
    NormalAsset(NormalAssetInfo),
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NormalAssetInfo {
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

impl AssetInfo {
    pub fn to_validated(self, api: &dyn Api) -> StdResult<AssetInfoValidated> {
        match self {
            AssetInfo::FutureAsset {
                contract_addr,
                collateralizeable,
                ratio,
                underlying,
                decimals,
            } => Ok(AssetInfoValidated::FutureAsset {
                contract_addr: api.addr_validate(contract_addr.as_str())?,
                collateralizeable,
                ratio,
                underlying,
                decimals, //TODO: Add some validation here
            }),
            AssetInfo::NormalAsset(normal_asset_info) => Ok(AssetInfoValidated::NormalAsset(
                normal_asset_info.to_validated(api)?,
            )),
        }
    }
}

impl NormalAssetInfo {
    pub fn to_validated(self, api: &dyn Api) -> StdResult<NormalAssetInfoValidated> {
        match self {
            NormalAssetInfo::CW20Token {
                contract_addr,
                ratio,
                collateralizeable,
                decimals,
            } => Ok(NormalAssetInfoValidated::CW20Token {
                contract_addr: api.addr_validate(contract_addr.as_str())?,
                ratio,
                collateralizeable,
                decimals,
            }),

            NormalAssetInfo::NativeToken {
                denom,
                ratio,
                collateralizeable,
                decimals,
            } => Ok(NormalAssetInfoValidated::NativeToken {
                denom,
                ratio,
                collateralizeable,
                decimals,
            }),
        }
    }
}

impl AssetInfoValidated {
    pub fn get_ratio(&self) -> Decimal {
        match self {
            AssetInfoValidated::FutureAsset { ratio, .. } => ratio.clone(),
            AssetInfoValidated::NormalAsset(normal_asset_info) => match normal_asset_info {
                NormalAssetInfoValidated::NativeToken { ratio, .. } => ratio.clone(),
                NormalAssetInfoValidated::CW20Token { ratio, .. } => ratio.clone(),
            },
        }
    }
}
