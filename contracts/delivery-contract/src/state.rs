use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

/// The `id_in_consignee` represents the  item_id.
/// `0` means that it has not consigned.
///
/// The `addr` represents address of consignee.
#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id_in_consignee: u32,
    pub addr: Addr,
}
