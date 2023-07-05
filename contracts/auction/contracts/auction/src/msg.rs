use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    StartAuction(StartAuctionMsg),
    PlaceBid(PlaceBidMsg),
    EndAuction {},
}

#[cw_serde]
pub struct StartAuctionMsg {
    pub expiration_time: u64,
    pub cw721_address: Addr,
    pub token_id: String,
    pub start_bid: u64,
}

#[cw_serde]
pub struct PlaceBidMsg {
    pub bid: u64,
}

#[cw_serde]
pub enum QueryMsg {
    GetHighestBid {},
    GetAuctionItem {},
    GetAuctionHistory { idx: u32 },
}

#[cw_serde]
pub struct HighestBidResponse {
    pub highest_bid: u64,
    pub bidder: Addr,
}

#[cw_serde]
pub struct AuctionItemResponse {
    pub end_time: Timestamp,
    pub cw721_address: Addr,
    pub token_id: String,
    pub start_bid: u64,
}

#[cw_serde]
pub struct AuctionHistoryResponse {
    pub end_time: Timestamp,
    pub seller: Addr,
    pub cw721_address: Addr,
    pub token_id: String,
    pub highest_bid: u64,
    pub bidder: Addr,
}
