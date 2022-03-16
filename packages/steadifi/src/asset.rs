use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Api, Decimal, StdResult};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssetInfoValidated {
    FutureAsset {
        contract_addr: Addr,
        collateralizeable: bool,
        ratio: Decimal,
        pool_oracle_addr: Addr,
        underlying: NormalAssetInfo,
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
        oracle_addr: Addr,
    },
    NativeToken {
        denom: String,
        collateralizeable: bool,
        ratio: Decimal,
        oracle_addr: Addr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssetInfo {
    FutureAsset {
        contract_addr: String,
        collateralizeable: bool,
        ratio: Decimal,
        pool_oracle_addr: String,
        underlying: NormalAssetInfo,
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
        oracle_addr: String,
    },
    NativeToken {
        denom: String,
        collateralizeable: bool,
        ratio: Decimal,
        oracle_addr: String,
    },
}

impl AssetInfo {
    pub fn to_validated(self, api: &dyn Api) -> StdResult<AssetInfoValidated> {
        match self {
            AssetInfo::FutureAsset {
                contract_addr,
                collateralizeable,
                ratio,
                pool_oracle_addr,
                underlying,
            } => Ok(AssetInfoValidated::FutureAsset {
                contract_addr: api.addr_validate(contract_addr.as_str())?,
                collateralizeable,
                ratio,
                pool_oracle_addr: api.addr_validate(pool_oracle_addr.as_str())?,
                underlying,
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
                oracle_addr,
                collateralizeable,
            } => Ok(NormalAssetInfoValidated::CW20Token {
                contract_addr: api.addr_validate(contract_addr.as_str())?,
                ratio,
                oracle_addr: api.addr_validate(oracle_addr.as_str())?,
                collateralizeable,
            }),

            NormalAssetInfo::NativeToken {
                denom,
                ratio,
                oracle_addr,
                collateralizeable,
            } => Ok(NormalAssetInfoValidated::NativeToken {
                denom,
                ratio,
                oracle_addr: api.addr_validate(oracle_addr.as_str())?,
                collateralizeable,
            }),
        }
    }
}
