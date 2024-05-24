use candid::{CandidType, Nat, Principal};
use serde::Deserialize;

use crate::impl_storable_for;

use super::request::Status;

impl_storable_for!(AirdropTransfer);

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct AirdropTransfer {
    pub status: Status,
    pub receiver: Principal,
    pub amount: Nat,
    pub canister_id: Principal,
}
