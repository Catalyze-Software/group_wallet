use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::impl_storable_for;

use super::{icrc_declaration::TransferArg, types::RequestDetails};

impl_storable_for!(TransactionRequest);

#[derive(CandidType, Deserialize, Clone)]
pub struct Dip20TransferArgs {
    pub to: Principal,
    pub amount: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum TransferRequestType {
    DIP20(Dip20TransferArgs),
    ICRC1(TransferArg),
}

#[derive(CandidType, Deserialize, Clone)]
pub struct TransactionRequest {
    pub args: TransferRequestType,
    pub canister_id: Principal,
    pub data: RequestDetails,
}
