use candid::Principal;
use ic_cdk::{init, query};
use logic::DAY_IN_NANOS;

pub mod helpers;
pub mod logic;
pub mod result;
pub mod storage;

pub mod calls;

#[init]
pub fn init(_owner: Principal) {
    // TODO: Fix
    // Store::init(owner);
}

#[query]
fn get_time_out() -> u64 {
    DAY_IN_NANOS
}

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use crate::result::CanisterResult;
    use types::{AirdropTransfers, Content, ProposalEntry, Status, Vote, WhitelistEntry};

    use candid::export_service;
    export_service!();
    __export_service()
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    use std::env;
    use std::fs::write;
    use std::path::PathBuf;

    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dir = dir.parent().unwrap().join("candid");
    write(dir.join("multisig.did"), __export_did_tmp_()).expect("Write failed.");
}
