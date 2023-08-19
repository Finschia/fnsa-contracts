use cosmwasm_schema::cw_serde;
use finschia_std::shim::Any;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::cosmos::base::v1beta1::Coin;
use finschia_std::types::lbm::foundation::v1::{Censorship, MemberRequest};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    FundTreasury {
        from: String,
        amount: Vec<Coin>,
    },
    WithdrawFromTreasury {
        authority: String,
        to: String,
        amount: Vec<Coin>,
    },
    SubmitProposal {
        proposers: Vec<String>,
        metadata: String,
        messages: Vec<Any>,
        exec: i32,
    },
    WithdrawProposal {
        proposal_id: u64,
        address: String,
    },
    Vote {
        proposal_id: u64,
        voter: String,
        option: i32,
        metadata: String,
        exec: i32,
    },
    Exec {
        proposal_id: u64,
        signer: String,
    },
    UpdateMembers {
        authority: String,
        member_updates: Vec<MemberRequest>,
    },
    LeaveFoundation {
        address: String,
    },
    UpdateDecisionPolicy {
        authority: String,
        decision_policy: Option<Any>,
    },
    UpdateCensorship {
        authority: String,
        censorship: Option<Censorship>,
    },
    Grant {
        authority: String,
        grantee: String,
        authorization: Option<Any>,
    },
    Revoke {
        authority: String,
        grantee: String,
        msg_type_url: String,
    },
}

#[cw_serde]
pub enum QueryMsg {
    FoundationInfo {},
    Treasury {},
    Member {
        address: String,
    },
    Members {
        pagination: Option<PageRequest>,
    },
    Proposal {
        proposal_id: u64,
    },
    Proposals {
        pagination: Option<PageRequest>,
    },
    Vote {
        proposal_id: u64,
        voter: String,
    },
    Votes {
        proposal_id: u64,
        pagination: Option<PageRequest>,
    },
    TallyResult {
        proposal_id: u64,
    },
    Censorships {
        pagination: Option<PageRequest>,
    },
    Grants {
        grantee: String,
        msg_type_url: String,
        pagination: Option<PageRequest>,
    },
    Params {},
}
