use candid::{CandidType, Nat, Principal};
use serde::Deserialize;

use crate::impl_storable_for;

use super::types::Status;

impl_storable_for!(AirdropTransaction);

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub enum TokenStandard {
    DIP20,
    ICRC1,
}

#[derive(CandidType, Deserialize, PartialEq, Eq, Clone)]
pub enum Amount {
    DIP20(u64),
    ICRC1(Nat),
}

#[derive(CandidType, Deserialize, Clone)]
pub struct AirdropTransaction {
    pub status: Status,
    pub receiver: Principal,
    pub amount: Amount,
    pub canister_id: Principal,
    pub token_standard: TokenStandard,
}
