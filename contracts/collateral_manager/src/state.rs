use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Map, Item};
use easylend::{AssetInfo} ;



// Maps string containing name of collateral type to the AssetInfo struct which contains its information
// Collateral can be any native token or a cw20 token (including future assets)
pub const WHITELIST_COLLATERAL: MAP<&str, AssetInfo> = Map::new("whitelist_collateral");

// Maps a string containing name of a future asset to its information
pub const WHITELIST_FUTUREASSET: MAP<&str, AssetInfo> = Map::new("whitelist_futureasset");


// Maps a user address and string containing name of collateral to how much of the collateral
// he/she has deposited
// One can assume assets on this list are all valid and on whitelist_collateral
pub const COLLATERAL: Map<(&Addr,&str), Uint128> = Map::new("collateral") ;

// Maps a user address and string containing future asset names to the amount they have borrowed
// One can assume assets on this list are all valid and on the whitelist_future_asset
pub const BORROW: Map<(&Addr,&str), Uint128> = Map::new("borrow") ;

// Owner of contract can add or remove assets on whitelists
pub const OWNER: Item<Addr> = Item::new("owner") ;