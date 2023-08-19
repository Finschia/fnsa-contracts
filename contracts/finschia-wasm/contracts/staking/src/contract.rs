#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use finschia_std::shim::Any;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::cosmos::base::v1beta1::Coin;
use finschia_std::types::cosmos::staking::v1beta1::{
    CommissionRates, Description, MsgBeginRedelegate, MsgCreateValidator, MsgDelegate,
    MsgEditValidator, MsgUndelegate, StakingQuerier,
};
use finschia_std::types::cosmos::staking::v1beta1::{
    QueryDelegationResponse, QueryDelegatorDelegationsResponse,
    QueryDelegatorUnbondingDelegationsResponse, QueryDelegatorValidatorResponse,
    QueryDelegatorValidatorsResponse, QueryHistoricalInfoResponse, QueryParamsResponse,
    QueryPoolResponse, QueryRedelegationsResponse, QueryUnbondingDelegationResponse,
    QueryValidatorDelegationsResponse, QueryValidatorResponse,
    QueryValidatorUnbondingDelegationsResponse, QueryValidatorsResponse,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-stargate-bank";
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
        ExecuteMsg::CreateValidator {
            description,
            commission,
            min_self_delegation,
            delegator_address,
            validator_address,
            pubkey,
            value,
        } => try_create_validator(
            deps,
            info,
            description,
            commission,
            min_self_delegation,
            delegator_address,
            validator_address,
            pubkey,
            value,
        ),
        ExecuteMsg::EditValidator {
            description,
            validator_address,
            commission_rate,
            min_self_delegation,
        } => try_edit_validator(
            deps,
            info,
            description,
            validator_address,
            commission_rate,
            min_self_delegation,
        ),
        ExecuteMsg::Delegate {
            delegator_address,
            validator_address,
            amount,
        } => try_delegate(deps, info, delegator_address, validator_address, amount),
        ExecuteMsg::BeginRedelegate {
            delegator_address,
            validator_src_address,
            validator_dst_address,
            amount,
        } => try_begin_redelegate(
            deps,
            info,
            delegator_address,
            validator_src_address,
            validator_dst_address,
            amount,
        ),
        ExecuteMsg::Undelegate {
            delegator_address,
            validator_address,
            amount,
        } => try_undelegate(deps, info, delegator_address, validator_address, amount),
    }
}

pub fn try_create_validator(
    _deps: DepsMut,
    _info: MessageInfo,
    description: Option<Description>,
    commission: Option<CommissionRates>,
    min_self_delegation: String,
    delegator_address: String,
    validator_address: String,
    pubkey: Option<Any>,
    value: Option<Coin>,
) -> Result<Response, ContractError> {
    let msg_create_validator: CosmosMsg = MsgCreateValidator {
        description,
        commission,
        min_self_delegation,
        delegator_address,
        validator_address,
        pubkey,
        value,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_create_validator")
        .add_message(msg_create_validator))
}

pub fn try_edit_validator(
    _deps: DepsMut,
    _info: MessageInfo,
    description: Option<Description>,
    validator_address: String,
    commission_rate: String,
    min_self_delegation: String,
) -> Result<Response, ContractError> {
    let msg_edit_validator: CosmosMsg = MsgEditValidator {
        description,
        validator_address,
        commission_rate,
        min_self_delegation,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_edit_validator")
        .add_message(msg_edit_validator))
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

pub fn try_begin_redelegate(
    _deps: DepsMut,
    _info: MessageInfo,
    delegator_address: String,
    validator_src_address: String,
    validator_dst_address: String,
    amount: Option<Coin>,
) -> Result<Response, ContractError> {
    let msg_begin_redelegate: CosmosMsg = MsgBeginRedelegate {
        delegator_address,
        validator_src_address,
        validator_dst_address,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_begin_redelegate")
        .add_message(msg_begin_redelegate))
}

pub fn try_undelegate(
    _deps: DepsMut,
    _info: MessageInfo,
    delegator_address: String,
    validator_address: String,
    amount: Option<Coin>,
) -> Result<Response, ContractError> {
    let msg_undelegate: CosmosMsg = MsgUndelegate {
        delegator_address,
        validator_address,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_undelegate")
        .add_message(msg_undelegate))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Validator { validator_addr } => {
            to_binary(&query_validator(deps, validator_addr)?)
        }
        QueryMsg::Validators { status, pagination } => {
            to_binary(&query_validators(deps, status, pagination)?)
        }
        QueryMsg::ValidatorDelegations {
            validator_addr,
            pagination,
        } => to_binary(&query_validator_delegations(
            deps,
            validator_addr,
            pagination,
        )?),
        QueryMsg::ValidatorUnbondingDelegations {
            validator_addr,
            pagination,
        } => to_binary(&query_validator_unbonding_delegations(
            deps,
            validator_addr,
            pagination,
        )?),
        QueryMsg::Delegation {
            delegator_addr,
            validator_addr,
        } => to_binary(&query_delegation(deps, delegator_addr, validator_addr)?),
        QueryMsg::UnbondingDelegation {
            delegator_addr,
            validator_addr,
        } => to_binary(&query_unbonding_delegation(
            deps,
            delegator_addr,
            validator_addr,
        )?),
        QueryMsg::DelegatorDelegations {
            delegator_addr,
            pagination,
        } => to_binary(&query_delegator_delegations(
            deps,
            delegator_addr,
            pagination,
        )?),
        QueryMsg::DelegatorUnbondingDelegations {
            delegator_addr,
            pagination,
        } => to_binary(&query_delegator_unbonding_delegations(
            deps,
            delegator_addr,
            pagination,
        )?),
        QueryMsg::Redelegations {
            delegator_addr,
            src_validator_addr,
            dst_validator_addr,
            pagination,
        } => to_binary(&query_redelegations(
            deps,
            delegator_addr,
            src_validator_addr,
            dst_validator_addr,
            pagination,
        )?),
        QueryMsg::DelegatorValidator {
            delegator_addr,
            validator_addr,
        } => to_binary(&query_delegator_validator(
            deps,
            delegator_addr,
            validator_addr,
        )?),
        QueryMsg::DelegatorValidators {
            delegator_addr,
            pagination,
        } => to_binary(&query_delegator_validators(
            deps,
            delegator_addr,
            pagination,
        )?),
        QueryMsg::HistoricalInfo { height } => to_binary(&query_historical_info(deps, height)?),
        QueryMsg::Pool {} => to_binary(&query_pool(deps)?),
        QueryMsg::Params {} => to_binary(&query_params(deps)?),
    }
}

fn query_validator(deps: Deps, validator_addr: String) -> StdResult<QueryValidatorResponse> {
    let bq = StakingQuerier::new(&deps.querier);
    let res = bq.validator(validator_addr)?;
    Ok(QueryValidatorResponse {
        validator: res.validator,
    })
}

fn query_validators(
    deps: Deps,
    status: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryValidatorsResponse> {
    let bq = StakingQuerier::new(&deps.querier);
    let res = bq.validators(status, pagination)?;
    Ok(QueryValidatorsResponse {
        validators: res.validators,
        pagination: res.pagination,
    })
}

fn query_validator_delegations(
    deps: Deps,
    validator_addr: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryValidatorDelegationsResponse> {
    let bq = StakingQuerier::new(&deps.querier);
    let res = bq.validator_delegations(validator_addr, pagination)?;
    Ok(QueryValidatorDelegationsResponse {
        delegation_responses: res.delegation_responses,
        pagination: res.pagination,
    })
}

fn query_validator_unbonding_delegations(
    deps: Deps,
    validator_addr: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryValidatorUnbondingDelegationsResponse> {
    let bq = StakingQuerier::new(&deps.querier);
    let res = bq.validator_unbonding_delegations(validator_addr, pagination)?;
    Ok(QueryValidatorUnbondingDelegationsResponse {
        unbonding_responses: res.unbonding_responses,
        pagination: res.pagination,
    })
}

fn query_delegation(
    deps: Deps,
    delegator_addr: String,
    validator_addr: String,
) -> StdResult<QueryDelegationResponse> {
    let bq = StakingQuerier::new(&deps.querier);
    let res = bq.delegation(delegator_addr, validator_addr)?;
    Ok(QueryDelegationResponse {
        delegation_response: res.delegation_response,
    })
}

fn query_unbonding_delegation(
    deps: Deps,
    delegator_addr: String,
    validator_addr: String,
) -> StdResult<QueryUnbondingDelegationResponse> {
    let bq = StakingQuerier::new(&deps.querier);
    let res = bq.unbonding_delegation(delegator_addr, validator_addr)?;
    Ok(QueryUnbondingDelegationResponse { unbond: res.unbond })
}

fn query_delegator_delegations(
    deps: Deps,
    delegator_addr: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryDelegatorDelegationsResponse> {
    let bq = StakingQuerier::new(&deps.querier);
    let res = bq.delegator_delegations(delegator_addr, pagination)?;
    Ok(QueryDelegatorDelegationsResponse {
        delegation_responses: res.delegation_responses,
        pagination: res.pagination,
    })
}

fn query_delegator_unbonding_delegations(
    deps: Deps,
    delegator_addr: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryDelegatorUnbondingDelegationsResponse> {
    let bq = StakingQuerier::new(&deps.querier);
    let res = bq.delegator_unbonding_delegations(delegator_addr, pagination)?;
    Ok(QueryDelegatorUnbondingDelegationsResponse {
        unbonding_responses: res.unbonding_responses,
        pagination: res.pagination,
    })
}

fn query_redelegations(
    deps: Deps,
    delegator_addr: String,
    src_validator_addr: String,
    dst_validator_addr: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryRedelegationsResponse> {
    let bq = StakingQuerier::new(&deps.querier);
    let res = bq.redelegations(
        delegator_addr,
        src_validator_addr,
        dst_validator_addr,
        pagination,
    )?;
    Ok(QueryRedelegationsResponse {
        redelegation_responses: res.redelegation_responses,
        pagination: res.pagination,
    })
}

fn query_delegator_validator(
    deps: Deps,
    delegator_addr: String,
    validator_addr: String,
) -> StdResult<QueryDelegatorValidatorResponse> {
    let bq = StakingQuerier::new(&deps.querier);
    let res = bq.delegator_validator(delegator_addr, validator_addr)?;
    Ok(QueryDelegatorValidatorResponse {
        validator: res.validator,
    })
}

fn query_delegator_validators(
    deps: Deps,
    delegator_addr: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryDelegatorValidatorsResponse> {
    let bq = StakingQuerier::new(&deps.querier);
    let res = bq.delegator_validators(delegator_addr, pagination)?;
    Ok(QueryDelegatorValidatorsResponse {
        validators: res.validators,
        pagination: res.pagination,
    })
}

fn query_historical_info(deps: Deps, height: i64) -> StdResult<QueryHistoricalInfoResponse> {
    let bq = StakingQuerier::new(&deps.querier);
    let res = bq.historical_info(height)?;
    Ok(QueryHistoricalInfoResponse { hist: res.hist })
}

fn query_pool(deps: Deps) -> StdResult<QueryPoolResponse> {
    let bq = StakingQuerier::new(&deps.querier);
    let res = bq.pool()?;
    Ok(QueryPoolResponse { pool: res.pool })
}

fn query_params(deps: Deps) -> StdResult<QueryParamsResponse> {
    let bq = StakingQuerier::new(&deps.querier);
    let res = bq.params()?;
    Ok(QueryParamsResponse { params: res.params })
}
