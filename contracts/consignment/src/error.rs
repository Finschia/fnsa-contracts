use cosmwasm_std::{Addr, StdError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Deserialize, Serialize)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("No item exists for that item_id:'{item_id}'")]
    NoItemExists { item_id: u32 },
    #[error("This item:'{item_id}' has already been consigned to '{consignee}'")]
    AlreadyConsignment { item_id: u32, consignee: Addr },
}
