use cosmwasm_std::{Binary, Uint128};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    StaticCall { value: Uint128, to: String, data: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(StaticCallResponse)]
    StaticCall { data: String, to: String },
}

#[cw_serde]
pub struct StaticCallResponse {
    pub data: Binary,
}