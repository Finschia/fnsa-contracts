use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("No item exists for that item_id:'{item_id}'")]
    NoItemExists { item_id: u32 },
    #[error("This item:'{item_id}' has already been moved to forwarding_addr:'{forwarding_addr}'")]
    AlreadyMoved { item_id: u32, forwarding_addr: Addr },
}
