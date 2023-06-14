use candid::{candid_method, Principal};
use ic_cdk::caller;
use ic_cdk_macros::{init, query, update};

use crate::{
    logic::store::Store,
    rust_declarations::types::{VoteType, WhitelistRequestType},
};

#[init]
#[candid_method(init)]
pub fn init(owner: Principal) {
    Store::init(owner);
}

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use candid::export_service;
    export_service!();
    __export_service()
}

#[update]
#[candid_method(update)]
fn whitelist_request(request_type: WhitelistRequestType) -> Result<(), String> {
    Store::whitelist_request(caller(), request_type)
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
