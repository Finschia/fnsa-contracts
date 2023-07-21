use cosmwasm_std::{callable_points, Binary, Empty, StdResult};
use cw2::set_contract_version;
pub use cw721::OwnerOfResponse;
pub use cw721_base::{ContractError, InstantiateMsg, MintMsg, MinterResponse};

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:cw721-base-dynamiclink";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Extension = Option<Empty>;

pub type Cw721BaseDynamicLinkContract<'a> =
    cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let res =
            Cw721BaseDynamicLinkContract::default().instantiate(deps.branch(), env, info, msg)?;
        // Explicitly set contract name and version, otherwise set to cw721-base info
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(ContractError::Std)?;
        Ok(res)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw721BaseDynamicLinkContract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721BaseDynamicLinkContract::default().query(deps, env, msg)
    }
}

// These callable points do not cover all cw721 message/query.
// Although these should cover all of them, they implements only
// callable points used by auction contract.
// After we define the starndard interfaces for cw721 for dynamic link,
// we should remake them according to them.
#[callable_points]
mod callable_points {
    use super::*;

    use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo};

    #[callable_point]
    fn transfer_nft(deps: DepsMut, env: Env, recipient: String, token_id: String) -> bool {
        let info = MessageInfo {
            sender: deps.api.get_caller_addr().unwrap(),
            funds: vec![],
        };
        if let Ok(_) = Cw721BaseDynamicLinkContract::default()
            ._transfer_nft(deps, &env, &info, &recipient, &token_id)
        {
            return true;
        }
        false
    }

    #[callable_point]
    fn owner_of(
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<Binary> {
        let query_msg = QueryMsg::OwnerOf {
            token_id: token_id.clone(),
            include_expired: Some(include_expired),
        };
        Cw721BaseDynamicLinkContract::default().query(deps, env, query_msg)
    }

    #[callable_point]
    fn approval(
        deps: Deps,
        env: Env,
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    ) -> StdResult<Binary> {
        let query_msg = QueryMsg::Approval {
            token_id: token_id.clone(),
            spender: spender,
            include_expired: include_expired,
        };
        Cw721BaseDynamicLinkContract::default().query(deps, env, query_msg)
    }
}
