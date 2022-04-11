#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw0::maybe_addr;
use terra_cosmwasm::TerraQuerier; //TODO: What the hell is this

use crate::error::ContractError;
use crate::state::{ADMIN, ORACLE};
use steadifi::mars_protocol_math::Decimal;
use steadifi::oracle_manager::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use steadifi::oracle_manager::{Oracle, OracleUnvalidated};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    ADMIN.set(deps, Some(info.sender))?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddAssetPriceOracle {
            asset_name,
            oracle_unvalidated,
        } => execute_add_asset_price_oracle(deps, info, asset_name, oracle_unvalidated),
        ExecuteMsg::UpdateAdmin { new_admin } => execute_update_admin(deps, info, new_admin),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPrice { asset_name } => to_binary(&query_get_price(deps, asset_name)?),
    }
}

fn execute_update_admin(
    deps: DepsMut,
    info: MessageInfo,
    new_admin: String,
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
    let new_admin = maybe_addr(deps.api, Some(new_admin))?;
    Ok(ADMIN.execute_update_admin(deps, info, new_admin)?)
}

fn execute_add_asset_price_oracle(
    deps: DepsMut,
    info: MessageInfo,
    asset_name: String,
    oracle_unvalidated: OracleUnvalidated,
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
    let new_oracle = oracle_unvalidated.to_validated(deps.api)?;
    let mut oracle_list: Vec<Oracle> = ORACLE
        .may_load(deps.storage, asset_name.as_str())?
        .unwrap_or_default();
    oracle_list.push(new_oracle.clone());
    let response = Response::new()
        .add_attribute("Asset name", format!("{}", asset_name))
        .add_attribute("Oracle details", format!("{:?}", new_oracle));
    Ok(response)
}

fn query_get_price(deps: Deps, asset_name: String) -> StdResult<Decimal> {
    if let Ok(oracle_list) = ORACLE.load(deps.storage, asset_name.as_str()) {
        let mut price_list = Vec::new();
        for oracle in &oracle_list {
            match oracle {
                Oracle::Fixed { price } => {
                    price_list.push(price.clone());
                }
                Oracle::Native { denom } => {
                    let terra_querier = TerraQuerier::new(&deps.querier);

                    // NOTE: Exchange rate returns how much of the quote (second argument) is required to
                    // buy one unit of the base_denom (first argument).
                    // We want to know how much uusd we need to buy 1 of the target currency
                    let native_price_query = terra_querier
                        .query_exchange_rates(denom.to_owned(), vec!["uusd".to_string()])?
                        .exchange_rates
                        .pop();
                    //TODO: Fix this and dont use this third party library
                    if let Some(exchange_rate_item) = native_price_query {
                        price_list.push(exchange_rate_item.exchange_rate.into())
                    } else {
                        return Err(StdError::NotFound {
                            kind: String::from("Native price was not found"),
                        });
                    }
                }

                _ => {
                    // TODO handle other cases of oracles
                    price_list.push(Decimal::zero())
                }
            }
        }
        Ok(get_median(&mut price_list))
    } else {
        Err(StdError::NotFound {
            kind: String::from(format!("Oracle list is empty for asset {}", asset_name)),
        })
    }
}

fn get_median(vec: &mut Vec<Decimal>) -> Decimal {
    if vec.is_empty() {
        return Decimal::zero();
    }
    if vec.len() == 1 {
        return vec[0];
    }
    vec.sort();
    let index = vec.len() / 2;
    vec[index]
    //TODO: Handle even cases properly
    //if vec.len() % 2 == 1 {
    //    vec[index]
    //} else {
    //   Decimal::from_ratio(Uint128::one(),Uint128::one()).mul(vec[index - 1] + vec[index]).into()
    //}
}
