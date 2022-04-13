use std::str;

use cosmwasm_std::{Addr, DepsMut, Order, Pair, StdResult, Uint128};

use crate::error::ContractError;
use crate::state::{BORROW, COLLATERAL, CONFIG, SUPPORTED_ASSETS};
use steadifi::asset::AssetInfo;
use steadifi::mars_protocol_math::Decimal;
use steadifi::oracle_manager::get_oracle_price;

///Returns Ok(true) if address has enough collateral to withdraw amount value of the given asset
/// and Ok(false) otherwise
pub fn can_withdraw(
    deps: &DepsMut,
    withdrawer_address: &Addr,
    withdraw_asset: AssetInfo,
    withdraw_amount: Uint128,
) -> Result<bool, ContractError> {
    let oracle_manager_addr = CONFIG.load(deps.storage)?.oracle_manager_addr;

    // Compute total collateral denominated in USD
    let mut total_collateral_value = Decimal::zero();
    let all_collateral: StdResult<Vec<Pair<Uint128>>> = COLLATERAL
        .prefix(withdrawer_address)
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    for (asset_name_bytes, amount) in all_collateral? {
        let asset_name: &str = str::from_utf8(&asset_name_bytes)?;
        let asset_info = SUPPORTED_ASSETS.load(deps.storage, asset_name)?;
        let ratio = asset_info.get_ratio();
        let price = get_oracle_price(deps.querier, &oracle_manager_addr, asset_name)?;

        total_collateral_value = total_collateral_value
            + Decimal::from_ratio(amount, asset_info.get_decimals())
                .checked_mul(price)?
                .checked_mul(ratio)?;
    }

    // Compute total debt denominated in USD
    let mut total_borrow_value = Decimal::zero();
    let all_borrows: StdResult<Vec<Pair<Uint128>>> = BORROW
        .prefix(withdrawer_address)
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    for (asset_name_bytes, amount) in all_borrows? {
        let asset_name: &str = str::from_utf8(&asset_name_bytes)?;
        let asset_info = SUPPORTED_ASSETS.load(deps.storage, asset_name)?;
        let price = get_oracle_price(deps.querier, &oracle_manager_addr, asset_name)?;

        total_borrow_value = total_borrow_value
            + Decimal::from_ratio(amount, asset_info.get_decimals()).checked_mul(price)?;
    }

    //Add withdrawn amount to total debt
    let price_withdraw_asset = get_oracle_price(
        deps.querier,
        &oracle_manager_addr,
        &withdraw_asset.get_name(),
    )?;
    total_borrow_value = total_borrow_value
        + Decimal::from_ratio(withdraw_amount, withdraw_asset.get_decimals())
            .checked_mul(price_withdraw_asset)?;

    // Return true if collateral value is greater or equal to borrow value
    return Ok(total_collateral_value >= total_borrow_value);
}
