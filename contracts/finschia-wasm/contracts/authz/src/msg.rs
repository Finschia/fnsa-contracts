use cosmwasm_schema::cw_serde;
use finschia_std::shim::Any;
use finschia_std::types::cosmos::authz::v1beta1::Grant;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Exec {
        grantee: String,
        msgs: Vec<Any>,
    },
    Grant {
        granter: String,
        grantee: String,
        grant: Option<Grant>,
    },
    Revoke {
        granter: String,
        grantee: String,
        msg_type_url: String,
    },
}

#[cw_serde]
pub enum QueryMsg {
    GranterGrants {
        granter: String,
        pagination: Option<PageRequest>,
    },
    GranteeGrants {
        grantee: String,
        pagination: Option<PageRequest>,
    },
    Grants {
        granter: String,
        grantee: String,
        msg_type_url: String,
        pagination: Option<PageRequest>,
    },
}
