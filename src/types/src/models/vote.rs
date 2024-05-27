use candid::{CandidType, Principal};
use ic_cdk::api::time;
use serde::Deserialize;

use crate::impl_storable_for;

impl_storable_for!(Vote);

#[derive(CandidType, Deserialize, PartialEq, Eq, Clone)]
pub enum VoteKind {
    Approve,
    Reject,
}

#[derive(CandidType, Deserialize, PartialEq, Eq, Clone)]
pub struct Vote {
    pub voter: Principal,
    pub kind: VoteKind,
    pub created_at: u64,
}

impl Vote {
    pub fn new(voter: Principal, kind: VoteKind) -> Self {
        Self {
            voter,
            kind,
            created_at: time(),
        }
    }
}

#[derive(CandidType, Clone, Deserialize, PartialEq, Eq)]
pub enum TallyResult {
    Approve,
    Reject,
    Deadlock,
    NotReached,
}
