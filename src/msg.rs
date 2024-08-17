use serde::{Deserialize, Serialize};
use cosmwasm_std::Binary;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QueryMsg {
    StaticCall { data: String, to: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StaticCallResponse {
    pub data: Binary,
}