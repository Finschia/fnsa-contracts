use cosmwasm_schema::cw_serde;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::cosmos::base::v1beta1::Coin;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    FundCommunityPool {
        amount: Vec<Coin>,
        depositor: String,
    },
    SetWithdrawAddress {
        delegator_address: String,
        withdraw_address: String,
    },
    WithdrawDelegatorReward {
        delegator_address: String,
        validator_address: String,
    },
    WithdrawValidatorCommission {
        validator_address: String,
    },
    Delegate {
        delegator_address: String,
        validator_address: String,
        amount: Option<Coin>,
    },
}

#[cw_serde]
pub enum QueryMsg {
    CommunityPool {},
    DelegationRewards {
        delegator_address: String,
        validator_address: String,
    },
    DelegationTotalRewards {
        delegator_address: String,
    },
    DelegatorValidators {
        delegator_address: String,
    },
    DelegatorWithdrawAddress {
        delegator_address: String,
    },
    ValidatorOutstandingRewards {
        validator_address: String,
    },
    ValidatorCommission {
        validator_address: String,
    },
    ValidatorSlashes {
        validator_address: String,
        starting_height: u64,
        ending_height: u64,
        pagination: Option<PageRequest>,
    },
    Params {},
}
