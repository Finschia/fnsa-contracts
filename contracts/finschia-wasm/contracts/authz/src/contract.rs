#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use finschia_std::shim::Any;
use finschia_std::types::cosmos::authz::v1beta1::{
    AuthzQuerier, Grant, MsgExec, MsgGrant, MsgRevoke,
};
use finschia_std::types::cosmos::authz::v1beta1::{
    QueryGranteeGrantsResponse, QueryGranterGrantsResponse, QueryGrantsResponse,
};
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-stargate-authz";
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
        ExecuteMsg::Exec { grantee, msgs } => try_exec(deps, info, grantee, msgs),
        ExecuteMsg::Grant {
            granter,
            grantee,
            grant,
        } => try_grant(deps, info, granter, grantee, grant),
        ExecuteMsg::Revoke {
            granter,
            grantee,
            msg_type_url,
        } => try_revoke(deps, info, granter, grantee, msg_type_url),
    }
}

pub fn try_exec(
    _deps: DepsMut,
    _info: MessageInfo,
    grantee: String,
    msgs: Vec<Any>,
) -> Result<Response, ContractError> {
    let msg_exec: CosmosMsg = MsgExec { grantee, msgs }.into();

    Ok(Response::new()
        .add_attribute("method", "try_exec")
        .add_message(msg_exec))
}

pub fn try_grant(
    _deps: DepsMut,
    _info: MessageInfo,
    granter: String,
    grantee: String,
    grant: Option<Grant>,
) -> Result<Response, ContractError> {
    let msg_grant: CosmosMsg = MsgGrant {
        granter,
        grantee,
        grant,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_grant")
        .add_message(msg_grant))
}

pub fn try_revoke(
    _deps: DepsMut,
    _info: MessageInfo,
    granter: String,
    grantee: String,
    msg_type_url: String,
) -> Result<Response, ContractError> {
    let msg_revoke: CosmosMsg = MsgRevoke {
        granter,
        grantee,
        msg_type_url,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_revoke")
        .add_message(msg_revoke))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GranterGrants {
            granter,
            pagination,
        } => to_binary(&query_granter_grants(deps, granter, pagination)?),
        QueryMsg::GranteeGrants {
            grantee,
            pagination,
        } => to_binary(&query_grantee_grants(deps, grantee, pagination)?),
        QueryMsg::Grants {
            granter,
            grantee,
            msg_type_url,
            pagination,
        } => to_binary(&query_grants(
            deps,
            granter,
            grantee,
            msg_type_url,
            pagination,
        )?),
    }
}

fn query_granter_grants(
    deps: Deps,
    granter: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryGranterGrantsResponse> {
    let bq = AuthzQuerier::new(&deps.querier);
    let res = bq.granter_grants(granter, pagination)?;
    Ok(QueryGranterGrantsResponse {
        grants: res.grants,
        pagination: res.pagination,
    })
}

fn query_grantee_grants(
    deps: Deps,
    grantee: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryGranteeGrantsResponse> {
    let bq = AuthzQuerier::new(&deps.querier);
    let res = bq.grantee_grants(grantee, pagination)?;
    Ok(QueryGranteeGrantsResponse {
        grants: res.grants,
        pagination: res.pagination,
    })
}

fn query_grants(
    deps: Deps,
    granter: String,
    grantee: String,
    msg_type_url: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryGrantsResponse> {
    let bq = AuthzQuerier::new(&deps.querier);
    let res = bq.grants(granter, grantee, msg_type_url, pagination)?;
    Ok(QueryGrantsResponse {
        grants: res.grants,
        pagination: res.pagination,
    })
}
