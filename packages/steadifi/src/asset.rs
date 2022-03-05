
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

use cosmwasm_std::{Addr, Decimal};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssetInfo {
    CW20Token {
            contract_addr: Addr,
            ratio: Decimal,
            oracle_addr: Addr,
    },
    NativeToken{
        denom: String,
        ratio: Decimal,
        oracle_addr: Addr,
    },
    FutureAssetToken{
        mint_addr: Addr,
        ratio: Decimal,
        pool_oracle_addr: Addr,
        underlying_oracle_addr: Addr
    }
}

impl AssetInfo {
    pub fn is_native_token(&self) -> bool {
        match self {
            AssetInfo::NativeToken { .. } => true,
            _ => false,
        }
    }
    pub fn is_future_token(&self) -> bool {
        match self {
            AssetInfo::FutureAssetToken { .. } => true,
            _ => false,
        }
    }
    pub fn is_cw20_token(&self) -> bool {
        match self {
            AssetInfo::CW20Token { .. } => true,
            _ => false,
        }
    }

    pub fn equal(&self, asset: &AssetInfo) -> bool {
        match self {
            AssetInfo::CW20Token { contract_addr, ratio, oracle_addr} => {
                let self_contract_addr = contract_addr;
                let self_ratio = ratio ;
                let self_oracle_addr = oracle_addr ;
                match asset {
                    AssetInfo::CW20Token { contract_addr, ratio, oracle_addr } =>
                        self_contract_addr == contract_addr && self_ratio == ratio && self_oracle_addr==oracle_addr ,
                    _ => false,
                }
            }
            AssetInfo::NativeToken { denom,  ratio, oracle_addr} => {
                let self_denom = denom;
                let self_ratio = ratio ;
                let self_oracle_addr = oracle_addr ;
                match asset {
                    AssetInfo::NativeToken { denom,  ratio, oracle_addr} =>
                        self_denom == denom  && self_ratio == ratio && self_oracle_addr==oracle_addr ,
                    _ => false
                }
            }
            AssetInfo::FutureAssetToken { mint_addr, ratio, pool_oracle_addr, underlying_oracle_addr} => {
                let self_mint_addr = mint_addr;
                let self_ratio = ratio ;
                let self_pool_oracle_addr = pool_oracle_addr ;
                let self_underlying_oracle_addr = underlying_oracle_addr ;
                match asset {
                    AssetInfo::FutureAssetToken {  mint_addr, ratio, pool_oracle_addr, underlying_oracle_addr} =>
                        self_mint_addr == mint_addr && self_ratio == ratio &&
                            self_pool_oracle_addr == pool_oracle_addr  &&  self_underlying_oracle_addr == underlying_oracle_addr,
                    _ => false,
                }
            }

        }
    }
}
