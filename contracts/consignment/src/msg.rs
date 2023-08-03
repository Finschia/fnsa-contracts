use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Consign { item_id: u32, consignee: Addr },
    Mint {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(OwnerResponse)]
    TerminalOwner { item_id: u32 },
}

#[cw_serde]
pub struct OwnerResponse {
    pub owner: Addr,
}
