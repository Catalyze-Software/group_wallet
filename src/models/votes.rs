use candid::{CandidType, Principal};
use serde::Deserialize;

#[derive(CandidType, Deserialize, PartialEq, Eq)]
pub enum Vote {
    Approve,
    Reject,
}

#[derive(CandidType, Deserialize, PartialEq, Eq)]
pub enum VoteResponse {
    Approve,
    Reject,
    Deadlock,
    NotReached,
}

#[derive(CandidType, Default, Deserialize, Clone, PartialEq, Eq)]
pub struct Votes {
    pub approvals: Vec<Principal>,
    pub rejections: Vec<Principal>,
}
