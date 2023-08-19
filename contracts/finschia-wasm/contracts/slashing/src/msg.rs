use cosmwasm_schema::cw_serde;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Unjail { validator_addr: String },
}

#[cw_serde]
pub enum QueryMsg {
    SigningInfo { cons_address: String },
    SigningInfos { pagination: Option<PageRequest> },
    Params {},
}
