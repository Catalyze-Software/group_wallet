use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::{impl_storable_for, Vote, VoteKind};

impl_storable_for!(Votes);

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct Votes(pub Vec<Vote>);

impl Votes {
    pub fn voted(&self, voter: &Principal) -> bool {
        self.0.iter().any(|v| &v.voter == voter)
    }

    pub fn add(&mut self, vote: Vote) {
        self.0.push(vote);
    }

    pub fn approvals(&self) -> usize {
        self.0
            .iter()
            .filter(|v| v.kind == VoteKind::Approve)
            .count()
    }

    pub fn rejections(&self) -> usize {
        self.0.iter().filter(|v| v.kind == VoteKind::Reject).count()
    }
}

pub type VotesEntry = (u64, Votes);
