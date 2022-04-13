use super::*;
use crate::error::ContractError::ControllerError;
use cosmwasm_std::testing::{
    mock_dependencies,
    mock_env,
    mock_info, //, MockApi, MockStorage, MOCK_CONTRACT_ADDR,
};
use cosmwasm_std::{Api, Decimal};
use cw_controllers::AdminError;
use steadifi::asset::{AssetInfoUnvalidated, NormalAssetInfoUnvalidated};
//use cosmwasm_std::{attr, coin, from_binary, BankMsg, OwnedDeps, SubMsg};

// Test initialization works
#[test]
fn test_initialization() {
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("Admin", &[]);
    let instantiate_msg = InstantiateMsg {};
    let contract_result = instantiate(deps.as_mut(), env, info, instantiate_msg);
    assert_eq!(contract_result, Ok(Response::default()));
    // Testing out the controllers
    assert!(ADMIN
        .is_admin(deps.as_ref(), &deps.api.addr_validate("Admin").unwrap())
        .unwrap());
    assert!(!ADMIN
        .is_admin(
            deps.as_ref(),
            &deps.api.addr_validate("someone_who_is_not_Admin").unwrap()
        )
        .unwrap());
}
#[test]
fn test_add_supported_assets() {
    // Initializations
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("Andmin", &[]);
    let instantiate_msg = InstantiateMsg {};
    // Instantiate contract
    instantiate(deps.as_mut(), env.clone(), info.clone(), instantiate_msg);
    // Add a native asset: "luna"
    let luna_asset_info =
        AssetInfoUnvalidated::NormalAsset(NormalAssetInfoUnvalidated::NativeToken {
            denom: "luna".to_string(),
            collateralizeable: true,
            ratio: Decimal::from_ratio(9u32, 10u32),
            oracle_addr: "luna_oracle_address".to_string(),
        });
    let execute_msg = ExecuteMsg::AddSupportedAsset {
        asset_name: "luna".to_string(),
        asset_info_unvalidated: luna_asset_info,
    };
    let contract_result = execute(deps.as_mut(), env.clone(), info.clone(), execute_msg);
    assert_eq!(
        contract_result,
        Ok(Response::new()
            .add_attribute("action", "add_supported_asset")
            .add_attribute("asset_name", "luna"))
    );
    // Add a cw20 asset
    let wbtc_asset_info =
        AssetInfoUnvalidated::NormalAsset(NormalAssetInfoUnvalidated::CW20Token {
            contract_addr: "wbtc_cw20_address".to_string(),
            collateralizeable: true,
            ratio: Decimal::from_ratio(8u32, 10u32),
            oracle_addr: "wbtc_oracle_address".to_string(),
        });
    let execute_msg = ExecuteMsg::AddSupportedAsset {
        asset_name: "wbtc".to_string(),
        asset_info_unvalidated: wbtc_asset_info,
    };
    let contract_result = execute(deps.as_mut(), env.clone(), info.clone(), execute_msg);
    assert_eq!(
        contract_result,
        Ok(Response::new()
            .add_attribute("action", "add_supported_asset")
            .add_attribute("asset_name", "wbtc"))
    );
    // Add a future asset
    let fbtc_asset_info = AssetInfoUnvalidated::FutureAsset {
        contract_addr: "fbtc_cw20_address".to_string(),
        collateralizeable: false,
        ratio: Default::default(),
        pool_oracle_addr: "address_to_fbtc/wbtc_pool".to_string(),
        underlying: NormalAssetInfoUnvalidated::CW20Token {
            contract_addr: "wbtc_cw20_address".to_string(),
            collateralizeable: true,
            ratio: Decimal::from_ratio(9u32, 10u32),
            oracle_addr: "wbtc_oracle_address".to_string(),
        },
    };
    let execute_msg = ExecuteMsg::AddSupportedAsset {
        asset_name: "fbtc".to_string(),
        asset_info_unvalidated: fbtc_asset_info,
    };
    let contract_result = execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        execute_msg.clone(),
    );
    assert_eq!(
        contract_result,
        Ok(Response::new()
            .add_attribute("action", "add_supported_asset")
            .add_attribute("asset_name", "fbtc"))
    );
    // Anyone other than admin can not add any assets
    let info = mock_info("someone_who_is_not_admin", &[]);
    let contract_result = execute(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        execute_msg.clone(),
    );
    assert_eq!(
        contract_result,
        Err(ContractError::ControllerError(AdminError::NotAdmin {}))
    );
}
#[test]
fn test_query_assetinfo() {
    // Initializations
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("Andmin", &[]);
    let instantiate_msg = InstantiateMsg {};
    // Instantiate contract
    instantiate(deps.as_mut(), env.clone(), info.clone(), instantiate_msg);
    // Add a native asset: "luna"
    let luna_asset_info =
        AssetInfoUnvalidated::NormalAsset(NormalAssetInfoUnvalidated::NativeToken {
            denom: "luna".to_string(),
            collateralizeable: true,
            ratio: Decimal::from_ratio(9u32, 10u32),
            oracle_addr: "luna_oracle_address".to_string(),
        });
    let execute_msg = ExecuteMsg::AddSupportedAsset {
        asset_name: "luna".to_string(),
        asset_info_unvalidated: luna_asset_info,
    };
    execute(deps.as_mut(), env.clone(), info.clone(), execute_msg);
}
