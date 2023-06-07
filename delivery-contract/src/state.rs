use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

/// The `item_id` represents the subcontractor's item_id.
/// `0` means that it has not been forwarded to the subcontractor.
///
/// The `addr` represents address of subcontractor
#[derive(Serialize, Deserialize)]
pub struct Subcontractor {
    pub item_id: u32,
    pub addr: Addr,
}
