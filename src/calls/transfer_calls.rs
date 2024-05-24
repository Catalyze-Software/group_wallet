use candid::Principal;
use ic_cdk::{caller, query, update};

use crate::{
    logic::transfer_logic::TransferLogic,
    models::{Status, TransferArg, TransferRequestEntry, Vote},
    result::CanisterResult,
};

#[query]
pub fn get_transfer_requests(status: Option<Status>) -> Vec<TransferRequestEntry> {
    TransferLogic::get_requests(status)
}

#[update]
pub fn transfer_request(
    canister_id: Principal,
    arg: TransferArg,
) -> CanisterResult<TransferRequestEntry> {
    TransferLogic::request(caller(), canister_id, arg)
}

#[update]
pub fn vote_on_transfer_request(id: u64, vote: Vote) -> CanisterResult<TransferRequestEntry> {
    TransferLogic::vote_request(caller(), id, vote)
}

#[update]
pub async fn execute_transfer_request(id: u64) -> CanisterResult<()> {
    TransferLogic::execute_request(caller(), id).await
}
