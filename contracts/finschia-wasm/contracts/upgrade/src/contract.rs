#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use finschia_std::types::cosmos::upgrade::v1beta1::{
    QueryCurrentPlanResponse, QueryAppliedPlanResponse, QueryModuleVersionsResponse, UpgradeQuerier,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-stargate-upgrade";
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
        QueryMsg::CurrentPlan {} => to_binary(&query_current_plan(deps)?),
        QueryMsg::AppliedPlan { name } => to_binary(&query_applied_plan(deps, name)?),
        QueryMsg::ModuleVersions { module_name } => to_binary(&query_module_versions(deps, module_name)?),
    }
}

fn query_current_plan(deps: Deps) -> StdResult<QueryCurrentPlanResponse> {
    let uq = UpgradeQuerier::new(&deps.querier);
    let res = uq.current_plan()?;
    Ok(QueryCurrentPlanResponse { plan: res.plan })
}

fn query_applied_plan(deps: Deps, name: String) -> StdResult<QueryAppliedPlanResponse> {
    let uq = UpgradeQuerier::new(&deps.querier);
    let res = uq.applied_plan(name)?;
    Ok(QueryAppliedPlanResponse { height: res.height })
}

fn query_module_versions(deps: Deps, module_name: String) -> StdResult<QueryModuleVersionsResponse> {
    let uq = UpgradeQuerier::new(&deps.querier);
    let res = uq.module_versions(module_name)?;
    Ok(QueryModuleVersionsResponse { module_versions: res.module_versions })
}
