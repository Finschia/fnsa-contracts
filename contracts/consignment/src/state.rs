use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

/// This represents an Item hand having no meta info for simple.
///
/// The `id_in_consignee` represents the item_id in next consignee.
/// `0` means that it has not consigned and this contrat is the owner of it.
///
/// The `consignee` represents address of consignee.
#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id_in_consignee: u32,
    pub consignee: Addr,
}
