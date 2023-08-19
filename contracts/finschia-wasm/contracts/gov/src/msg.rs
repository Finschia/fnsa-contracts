use cosmwasm_schema::cw_serde;
use finschia_std::shim::Any;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::cosmos::base::v1beta1::Coin;
use finschia_std::types::cosmos::gov::v1beta1::WeightedVoteOption;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Vote {
        proposal_id: u64,
        voter: String,
        option: i32,
    },
    VoteWeighted {
        proposal_id: u64,
        voter: String,
        options: Vec<WeightedVoteOption>,
    },
    Deposit {
        proposal_id: u64,
        depositor: String,
        amount: Vec<Coin>,
    },
    SubmitProposal {
        content: Option<Any>,
        initial_deposit: Vec<Coin>,
        proposer: String,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Vote {
        proposal_id: u64,
        voter: String,
    },
    Votes {
        proposal_id: u64,
        pagination: Option<PageRequest>,
    },
    Deposit {
        proposal_id: u64,
        depositor: String,
    },
    Deposits {
        proposal_id: u64,
        pagination: Option<PageRequest>,
    },
    Proposal {
        proposal_id: u64,
    },
    Proposals {
        proposal_status: i32,
        voter: String,
        depositor: String,
        pagination: Option<PageRequest>,
    },
    TallyResult {
        proposal_id: u64,
    },
    Params {
        param_types: String,
    },
}
