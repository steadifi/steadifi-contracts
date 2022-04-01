use cosmwasm_std::Uint128;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub name: String,   //Name of CW20 asset that this contract is the mint authortiy of
    pub symbol: String, //Symbol of CW20 asset that this contract is the mint authority of
    pub decimals: u8,   //Number of decimals in the CW20 contract
    pub address_cw20: String, //Address of CW20 contract
    pub address_collateral_manager: String, //Address of the collateral manager contract that requests mints
    pub total_mint: Uint128,                //Total number of tokens minted so far
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdatePrice {
        asset_name: String, //Address where minted tokens are sent to
        new_price: Uint128, //Amount of tokens to mint and send
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetPrice { asset_name: String },
}
