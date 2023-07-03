use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unauthorized {val1:?} {val2:?}")]
    Unauthorized2 { val1: Addr, val2: Addr },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("expiration time is too long: {val:?}")]
    ExpirationTimeError { val: u64 },

    #[error("transfer NFT is failed: sender:{sender:?}, token_id:{token_id:?}")]
    TransferNFTError { sender: Addr, token_id: String },

    #[error("another auction is progress")]
    AuctionProgressError {},

    #[error("auction is not progress")]
    AuctionNoProgressError {},

    #[error("bid is less than the highest bid: bid:{bid:?}, highest bid:{highest_bid:?}")]
    InvalidBidError { bid: u64, highest_bid: u64 },

    #[error("not yet expiration time for the auction to end")]
    AuctionTimeError {},

    #[error("insufficient balance")]
    InsufficientBalanceError {},
}
