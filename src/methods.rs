use candid::candid_method;
use ic_cdk_macros::{init, query};

#[init]
#[candid_method(init)]
pub fn init() {}

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use candid::export_service;
    export_service!();
    __export_service()
}

#[query]
#[candid_method(query)]
fn test() -> String {
    "test".to_string()
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
