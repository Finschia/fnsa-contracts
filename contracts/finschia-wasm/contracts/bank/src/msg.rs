use cosmwasm_schema::cw_serde;
use finschia_std::types::cosmos::bank::v1beta1::{Input, Output};
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::cosmos::base::v1beta1::Coin;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Send {
        from_address: String,
        to_address: String,
        amount: Vec<Coin>,
    },
    MultiSend {
        inputs: Vec<Input>,
        outputs: Vec<Output>,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Balance {
        address: String,
        denom: String,
    },
    AllBalances {
        address: String,
        pagination: Option<PageRequest>,
    },
    SpendableBalances {
        address: String,
        pagination: Option<PageRequest>,
    },
    SupplyOf {
        denom: String,
    },
    TotalSupply {
        pagination: Option<PageRequest>,
    },
    DenomMetadata {
        denom: String,
    },
    DenomsMetadata {
        pagination: Option<PageRequest>,
    },
    Params {},
}
