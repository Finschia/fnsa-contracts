#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use finschia_std::shim::Any;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::cosmos::feegrant::v1beta1::{
    FeegrantQuerier, MsgGrantAllowance, MsgRevokeAllowance,
};
use finschia_std::types::cosmos::feegrant::v1beta1::{
    QueryAllowanceResponse, QueryAllowancesByGranterResponse, QueryAllowancesResponse,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-stargate-feegrant";
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
        ExecuteMsg::GrantAllowance {
            granter,
            grantee,
            allowance,
        } => try_grant_allowance(deps, info, granter, grantee, allowance),
        ExecuteMsg::RevokeAllowance { granter, grantee } => {
            try_revoke_allowance(deps, info, granter, grantee)
        }
    }
}

pub fn try_grant_allowance(
    _deps: DepsMut,
    _info: MessageInfo,
    granter: String,
    grantee: String,
    allowance: Option<Any>,
) -> Result<Response, ContractError> {
    let msg_grant_allowance: CosmosMsg = MsgGrantAllowance {
        granter,
        grantee,
        allowance,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_grant_allowance")
        .add_message(msg_grant_allowance))
}

pub fn try_revoke_allowance(
    _deps: DepsMut,
    _info: MessageInfo,
    granter: String,
    grantee: String,
) -> Result<Response, ContractError> {
    let msg_revoke_allowance: CosmosMsg = MsgRevokeAllowance { granter, grantee }.into();

    Ok(Response::new()
        .add_attribute("method", "try_revoke_allowance")
        .add_message(msg_revoke_allowance))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Allowance { granter, grantee } => {
            to_binary(&query_allowance(deps, granter, grantee)?)
        }
        QueryMsg::Allowances {
            grantee,
            pagination,
        } => to_binary(&query_allowances(deps, grantee, pagination)?),
        QueryMsg::AllowancesByGranter {
            granter,
            pagination,
        } => to_binary(&query_allowances_by_granter(deps, granter, pagination)?),
    }
}

fn query_allowance(
    deps: Deps,
    granter: String,
    grantee: String,
) -> StdResult<QueryAllowanceResponse> {
    let bq = FeegrantQuerier::new(&deps.querier);
    let res = bq.allowance(granter, grantee)?;
    Ok(QueryAllowanceResponse {
        allowance: res.allowance,
    })
}

fn query_allowances(
    deps: Deps,
    grantee: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryAllowancesResponse> {
    let bq = FeegrantQuerier::new(&deps.querier);
    let res = bq.allowances(grantee, pagination)?;
    Ok(QueryAllowancesResponse {
        allowances: res.allowances,
        pagination: res.pagination,
    })
}

fn query_allowances_by_granter(
    deps: Deps,
    granter: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryAllowancesByGranterResponse> {
    let bq = FeegrantQuerier::new(&deps.querier);
    let res = bq.allowances_by_granter(granter, pagination)?;
    Ok(QueryAllowancesByGranterResponse {
        allowances: res.allowances,
        pagination: res.pagination,
    })
}
