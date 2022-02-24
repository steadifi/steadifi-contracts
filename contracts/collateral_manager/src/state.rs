use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Map};






//Maps Address and string containing collateral type to amount
pub const COLLATERAL: Map<(&Addr,&str), Uint128> = Map::new("collaterals") ;
