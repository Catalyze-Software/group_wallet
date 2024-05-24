use candid::{CandidType, Principal};
use icrc_ledger_types::icrc1::transfer::TransferArg;
use serde::Deserialize;

use crate::impl_storable_for;

use super::{request::RequestDetails, Request};

impl_storable_for!(TransferRequest);

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct TransferRequest {
    pub args: TransferArg,
    pub canister_id: Principal,
    pub details: RequestDetails,
}

impl Request for TransferRequest {
    fn details(&self) -> &RequestDetails {
        &self.details
    }

    fn details_mut(&mut self) -> &mut RequestDetails {
        &mut self.details
    }
}

impl TransferRequest {
    pub fn new(canister_id: Principal, args: TransferArg) -> Self {
        Self {
            canister_id,
            args,
            details: RequestDetails::default(),
        }
    }
}

pub type TransferRequestEntry = (u64, TransferRequest);
