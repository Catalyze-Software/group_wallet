use candid::{CandidType, Principal};
use serde::Deserialize;

use crate::impl_storable_for;

use super::transaction_request::TransferRequestType;
use super::types::RequestDetails;

impl_storable_for!(AirdropRequest);

#[derive(CandidType, Deserialize, Clone)]
pub struct AirdropRequest {
    pub tranfer_args: Vec<TransferRequestType>,
    pub canister_id: Principal,
    pub data: RequestDetails,
}
