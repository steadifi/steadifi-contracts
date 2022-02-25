use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Map};




//Maps string containing a collateral type to a contract address if it is a suporrted cw20 token
//and to the empty string if it is a supported native token
pub const WHITELIST_COLLATERAL: MAP<&str, TokenInfo> = Map::new("whitelist_collateral");

//Maps string containing a future asset to a cw20 address if it is valid
pub const WHITELIST_FUTUREASSET: MAP<&str, TokenInfo> = Map::new("whitelist_futureasset");


//Maps a user address and string containing future asset type to the amount they have borrowed
pub const BORROW: Map<(&Addr,&str), Uint128> = Map::new("borrow") ;
//Maps a user address and string containing collateral type to amount ;
pub const COLLATERAL: Map<(&Addr,&str), Uint128> = Map::new("collateral") ;


