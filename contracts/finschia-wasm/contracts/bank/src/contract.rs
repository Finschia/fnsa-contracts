#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use finschia_std::types::cosmos::bank::v1beta1::{
    BankQuerier, Input, MsgMultiSend, MsgSend, Output,
};
use finschia_std::types::cosmos::bank::v1beta1::{
    QueryAllBalancesResponse, QueryBalanceResponse, QueryDenomMetadataResponse,
    QueryDenomsMetadataResponse, QueryParamsResponse, QuerySpendableBalancesResponse,
    QuerySupplyOfResponse, QueryTotalSupplyResponse,
};
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::cosmos::base::v1beta1::Coin;

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
        ExecuteMsg::Send {
            from_address,
            to_address,
            amount,
        } => try_send(deps, info, from_address, to_address, amount),
        ExecuteMsg::MultiSend { inputs, outputs } => try_multi_send(deps, info, inputs, outputs),
    }
}

pub fn try_send(
    _deps: DepsMut,
    _info: MessageInfo,
    from_address: String,
    to_address: String,
    amount: Vec<Coin>,
) -> Result<Response, ContractError> {
    let msg_send: CosmosMsg = MsgSend {
        from_address,
        to_address,
        amount,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_send")
        .add_message(msg_send))
}

pub fn try_multi_send(
    _deps: DepsMut,
    _info: MessageInfo,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
) -> Result<Response, ContractError> {
    let msg_multi_send: CosmosMsg = MsgMultiSend { inputs, outputs }.into();

    Ok(Response::new()
        .add_attribute("method", "try_multi_send")
        .add_message(msg_multi_send))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address, denom } => to_binary(&query_balance(deps, address, denom)?),
        QueryMsg::AllBalances {
            address,
            pagination,
        } => to_binary(&query_all_balances(deps, address, pagination)?),
        QueryMsg::SpendableBalances {
            address,
            pagination,
        } => to_binary(&query_spendable_balances(deps, address, pagination)?),
        QueryMsg::SupplyOf { denom } => to_binary(&query_supply_of(deps, denom)?),
        QueryMsg::TotalSupply { pagination } => to_binary(&query_total_supply(deps, pagination)?),
        QueryMsg::DenomMetadata { denom } => to_binary(&query_denom_metadata(deps, denom)?),
        QueryMsg::DenomsMetadata { pagination } => {
            to_binary(&query_denoms_metadata(deps, pagination)?)
        }
        QueryMsg::Params {} => to_binary(&query_params(deps)?),
    }
}

fn query_balance(deps: Deps, address: String, denom: String) -> StdResult<QueryBalanceResponse> {
    let bq = BankQuerier::new(&deps.querier);
    let res = bq.balance(address, denom)?;
    Ok(QueryBalanceResponse {
        balance: res.balance,
    })
}

fn query_all_balances(
    deps: Deps,
    address: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryAllBalancesResponse> {
    let bq = BankQuerier::new(&deps.querier);
    let res = bq.all_balances(address, pagination)?;
    Ok(QueryAllBalancesResponse {
        balances: res.balances,
        pagination: res.pagination,
    })
}

fn query_spendable_balances(
    deps: Deps,
    address: String,
    pagination: Option<PageRequest>,
) -> StdResult<QuerySpendableBalancesResponse> {
    let bq = BankQuerier::new(&deps.querier);
    let res = bq.spendable_balances(address, pagination)?;
    Ok(QuerySpendableBalancesResponse {
        balances: res.balances,
        pagination: res.pagination,
    })
}

fn query_supply_of(deps: Deps, denom: String) -> StdResult<QuerySupplyOfResponse> {
    let bq = BankQuerier::new(&deps.querier);
    let res = bq.supply_of(denom)?;
    Ok(QuerySupplyOfResponse { amount: res.amount })
}

fn query_total_supply(
    deps: Deps,
    pagination: Option<PageRequest>,
) -> StdResult<QueryTotalSupplyResponse> {
    let bq = BankQuerier::new(&deps.querier);
    let res = bq.total_supply(pagination)?;
    Ok(QueryTotalSupplyResponse {
        supply: res.supply,
        pagination: res.pagination,
    })
}

fn query_denom_metadata(deps: Deps, denom: String) -> StdResult<QueryDenomMetadataResponse> {
    let bq = BankQuerier::new(&deps.querier);
    let res = bq.denom_metadata(denom)?;
    Ok(QueryDenomMetadataResponse {
        metadata: res.metadata,
    })
}

fn query_denoms_metadata(
    deps: Deps,
    pagination: Option<PageRequest>,
) -> StdResult<QueryDenomsMetadataResponse> {
    let bq = BankQuerier::new(&deps.querier);
    let res = bq.denoms_metadata(pagination)?;
    Ok(QueryDenomsMetadataResponse {
        metadatas: res.metadatas,
        pagination: res.pagination,
    })
}

fn query_params(deps: Deps) -> StdResult<QueryParamsResponse> {
    let bq = BankQuerier::new(&deps.querier);
    let res = bq.params()?;
    Ok(QueryParamsResponse { params: res.params })
}
