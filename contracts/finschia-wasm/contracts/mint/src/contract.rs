#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use finschia_std::types::cosmos::mint::v1beta1::{
    MintQuerier, QueryAnnualProvisionsResponse, QueryInflationResponse, QueryParamsResponse,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-stargate-mint";
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
        QueryMsg::Inflation {} => to_binary(&query_inflation(deps)?),
        QueryMsg::AnnualProvisions {} => to_binary(&query_annual_provisions(deps)?),
        QueryMsg::Params {} => to_binary(&query_params(deps)?),
    }
}

fn query_inflation(deps: Deps) -> StdResult<QueryInflationResponse> {
    let mq = MintQuerier::new(&deps.querier);
    let res = mq.inflation()?;
    Ok(QueryInflationResponse {
        inflation: res.inflation,
    })
}

fn query_annual_provisions(deps: Deps) -> StdResult<QueryAnnualProvisionsResponse> {
    let mq = MintQuerier::new(&deps.querier);
    let res = mq.annual_provisions()?;
    Ok(QueryAnnualProvisionsResponse {
        annual_provisions: res.annual_provisions,
    })
}

fn query_params(deps: Deps) -> StdResult<QueryParamsResponse> {
    let mq = MintQuerier::new(&deps.querier);
    let res = mq.params()?;
    Ok(QueryParamsResponse { params: res.params })
}
