use candid::{CandidType, Principal};
use icrc_ledger_types::icrc1::transfer::TransferArg;
use serde::Deserialize;

use crate::impl_storable_for;

use super::request::RequestDetails;
use super::Request;

impl_storable_for!(AirdropRequest);

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct AirdropRequest {
    pub canister_id: Principal,
    pub details: RequestDetails,
    pub transfer_args: Vec<TransferArg>,
}

impl Request for AirdropRequest {
    fn details(&self) -> &RequestDetails {
        &self.details
    }

    fn details_mut(&mut self) -> &mut RequestDetails {
        &mut self.details
    }
}

impl AirdropRequest {
    pub fn new(canister_id: Principal, transfer_args: Vec<TransferArg>) -> Self {
        Self {
            canister_id,
            details: RequestDetails::default(),
            transfer_args,
        }
    }
}

pub type AirdropRequestEntry = (u64, AirdropRequest);
