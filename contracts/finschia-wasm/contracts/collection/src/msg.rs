use cosmwasm_schema::cw_serde;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::lbm::collection::v1::{Attribute, Coin, MintNftParam};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub name: String,
    pub uri: String,
    pub meta: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    IssueNft {
        name: String,
        meta: String,
        owner: String,
    },
    MintNft {
        from: String,
        to: String,
        params: Vec<MintNftParam>,
    },
    SendNft {
        from: String,
        to: String,
        token_ids: Vec<String>,
    },
    BurnNft {
        from: String,
        token_ids: Vec<String>,
    },
    OperatorSendNft {
        operator: String,
        from: String,
        to: String,
        token_ids: Vec<String>,
    },
    OperatorBurnNft {
        operator: String,
        from: String,
        token_ids: Vec<String>,
    },
    IssueFt {
        name: String,
        meta: String,
        decimals: i32,
        mintable: bool,
        owner: String,
        to: String,
        amount: String,
    },
    MintFt {
        from: String,
        to: String,
        amount: Vec<Coin>,
    },
    SendFt {
        from: String,
        to: String,
        amount: Vec<Coin>,
    },
    BurnFt {
        from: String,
        amount: Vec<Coin>,
    },
    OperatorSendFt {
        operator: String,
        from: String,
        to: String,
        amount: Vec<Coin>,
    },
    OperatorBurnFt {
        operator: String,
        from: String,
        amount: Vec<Coin>,
    },
    Attach {
        from: String,
        token_id: String,
        to_token_id: String,
    },
    Detach {
        from: String,
        token_id: String,
    },
    OperatorAttach {
        operator: String,
        from: String,
        token_id: String,
        to_token_id: String,
    },
    OperatorDetach {
        operator: String,
        from: String,
        token_id: String,
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
        token_type: String,
        token_index: String,
        changes: Vec<Attribute>,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Contract {},
    NftMinted {
        token_type: String,
    },
    NftBurnt {
        token_type: String,
    },
    NftSupply {
        token_type: String,
    },
    FtMinted {
        token_id: String,
    },
    FtBurnt {
        token_id: String,
    },
    FtSupply {
        token_id: String,
    },
    Root {
        token_id: String,
    },
    HasParent {
        token_id: String,
    },
    Parent {
        token_id: String,
    },
    Children {
        token_id: String,
        pagination: Option<PageRequest>,
    },
    Balance {
        address: String,
        token_id: String,
    },
    AllBalance {
        address: String,
        pagination: Option<PageRequest>,
    },
    Token {
        token_id: String,
    },
    TokenType {
        token_type: String,
    },
    TokenClassTypeName {
        class_id: String,
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
