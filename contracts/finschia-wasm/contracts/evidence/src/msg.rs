use cosmwasm_schema::cw_serde;
use finschia_std::shim::Any;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    SubmitEvidence {
        submitter: String,
        evidence: Option<Any>,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Evidence { evidence_hash: Vec<u8> },
    AllEvidence { pagination: Option<PageRequest> },
}
