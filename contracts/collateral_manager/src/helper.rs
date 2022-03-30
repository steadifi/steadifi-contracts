use crate::error::ContractError;
use crate::state::{BORROW, COLLATERAL, SUPPORTED_ASSETS};
use cosmwasm_std::{Addr, ContractResult, Decimal, DepsMut, Order, Pair, StdResult, Uint128};
use std::str;
use steadifi::AssetInfoValidated;
pub fn can_withdraw(
    deps: &DepsMut,
    withdrawer_address: &Addr,
    withdraw_asset: AssetInfoValidated,
    withdraw_amount: Uint128,
) -> Result<bool, ContractError> {
    // Compute total collateral denominated in USD
    total_collateral_value = Decimal::zero();
    let all_collateral: Vec<Pair<Uint128>> = COLLATERAL
        .prefix(withdrawer_address)
        .range(&store, None, None, Order::Ascending)
        .collect()?;
    for (asset_name_bytes, amount) in all_collateral {
        let asset_info_validated = SUPPORTED_ASSETS.load(deps, &asset_name)?;
    }

    // Compute total debt
    let total_borrow_value = Decimal::zero();

    // Return true is collateral value is greater or equal to borrow value
    return Ok(total_collateral_valie > total_borrow_value);
}
pub fn get_orcale_price(oracle_addr: &Addr) -> Decimal {}
