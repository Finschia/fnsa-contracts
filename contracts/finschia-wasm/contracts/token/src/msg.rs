use cosmwasm_schema::cw_serde;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::lbm::token::v1::Attribute;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub meta: String,
    pub decimals: i32,
    pub mintable: bool,
    pub owner: String,
    pub to: String,
    pub amount: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint {
        from: String,
        to: String,
        amount: String,
    },
    Burn {
        from: String,
        amount: String,
    },
    Send {
        from: String,
        to: String,
        amount: String,
    },
    OperatorBurn {
        operator: String,
        from: String,
        amount: String,
    },
    OperatorSend {
        operator: String,
        from: String,
        to: String,
        amount: String,
    },
    AuthorizeOperator {
        holder: String,
        operator: String,
    },
    RevokeOperator {
        holder: String,
        operator: String,
    },
    GrantPermission {
        from: String,
        to: String,
        permission: String,
    },
    RevokePermission {
        from: String,
        permission: String,
    },
    Modify {
        owner: String,
        changes: Vec<Attribute>,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Contract {},
    Minted {},
    Burnt {},
    Supply {},
    Balance {
        address: String,
    },
    GranteeGrants {
        grantee: String,
        pagination: Option<PageRequest>,
    },
    IsOperatorFor {
        operator: String,
        holder: String,
    },
    HoldersByOperator {
        operator: String,
        pagination: Option<PageRequest>,
    },
}
