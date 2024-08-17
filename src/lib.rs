use cosmwasm_std::{
  entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use sei_cosmwasm::{SeiQueryWrapper, SeiQuerier, SeiMsg};

pub mod msg;
use msg::{ExecuteMsg, InstantiateMsg, QueryMsg, StaticCallResponse};

#[entry_point]
pub fn instantiate(
  _deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  _msg: InstantiateMsg,
) -> StdResult<Response<SeiMsg>> {
  Ok(Response::new())
}

#[entry_point]
pub fn execute(
  _deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  msg: ExecuteMsg,
) -> StdResult<Response<SeiMsg>> {
  match msg {
      ExecuteMsg::StaticCall { value, to, data } => execute_static_call(value, to, data),
  }
}

fn execute_static_call(value: cosmwasm_std::Uint128, to: String, data: String) -> StdResult<Response<SeiMsg>> {
  let call_evm = SeiMsg::CallEvm { value, to, data };
  Ok(Response::new().add_message(call_evm))
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