use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use crate::impl_storable_for;

impl_storable_for!(Metadata);

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Metadata {
    pub group_id: u64,
    pub proxy_canister: Principal,
    pub index_canister: Principal,
}

impl Metadata {
    pub fn new(group_id: u64, proxy_canister: Principal, index_canister: Principal) -> Self {
        Self {
            group_id,
            proxy_canister,
            index_canister,
        }
    }
}
