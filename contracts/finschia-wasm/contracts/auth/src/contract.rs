#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use finschia_std::types::cosmos::auth::v1beta1::{
    AuthQuerier, QueryAccountResponse, QueryAccountsResponse, QueryModuleAccountByNameResponse,
    QueryParamsResponse,
};
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-stargate-auth";
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
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Account { address } => to_binary(&query_account(deps, address)?),
        QueryMsg::Accounts { pagination } => to_binary(&query_accounts(deps, pagination)?),
        QueryMsg::ModuleAccountByName { name } => {
            to_binary(&query_module_account_by_name(deps, name)?)
        }
        QueryMsg::Params {} => to_binary(&query_params(deps)?),
    }
}

fn query_account(deps: Deps, address: String) -> StdResult<QueryAccountResponse> {
    let aq = AuthQuerier::new(&deps.querier);
    let res = aq.account(address)?;
    Ok(QueryAccountResponse {
        account: res.account,
    })
}

fn query_accounts(deps: Deps, pagination: Option<PageRequest>) -> StdResult<QueryAccountsResponse> {
    let aq = AuthQuerier::new(&deps.querier);
    let res = aq.accounts(pagination)?;
    Ok(QueryAccountsResponse {
        accounts: res.accounts,
        pagination: res.pagination,
    })
}

fn query_module_account_by_name(
    deps: Deps,
    name: String,
) -> StdResult<QueryModuleAccountByNameResponse> {
    let aq = AuthQuerier::new(&deps.querier);
    let res = aq.module_account_by_name(name)?;
    Ok(QueryModuleAccountByNameResponse {
        account: res.account,
    })
}

fn query_params(deps: Deps) -> StdResult<QueryParamsResponse> {
    let aq = AuthQuerier::new(&deps.querier);
    let res = aq.params()?;
    Ok(QueryParamsResponse { params: res.params })
}
