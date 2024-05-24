use candid::Principal;
use ic_cdk::{caller, query, update};

use crate::{
    helpers::guards::unsupported,
    logic::airdrop_logic::AirdropLogic,
    models::{AirdropRequestEntry, AirdropTransfers, Status, TransferArg, Vote},
    result::CanisterResult,
};

#[query(guard = "unsupported")]
pub fn get_airdrop_requests(status: Option<Status>) -> Vec<AirdropRequestEntry> {
    AirdropLogic::get_requests(status)
}

#[query(guard = "unsupported")]
pub fn get_airdrop_transfers(id: u64) -> CanisterResult<AirdropTransfers> {
    AirdropLogic::get_transfers(caller(), id)
}

#[update(guard = "unsupported")]
// #[query(guard = "is_whitelisted")]
pub async fn airdrop_request(
    canister_id: Principal,
    transfer_args: Vec<TransferArg>,
) -> CanisterResult<AirdropRequestEntry> {
    AirdropLogic::request(caller(), canister_id, transfer_args).await
}

#[update(guard = "unsupported")]
// #[query(guard = "is_whitelisted")]
pub fn vote_on_airdrop_request(id: u64, vote: Vote) -> CanisterResult<AirdropRequestEntry> {
    AirdropLogic::vote_request(caller(), id, vote)
}

#[update(guard = "unsupported")]
// #[query(guard = "is_whitelisted")]
pub async fn execute_airdrop_request(id: u64) -> CanisterResult<()> {
    AirdropLogic::execute_request(id).await
}
