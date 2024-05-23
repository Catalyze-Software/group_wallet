use candid::{CandidType, Principal};
use ic_cdk::{api::time, caller};
use serde::Deserialize;

use super::{votes::Votes, Vote};

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct RequestDetails {
    pub status: Status,
    pub votes: Votes,
    pub requested_by: Principal,
    pub send_at: Option<u64>,
    pub created_at: u64,
}

impl Default for RequestDetails {
    fn default() -> Self {
        Self {
            status: Status::Pending,
            votes: Votes::default(),
            requested_by: caller(),
            send_at: None,
            created_at: time(),
        }
    }
}

impl RequestDetails {
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

    pub fn set_send_at(&mut self, send_at: u64) {
        self.send_at = Some(send_at);
    }
}

#[derive(CandidType, Deserialize, PartialEq, Eq, Clone)]
pub enum Status {
    Pending,
    Approved,
    Rejected,
    Expired,
    Deadlock,
}
