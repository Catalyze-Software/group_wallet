use candid::{CandidType, Principal};
use ic_cdk::{api::time, caller};
use serde::Deserialize;

use super::{votes::Votes, Vote};

pub trait Request {
    fn details_mut(&mut self) -> &mut RequestDetails;

    fn details(&self) -> &RequestDetails;

    fn update_status(&mut self, status: Status) {
        self.details_mut().update_status(status);
    }

    fn set_sent_at(&mut self, sent_at: u64) {
        self.details_mut().set_sent_at(sent_at)
    }

    fn status(&self) -> Status {
        self.details().status.clone()
    }

    fn add_vote(&mut self, caller: Principal, vote: Vote) {
        self.details_mut().add_vote(caller, vote);
    }
}

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct RequestDetails {
    pub status: Status,
    pub votes: Votes,
    pub requested_by: Principal,
    pub sent_at: Option<u64>,
    pub created_at: u64,
}

impl Default for RequestDetails {
    fn default() -> Self {
        Self {
            status: Status::Pending,
            votes: Votes::default(),
            requested_by: caller(),
            sent_at: None,
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

    pub fn set_sent_at(&mut self, sent_at: u64) {
        self.sent_at = Some(sent_at);
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
