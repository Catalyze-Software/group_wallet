use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::impl_storable_for;

use super::{types::RequestDetails, Status, Vote};

impl_storable_for!(WhitelistRequest);

#[derive(CandidType, Deserialize, PartialEq, Eq, Clone)]
pub enum WhitelistRequestKind {
    Add(Principal),
    Remove(Principal),
}

impl WhitelistRequestKind {
    pub fn principal(&self) -> &Principal {
        match self {
            WhitelistRequestKind::Add(principal) => principal,
            WhitelistRequestKind::Remove(principal) => principal,
        }
    }
}

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct WhitelistRequest {
    pub kind: WhitelistRequestKind,
    pub details: RequestDetails,
}

impl WhitelistRequest {
    pub fn new(kind: WhitelistRequestKind) -> Self {
        Self {
            kind,
            details: RequestDetails::default(),
        }
    }

    pub fn add_vote(&mut self, caller: Principal, vote: Vote) {
        self.details.add_vote(caller, vote)
    }

    pub fn update_status(&mut self, status: Status) {
        self.details.update_status(status);
    }

    pub fn set_send_at(&mut self, send_at: u64) {
        self.details.set_send_at(send_at);
    }
}

pub type WhitelistRequestEntry = (u64, WhitelistRequest);
