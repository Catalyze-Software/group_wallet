use ic_cdk::{caller, query};

use crate::{logic::airdrop_logic::AirdropLogic, result::CanisterResult};
use types::AirdropTransfers;

#[query]
pub fn get_airdrop_transfers(id: u64) -> CanisterResult<AirdropTransfers> {
    AirdropLogic::get_transfers(caller(), id)
}
