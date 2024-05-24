use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::impl_storable_for;

use super::{request::RequestDetails, Request};

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

impl Request for WhitelistRequest {
    fn details(&self) -> &RequestDetails {
        &self.details
    }

    fn details_mut(&mut self) -> &mut RequestDetails {
        &mut self.details
    }
}

impl WhitelistRequest {
    pub fn new(kind: WhitelistRequestKind) -> Self {
        Self {
            kind,
            details: RequestDetails::default(),
        }
    }

    pub fn set_send_at(&mut self, send_at: u64) {
        self.details.set_sent_at(send_at);
    }
}

pub type WhitelistRequestEntry = (u64, WhitelistRequest);
pub type WhitelistEntry = (u64, Principal);
