#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::PRICE;
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    PRICE.save(deps.storage, "BTC", &Decimal(Uint128(40000)))?;
    PRICE.save(deps.storage, "ETH", &Decimal(Uint128(3800)))?;
    PRICE.save(deps.storage, "LUNA", &Decimal(Uint128(107)))?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let res = match msg {
        ExecuteMsg::MintAndSend { recipient, amount } => {
            if amount == Uint128::zero() {
                return Err(ContractError::InvalidZeroAmount {});
            }
            let rcpt_addr = deps.api.addr_validate(&recipient)?;
            //Execute Mint
            let cw20_contract_address: Addr = get_cw20_address(&deps)?;
            let response = Response::new()
                .add_attribute("recipient", &recipient)
                .add_attribute("amount minted", amount.to_string())
                .add_message(
                    cw20::Cw20ExecuteMsg::Mint {
                        recipient: rcpt_addr.to_string(),
                        amount: amount,
                    }
                    .into_cosmos_msg(cw20_contract_address)?,
                );
            Ok(response)
        }
    };
    res
}

fn get_cw20_address(deps: &DepsMut) -> StdResult<Addr> {
    let info = MINT_AUTHORITY_INFO.load(deps.storage)?;
    Ok(info.address_cw20)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::MintAuthorityInfo {} => to_binary(&query_mint_authority_info(deps)?),
    }
}

fn query_mint_authority_info(deps: Deps) -> StdResult<MintAuthorityInfo> {
    let info = MINT_AUTHORITY_INFO.load(deps.storage)?;
    Ok(info)
}
