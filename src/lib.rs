use cosmwasm_std::{
  entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use sei_cosmwasm::{SeiQueryWrapper, SeiQuerier};

pub mod msg;
use msg::{ExecuteMsg, InstantiateMsg, QueryMsg, StaticCallResponse};

#[entry_point]
pub fn instantiate(
  _deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  _msg: InstantiateMsg,
) -> StdResult<Response> {
  Ok(Response::default())
}

#[entry_point]
pub fn execute(
  _deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  _msg: ExecuteMsg,
) -> StdResult<Response> {
  Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps<SeiQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
      QueryMsg::StaticCall { data, to } => to_json_binary(&query_static_call(deps, data, to)?),
  }
}

fn query_static_call(deps: Deps<SeiQueryWrapper>, data: String, to: String) -> StdResult<StaticCallResponse> {
  let querier = SeiQuerier::new(&deps.querier);
  let response = querier.static_call(String::new(), to, data)?;
  Ok(StaticCallResponse { 
      data: Binary(response.encoded_data.into_bytes())
  })
}