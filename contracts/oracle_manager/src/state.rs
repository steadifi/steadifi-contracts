use cosmwasm_std::Addr;
use cw_controllers::Admin;
use cw_storage_plus::Map;
use steadifi::oracle_manager::Oracle;

// Admin of contract can add or remove supported assets
// Eventually the admin will be the governance contract
pub const ADMIN: Admin = Admin::new("admin");

// Maps asset name to list of Oracles
pub const ORACLE: Map<&str, Vec<Oracle>> = Map::new("oracle");
