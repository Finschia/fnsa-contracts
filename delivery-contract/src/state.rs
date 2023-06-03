use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Subcontructor {
    pub item_id: u32,
    pub addr: Addr,
}
