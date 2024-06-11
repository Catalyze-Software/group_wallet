use ic_cdk::{caller, query};

use crate::{
    helpers::guards::is_authorized, logic::airdrop_logic::AirdropLogic, result::CanisterResult,
};
use types::AirdropTransfers;

#[query(guard = "is_authorized")]
pub fn get_airdrop_transfers(proposal_id: u64) -> CanisterResult<AirdropTransfers> {
    AirdropLogic::get_transfers(caller(), proposal_id)
}
