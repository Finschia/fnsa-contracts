use cosmwasm_schema::cw_serde;
use finschia_std::shim::Any;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::cosmos::base::v1beta1::Coin;
use finschia_std::types::cosmos::staking::v1beta1::{CommissionRates, Description};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateValidator {
        description: Option<Description>,
        commission: Option<CommissionRates>,
        min_self_delegation: String,
        delegator_address: String,
        validator_address: String,
        pubkey: Option<Any>,
        value: Option<Coin>,
    },
    EditValidator {
        description: Option<Description>,
        validator_address: String,
        commission_rate: String,
        min_self_delegation: String,
    },
    Delegate {
        delegator_address: String,
        validator_address: String,
        amount: Option<Coin>,
    },
    BeginRedelegate {
        delegator_address: String,
        validator_src_address: String,
        validator_dst_address: String,
        amount: Option<Coin>,
    },
    Undelegate {
        delegator_address: String,
        validator_address: String,
        amount: Option<Coin>,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Validator {
        validator_addr: String,
    },
    Validators {
        status: String,
        pagination: Option<PageRequest>,
    },
    ValidatorDelegations {
        validator_addr: String,
        pagination: Option<PageRequest>,
    },
    ValidatorUnbondingDelegations {
        validator_addr: String,
        pagination: Option<PageRequest>,
    },
    Delegation {
        delegator_addr: String,
        validator_addr: String,
    },
    UnbondingDelegation {
        delegator_addr: String,
        validator_addr: String,
    },
    DelegatorDelegations {
        delegator_addr: String,
        pagination: Option<PageRequest>,
    },
    DelegatorUnbondingDelegations {
        delegator_addr: String,
        pagination: Option<PageRequest>,
    },
    Redelegations {
        delegator_addr: String,
        src_validator_addr: String,
        dst_validator_addr: String,
        pagination: Option<PageRequest>,
    },
    DelegatorValidator {
        delegator_addr: String,
        validator_addr: String,
    },
    DelegatorValidators {
        delegator_addr: String,
        pagination: Option<PageRequest>,
    },
    HistoricalInfo {
        height: i64,
    },
    Pool {},
    Params {},
}
