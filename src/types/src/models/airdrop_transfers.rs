use candid::CandidType;
use serde::Deserialize;

use crate::impl_storable_for;

use super::airdrop_transfer::AirdropTransfer;

impl_storable_for!(AirdropTransfers);

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct AirdropTransfers(pub Vec<AirdropTransfer>);

pub type AirdropTransfersEntry = (u64, AirdropTransfers);
