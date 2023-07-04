#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    dynamic_link, from_binary, to_binary, Addr, BankMsg, Binary, Coin, Contract, Deps, DepsMut,
    Empty, Env, MessageInfo, Response, StdResult, SubMsg, Timestamp, Uint128,
};
use cw2::set_contract_version;
use cw721::OwnerOfResponse;
use cw_utils::one_coin;

use crate::error::ContractError;
use crate::msg::{
    AuctionHistoryResponse, AuctionItemResponse, ExecuteMsg, HighestBidResponse, InstantiateMsg,
    PlaceBidMsg, QueryMsg, StartAuctionMsg,
};
use crate::state::{Bid, History, Mode, State, BID, HISTORIES, HISTORY_INDEX, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:auction";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const MAX_EXPIRATION_SECONDS: u64 = 604800; // 1 week(60 * 60 * 24 * 7)

pub type Extension = Option<Empty>;

#[derive(Contract)]
struct Cw721Contract {
    address: Addr,
}

#[dynamic_link(Cw721Contract)]
trait Cw721: Contract {
    fn transfer_nft(&self, recipient: Addr, token_id: String) -> bool;
    fn owner_of(&self, token_id: String, include_expired: bool) -> StdResult<Binary>;
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // reset auction state
    STATE.save(
        deps.storage,
        &State {
            mode: Mode::End,
            end_time: Timestamp::from_nanos(0),
            seller: Addr::unchecked(""),
            cw721_address: Addr::unchecked(""),
            token_id: String::from(""),
            start_bid: 0,
        },
    )?;

    // reset history index
    HISTORY_INDEX.save(deps.storage, &0)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::StartAuction(msg) => start_auction(deps, env, info, msg),
        ExecuteMsg::PlaceBid(msg) => place_bid(deps, env, info, msg),
        ExecuteMsg::EndAuction {} => end_auction(deps, env, info),
    }
}

pub fn start_auction(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: StartAuctionMsg,
) -> Result<Response, ContractError> {
    let s = STATE.load(deps.storage)?;
    if s.mode == Mode::Progress {
        return Err(ContractError::AuctionProgressError {});
    }

    if msg.expiration_time > MAX_EXPIRATION_SECONDS {
        return Err(ContractError::ExpirationTimeError {
            val: msg.expiration_time,
        });
    }

    // set start bid and seller
    BID.save(
        deps.storage,
        &Bid {
            highest_bid: msg.start_bid.clone(),
            bidder: info.sender.clone(),
        },
    )?;

    // check owner of nft and transfer it to contract
    let contract = Cw721Contract {
        address: msg.cw721_address.clone(),
    };
    let owner =
        from_binary::<OwnerOfResponse>(&contract.owner_of(msg.token_id.clone(), false)?)?.owner;
    if owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    let is_success = contract.transfer_nft(env.contract.address.clone(), msg.token_id.clone());
    if !is_success {
        return Err(ContractError::TransferNFTError {
            sender: info.sender,
            token_id: msg.token_id.clone(),
        });
    }

    // save auction state
    STATE.save(
        deps.storage,
        &State {
            mode: Mode::Progress,
            end_time: env.block.time.plus_seconds(msg.expiration_time),
            seller: info.sender.clone(),
            cw721_address: msg.cw721_address.clone(),
            token_id: msg.token_id.clone(),
            start_bid: msg.start_bid.clone(),
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "start_auction")
        .add_attribute("expiration_time", msg.expiration_time.to_string())
        .add_attribute("seller", info.sender)
        .add_attribute("cw721_address", msg.cw721_address)
        .add_attribute("token_id", msg.token_id)
        .add_attribute("start_bid", msg.start_bid.to_string()))
}

pub fn place_bid(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: PlaceBidMsg,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if state.mode != Mode::Progress {
        return Err(ContractError::AuctionNoProgressError {});
    }

    if state.end_time < env.block.time {
        return Err(ContractError::AuctionTimeError {});
    }

    let balance = deps.querier.query_balance(info.sender.clone(), "cony")?;
    if balance.amount < Uint128::from(msg.bid.clone()) {
        return Err(ContractError::InsufficientBalanceError {});
    }

    // update bid if it is higher than previous highest bid
    BID.update(deps.storage, |mut b| {
        if b.highest_bid >= msg.bid.clone() {
            return Err(ContractError::InvalidBidError {
                bid: msg.bid,
                highest_bid: b.highest_bid,
            });
        }

        b.highest_bid = msg.bid;
        b.bidder = info.sender.clone();
        Ok(b)
    })?;

    Ok(Response::new()
        .add_attribute("method", "place_bid")
        .add_attribute("bid", msg.bid.to_string())
        .add_attribute("bidder", info.sender))
}

pub fn end_auction(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if state.mode != Mode::Progress {
        return Err(ContractError::AuctionNoProgressError {});
    }

    if state.end_time.clone() >= env.block.time {
        return Err(ContractError::AuctionTimeError {});
    }

    // only highest bidder can end auction
    let bid = BID.load(deps.storage)?;
    if bid.bidder != info.sender.clone() {
        return Err(ContractError::Unauthorized {});
    }

    // reset auction state
    STATE.save(
        deps.storage,
        &State {
            mode: Mode::End,
            end_time: Timestamp::from_nanos(0),
            seller: Addr::unchecked(""),
            cw721_address: Addr::unchecked(""),
            token_id: String::from(""),
            start_bid: 0,
        },
    )?;

    if let Ok(coin) = one_coin(&info) {
        if coin.denom != "cony" || coin.amount < Uint128::from(bid.highest_bid.clone()) {
            return Err(ContractError::InsufficientBalanceError {});
        }
    } else {
        return Err(ContractError::FundsError {});
    }

    let bank_msg = BankMsg::Send {
        to_address: state.seller.to_string(),
        amount: vec![Coin {
            denom: String::from("cony"),
            amount: Uint128::from(bid.highest_bid),
        }],
    };

    // transfer nft to bidder or seller
    let contract = Cw721Contract {
        address: state.cw721_address.clone(),
    };
    let is_success = contract.transfer_nft(info.sender.clone(), state.token_id.clone());
    if !is_success {
        return Err(ContractError::TransferNFTError {
            sender: env.contract.address,
            token_id: state.token_id.clone(),
        });
    }

    // add auction history
    let idx = HISTORY_INDEX.load(deps.storage)?;
    HISTORIES.save(
        deps.storage,
        idx,
        &History {
            end_time: state.end_time,
            seller: state.seller,
            cw721_address: state.cw721_address,
            token_id: state.token_id,
            highest_bid: bid.highest_bid,
            bidder: bid.bidder.clone(),
        },
    )?;
    HISTORY_INDEX.save(deps.storage, &(idx + 1))?;

    Ok(Response::new()
        .add_submessage(SubMsg::new(bank_msg))
        .add_attribute("method", "end_auction")
        .add_attribute("highest_bid", bid.highest_bid.to_string())
        .add_attribute("bidder", bid.bidder))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetHighestBid {} => to_binary(&get_highest_bid(deps)?),
        QueryMsg::GetAuctionItem {} => to_binary(&get_auction_item(deps)?),
        QueryMsg::GetAuctionHistory { idx } => to_binary(&get_auction_history(deps, idx)?),
    }
}

fn get_highest_bid(deps: Deps) -> StdResult<HighestBidResponse> {
    let bid = BID.load(deps.storage)?;
    Ok(HighestBidResponse {
        highest_bid: bid.highest_bid,
        bidder: bid.bidder,
    })
}

fn get_auction_item(deps: Deps) -> StdResult<AuctionItemResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(AuctionItemResponse {
        end_time: state.end_time,
        cw721_address: state.cw721_address,
        token_id: state.token_id,
        start_bid: state.start_bid,
    })
}

fn get_auction_history(deps: Deps, idx: u32) -> StdResult<AuctionHistoryResponse> {
    let history = HISTORIES.load(deps.storage, idx)?;
    Ok(AuctionHistoryResponse {
        end_time: history.end_time,
        seller: history.seller,
        cw721_address: history.cw721_address,
        token_id: history.token_id,
        highest_bid: history.highest_bid,
        bidder: history.bidder,
    })
}
