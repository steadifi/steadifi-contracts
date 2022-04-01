use cosmwasm_std::Decimal;
use cw_storage_plus::Map;

pub const PRICE: Map<&str, Decimal> = Map::new("price");
