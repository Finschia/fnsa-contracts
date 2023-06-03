use cosmwasm_std::{
    callable_points, dynamic_link, entry_point, from_slice, to_binary, to_vec, Addr, Binary,
    Contract, Deps, DepsMut, Env, MessageInfo, Order, Response,
};

use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, PlaceResponse, QueryMsg};
use crate::state::Subcontructor;

const BUCKET_KEY: &[u8] = b"bucket_items";
const COMPANY_PLACE: &[u8] = b"company_place";

#[derive(Contract)]
struct DeliveryContract {
    address: Addr,
}

#[dynamic_link(DeliveryContract)]
trait Delivery: Contract {
    fn deposit_item(&self) -> u32;
    fn trace_place(&self, item_id: u32) -> String;
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let _: Bucket<Subcontructor> = bucket(deps.storage, BUCKET_KEY);
    deps.storage.set(COMPANY_PLACE, &to_vec(&msg.place)?);
    let response = Response::default()
        .add_attribute("place", &msg.place)
        .add_attribute("bucket_name", "bucket_items");
    Ok(response)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ItemForwarding {
            item_id,
            forwarding_addr,
        } => do_item_forwarding(deps, item_id, forwarding_addr),
        ExecuteMsg::DepositItem {} => do_deposit_item(deps),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Place { item_id } => Ok(to_binary(&do_trace_place(deps, item_id)?)?),
    }
}

#[callable_points]
mod callable_points {
    use super::*;

    #[callable_point]
    fn deposit_item(deps: DepsMut, _env: Env) -> u32 {
        handle_deposite_item(deps)
    }

    #[callable_point]
    fn trace_place(deps: Deps, _env: Env, item_id: u32) -> String {
        handle_trace_place(deps, item_id).unwrap()
    }
}

fn count_bucket_items(deps: &DepsMut) -> u32 {
    let bucket_items: ReadonlyBucket<Subcontructor> = bucket_read(deps.storage, BUCKET_KEY);
    let mut iter = bucket_items.range(None, None, Order::Ascending);

    let mut count = 0;
    while let Some(_) = iter.next() {
        count += 1;
    }
    count
}

fn do_item_forwarding(
    deps: DepsMut,
    item_id: u32,
    forwarding_addr: Addr,
) -> Result<Response, ContractError> {
    let mut bucket_items: Bucket<Subcontructor> = bucket(deps.storage, BUCKET_KEY);
    let loaded_item = match bucket_items.load(&item_id.to_be_bytes()) {
        Ok(value) => value,
        Err(_err) => return Err(ContractError::NoItemExists { item_id: item_id }),
    };

    if loaded_item.item_id != 0 {
        return Err(ContractError::AlreadyMoved {
            item_id: loaded_item.item_id,
            forwarding_addr: loaded_item.addr,
        });
    }

    let contract = DeliveryContract {
        address: forwarding_addr.clone(),
    };
    let forwarding_item_id = contract.deposit_item();
    let sub_contructor = Subcontructor {
        item_id: forwarding_item_id,
        addr: forwarding_addr.clone(),
    };
    bucket_items.save(&item_id.to_be_bytes(), &sub_contructor)?;

    let response = Response::default()
        .add_attribute("forwarding_item_id", forwarding_item_id.to_string())
        .add_attribute("forwarding_addr", forwarding_addr);

    Ok(response)
}

fn do_deposit_item(deps: DepsMut) -> Result<Response, ContractError> {
    let item_id = handle_deposite_item(deps);
    let response = Response::default().add_attribute("item_id", item_id.to_string());
    Ok(response)
}

fn do_trace_place(deps: Deps, item_id: u32) -> Result<PlaceResponse, ContractError> {
    let place = handle_trace_place(deps, item_id).unwrap();
    Ok(PlaceResponse { place: place })
}

fn handle_deposite_item(deps: DepsMut) -> u32 {
    let item_id = count_bucket_items(&deps) + 1;
    let mut bucket_items: Bucket<Subcontructor> = bucket(deps.storage, BUCKET_KEY);
    let subcontructor_attr = Subcontructor {
        item_id: 0,
        addr: Addr::unchecked(""),
    };
    bucket_items
        .save(&item_id.to_be_bytes(), &subcontructor_attr)
        .unwrap();
    item_id
}

fn handle_trace_place(deps: Deps, item_id: u32) -> Result<String, ContractError> {
    let bucket_items: ReadonlyBucket<Subcontructor> = bucket_read(deps.storage, BUCKET_KEY);
    let loaded_item = match bucket_items.load(&item_id.to_be_bytes()) {
        Ok(value) => value,
        Err(_err) => return Err(ContractError::NoItemExists { item_id: item_id }),
    };
    if loaded_item.item_id == 0 {
        return Ok(from_slice(&deps.storage.get(COMPANY_PLACE).unwrap())?);
    }

    let contract = DeliveryContract {
        address: loaded_item.addr,
    };
    Ok(contract.trace_place(loaded_item.item_id))
}

// aaaaaaaa link14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf2vn8

// bbbbbb link1suhgf5svhu4usrurvxzlgn54ksxmn8gljarjtxqnapv8kjnp4nrshuxemw

// cccccc link1yyca08xqdgvjz0psg56z67ejh9xms6l436u8y58m82npdqqhmmtq6cjue5
