use candid::{candid_method, Principal};
use ic_cdk::{caller, storage};
use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};

use crate::{
    logic::store::{Store, DATA},
    rust_declarations::types::{
        Status, TokenStandard, TransactionRequestData, TransferRequestType, VoteType,
        WhitelistRequestData, WhitelistRequestType,
    },
};

#[init]
#[candid_method(init)]
pub fn init(owner: Principal) {
    Store::init(owner);
}

#[pre_upgrade]
pub fn pre_upgrade() {
    DATA.with(|data| storage::stable_save((&*data.borrow(),)).unwrap());
}

#[post_upgrade]
pub fn post_upgrade() {
    let (old_store,): (Store,) = storage::stable_restore().unwrap();
    DATA.with(|data| *data.borrow_mut() = old_store);
}

#[query]
#[candid_method(query)]
fn get_token_list() -> Vec<(Principal, TokenStandard)> {
    Store::get_token_list()
}

#[update]
#[candid_method(update)]
fn add_token_from_list(canister_id: Principal, standard: TokenStandard) -> Result<(), String> {
    Store::add_token_to_list(caller(), canister_id, standard)
}

#[update]
#[candid_method(update)]
fn remove_token_from_list(canister_id: Principal) -> Result<(), String> {
    Store::remove_token_from_list(caller(), canister_id)
}

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use candid::export_service;
    export_service!();
    __export_service()
}

///////////////
// WHITELIST //
///////////////
#[update]
#[candid_method(update)]
fn whitelist_request(request_type: WhitelistRequestType) -> Result<String, String> {
    Store::whitelist_request(caller(), request_type)
}

#[query]
#[candid_method(query)]
fn get_whitelist_requests(status: Option<Status>) -> Vec<WhitelistRequestData> {
    Store::get_whitelist_requests(status)
}

#[update]
#[candid_method(update)]
fn vote_on_whitelist_request(request_id: u32, vote_type: VoteType) -> Result<String, String> {
    Store::vote_on_whitelist_request(caller(), request_id, vote_type)
}

#[query]
#[candid_method(query)]
fn get_whitelist() -> Vec<Principal> {
    Store::get_whitelist()
}

/////////////////
// TRANSACTION //
/////////////////
#[update]
#[candid_method(update)]
async fn transaction_request(
    canister_id: Principal,
    request_type: TransferRequestType,
) -> Result<String, String> {
    Store::transaction_request(caller(), canister_id, request_type).await
}

#[query]
#[candid_method(query)]
fn get_transaction_requests(status: Option<Status>) -> Vec<TransactionRequestData> {
    Store::get_transaction_requests(status)
}

#[update]
#[candid_method(update)]
async fn vote_on_transaction_request(
    request_id: u32,
    vote_type: VoteType,
) -> Result<String, String> {
    Store::vote_on_transaction_request(caller(), request_id, vote_type).await
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    use std::env;
    use std::fs::write;
    use std::path::PathBuf;

    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dir = dir.parent().unwrap().join("candid");
    write(dir.join(format!("multisig.did")), __export_did_tmp_()).expect("Write failed.");
}
