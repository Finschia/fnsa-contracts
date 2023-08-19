#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use finschia_std::shim::Any;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::cosmos::evidence::v1beta1::{EvidenceQuerier, MsgSubmitEvidence};
use finschia_std::types::cosmos::evidence::v1beta1::{
    QueryAllEvidenceResponse, QueryEvidenceResponse,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-stargate-slashing";
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
        ExecuteMsg::SubmitEvidence {
            submitter,
            evidence,
        } => try_submit_evidence(deps, info, submitter, evidence),
    }
}

pub fn try_submit_evidence(
    _deps: DepsMut,
    _info: MessageInfo,
    submitter: String,
    evidence: Option<Any>,
) -> Result<Response, ContractError> {
    let msg_submit_evidence: CosmosMsg = MsgSubmitEvidence {
        submitter,
        evidence,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_submit_evidence")
        .add_message(msg_submit_evidence))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Evidence { evidence_hash } => to_binary(&query_evidence(deps, evidence_hash)?),
        QueryMsg::AllEvidence { pagination } => to_binary(&query_all_evidence(deps, pagination)?),
    }
}

fn query_evidence(deps: Deps, evidence_hash: Vec<u8>) -> StdResult<QueryEvidenceResponse> {
    let bq = EvidenceQuerier::new(&deps.querier);
    let res = bq.evidence(evidence_hash)?;
    Ok(QueryEvidenceResponse {
        evidence: res.evidence,
    })
}

fn query_all_evidence(
    deps: Deps,
    pagination: Option<PageRequest>,
) -> StdResult<QueryAllEvidenceResponse> {
    let bq = EvidenceQuerier::new(&deps.querier);
    let res = bq.all_evidence(pagination)?;
    Ok(QueryAllEvidenceResponse {
        evidence: res.evidence,
        pagination: res.pagination,
    })
}
