use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub enum Mode {
    Progress,
    End,
}

#[cw_serde]
pub struct State {
    pub mode: Mode,
    pub end_time: Timestamp,
    pub seller: Addr,
    pub cw721_address: Addr,
    pub token_id: String,
    pub start_bid: u64,
}

#[cw_serde]
pub struct Bid {
    pub highest_bid: u64,
    pub bidder: Addr,
}

#[cw_serde]
pub struct History {
    pub end_time: Timestamp,
    pub seller: Addr,
    pub cw721_address: Addr,
    pub token_id: String,
    pub highest_bid: u64,
    pub bidder: Addr,
}

pub const STATE: Item<State> = Item::new("state");
pub const BID: Item<Bid> = Item::new("bid");
pub const HISTORIES: Map<u32, History> = Map::new("histories");
pub const HISTORY_INDEX: Item<u32> = Item::new("history_index");
