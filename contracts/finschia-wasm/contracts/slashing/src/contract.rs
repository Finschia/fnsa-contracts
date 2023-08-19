#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::cosmos::slashing::v1beta1::{MsgUnjail, SlashingQuerier};
use finschia_std::types::cosmos::slashing::v1beta1::{
    QueryParamsResponse, QuerySigningInfoResponse, QuerySigningInfosResponse,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-stargate-slashing";
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
        ExecuteMsg::Unjail { validator_addr } => try_unjail(deps, info, validator_addr),
    }
}

pub fn try_unjail(
    _deps: DepsMut,
    _info: MessageInfo,
    validator_addr: String,
) -> Result<Response, ContractError> {
    let msg_unjail: CosmosMsg = MsgUnjail { validator_addr }.into();

    Ok(Response::new()
        .add_attribute("method", "try_unjail")
        .add_message(msg_unjail))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::SigningInfo { cons_address } => {
            to_binary(&query_signing_info(deps, cons_address)?)
        }
        QueryMsg::SigningInfos { pagination } => to_binary(&query_signing_infos(deps, pagination)?),
        QueryMsg::Params {} => to_binary(&query_params(deps)?),
    }
}

fn query_signing_info(deps: Deps, cons_address: String) -> StdResult<QuerySigningInfoResponse> {
    let bq = SlashingQuerier::new(&deps.querier);
    let res = bq.signing_info(cons_address)?;
    Ok(QuerySigningInfoResponse {
        val_signing_info: res.val_signing_info,
    })
}

fn query_signing_infos(
    deps: Deps,
    pagination: Option<PageRequest>,
) -> StdResult<QuerySigningInfosResponse> {
    let bq = SlashingQuerier::new(&deps.querier);
    let res = bq.signing_infos(pagination)?;
    Ok(QuerySigningInfosResponse {
        info: res.info,
        pagination: res.pagination,
    })
}

fn query_params(deps: Deps) -> StdResult<QueryParamsResponse> {
    let bq = SlashingQuerier::new(&deps.querier);
    let res = bq.params()?;
    Ok(QueryParamsResponse { params: res.params })
}
