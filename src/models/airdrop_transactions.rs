use candid::CandidType;
use serde::Deserialize;

use crate::impl_storable_for;

use super::airdrop_transaction::AirdropTransaction;

impl_storable_for!(AirdropTransactions);

#[derive(CandidType, Deserialize, Clone)]
pub struct AirdropTransactions(Vec<AirdropTransaction>);
