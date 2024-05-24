use candid::Principal;
use ic_cdk::{caller, query, update};
use icrc_ledger_types::icrc1::transfer::TransferArg;

use crate::{
    helpers::guards::is_whitelisted, logic::transfer_logic::TransferLogic, result::CanisterResult,
};
use types::{Status, TransferRequestEntry, Vote};

#[query]
pub fn get_transfer_requests(status: Option<Status>) -> Vec<TransferRequestEntry> {
    TransferLogic::get_requests(status)
}

#[update(guard = "is_whitelisted")]
pub async fn transfer_request(
    canister_id: Principal,
    arg: TransferArg,
) -> CanisterResult<TransferRequestEntry> {
    TransferLogic::request(caller(), canister_id, arg).await
}

#[update(guard = "is_whitelisted")]
pub fn vote_on_transfer_request(id: u64, vote: Vote) -> CanisterResult<TransferRequestEntry> {
    TransferLogic::vote_request(caller(), id, vote)
}

#[update(guard = "is_whitelisted")]
pub async fn execute_transfer_request(id: u64) -> CanisterResult<()> {
    TransferLogic::execute_request(id).await
}
