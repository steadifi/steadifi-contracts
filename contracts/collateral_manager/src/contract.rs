use crate::error::ContractError;
use crate::helper::can_withdraw;
use crate::msg::{BalanceResponse, Cw20HookMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ADMIN, BORROW, COLLATERAL, SUPPORTED_ASSETS};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw0::maybe_addr;
use cw20::Cw20ReceiveMsg;
use steadifi::{AssetInfo, AssetInfoUnvalidated, NormalAssetInfo};

//TODO make CW2 compliant

////////////////////////////////////////////////////////////////////////////////////////////////////
//Instantiates
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    ADMIN.set(deps.branch(), Some(info.sender))?;
    Ok(Response::default())
}
////////////////////////////////////////////////////////////////////////////////////////////////////
//Executes
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // Handling of native tokens
        ExecuteMsg::NativeDeposit {} => execute_native_deposit(deps, info),
        ExecuteMsg::NativeSettle { .. } => Ok(Response::default()),
        ExecuteMsg::NativeWithdraw { coin_denom, amount } => {
            execute_native_withdraw(deps, info, coin_denom, amount)
        }
        ExecuteMsg::NativeLiquidate { .. } => Ok(Response::default()),

        // Handling of CW20 tokens
        ExecuteMsg::Receive(msg) => execute_receive_cw20(deps, env, info, msg),

        // Handling of supported assets
        ExecuteMsg::AddSupportedAsset {
            asset_name,
            asset_info_unvalidated,
        } => execute_add_supported_asset(deps, info, asset_name, asset_info_unvalidated),
        ExecuteMsg::RemoveSupportedAsset { asset_name } => {
            execute_remove_supported_asset(deps, info, asset_name)
        }

        ExecuteMsg::UpdateAdmin { new_admin } => execute_update_admin(deps, info, new_admin),
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

/// Native Deposits
fn execute_native_deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    for coin in info.funds.into_iter() {
        // Check to see if token is on whitelist
        let asset_info = SUPPORTED_ASSETS.may_load(deps.storage, &coin.denom)?;
        match asset_info {
            Some(..) => COLLATERAL.update(
                deps.storage,
                (&info.sender, &coin.denom),
                |balance: Option<Uint128>| -> StdResult<_> {
                    Ok(balance.unwrap_or_default().checked_add(coin.amount)?)
                },
            )?,
            None => {
                return Err(ContractError::AssetNotSupported {});
            }
        };
    }
    // TODO: A more informative response
    Ok(Response::default())
}

///Native Withdrawals
fn execute_native_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    coin_denom: String,
    withdraw_amount: Uint128,
) -> Result<Response, ContractError> {
    let address = info.sender;
    // Check if asset is supported
    if let Some(asset_info) = SUPPORTED_ASSETS.may_load(deps.storage, &coin_denom)? {
        let current_amount = COLLATERAL
            .may_load(deps.storage, (&address, &coin_denom))?
            .unwrap_or_default();
        // Current amount shouldn't be zero
        if current_amount.is_zero() {
            return Err(ContractError::AssetIsZero {});
        }
        // Withdraw amount should be less or equal to current amount
        if let Err(_) = current_amount.checked_sub(withdraw_amount) {
            return Err(ContractError::NotEnoughAsset {
                coin_denom,
                current_amount,
                withdraw_amount,
            });
        }
        // Check collateral requirements with oracle prices and corresponding rations
        if can_withdraw(&deps, &address, asset_info, withdraw_amount)? == false {
            return Err(ContractError::NotEnoughTotalCollateral {});
        }
    } else {
        //Asset not supported by contract
        return Err(ContractError::AssetNotSupported {});
    }

    let response = Response::new().add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: address.into(),
        amount: vec![Coin {
            denom: coin_denom,
            amount: withdraw_amount,
        }],
    }));
    Ok(response)
}

fn execute_receive_cw20(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Deposit { asset_name }) => {
            let cw20_sender = deps.api.addr_validate(cw20_msg.sender.as_str())?;
            execute_cw20_deposit(deps, cw20_sender, info.sender, cw20_msg.amount, asset_name)
        }
        Ok(Cw20HookMsg::Withdraw { .. }) => Ok(Response::new()),
        Ok(_) => Ok(Response::new()),
        Err(_) => Err(StdError::generic_err("invalid cw20 hook message").into()),
    }
}

/// CW20 Deposits
fn execute_cw20_deposit(
    deps: DepsMut,
    sender: Addr,
    cw20_contract_addr: Addr,
    amount: Uint128,
    asset_name: String,
) -> Result<Response, ContractError> {
    if let Some(asset_info) = SUPPORTED_ASSETS.may_load(deps.storage, &asset_name)? {
        match asset_info {
            AssetInfo::NormalAsset(normal_asset_info_validated) => {
                match normal_asset_info_validated {
                    NormalAssetInfo::CW20Token { contract_addr, .. } => {
                        if cw20_contract_addr != contract_addr {
                            return Err(StdError::generic_err
                                           (format!("Address on whitelist and sender contract address for cw20 asset {} do not match", asset_name)).into());
                        }
                        COLLATERAL.update(
                            deps.storage,
                            (&sender, &asset_name),
                            |balance: Option<Uint128>| -> StdResult<_> {
                                Ok(balance.unwrap_or_default().checked_add(amount)?)
                            },
                        )?;
                    }

                    NormalAssetInfo::NativeToken { .. } => {
                        return Err(StdError::generic_err(format!(
                            "{} corresponds to a native token",
                            asset_name
                        ))
                        .into());
                    }
                }
            }

            AssetInfo::FutureAsset {
                contract_addr,
                collateralizeable,
                ..
            } => {
                if cw20_contract_addr != contract_addr {
                    return Err(StdError::generic_err(format!("Address on whitelist and sender contract address for cw20 asset {} do not match", asset_name)).into());
                }
                if collateralizeable {
                    if let Some(borrow_amount) =
                        BORROW.may_load(deps.storage, (&sender, &asset_name))?
                    {
                        let excess = amount.checked_sub(borrow_amount);
                        match excess {
                            Ok(collateral_amount) => {
                                //Deposit is greater equal to the current borrow
                                BORROW.remove(deps.storage, (&sender, &asset_name));
                                COLLATERAL.update(
                                    deps.storage,
                                    (&sender, &asset_name),
                                    |balance: Option<Uint128>| -> StdResult<_> {
                                        Ok(balance
                                            .unwrap_or_default()
                                            .checked_add(collateral_amount)?)
                                    },
                                )?;
                            }
                            Err(_) => {
                                //Deposit is less than current borrow
                                BORROW.update(
                                    deps.storage,
                                    (&sender, &asset_name),
                                    |balance: Option<Uint128>| -> StdResult<_> {
                                        Ok(balance.unwrap_or_default().checked_sub(amount)?)
                                    },
                                )?;
                            }
                        }
                    } else {
                        //Not borrowed
                        COLLATERAL.update(
                            deps.storage,
                            (&sender, &asset_name),
                            |balance: Option<Uint128>| -> StdResult<_> {
                                Ok(balance.unwrap_or_default().checked_sub(amount)?)
                            },
                        )?;
                    }
                } else {
                    // Asset is not collateralizeable therefore:
                    // Only accept deposits if it is a borrowed asset and amount deposited is less
                    // or equal to the amount that is borrowed
                    if let Some(borrow_amount) =
                        BORROW.may_load(deps.storage, (&sender, &asset_name))?
                    {
                        if borrow_amount <= amount {
                            return Err(StdError::generic_err("After deposit balance becomes positive and asset can not be used as collateral").into()) ;
                        } else {
                            BORROW.update(
                                deps.storage,
                                (&sender, &asset_name),
                                |balance: Option<Uint128>| -> StdResult<_> {
                                    Ok(balance.unwrap_or_default().checked_sub(amount)?)
                                },
                            )?;
                        }
                    } else {
                        return Err(ContractError::AssetNotCollaterlizeable {});
                    }
                }
            }
        }
    } else {
        return Err(ContractError::AssetNotSupported {});
    }

    let res = Response::new()
        .add_attribute("action", "add  asset as collateral")
        .add_attribute("from", sender)
        .add_attribute("amount", amount)
        .add_attribute("asset_name", asset_name);

    Ok(res)
}

fn execute_add_supported_asset(
    deps: DepsMut,
    info: MessageInfo,
    asset_name: String,
    asset_info_unvalidated: AssetInfoUnvalidated,
) -> Result<Response, ContractError> {
    // Only contract admin can add new supported assets
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let asset_info = asset_info_unvalidated.to_validated(deps.api)?;
    let check_exists = SUPPORTED_ASSETS.may_load(deps.storage, &asset_name)?;
    match check_exists {
        Some(..) => {
            return Err(ContractError::AssetAlreadySupported {});
        }
        None => {
            SUPPORTED_ASSETS.save(deps.storage, &asset_name, &asset_info)?;
        }
    }

    Ok(Response::new()
        .add_attribute("action", "add_supported_asset")
        .add_attribute("asset_name", asset_name))
}

fn execute_remove_supported_asset(
    deps: DepsMut,
    info: MessageInfo,
    asset_name: String,
) -> Result<Response, ContractError> {
    // Only contract owner can remove supported assets
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    SUPPORTED_ASSETS.remove(deps.storage, &asset_name);
    Ok(Response::new()
        .add_attribute("action", "remove_supported_asset")
        .add_attribute("asset_name", asset_name))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//Queries
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance {
            address,
            asset_name,
        } => to_binary(&query_balance(deps, address, asset_name)?),
        QueryMsg::AssetInfo { asset_name } => to_binary(&query_asset_info(deps, asset_name)?),
    }
}

fn query_balance(deps: Deps, address: String, asset_name: String) -> StdResult<BalanceResponse> {
    let address = deps.api.addr_validate(&address)?;
    let balance_response = BalanceResponse {
        collateral: COLLATERAL
            .may_load(deps.storage, (&address, &asset_name))?
            .unwrap_or_default(),
        borrow: BORROW
            .may_load(deps.storage, (&address, &asset_name))?
            .unwrap_or_default(),
    };
    Ok(balance_response)
}

fn query_asset_info(deps: Deps, asset_name: String) -> StdResult<Option<AssetInfo>> {
    let asset_info_unvalidated = SUPPORTED_ASSETS.may_load(deps.storage, &asset_name)?;
    Ok(asset_info_unvalidated)
}

#[cfg(test)]
#[path = "./contract_unittests.rs"]
mod contract_unittests;
