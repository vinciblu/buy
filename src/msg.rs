use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Uint128, Binary};

#[derive(Debug, Clone, PartialEq, JsonSchema, Serialize, Deserialize)]
pub struct BuyMsg {}

#[derive(Debug, Clone, PartialEq, JsonSchema, Serialize, Deserialize)]
pub struct InstantiateMsg {
    pub owner: String,
}

#[derive(Debug, Clone, PartialEq, JsonSchema, Serialize, Deserialize)]
pub struct WithdrawFundsMsg {
    pub amount: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    BuyAndStoreData {
        cart_price_usd: Uint128,
        encrypted_data: String,
    },
    WithdrawFunds {
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EncryptedData {
    pub address: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, QueryResponses)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    #[serde(rename_all = "snake_case")]
    #[schemars(with = "EncryptedData")]
    #[returns(EncryptedData)]
    GetEncryptedData {
        buyer_address: String,
        sender: String,
    },
}

