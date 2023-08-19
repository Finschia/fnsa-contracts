#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::cosmos::base::v1beta1::Coin;
use finschia_std::types::cosmos::staking::v1beta1::MsgDelegate;
use finschia_std::types::cosmos::distribution::v1beta1::{
    DistributionQuerier, MsgFundCommunityPool, MsgSetWithdrawAddress, MsgWithdrawDelegatorReward,
    MsgWithdrawValidatorCommission,
};
use finschia_std::types::cosmos::distribution::v1beta1::{
    QueryCommunityPoolResponse, QueryDelegationRewardsResponse,
    QueryDelegationTotalRewardsResponse, QueryDelegatorValidatorsResponse,
    QueryDelegatorWithdrawAddressResponse, QueryParamsResponse, QueryValidatorCommissionResponse,
    QueryValidatorOutstandingRewardsResponse, QueryValidatorSlashesResponse,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-stargate-distribution";
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
        ExecuteMsg::FundCommunityPool { amount, depositor } => {
            try_fund_community_pool(deps, info, amount, depositor)
        }
        ExecuteMsg::SetWithdrawAddress {
            delegator_address,
            withdraw_address,
        } => try_set_withdraw_address(deps, info, delegator_address, withdraw_address),
        ExecuteMsg::WithdrawDelegatorReward {
            delegator_address,
            validator_address,
        } => try_withdraw_delegator_reward(deps, info, delegator_address, validator_address),
        ExecuteMsg::WithdrawValidatorCommission { validator_address } => {
            try_withdraw_validator_commission(deps, info, validator_address)
        }
        ExecuteMsg::Delegate {
            delegator_address,
            validator_address,
            amount,
        } => try_delegate(deps, info, delegator_address, validator_address, amount),
    }
}

pub fn try_fund_community_pool(
    _deps: DepsMut,
    _info: MessageInfo,
    amount: Vec<Coin>,
    depositor: String,
) -> Result<Response, ContractError> {
    let msg_fund_community_pool: CosmosMsg = MsgFundCommunityPool { amount, depositor }.into();

    Ok(Response::new()
        .add_attribute("method", "try_fund_community_pool")
        .add_message(msg_fund_community_pool))
}

pub fn try_set_withdraw_address(
    _deps: DepsMut,
    _info: MessageInfo,
    delegator_address: String,
    withdraw_address: String,
) -> Result<Response, ContractError> {
    let msg_set_withdraw_address: CosmosMsg = MsgSetWithdrawAddress {
        delegator_address,
        withdraw_address,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_set_withdraw_address")
        .add_message(msg_set_withdraw_address))
}

pub fn try_withdraw_delegator_reward(
    _deps: DepsMut,
    _info: MessageInfo,
    delegator_address: String,
    validator_address: String,
) -> Result<Response, ContractError> {
    let msg_withdraw_delegator_reward: CosmosMsg = MsgWithdrawDelegatorReward {
        delegator_address,
        validator_address,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_withdraw_delegator_reward")
        .add_message(msg_withdraw_delegator_reward))
}

pub fn try_withdraw_validator_commission(
    _deps: DepsMut,
    _info: MessageInfo,
    validator_address: String,
) -> Result<Response, ContractError> {
    let msg_withdraw_validator_commission: CosmosMsg =
        MsgWithdrawValidatorCommission { validator_address }.into();

    Ok(Response::new()
        .add_attribute("method", "try_withdraw_validator_commission")
        .add_message(msg_withdraw_validator_commission))
}

pub fn try_delegate(
    _deps: DepsMut,
    _info: MessageInfo,
    delegator_address: String,
    validator_address: String,
    amount: Option<Coin>,
) -> Result<Response, ContractError> {
    let msg_delegate: CosmosMsg = MsgDelegate {
        delegator_address,
        validator_address,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_delegate")
        .add_message(msg_delegate))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::CommunityPool {} => to_binary(&query_community_pool(deps)?),
        QueryMsg::DelegationRewards {
            delegator_address,
            validator_address,
        } => to_binary(&query_delegation_rewards(
            deps,
            delegator_address,
            validator_address,
        )?),
        QueryMsg::DelegationTotalRewards { delegator_address } => {
            to_binary(&query_delegation_total_rewards(deps, delegator_address)?)
        }
        QueryMsg::DelegatorValidators { delegator_address } => {
            to_binary(&query_delegator_validators(deps, delegator_address)?)
        }
        QueryMsg::DelegatorWithdrawAddress { delegator_address } => {
            to_binary(&query_delegator_withdraw_address(deps, delegator_address)?)
        }
        QueryMsg::ValidatorOutstandingRewards { validator_address } => to_binary(
            &query_validator_outstanding_rewards(deps, validator_address)?,
        ),
        QueryMsg::ValidatorCommission { validator_address } => {
            to_binary(&query_validator_commission(deps, validator_address)?)
        }
        QueryMsg::ValidatorSlashes {
            validator_address,
            starting_height,
            ending_height,
            pagination,
        } => to_binary(&query_validator_slashes(
            deps,
            validator_address,
            starting_height,
            ending_height,
            pagination,
        )?),
        QueryMsg::Params {} => to_binary(&query_params(deps)?),
    }
}

fn query_community_pool(deps: Deps) -> StdResult<QueryCommunityPoolResponse> {
    let bq = DistributionQuerier::new(&deps.querier);
    let res = bq.community_pool()?;
    Ok(QueryCommunityPoolResponse { pool: res.pool })
}

fn query_delegation_rewards(
    deps: Deps,
    delegator_address: String,
    validator_address: String,
) -> StdResult<QueryDelegationRewardsResponse> {
    let bq = DistributionQuerier::new(&deps.querier);
    let res = bq.delegation_rewards(delegator_address, validator_address)?;
    Ok(QueryDelegationRewardsResponse {
        rewards: res.rewards,
    })
}

fn query_delegation_total_rewards(
    deps: Deps,
    delegator_address: String,
) -> StdResult<QueryDelegationTotalRewardsResponse> {
    let bq = DistributionQuerier::new(&deps.querier);
    let res = bq.delegation_total_rewards(delegator_address)?;
    Ok(QueryDelegationTotalRewardsResponse {
        rewards: res.rewards,
        total: res.total,
    })
}

fn query_delegator_validators(
    deps: Deps,
    delegator_address: String,
) -> StdResult<QueryDelegatorValidatorsResponse> {
    let bq = DistributionQuerier::new(&deps.querier);
    let res = bq.delegator_validators(delegator_address)?;
    Ok(QueryDelegatorValidatorsResponse {
        validators: res.validators,
    })
}

fn query_delegator_withdraw_address(
    deps: Deps,
    delegator_address: String,
) -> StdResult<QueryDelegatorWithdrawAddressResponse> {
    let bq = DistributionQuerier::new(&deps.querier);
    let res = bq.delegator_withdraw_address(delegator_address)?;
    Ok(QueryDelegatorWithdrawAddressResponse {
        withdraw_address: res.withdraw_address,
    })
}

fn query_validator_outstanding_rewards(
    deps: Deps,
    validator_address: String,
) -> StdResult<QueryValidatorOutstandingRewardsResponse> {
    let bq = DistributionQuerier::new(&deps.querier);
    let res = bq.validator_outstanding_rewards(validator_address)?;
    Ok(QueryValidatorOutstandingRewardsResponse {
        rewards: res.rewards,
    })
}

fn query_validator_commission(
    deps: Deps,
    validator_address: String,
) -> StdResult<QueryValidatorCommissionResponse> {
    let bq = DistributionQuerier::new(&deps.querier);
    let res = bq.validator_commission(validator_address)?;
    Ok(QueryValidatorCommissionResponse {
        commission: res.commission,
    })
}

fn query_validator_slashes(
    deps: Deps,
    validator_address: String,
    starting_height: u64,
    ending_height: u64,
    pagination: Option<PageRequest>,
) -> StdResult<QueryValidatorSlashesResponse> {
    let bq = DistributionQuerier::new(&deps.querier);
    let res = bq.validator_slashes(
        validator_address,
        starting_height,
        ending_height,
        pagination,
    )?;
    Ok(QueryValidatorSlashesResponse {
        slashes: res.slashes,
        pagination: res.pagination,
    })
}

fn query_params(deps: Deps) -> StdResult<QueryParamsResponse> {
    let bq = DistributionQuerier::new(&deps.querier);
    let res = bq.params()?;
    Ok(QueryParamsResponse { params: res.params })
}
