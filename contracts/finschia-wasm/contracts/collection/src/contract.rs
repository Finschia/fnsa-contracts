use std::convert::TryFrom;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
    StdResult, SubMsg, SubMsgResponse, SubMsgResult,
};
use cw2::set_contract_version;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::lbm::collection::v1::{
    Attribute, Coin, MintNftParam, MsgAttach, MsgAuthorizeOperator, MsgBurnFt, MsgBurnNft,
    MsgCreateContract, MsgCreateContractResponse, MsgDetach, MsgGrantPermission, MsgIssueFt,
    MsgIssueFtResponse, MsgIssueNft, MsgIssueNftResponse, MsgMintFt, MsgMintNft,
    MsgMintNftResponse, MsgModify, MsgOperatorAttach, MsgOperatorBurnFt, MsgOperatorBurnNft,
    MsgOperatorDetach, MsgOperatorSendFt, MsgOperatorSendNft, MsgRevokeOperator,
    MsgRevokePermission, MsgSendFt, MsgSendNft,
};
use finschia_std::types::lbm::collection::v1::{
    CollectionQuerier, QueryAllBalancesResponse, QueryBalanceResponse, QueryChildrenResponse,
    QueryContractResponse, QueryFtBurntResponse, QueryFtMintedResponse, QueryFtSupplyResponse,
    QueryGranteeGrantsResponse, QueryHasParentResponse, QueryHoldersByOperatorResponse,
    QueryIsOperatorForResponse, QueryNftBurntResponse, QueryNftMintedResponse,
    QueryNftSupplyResponse, QueryParentResponse, QueryRootResponse,
    QueryTokenClassTypeNameResponse, QueryTokenResponse, QueryTokenTypeResponse,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::CONTRACT_ID;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-stargate-collection";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const CREATE_CONTRACT_REPLY_ID: u64 = 1;
const ISSUE_NFT_REPLY_ID: u64 = 2;
const MINT_NFT_REPLY_ID: u64 = 3;
const ISSUE_FT_REPLY_ID: u64 = 4;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let msg_create_contract: CosmosMsg = MsgCreateContract {
        owner: msg.owner,
        name: msg.name,
        uri: msg.uri,
        meta: msg.meta,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_submessage(SubMsg::reply_on_success(
            msg_create_contract,
            CREATE_CONTRACT_REPLY_ID,
        )))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        CREATE_CONTRACT_REPLY_ID => handle_create_contract_reply(deps, msg),
        ISSUE_NFT_REPLY_ID => handle_issue_nft_reply(deps, msg),
        MINT_NFT_REPLY_ID => handle_mint_nft_reply(deps, msg),
        ISSUE_FT_REPLY_ID => handle_issue_ft_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

fn handle_create_contract_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result {
        let res = MsgCreateContractResponse::try_from(b)?;
        let _ = CONTRACT_ID.save(deps.storage, &res.contract_id);
        return Ok(Response::new().add_attribute("contract_id", res.contract_id));
    } else {
        return Err(StdError::generic_err(
            "failed to get MsgCreateContractResponse",
        ));
    }
}

fn handle_issue_nft_reply(_deps: DepsMut, msg: Reply) -> StdResult<Response> {
    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result {
        let res = MsgIssueNftResponse::try_from(b)?;
        return Ok(Response::new().add_attribute("token_type", res.token_type));
    } else {
        return Err(StdError::generic_err("failed to get MsgIssueNftResponse"));
    }
}

fn handle_mint_nft_reply(_deps: DepsMut, msg: Reply) -> StdResult<Response> {
    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result {
        let res = MsgMintNftResponse::try_from(b)?;
        let attrs = res
            .token_ids
            .iter()
            .enumerate()
            .map(|(idx, id)| attr(format!("token_id_{}", idx), id));
        return Ok(Response::new().add_attributes(attrs));
    } else {
        return Err(StdError::generic_err("failed to get MsgMintNftResponse"));
    }
}

fn handle_issue_ft_reply(_deps: DepsMut, msg: Reply) -> StdResult<Response> {
    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result {
        let res = MsgIssueFtResponse::try_from(b)?;
        return Ok(Response::new().add_attribute("token_id", res.token_id));
    } else {
        return Err(StdError::generic_err("failed to get MsgIssueFtResponse"));
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
        ExecuteMsg::IssueNft { name, meta, owner } => try_issue_nft(deps, name, meta, owner),
        ExecuteMsg::MintNft { from, to, params } => try_mint_nft(deps, info, from, to, params),
        ExecuteMsg::SendNft {
            from,
            to,
            token_ids,
        } => try_send_nft(deps, info, from, to, token_ids),
        ExecuteMsg::BurnNft { from, token_ids } => try_burn_nft(deps, info, from, token_ids),
        ExecuteMsg::OperatorSendNft {
            operator,
            from,
            to,
            token_ids,
        } => try_operator_send_nft(deps, info, operator, from, to, token_ids),
        ExecuteMsg::OperatorBurnNft {
            operator,
            from,
            token_ids,
        } => try_operator_burn_nft(deps, info, operator, from, token_ids),
        ExecuteMsg::IssueFt {
            name,
            meta,
            decimals,
            mintable,
            owner,
            to,
            amount,
        } => try_issue_ft(
            deps, info, name, meta, decimals, mintable, owner, to, amount,
        ),
        ExecuteMsg::MintFt { from, to, amount } => try_mint_ft(deps, info, from, to, amount),
        ExecuteMsg::SendFt { from, to, amount } => try_send_ft(deps, info, from, to, amount),
        ExecuteMsg::BurnFt { from, amount } => try_burn_ft(deps, info, from, amount),
        ExecuteMsg::OperatorSendFt {
            operator,
            from,
            to,
            amount,
        } => try_operator_send_ft(deps, info, operator, from, to, amount),
        ExecuteMsg::OperatorBurnFt {
            operator,
            from,
            amount,
        } => try_operator_burn_ft(deps, info, operator, from, amount),
        ExecuteMsg::Attach {
            from,
            token_id,
            to_token_id,
        } => try_attach(deps, info, from, token_id, to_token_id),
        ExecuteMsg::Detach { from, token_id } => try_detach(deps, info, from, token_id),
        ExecuteMsg::OperatorAttach {
            operator,
            from,
            token_id,
            to_token_id,
        } => try_operator_attach(deps, info, operator, from, token_id, to_token_id),
        ExecuteMsg::OperatorDetach {
            operator,
            from,
            token_id,
        } => try_operator_detach(deps, info, operator, from, token_id),
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
        ExecuteMsg::Modify {
            owner,
            token_type,
            token_index,
            changes,
        } => try_modify(deps, info, owner, token_type, token_index, changes),
    }
}

pub fn try_issue_nft(
    deps: DepsMut,
    name: String,
    meta: String,
    owner: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_issue_nft: CosmosMsg = MsgIssueNft {
        contract_id,
        name,
        meta,
        owner,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_issue_nft")
        .add_submessage(SubMsg::reply_on_success(msg_issue_nft, ISSUE_NFT_REPLY_ID)))
}

pub fn try_mint_nft(
    deps: DepsMut,
    _info: MessageInfo,
    from: String,
    to: String,
    params: Vec<MintNftParam>,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_mint_nft: CosmosMsg = MsgMintNft {
        contract_id,
        from,
        to,
        params,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_mint_nft")
        .add_submessage(SubMsg::reply_on_success(msg_mint_nft, MINT_NFT_REPLY_ID)))
}

pub fn try_send_nft(
    deps: DepsMut,
    _info: MessageInfo,
    from: String,
    to: String,
    token_ids: Vec<String>,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_send_nft: CosmosMsg = MsgSendNft {
        contract_id,
        from,
        to,
        token_ids,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_mint_nft")
        .add_message(msg_send_nft))
}

pub fn try_burn_nft(
    deps: DepsMut,
    _info: MessageInfo,
    from: String,
    token_ids: Vec<String>,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_burn_nft: CosmosMsg = MsgBurnNft {
        contract_id,
        from,
        token_ids,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_burn_nft")
        .add_message(msg_burn_nft))
}

pub fn try_operator_send_nft(
    deps: DepsMut,
    _info: MessageInfo,
    operator: String,
    from: String,
    to: String,
    token_ids: Vec<String>,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_operator_send_nft: CosmosMsg = MsgOperatorSendNft {
        contract_id,
        operator,
        from,
        to,
        token_ids,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_operator_send_nft")
        .add_message(msg_operator_send_nft))
}

pub fn try_operator_burn_nft(
    deps: DepsMut,
    _info: MessageInfo,
    operator: String,
    from: String,
    token_ids: Vec<String>,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_operator_burn_nft: CosmosMsg = MsgOperatorBurnNft {
        contract_id,
        operator,
        from,
        token_ids,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_operator_burn_nft")
        .add_message(msg_operator_burn_nft))
}

pub fn try_issue_ft(
    deps: DepsMut,
    _info: MessageInfo,
    name: String,
    meta: String,
    decimals: i32,
    mintable: bool,
    owner: String,
    to: String,
    amount: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_issue_ft: CosmosMsg = MsgIssueFt {
        contract_id,
        name,
        meta,
        decimals,
        mintable,
        owner,
        to,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_issue_ft")
        .add_submessage(SubMsg::reply_on_success(msg_issue_ft, ISSUE_FT_REPLY_ID)))
}

pub fn try_mint_ft(
    deps: DepsMut,
    _info: MessageInfo,
    from: String,
    to: String,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_mint_ft: CosmosMsg = MsgMintFt {
        contract_id,
        from,
        to,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_mint_ft")
        .add_message(msg_mint_ft))
}

pub fn try_send_ft(
    deps: DepsMut,
    _info: MessageInfo,
    from: String,
    to: String,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_send_ft: CosmosMsg = MsgSendFt {
        contract_id,
        from,
        to,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_send_ft")
        .add_message(msg_send_ft))
}

pub fn try_burn_ft(
    deps: DepsMut,
    _info: MessageInfo,
    from: String,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_burn_ft: CosmosMsg = MsgBurnFt {
        contract_id,
        from,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_burn_ft")
        .add_message(msg_burn_ft))
}

pub fn try_operator_send_ft(
    deps: DepsMut,
    _info: MessageInfo,
    operator: String,
    from: String,
    to: String,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_operator_send_ft: CosmosMsg = MsgOperatorSendFt {
        contract_id,
        operator,
        from,
        to,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_operator_send_ft")
        .add_message(msg_operator_send_ft))
}

pub fn try_operator_burn_ft(
    deps: DepsMut,
    _info: MessageInfo,
    operator: String,
    from: String,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_operator_burn_ft: CosmosMsg = MsgOperatorBurnFt {
        contract_id,
        operator,
        from,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_operator_burn_ft")
        .add_message(msg_operator_burn_ft))
}

pub fn try_attach(
    deps: DepsMut,
    _info: MessageInfo,
    from: String,
    token_id: String,
    to_token_id: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_attach: CosmosMsg = MsgAttach {
        contract_id,
        from,
        token_id,
        to_token_id,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_attach")
        .add_message(msg_attach))
}

pub fn try_detach(
    deps: DepsMut,
    _info: MessageInfo,
    from: String,
    token_id: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_detach: CosmosMsg = MsgDetach {
        contract_id,
        from,
        token_id,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_detach")
        .add_message(msg_detach))
}

pub fn try_operator_attach(
    deps: DepsMut,
    _info: MessageInfo,
    operator: String,
    from: String,
    token_id: String,
    to_token_id: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_operator_attach: CosmosMsg = MsgOperatorAttach {
        contract_id,
        operator,
        from,
        token_id,
        to_token_id,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_operator_attach")
        .add_message(msg_operator_attach))
}

pub fn try_operator_detach(
    deps: DepsMut,
    _info: MessageInfo,
    operator: String,
    from: String,
    token_id: String,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_operator_detach: CosmosMsg = MsgOperatorDetach {
        contract_id,
        operator,
        from,
        token_id,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_operator_detach")
        .add_message(msg_operator_detach))
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
    token_type: String,
    token_index: String,
    changes: Vec<Attribute>,
) -> Result<Response, ContractError> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let msg_modify: CosmosMsg = MsgModify {
        contract_id,
        owner,
        token_type,
        token_index,
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
        QueryMsg::NftMinted { token_type } => to_binary(&query_nft_minted(deps, token_type)?),
        QueryMsg::NftBurnt { token_type } => to_binary(&query_nft_burnt(deps, token_type)?),
        QueryMsg::NftSupply { token_type } => to_binary(&query_nft_supply(deps, token_type)?),
        QueryMsg::FtMinted { token_id } => to_binary(&query_ft_minted(deps, token_id)?),
        QueryMsg::FtBurnt { token_id } => to_binary(&query_ft_burnt(deps, token_id)?),
        QueryMsg::FtSupply { token_id } => to_binary(&query_ft_supply(deps, token_id)?),
        QueryMsg::Root { token_id } => to_binary(&query_root(deps, token_id)?),
        QueryMsg::HasParent { token_id } => to_binary(&query_has_parent(deps, token_id)?),
        QueryMsg::Parent { token_id } => to_binary(&query_parent(deps, token_id)?),
        QueryMsg::Children {
            token_id,
            pagination,
        } => to_binary(&query_children(deps, token_id, pagination)?),
        QueryMsg::Balance { address, token_id } => {
            to_binary(&query_balance(deps, address, token_id)?)
        }
        QueryMsg::AllBalance {
            address,
            pagination,
        } => to_binary(&query_all_balances(deps, address, pagination)?),
        QueryMsg::Token { token_id } => to_binary(&query_token(deps, token_id)?),
        QueryMsg::TokenType { token_type } => to_binary(&query_token_type(deps, token_type)?),
        QueryMsg::TokenClassTypeName { class_id } => {
            to_binary(&query_token_class_type_name(deps, class_id)?)
        }
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
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.contract(contract_id)?;
    Ok(QueryContractResponse {
        contract: res.contract,
    })
}

fn query_nft_minted(deps: Deps, token_type: String) -> StdResult<QueryNftMintedResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.nft_minted(contract_id, token_type)?;
    Ok(QueryNftMintedResponse { minted: res.minted })
}

fn query_nft_burnt(deps: Deps, token_type: String) -> StdResult<QueryNftBurntResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.nft_burnt(contract_id, token_type)?;
    Ok(QueryNftBurntResponse { burnt: res.burnt })
}

fn query_nft_supply(deps: Deps, token_type: String) -> StdResult<QueryNftSupplyResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.nft_supply(contract_id, token_type)?;
    Ok(QueryNftSupplyResponse { supply: res.supply })
}

fn query_ft_minted(deps: Deps, token_id: String) -> StdResult<QueryFtMintedResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.ft_minted(contract_id, token_id)?;
    Ok(QueryFtMintedResponse { minted: res.minted })
}

fn query_ft_burnt(deps: Deps, token_id: String) -> StdResult<QueryFtBurntResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.ft_burnt(contract_id, token_id)?;
    Ok(QueryFtBurntResponse { burnt: res.burnt })
}

fn query_ft_supply(deps: Deps, token_id: String) -> StdResult<QueryFtSupplyResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.ft_supply(contract_id, token_id)?;
    Ok(QueryFtSupplyResponse { supply: res.supply })
}

fn query_root(deps: Deps, token_id: String) -> StdResult<QueryRootResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.root(contract_id, token_id)?;
    Ok(QueryRootResponse { root: res.root })
}

fn query_has_parent(deps: Deps, token_id: String) -> StdResult<QueryHasParentResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.has_parent(contract_id, token_id)?;
    Ok(QueryHasParentResponse {
        has_parent: res.has_parent,
    })
}

fn query_parent(deps: Deps, token_id: String) -> StdResult<QueryParentResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.parent(contract_id, token_id)?;
    Ok(QueryParentResponse { parent: res.parent })
}

fn query_children(
    deps: Deps,
    token_id: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryChildrenResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.children(contract_id, token_id, pagination)?;
    Ok(QueryChildrenResponse {
        children: res.children,
        pagination: res.pagination,
    })
}

fn query_balance(deps: Deps, address: String, token_id: String) -> StdResult<QueryBalanceResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.balance(contract_id, address, token_id)?;
    Ok(QueryBalanceResponse {
        balance: res.balance,
    })
}

fn query_all_balances(
    deps: Deps,
    address: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryAllBalancesResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.all_balances(contract_id, address, pagination)?;
    Ok(QueryAllBalancesResponse {
        balances: res.balances,
        pagination: res.pagination,
    })
}

fn query_token(deps: Deps, token_id: String) -> StdResult<QueryTokenResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.token(contract_id, token_id)?;
    Ok(QueryTokenResponse { token: res.token })
}

fn query_token_type(deps: Deps, token_type: String) -> StdResult<QueryTokenTypeResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.token_type(contract_id, token_type)?;
    Ok(QueryTokenTypeResponse {
        token_type: res.token_type,
    })
}

fn query_token_class_type_name(
    deps: Deps,
    class_id: String,
) -> StdResult<QueryTokenClassTypeNameResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.token_class_type_name(contract_id, class_id)?;
    Ok(QueryTokenClassTypeNameResponse { name: res.name })
}

fn query_grantee_grants(
    deps: Deps,
    grantee: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryGranteeGrantsResponse> {
    let contract_id = CONTRACT_ID.load(deps.storage)?;
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.grantee_grants(contract_id, grantee, pagination)?;
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
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.is_operator_for(contract_id, operator, holder)?;
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
    let cq = CollectionQuerier::new(&deps.querier);
    let res = cq.holders_by_operator(contract_id, operator, pagination)?;
    Ok(QueryHoldersByOperatorResponse {
        holders: res.holders,
        pagination: res.pagination,
    })
}
