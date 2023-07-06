use cosmwasm_std::{
    callable_points, dynamic_link, entry_point, to_binary, Addr, Binary, Contract, Deps, DepsMut,
    Env, MessageInfo, Response,
};

use cosmwasm_storage::{bucket, bucket_read, singleton, Bucket, ReadonlyBucket};

use crate::error::ContractError;
use crate::msg::{ConsigneeResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::Item;

const BUCKET_KEY: &[u8] = b"bucket_items";
const NEXT_ITEM_ID: &[u8] = b"next_item_it";

#[derive(Contract)]
struct ConsigneeContract {
    address: Addr,
}

#[dynamic_link(ConsigneeContract)]
trait Consignee: Contract {
    fn get_consigned(&self) -> Result<u32, ContractError>;
    fn trace_terminal_consignee(&self, item_id: u32) -> Result<Addr, ContractError>;
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let _: Bucket<Item> = bucket(deps.storage, BUCKET_KEY);
    let mut next_id = singleton(deps.storage, NEXT_ITEM_ID);
    let first_id: u32 = 1;
    next_id.save(&first_id)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Consign { item_id, consignee } => handle_consign(deps, item_id, consignee),
        ExecuteMsg::Mint {} => handle_mint(deps),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::TerminalConsignee { item_id } => Ok(to_binary(&query_trace_terminal_consignee(
            deps, env, item_id,
        )?)?),
    }
}

#[callable_points]
mod callable_points {
    use super::*;

    // returns item id in this contract
    #[callable_point]
    fn get_consigned(deps: DepsMut, _env: Env) -> Result<u32, ContractError> {
        mint(deps)
    }

    // returns the terminal consignee
    #[callable_point]
    fn trace_terminal_consignee(deps: Deps, env: Env, item_id: u32) -> Result<Addr, ContractError> {
        super::trace_terminal_consignee(deps, env, item_id)
    }
}

fn handle_consign(deps: DepsMut, item_id: u32, consignee: Addr) -> Result<Response, ContractError> {
    let mut bucket_items: Bucket<Item> = bucket(deps.storage, BUCKET_KEY);
    let loaded_item = match bucket_items.load(&item_id.to_be_bytes()) {
        Ok(value) => value,
        Err(_err) => return Err(ContractError::NoItemExists { item_id: item_id }),
    };

    if loaded_item.id_in_consignee != 0 {
        return Err(ContractError::AlreadyConsignment {
            item_id: loaded_item.id_in_consignee,
            consignee: loaded_item.consignee,
        });
    }

    let contract = ConsigneeContract {
        address: consignee.clone(),
    };
    let id_in_consignee = contract.get_consigned()?;
    let subcontractor_attr = Item {
        id_in_consignee,
        consignee: consignee.clone(),
    };
    bucket_items.save(&item_id.to_be_bytes(), &subcontractor_attr)?;

    let response = Response::default()
        .add_attribute("id_in_consignee", id_in_consignee.to_string())
        .add_attribute("consignee", consignee.to_string());

    Ok(response)
}

fn handle_mint(deps: DepsMut) -> Result<Response, ContractError> {
    let _ = mint(deps)?;
    Ok(Response::default())
}

fn query_trace_terminal_consignee(
    deps: Deps,
    env: Env,
    item_id: u32,
) -> Result<ConsigneeResponse, ContractError> {
    let consignee = trace_terminal_consignee(deps, env, item_id)?;
    Ok(ConsigneeResponse { consignee })
}

fn mint(deps: DepsMut) -> Result<u32, ContractError> {
    let mut next_id = singleton(deps.storage, NEXT_ITEM_ID);
    let item_id: u32 = next_id.load()?;
    let next_item_id: u32 = item_id + 1;
    next_id.save(&next_item_id)?;
    let mut bucket_items: Bucket<Item> = bucket(deps.storage, BUCKET_KEY);
    let item = Item {
        id_in_consignee: 0,
        consignee: Addr::unchecked(""),
    };
    bucket_items.save(&item_id.to_be_bytes(), &item)?;
    deps.api.add_attribute("item_id", &item_id.to_string())?;
    Ok(item_id)
}

fn trace_terminal_consignee(deps: Deps, env: Env, item_id: u32) -> Result<Addr, ContractError> {
    let bucket_items: ReadonlyBucket<Item> = bucket_read(deps.storage, BUCKET_KEY);
    let loaded_item = match bucket_items.load(&item_id.to_be_bytes()) {
        Ok(value) => value,
        Err(_err) => return Err(ContractError::NoItemExists { item_id: item_id }),
    };
    if loaded_item.id_in_consignee == 0 {
        return Ok(env.contract.address);
    }

    let consignee = ConsigneeContract {
        address: loaded_item.consignee,
    };
    consignee.trace_terminal_consignee(loaded_item.id_in_consignee)
}
