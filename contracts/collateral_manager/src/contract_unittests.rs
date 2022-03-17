use super::*;
use cosmwasm_std::testing::{
    mock_dependencies,
    mock_env,
    mock_info, //, MockApi, MockStorage, MOCK_CONTRACT_ADDR,
};
use cosmwasm_std::{Api, Decimal};
use steadifi::AssetInfo::NormalAsset;
use steadifi::NormalAssetInfo;
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
            &deps.api.addr_validate("Admin's Enemy").unwrap()
        )
        .unwrap());
}
#[test]
fn test_add_supported_asset() {
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    // Instantiate contract
    let info = mock_info("Andisheh", &[]);
    let instantiate_msg = InstantiateMsg {};
    instantiate(deps.as_mut(), env.clone(), info, instantiate_msg);
    // Add a native asset: "luna"
    let info = mock_info("Andisheh", &[]);
    let asset_info = AssetInfo::NormalAsset(NormalAssetInfo::NativeToken {
        denom: "luna".to_string(),
        collateralizeable: true,
        ratio: Decimal::from_ratio(9u32, 10u32),
        oracle_addr: "".to_string(),
    });
    let execute_msg = ExecuteMsg::AddSupportedAsset {
        asset_name: "luna".to_string(),
        asset_info,
    };

    let contract_result = execute(deps.as_mut(), env.clone(), info, execute_msg);
    assert_eq!(
        contract_result,
        Ok(Response::new()
            .add_attribute("action", "add_supported_asset")
            .add_attribute("asset_name", "luna"))
    )
}
