use candid::Principal;
use ic_cdk::{caller, init, query, update};

use crate::{
    logic::{WhitelistLogic, DAY_IN_NANOS},
    models::{
        AirdropRequest, AirdropTransaction, Status, TransactionRequest, TransferRequestType, Vote,
        WhitelistRequest, WhitelistRequestEntry, WhitelistRequestKind,
    },
    result::CanisterResult,
};

// minimum 2 principal on init

// /////////////////
// // TRANSACTION //
// /////////////////
// #[update]
// async fn transaction_request(
//     canister_id: Principal,
//     request_type: TransferRequestType,
// ) -> Result<String, String> {
//     Store::transaction_request(caller(), canister_id, request_type).await
// }

// #[query]
// fn get_transaction_requests(status: Option<Status>) -> Vec<TransactionRequest> {
//     Store::get_transaction_requests(status)
// }

// #[update]
// async fn vote_on_transaction_request(request_id: u32, vote_type: Vote) -> Result<String, String> {
//     Store::vote_on_transaction_request(caller(), request_id, vote_type)
// }

// /////////////
// // AIRDROP //
// /////////////
// #[update]
// async fn airdrop_request(
//     canister_id: Principal,
//     transfer_args: Vec<TransferRequestType>,
// ) -> Result<String, String> {
//     Store::airdrop_request(caller(), transfer_args, canister_id).await
// }

// #[query]
// fn get_airdrop_requests(status: Option<Status>) -> Vec<AirdropRequest> {
//     Store::get_airdrop_requests(status)
// }

// #[query]
// fn get_airdrop_error(request_id: u32) -> Result<String, String> {
//     Store::get_airdrop_error(caller(), request_id)
// }

// #[update]
// async fn vote_on_airdrop_request(request_id: u32, vote_type: Vote) -> Result<String, String> {
//     Store::vote_on_airdrop_request(caller(), request_id, vote_type).await
// }

// #[query]
// fn get_airdrop_transactions(request_id: u32) -> Result<Vec<AirdropTransaction>, String> {
//     Store::get_airdrop_transactions(caller(), request_id)
// }
