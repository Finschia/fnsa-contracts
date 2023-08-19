#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use finschia_std::shim::Any;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::cosmos::base::v1beta1::Coin;
use finschia_std::types::cosmos::gov::v1beta1::{
    GovQuerier, MsgDeposit, MsgSubmitProposal, MsgVote, MsgVoteWeighted, WeightedVoteOption,
};
use finschia_std::types::cosmos::gov::v1beta1::{
    QueryDepositResponse, QueryDepositsResponse, QueryParamsResponse, QueryProposalResponse,
    QueryProposalsResponse, QueryTallyResultResponse, QueryVoteResponse, QueryVotesResponse,
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
        ExecuteMsg::Vote {
            proposal_id,
            voter,
            option,
        } => try_vote(deps, info, proposal_id, voter, option),
        ExecuteMsg::VoteWeighted {
            proposal_id,
            voter,
            options,
        } => try_vote_weighted(deps, info, proposal_id, voter, options),
        ExecuteMsg::Deposit {
            proposal_id,
            depositor,
            amount,
        } => try_deposit(deps, info, proposal_id, depositor, amount),
        ExecuteMsg::SubmitProposal {
            content,
            initial_deposit,
            proposer,
        } => try_submit_proposal(deps, info, content, initial_deposit, proposer),
    }
}

pub fn try_vote(
    _deps: DepsMut,
    _info: MessageInfo,
    proposal_id: u64,
    voter: String,
    option: i32,
) -> Result<Response, ContractError> {
    let msg_vote: CosmosMsg = MsgVote {
        proposal_id,
        voter,
        option,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_vote")
        .add_message(msg_vote))
}

pub fn try_vote_weighted(
    _deps: DepsMut,
    _info: MessageInfo,
    proposal_id: u64,
    voter: String,
    options: Vec<WeightedVoteOption>,
) -> Result<Response, ContractError> {
    let msg_vote_weighted: CosmosMsg = MsgVoteWeighted {
        proposal_id,
        voter,
        options,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_vote_weighted")
        .add_message(msg_vote_weighted))
}

pub fn try_deposit(
    _deps: DepsMut,
    _info: MessageInfo,
    proposal_id: u64,
    depositor: String,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    let msg_deposit: CosmosMsg = MsgDeposit {
        proposal_id,
        depositor,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_deposit")
        .add_message(msg_deposit))
}

pub fn try_submit_proposal(
    _deps: DepsMut,
    _info: MessageInfo,
    content: Option<Any>,
    initial_deposit: Vec<Coin>,
    proposer: String,
) -> Result<Response, ContractError> {
    let msg_submit_proposal: CosmosMsg = MsgSubmitProposal {
        content,
        initial_deposit,
        proposer,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_submit_proposal")
        .add_message(msg_submit_proposal))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Vote { proposal_id, voter } => to_binary(&query_vote(deps, proposal_id, voter)?),
        QueryMsg::Votes {
            proposal_id,
            pagination,
        } => to_binary(&query_votes(deps, proposal_id, pagination)?),
        QueryMsg::Deposit {
            proposal_id,
            depositor,
        } => to_binary(&query_deposit(deps, proposal_id, depositor)?),
        QueryMsg::Deposits {
            proposal_id,
            pagination,
        } => to_binary(&query_deposits(deps, proposal_id, pagination)?),
        QueryMsg::Proposal { proposal_id } => to_binary(&query_proposal(deps, proposal_id)?),
        QueryMsg::Proposals {
            proposal_status,
            voter,
            depositor,
            pagination,
        } => to_binary(&query_proposals(
            deps,
            proposal_status,
            voter,
            depositor,
            pagination,
        )?),
        QueryMsg::TallyResult { proposal_id } => to_binary(&query_tally_result(deps, proposal_id)?),
        QueryMsg::Params { param_types } => to_binary(&query_params(deps, param_types)?),
    }
}

fn query_vote(deps: Deps, proposal_id: u64, voter: String) -> StdResult<QueryVoteResponse> {
    let gq = GovQuerier::new(&deps.querier);
    let res = gq.vote(proposal_id, voter)?;
    Ok(QueryVoteResponse { vote: res.vote })
}

fn query_votes(
    deps: Deps,
    proposal_id: u64,
    pagination: Option<PageRequest>,
) -> StdResult<QueryVotesResponse> {
    let gq = GovQuerier::new(&deps.querier);
    let res = gq.votes(proposal_id, pagination)?;
    Ok(QueryVotesResponse {
        votes: res.votes,
        pagination: res.pagination,
    })
}

fn query_deposit(
    deps: Deps,
    proposal_id: u64,
    depositor: String,
) -> StdResult<QueryDepositResponse> {
    let gq = GovQuerier::new(&deps.querier);
    let res = gq.deposit(proposal_id, depositor)?;
    Ok(QueryDepositResponse {
        deposit: res.deposit,
    })
}

fn query_deposits(
    deps: Deps,
    proposal_id: u64,
    pagination: Option<PageRequest>,
) -> StdResult<QueryDepositsResponse> {
    let gq = GovQuerier::new(&deps.querier);
    let res = gq.deposits(proposal_id, pagination)?;
    Ok(QueryDepositsResponse {
        deposits: res.deposits,
        pagination: res.pagination,
    })
}

fn query_proposal(deps: Deps, proposal_id: u64) -> StdResult<QueryProposalResponse> {
    let gq = GovQuerier::new(&deps.querier);
    let res = gq.proposal(proposal_id)?;
    Ok(QueryProposalResponse {
        proposal: res.proposal,
    })
}

fn query_proposals(
    deps: Deps,
    proposal_status: i32,
    voter: String,
    depositor: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryProposalsResponse> {
    let gq = GovQuerier::new(&deps.querier);
    let res = gq.proposals(proposal_status, voter, depositor, pagination)?;
    Ok(QueryProposalsResponse {
        proposals: res.proposals,
        pagination: res.pagination,
    })
}

fn query_tally_result(deps: Deps, proposal_id: u64) -> StdResult<QueryTallyResultResponse> {
    let gq = GovQuerier::new(&deps.querier);
    let res = gq.tally_result(proposal_id)?;
    Ok(QueryTallyResultResponse { tally: res.tally })
}

fn query_params(deps: Deps, param_types: String) -> StdResult<QueryParamsResponse> {
    let gq = GovQuerier::new(&deps.querier);
    let res = gq.params(param_types)?;
    Ok(QueryParamsResponse {
        voting_params: res.voting_params,
        deposit_params: res.deposit_params,
        tally_params: res.tally_params,
    })
}
