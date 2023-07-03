use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use auction::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, StartAuctionMsg, PlaceBidMsg, HighestBidResponse, AuctionItemResponse, AuctionHistoryResponse};
use auction::state::{State, Bid, History};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(StartAuctionMsg), &out_dir);
    export_schema(&schema_for!(PlaceBidMsg), &out_dir);
    export_schema(&schema_for!(HighestBidResponse), &out_dir);
    export_schema(&schema_for!(AuctionItemResponse), &out_dir);
    export_schema(&schema_for!(AuctionHistoryResponse), &out_dir);
    export_schema(&schema_for!(State), &out_dir);
    export_schema(&schema_for!(Bid), &out_dir);
    export_schema(&schema_for!(History), &out_dir);
}
