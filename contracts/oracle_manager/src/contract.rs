use crate::error::ContractError;
use steadifi::oracle_manager::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ADMIN, ORACLE, PRICE};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Decimal, Uint128, Binary};
use cw0::maybe_addr;
use steadifi::oracle_manager::{Oracle, OracleUnvalidated};
use terra_cosmwasm::TerraQuerier ;
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    ADMIN.set(deps.branch(), Some(info.sender))?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddAssetPriceOracle {
            asset_name,
            oracle_unvalidated,
        } => add_asset_price_oracle(deps, asset_name, oracle_unvalidated),
        ExecuteMsg::UpdateAdmin { new_admin } => execute_update_admin(deps, info, new_admin),
    }
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPrice {asset_name} => to_binary(&query_get_price(deps, asset_name)?),
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

fn add_asset_price_oracle(
    deps: DepsMut,
    asset_name: String,
    oracle_unvalidated: OracleUnvalidated,
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
    let new_oracle = oracle_unvalidated.to_validated(deps.api)?;
    oracle_list: Vec<Oracle> = ORACLE
        .may_load(deps.storage, asset_name.as_str())?
        .unwrap_or_default();
    oracle_list.push(new_oracle);
    response = Response::new()
        .add_attribute("Asset name", format!("{}", asset_name))
        .add_attribute("Oracle details", format!("{:?}", Oracle);
    Ok(response)
}

fn query_get_price(deps:Deps, asset_name: String) -> Result<Decimal, ContractError>
{
    if let Ok(oracle_list) = ORACLE.load(deps.storage,asset_name.as_str()){
        let  mut price_list = Vec::new() ;
        for oracle in &oracle_list{
            match oracle{
                Oracle::Fixed{price} => {
                    price_list.push(price.clone()) ;
                }
                Oracle::Native{denom} => {
                    let terra_querier = TerraQuerier::new(&deps.querier);

                    // NOTE: Exchange rate returns how much of the quote (second argument) is required to
                    // buy one unit of the base_denom (first argument).
                    // We want to know how much uusd we need to buy 1 of the target currency
                    let native_price_query = terra_querier
                        .query_exchange_rates(denom.to_owned(), vec!["uusd".to_string()])?
                        .exchange_rates
                        .pop();

                    if let Some(exchange_rate_item) = native_price_query{
                        price_list.push(exchange_rate_item.exchange_rate)
                    } else{
                        return Err(ContractError::NativePriceNotFound {}) ;
                    }
                }

                _=>{
                    // TODO handle other cases of oracles
                    price_list.push(Decimal::zero())
                }
            }
        }
        Ok(get_median(& mut price_list))
    }
    else{
        Err(ContractError::OracleNotFound(asset_name))
    }
}


fn get_median(vec: &mut Vec<Decimal>) -> Decimal {
    if vec.is_empty() {
        return Decimal::zero();
    }
    if vec.len() == 1{
        return vec[0] ;
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















