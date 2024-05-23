use candid::Principal;
use ic_cdk::{caller, query, update};

use crate::{
    logic::WhitelistLogic,
    models::{Status, Vote, WhitelistRequest, WhitelistRequestEntry, WhitelistRequestKind},
    result::CanisterResult,
};

#[update]
fn whitelist_request(kind: WhitelistRequestKind) -> CanisterResult<WhitelistRequestEntry> {
    WhitelistLogic::request(caller(), kind)
}

#[query]
fn get_whitelist_requests(status: Option<Status>) -> Vec<WhitelistRequest> {
    WhitelistLogic::get_requests(status)
}

#[update]
fn vote_on_whitelist_request(id: u64, vote: Vote) -> CanisterResult<WhitelistRequestEntry> {
    WhitelistLogic::vote_request(caller(), id, vote)
}

#[query]
fn get_whitelist() -> Vec<Principal> {
    WhitelistLogic::get_whitelist()
}
