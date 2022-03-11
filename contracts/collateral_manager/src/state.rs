use cosmwasm_std::{Addr, Int128};
use cw_storage_plus::{Map, Item};
use steadifi::{AssetInfo,AssetInfoValidated} ;
use cw_controllers::{Admin} ;


// Maps string containing name of asset to the AssetInfo struct which contains its information
pub const SUPPORTED_ASSETS: Map<String, AssetInfoValidated> = Map::new("whitelist_collateral");



// Maps a user address and string containing name of asset to how much the balance of that asset is
pub const COLLATERAL: Map<(&Addr, String), Uint128> = Map::new("collateral") ;
// Maps a user address and string containing name of asset to how much of that asset is borrowed
// Only future assets can be borrowed
pub const BORROW: Map<(&Addr, String), Uint128> = Map::new("borrow") ;

// Admin of contract can add or remove supported assets
// Eventually the admin will be the governance contract
pub const ADMIN: Admin = Admin::new("admin") ;