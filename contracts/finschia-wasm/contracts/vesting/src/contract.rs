#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use finschia_std::types::cosmos::base::v1beta1::Coin;
use finschia_std::types::cosmos::vesting::v1beta1::MsgCreateVestingAccount;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-stargate-vesting";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateVestingAccount {
            from_address,
            to_address,
            amount,
            end_time,
            delayed,
        } => try_create_vesting_account(
            deps,
            info,
            from_address,
            to_address,
            amount,
            end_time,
            delayed,
        ),
    }
}

pub fn try_create_vesting_account(
    _deps: DepsMut,
    _info: MessageInfo,
    from_address: String,
    to_address: String,
    amount: Vec<Coin>,
    end_time: i64,
    delayed: bool,
) -> Result<Response, ContractError> {
    let msg_create_vesting_account: CosmosMsg = MsgCreateVestingAccount {
        from_address,
        to_address,
        amount,
        end_time,
        delayed,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_create_vesting_account")
        .add_message(msg_create_vesting_account))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    to_binary(&"")
}
