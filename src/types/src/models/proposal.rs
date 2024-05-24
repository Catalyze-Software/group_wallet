use crate::impl_storable_for;
use candid::{CandidType, Principal};
use icrc_ledger_types::icrc1::transfer::TransferArg;
use serde::Deserialize;

use super::{votes::Votes, Vote};

impl_storable_for!(Proposal);

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct Proposal {
    pub status: Status,
    pub votes: Votes,
    pub creator: Principal,
    pub sent_at: Option<u64>,
    pub created_at: u64,
    pub content: Content,
}

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct AirdropProposalContent {
    pub canister_id: Principal,
    pub args: Vec<TransferArg>,
}

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct TransferProposalContent {
    pub canister_id: Principal,
    pub args: TransferArg,
}

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub enum Content {
    Airdrop(AirdropProposalContent),
    Transfer(TransferProposalContent),
}

impl Proposal {
    pub fn new(creator: Principal, content: Content) -> Self {
        Self {
            status: Status::Pending,
            votes: Votes::default(),
            creator,
            sent_at: None,
            created_at: ic_cdk::api::time(),
            content,
        }
    }
    pub fn status(&self) -> Status {
        self.status.clone()
    }

    pub fn add_vote(&mut self, caller: Principal, vote: Vote) {
        match vote {
            Vote::Approve => self.add_approve_vote(caller),
            Vote::Reject => self.add_reject_vote(caller),
        }
    }

    pub fn add_approve_vote(&mut self, caller: Principal) {
        if !self.votes.approvals.contains(&caller) && !self.votes.rejections.contains(&caller) {
            self.votes.approvals.push(caller);
        }
    }

    pub fn add_reject_vote(&mut self, caller: Principal) {
        if !self.votes.approvals.contains(&caller) && !self.votes.rejections.contains(&caller) {
            self.votes.rejections.push(caller);
        }
    }

    pub fn update_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn set_sent_at(&mut self, sent_at: u64) {
        self.sent_at = Some(sent_at);
    }
}

pub type ProposalEntry = (u64, Proposal);

#[derive(CandidType, Deserialize, PartialEq, Eq, Clone)]
pub enum Status {
    Pending,
    Approved,
    Rejected,
    Expired,
    Deadlock,
}
