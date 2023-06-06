use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

/// The `item_id` starts with `1` and `0` is defined as the initial value
///
///  `addr` is address of subcontractor and `""` is defined as the initial value
#[derive(Serialize, Deserialize)]
pub struct Subcontractor {
    pub item_id: u32,
    pub addr: Addr,
}
