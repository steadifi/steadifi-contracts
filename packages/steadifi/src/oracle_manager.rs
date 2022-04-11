use crate::mars_protocol_math::Decimal;
use crate::oracle_manager::msg::QueryMsg as OracleQueryMsg;
use cosmwasm_std::{Addr, Api, QuerierWrapper, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

///Get asset price denominated in USD from the oracle_manager
pub fn get_oracle_price(
    querier: QuerierWrapper,
    oracle_manager_address: &Addr,
    asset_name: &str,
) -> StdResult<Decimal> {
    // For UST, we skip the query and just return 1 to save gas
    if asset_name == "uusd" {
        Ok(Decimal::one())
    } else {
        // TODO: This is wrong need an execute message type here
        querier.query_wasm_smart(
            oracle_manager_address.as_str(),
            &OracleQueryMsg::GetPrice {
                asset_name: asset_name.to_string(),
            },
        )
    }
}

/// Oracle types used by oracle manager
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Oracle {
    AstroportTWAP { address: Addr },
    Native { denom: String },
    // Only used for integration tests
    Fixed { price: Decimal },
}

/// Oracles types with unvalidated addresses
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OracleUnvalidated {
    AstroportTWAP { address_unvalidated: String },
    Native { denom: String },
    Fixed { price: Decimal },
}

impl OracleUnvalidated {
    pub fn to_validated(self, api: &dyn Api) -> StdResult<Oracle> {
        match self {
            OracleUnvalidated::AstroportTWAP {
                address_unvalidated,
            } => Ok(Oracle::AstroportTWAP {
                address: api.addr_validate(address_unvalidated.as_str())?,
            }),
            OracleUnvalidated::Native { denom } => Ok(Oracle::Native { denom }),
            OracleUnvalidated::Fixed { price } => Ok(Oracle::Fixed { price }),
        }
    }
}

pub mod msg {
    use super::{Deserialize, JsonSchema, OracleUnvalidated, Serialize};

    #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
    #[serde(rename_all = "snake_case")]
    pub struct InstantiateMsg {}

    #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
    #[serde(rename_all = "snake_case")]
    pub enum ExecuteMsg {
        AddAssetPriceOracle {
            asset_name: String,
            oracle_unvalidated: OracleUnvalidated,
        },
        UpdateAdmin {
            new_admin: String,
        },
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum QueryMsg {
        GetPrice { asset_name: String },
    }
}
