#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use finschia_std::types::cosmos::crisis::v1beta1::MsgVerifyInvariant;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-stargate-crisis";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::VerifyInvariant { sender, invariant_module_name, invariant_route } => try_verify_invariant(deps, info, sender, invariant_module_name, invariant_route),
    }
}

pub fn try_verify_invariant(
    _deps: DepsMut,
    _info: MessageInfo,
    sender: String,
    invariant_module_name: String,
    invariant_route: String,
) -> Result<Response, ContractError> {
    let msg_verify_invariant: CosmosMsg = MsgVerifyInvariant { sender, invariant_module_name, invariant_route }.into();

    Ok(Response::new()
        .add_attribute("method", "try_verify_invariant")
        .add_message(msg_verify_invariant))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    to_binary(&"")
}
