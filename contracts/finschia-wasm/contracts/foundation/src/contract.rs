#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use finschia_std::shim::Any;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::cosmos::base::v1beta1::Coin;
use finschia_std::types::lbm::foundation::v1::{
    Censorship, FoundationQuerier, MemberRequest, MsgExec, MsgFundTreasury, MsgGrant,
    MsgLeaveFoundation, MsgRevoke, MsgSubmitProposal, MsgUpdateCensorship, MsgUpdateDecisionPolicy,
    MsgUpdateMembers, MsgVote, MsgWithdrawFromTreasury, MsgWithdrawProposal,
};
use finschia_std::types::lbm::foundation::v1::{
    QueryCensorshipsResponse, QueryFoundationInfoResponse, QueryGrantsResponse,
    QueryMemberResponse, QueryMembersResponse, QueryParamsResponse, QueryProposalResponse,
    QueryProposalsResponse, QueryTallyResultResponse, QueryTreasuryResponse, QueryVoteResponse,
    QueryVotesResponse,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-stargate-gov";
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
        ExecuteMsg::FundTreasury { from, amount } => try_fund_treasury(deps, info, from, amount),
        ExecuteMsg::WithdrawFromTreasury {
            authority,
            to,
            amount,
        } => try_withdraw_from_treasury(deps, info, authority, to, amount),
        ExecuteMsg::SubmitProposal {
            proposers,
            metadata,
            messages,
            exec,
        } => try_submit_proposal(deps, info, proposers, metadata, messages, exec),
        ExecuteMsg::WithdrawProposal {
            proposal_id,
            address,
        } => try_withdraw_proposal(deps, info, proposal_id, address),
        ExecuteMsg::Vote {
            proposal_id,
            voter,
            option,
            metadata,
            exec,
        } => try_vote(deps, info, proposal_id, voter, option, metadata, exec),
        ExecuteMsg::Exec {
            proposal_id,
            signer,
        } => try_exec(deps, info, proposal_id, signer),
        ExecuteMsg::UpdateMembers {
            authority,
            member_updates,
        } => try_update_members(deps, info, authority, member_updates),
        ExecuteMsg::LeaveFoundation { address } => try_leave_foundation(deps, info, address),
        ExecuteMsg::UpdateDecisionPolicy {
            authority,
            decision_policy,
        } => try_update_decision_policy(deps, info, authority, decision_policy),
        ExecuteMsg::UpdateCensorship {
            authority,
            censorship,
        } => try_update_censorship(deps, info, authority, censorship),
        ExecuteMsg::Grant {
            authority,
            grantee,
            authorization,
        } => try_grant(deps, info, authority, grantee, authorization),
        ExecuteMsg::Revoke {
            authority,
            grantee,
            msg_type_url,
        } => try_revoke(deps, info, authority, grantee, msg_type_url),
    }
}

pub fn try_fund_treasury(
    _deps: DepsMut,
    _info: MessageInfo,
    from: String,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    let msg_fund_treasury: CosmosMsg = MsgFundTreasury { from, amount }.into();

    Ok(Response::new()
        .add_attribute("method", "try_fund_treasury")
        .add_message(msg_fund_treasury))
}

pub fn try_withdraw_from_treasury(
    _deps: DepsMut,
    _info: MessageInfo,
    authority: String,
    to: String,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    let msg_withdraw_from_treasury: CosmosMsg = MsgWithdrawFromTreasury {
        authority,
        to,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_withdraw_from_treasury")
        .add_message(msg_withdraw_from_treasury))
}

pub fn try_submit_proposal(
    _deps: DepsMut,
    _info: MessageInfo,
    proposers: Vec<String>,
    metadata: String,
    messages: Vec<Any>,
    exec: i32,
) -> Result<Response, ContractError> {
    let msg_submit_proposal: CosmosMsg = MsgSubmitProposal {
        proposers,
        metadata,
        messages,
        exec,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_submit_proposal")
        .add_message(msg_submit_proposal))
}

pub fn try_withdraw_proposal(
    _deps: DepsMut,
    _info: MessageInfo,
    proposal_id: u64,
    address: String,
) -> Result<Response, ContractError> {
    let msg_withdraw_proposal: CosmosMsg = MsgWithdrawProposal {
        proposal_id,
        address,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_withdraw_proposal")
        .add_message(msg_withdraw_proposal))
}

pub fn try_vote(
    _deps: DepsMut,
    _info: MessageInfo,
    proposal_id: u64,
    voter: String,
    option: i32,
    metadata: String,
    exec: i32,
) -> Result<Response, ContractError> {
    let msg_vote: CosmosMsg = MsgVote {
        proposal_id,
        voter,
        option,
        metadata,
        exec,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_vote")
        .add_message(msg_vote))
}

pub fn try_exec(
    _deps: DepsMut,
    _info: MessageInfo,
    proposal_id: u64,
    signer: String,
) -> Result<Response, ContractError> {
    let msg_exec: CosmosMsg = MsgExec {
        proposal_id,
        signer,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_exec")
        .add_message(msg_exec))
}

pub fn try_update_members(
    _deps: DepsMut,
    _info: MessageInfo,
    authority: String,
    member_updates: Vec<MemberRequest>,
) -> Result<Response, ContractError> {
    let msg_update_members: CosmosMsg = MsgUpdateMembers {
        authority,
        member_updates,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_update_members")
        .add_message(msg_update_members))
}

pub fn try_leave_foundation(
    _deps: DepsMut,
    _info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    let msg_leave_foundation: CosmosMsg = MsgLeaveFoundation { address }.into();

    Ok(Response::new()
        .add_attribute("method", "try_leave_foundation")
        .add_message(msg_leave_foundation))
}

pub fn try_update_decision_policy(
    _deps: DepsMut,
    _info: MessageInfo,
    authority: String,
    decision_policy: Option<Any>,
) -> Result<Response, ContractError> {
    let msg_update_decision_policy: CosmosMsg = MsgUpdateDecisionPolicy {
        authority,
        decision_policy,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_update_decision_policy")
        .add_message(msg_update_decision_policy))
}

pub fn try_update_censorship(
    _deps: DepsMut,
    _info: MessageInfo,
    authority: String,
    censorship: Option<Censorship>,
) -> Result<Response, ContractError> {
    let msg_update_censorship: CosmosMsg = MsgUpdateCensorship {
        authority,
        censorship,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_update_censorship")
        .add_message(msg_update_censorship))
}

pub fn try_grant(
    _deps: DepsMut,
    _info: MessageInfo,
    authority: String,
    grantee: String,
    authorization: Option<Any>,
) -> Result<Response, ContractError> {
    let msg_grant: CosmosMsg = MsgGrant {
        authority,
        grantee,
        authorization,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_grant")
        .add_message(msg_grant))
}

pub fn try_revoke(
    _deps: DepsMut,
    _info: MessageInfo,
    authority: String,
    grantee: String,
    msg_type_url: String,
) -> Result<Response, ContractError> {
    let msg_revoke: CosmosMsg = MsgRevoke {
        authority,
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
        QueryMsg::FoundationInfo {} => to_binary(&query_foundation_info(deps)?),
        QueryMsg::Treasury {} => to_binary(&query_treasury(deps)?),
        QueryMsg::Member { address } => to_binary(&query_member(deps, address)?),
        QueryMsg::Members { pagination } => to_binary(&query_members(deps, pagination)?),
        QueryMsg::Proposal { proposal_id } => to_binary(&query_proposal(deps, proposal_id)?),
        QueryMsg::Proposals { pagination } => to_binary(&query_proposals(deps, pagination)?),
        QueryMsg::Vote { proposal_id, voter } => to_binary(&query_vote(deps, proposal_id, voter)?),
        QueryMsg::Votes {
            proposal_id,
            pagination,
        } => to_binary(&query_votes(deps, proposal_id, pagination)?),
        QueryMsg::TallyResult { proposal_id } => to_binary(&query_tally_result(deps, proposal_id)?),
        QueryMsg::Censorships { pagination } => to_binary(&query_censorships(deps, pagination)?),
        QueryMsg::Grants {
            grantee,
            msg_type_url,
            pagination,
        } => to_binary(&query_grants(deps, grantee, msg_type_url, pagination)?),
        QueryMsg::Params {} => to_binary(&query_params(deps)?),
    }
}

fn query_foundation_info(deps: Deps) -> StdResult<QueryFoundationInfoResponse> {
    let fq = FoundationQuerier::new(&deps.querier);
    let res = fq.foundation_info()?;
    Ok(QueryFoundationInfoResponse { info: res.info })
}

fn query_treasury(deps: Deps) -> StdResult<QueryTreasuryResponse> {
    let fq = FoundationQuerier::new(&deps.querier);
    let res = fq.treasury()?;
    Ok(QueryTreasuryResponse { amount: res.amount })
}

fn query_member(deps: Deps, address: String) -> StdResult<QueryMemberResponse> {
    let fq = FoundationQuerier::new(&deps.querier);
    let res = fq.member(address)?;
    Ok(QueryMemberResponse { member: res.member })
}

fn query_members(deps: Deps, pagination: Option<PageRequest>) -> StdResult<QueryMembersResponse> {
    let fq = FoundationQuerier::new(&deps.querier);
    let res = fq.members(pagination)?;
    Ok(QueryMembersResponse {
        members: res.members,
        pagination: res.pagination,
    })
}

fn query_proposal(deps: Deps, proposal_id: u64) -> StdResult<QueryProposalResponse> {
    let fq = FoundationQuerier::new(&deps.querier);
    let res = fq.proposal(proposal_id)?;
    Ok(QueryProposalResponse {
        proposal: res.proposal,
    })
}

fn query_proposals(
    deps: Deps,
    pagination: Option<PageRequest>,
) -> StdResult<QueryProposalsResponse> {
    let fq = FoundationQuerier::new(&deps.querier);
    let res = fq.proposals(pagination)?;
    Ok(QueryProposalsResponse {
        proposals: res.proposals,
        pagination: res.pagination,
    })
}

fn query_vote(deps: Deps, proposal_id: u64, voter: String) -> StdResult<QueryVoteResponse> {
    let fq = FoundationQuerier::new(&deps.querier);
    let res = fq.vote(proposal_id, voter)?;
    Ok(QueryVoteResponse { vote: res.vote })
}

fn query_votes(
    deps: Deps,
    proposal_id: u64,
    pagination: Option<PageRequest>,
) -> StdResult<QueryVotesResponse> {
    let fq = FoundationQuerier::new(&deps.querier);
    let res = fq.votes(proposal_id, pagination)?;
    Ok(QueryVotesResponse {
        votes: res.votes,
        pagination: res.pagination,
    })
}

fn query_tally_result(deps: Deps, proposal_id: u64) -> StdResult<QueryTallyResultResponse> {
    let fq = FoundationQuerier::new(&deps.querier);
    let res = fq.tally_result(proposal_id)?;
    Ok(QueryTallyResultResponse { tally: res.tally })
}

fn query_censorships(
    deps: Deps,
    pagination: Option<PageRequest>,
) -> StdResult<QueryCensorshipsResponse> {
    let fq = FoundationQuerier::new(&deps.querier);
    let res = fq.censorships(pagination)?;
    Ok(QueryCensorshipsResponse {
        censorships: res.censorships,
        pagination: res.pagination,
    })
}

fn query_grants(
    deps: Deps,
    grantee: String,
    msg_type_url: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryGrantsResponse> {
    let fq = FoundationQuerier::new(&deps.querier);
    let res = fq.grants(grantee, msg_type_url, pagination)?;
    Ok(QueryGrantsResponse {
        authorizations: res.authorizations,
        pagination: res.pagination,
    })
}

fn query_params(deps: Deps) -> StdResult<QueryParamsResponse> {
    let fq = FoundationQuerier::new(&deps.querier);
    let res = fq.params()?;
    Ok(QueryParamsResponse { params: res.params })
}
