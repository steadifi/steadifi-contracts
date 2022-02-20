use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint128, Addr};
use cw_storage_plus::{Item};


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MintAuthorityInfo {
    pub name: String, //Name of CW20 asset that this contract is the mint authority of
    pub symbol: String, //Symbol of CW20 asset that this contract is the mint authority of
    pub decimals: u8, //Number of decimals in the CW20 contract
    pub address_cw20: Addr, //Address of CW20 contract
    pub address_collateral_manager: Addr, //Address of the collateral manager contract that requests mints
    pub total_mint: Uint128, //Total number of tokens minted so far by this contract
}


pub const MINT_AUTHORITY_INFO: Item<MintAuthorityInfo> = Item::new("mint_authority_info");
