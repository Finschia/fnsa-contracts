use cosmwasm_schema::cw_serde;
use finschia_std::shim::Any;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    GrantAllowance {
        granter: String,
        grantee: String,
        allowance: Option<Any>,
    },
    RevokeAllowance {
        granter: String,
        grantee: String,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Allowance {
        granter: String,
        grantee: String,
    },
    Allowances {
        grantee: String,
        pagination: Option<PageRequest>,
    },
    AllowancesByGranter {
        granter: String,
        pagination: Option<PageRequest>,
    },
}
