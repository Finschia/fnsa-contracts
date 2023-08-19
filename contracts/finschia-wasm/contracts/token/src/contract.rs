use std::convert::TryFrom;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
    StdResult, SubMsg, SubMsgResponse, SubMsgResult,
};
use cw2::set_contract_version;

use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::lbm::token::v1::{
    Attribute, MsgAuthorizeOperator, MsgBurn, MsgGrantPermission, MsgIssue, MsgIssueResponse,
    MsgMint, MsgModify, MsgOperatorBurn, MsgOperatorSend, MsgRevokeOperator, MsgRevokePermission,
    MsgSend, TokenQuerier,
};
use finschia_std::types::lbm::token::v1::{
    QueryBalanceResponse, QueryBurntResponse, QueryContractResponse, QueryGranteeGrantsResponse,
    QueryHoldersByOperatorResponse, QueryIsOperatorForResponse, QueryMintedResponse,
    QuerySupplyResponse,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::CONTRACT_ID;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-stargate-token";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const ISSUE_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let msg_issue: CosmosMsg = MsgIssue {
        name: msg.name,
        symbol: msg.symbol,
        uri: msg.uri,
        meta: msg.meta,
        decimals: msg.decimals,
        mintable: msg.mintable,
        owner: msg.owner,
        to: msg.to,
        amount: msg.amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_submessage(SubMsg::reply_on_success(msg_issue, ISSUE_REPLY_ID)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        ISSUE_REPLY_ID => handle_issue_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

fn handle_issue_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result {
        let res = MsgIssueResponse::try_from(b)?;
        let _ = CONTRACT_ID.save(deps.storage, &res.contract_id);
        return Ok(Response::new().add_attribute("contract_id", res.contract_id));
    } else {
        return Err(StdError::generic_err("failed to get MsgIssueResponse"));
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { from, to, amount } => try_mint(deps, info, from, to, amount),
        ExecuteMsg::Burn { from, amount } => try_burn(deps, info, from, amount),
        ExecuteMsg::Send { from, to, amount } => try_send(deps, info, from, to, amount),
        ExecuteMsg::OperatorBurn {
            operator,
            from,
            amount,
        } => try_operator_burn(deps, info, operator, from, amount),
        ExecuteMsg::OperatorSend {
            operator,
            from,
            to,
            amount,
        } => try_operator_send(deps, info, operator, from, to, amount),
        ExecuteMsg::AuthorizeOperator { holder, operator } => {
            try_authorize_operator(deps, info, holder, operator)
        }
        ExecuteMsg::RevokeOperator { holder, operator } => {
            try_revoke_operator(deps, info, holder, operator)
        }
        ExecuteMsg::GrantPermission {
            from,
            to,
            permission,
        } => try_grant_permission(deps, info, from, to, permission),
        ExecuteMsg::RevokePermission { from, permission } => {
            try_revoke_permission(deps, info, from, permission)
        }
        ExecuteMsg::Modify { owner, changes } => try_modify(deps, info, owner, changes),
    }
}

pub fn try_mint(
    deps: DepsMut,
    _info: MessageInfo,
    from: String,
    to: String,
    amount: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_mint: CosmosMsg = MsgMint {
        contract_id,
        from,
        to,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_mint")
        .add_message(msg_mint))
}

pub fn try_burn(
    deps: DepsMut,
    _info: MessageInfo,
    from: String,
    amount: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_burn: CosmosMsg = MsgBurn {
        contract_id,
        from,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_burn")
        .add_message(msg_burn))
}

pub fn try_send(
    deps: DepsMut,
    _info: MessageInfo,
    from: String,
    to: String,
    amount: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_send: CosmosMsg = MsgSend {
        contract_id,
        from,
        to,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_send")
        .add_message(msg_send))
}

pub fn try_operator_burn(
    deps: DepsMut,
    _info: MessageInfo,
    operator: String,
    from: String,
    amount: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_operator_burn: CosmosMsg = MsgOperatorBurn {
        contract_id,
        operator,
        from,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_operator_burn")
        .add_message(msg_operator_burn))
}

pub fn try_operator_send(
    deps: DepsMut,
    _info: MessageInfo,
    operator: String,
    from: String,
    to: String,
    amount: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_operator_send: CosmosMsg = MsgOperatorSend {
        contract_id,
        operator,
        from,
        to,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_operator_send")
        .add_message(msg_operator_send))
}

pub fn try_authorize_operator(
    deps: DepsMut,
    _info: MessageInfo,
    holder: String,
    operator: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_authorize_operator: CosmosMsg = MsgAuthorizeOperator {
        contract_id,
        holder,
        operator,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_authorize_operator")
        .add_message(msg_authorize_operator))
}

pub fn try_revoke_operator(
    deps: DepsMut,
    _info: MessageInfo,
    holder: String,
    operator: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_revoke_operator: CosmosMsg = MsgRevokeOperator {
        contract_id,
        holder,
        operator,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_revoke_operator")
        .add_message(msg_revoke_operator))
}

pub fn try_grant_permission(
    deps: DepsMut,
    _info: MessageInfo,
    from: String,
    to: String,
    permission: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_grant_permission: CosmosMsg = MsgGrantPermission {
        contract_id,
        from,
        to,
        permission,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_grant_permission")
        .add_message(msg_grant_permission))
}

pub fn try_revoke_permission(
    deps: DepsMut,
    _info: MessageInfo,
    from: String,
    permission: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_revoke_permission: CosmosMsg = MsgRevokePermission {
        contract_id,
        from,
        permission,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_revoke_permission")
        .add_message(msg_revoke_permission))
}

pub fn try_modify(
    deps: DepsMut,
    _info: MessageInfo,
    owner: String,
    changes: Vec<Attribute>,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_modify: CosmosMsg = MsgModify {
        contract_id,
        owner,
        changes,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_modify")
        .add_message(msg_modify))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Contract {} => to_binary(&query_contract(deps)?),
        QueryMsg::Minted {} => to_binary(&query_minted(deps)?),
        QueryMsg::Burnt {} => to_binary(&query_burnt(deps)?),
        QueryMsg::Supply {} => to_binary(&query_supply(deps)?),
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::GranteeGrants {
            grantee,
            pagination,
        } => to_binary(&query_grantee_grants(deps, grantee, pagination)?),
        QueryMsg::IsOperatorFor { operator, holder } => {
            to_binary(&query_is_operator_for(deps, operator, holder)?)
        }
        QueryMsg::HoldersByOperator {
            operator,
            pagination,
        } => to_binary(&query_holders_by_operator(deps, operator, pagination)?),
    }
}

fn query_contract(deps: Deps) -> StdResult<QueryContractResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let tq = TokenQuerier::new(&deps.querier);
    let res = tq.contract(contract_id)?;
    Ok(QueryContractResponse {
        contract: res.contract,
    })
}

fn query_minted(deps: Deps) -> StdResult<QueryMintedResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let tq = TokenQuerier::new(&deps.querier);
    let res = tq.minted(contract_id)?;
    Ok(QueryMintedResponse { amount: res.amount })
}

fn query_burnt(deps: Deps) -> StdResult<QueryBurntResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let tq = TokenQuerier::new(&deps.querier);
    let res = tq.burnt(contract_id)?;
    Ok(QueryBurntResponse { amount: res.amount })
}

fn query_supply(deps: Deps) -> StdResult<QuerySupplyResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let tq = TokenQuerier::new(&deps.querier);
    let res = tq.supply(contract_id)?;
    Ok(QuerySupplyResponse { amount: res.amount })
}

fn query_balance(deps: Deps, address: String) -> StdResult<QueryBalanceResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let tq = TokenQuerier::new(&deps.querier);
    let res = tq.balance(contract_id, address)?;
    Ok(QueryBalanceResponse { amount: res.amount })
}

fn query_grantee_grants(
    deps: Deps,
    grantee: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryGranteeGrantsResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let tq = TokenQuerier::new(&deps.querier);
    let res = tq.grantee_grants(contract_id, grantee, pagination)?;
    Ok(QueryGranteeGrantsResponse {
        grants: res.grants,
        pagination: res.pagination,
    })
}

fn query_is_operator_for(
    deps: Deps,
    operator: String,
    holder: String,
) -> StdResult<QueryIsOperatorForResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let tq = TokenQuerier::new(&deps.querier);
    let res = tq.is_operator_for(contract_id, operator, holder)?;
    Ok(QueryIsOperatorForResponse {
        authorized: res.authorized,
    })
}

fn query_holders_by_operator(
    deps: Deps,
    operator: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryHoldersByOperatorResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let tq = TokenQuerier::new(&deps.querier);
    let res = tq.holders_by_operator(contract_id, operator, pagination)?;
    Ok(QueryHoldersByOperatorResponse {
        holders: res.holders,
        pagination: res.pagination,
    })
}
