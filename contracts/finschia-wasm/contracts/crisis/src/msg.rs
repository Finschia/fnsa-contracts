use cosmwasm_schema::cw_serde;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    VerifyInvariant {
        sender: String,
        invariant_module_name: String,
        invariant_route: String,
    },
}

#[cw_serde]
pub enum QueryMsg {
    SigningInfo { cons_address: String },
    SigningInfos { pagination: Option<PageRequest> },
    Params {},
}
