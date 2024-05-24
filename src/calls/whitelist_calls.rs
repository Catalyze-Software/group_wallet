use candid::Principal;
use ic_cdk::{caller, query, update};

use crate::{
    logic::WhitelistLogic,
    models::{Status, Vote, WhitelistRequestEntry, WhitelistRequestKind},
    result::CanisterResult,
};

#[query]
pub fn get_whitelist() -> Vec<Principal> {
    WhitelistLogic::get_whitelist()
}

#[query]
pub fn get_whitelist_requests(status: Option<Status>) -> Vec<WhitelistRequestEntry> {
    WhitelistLogic::get_requests(status)
}

#[update]
pub fn whitelist_request(kind: WhitelistRequestKind) -> CanisterResult<WhitelistRequestEntry> {
    WhitelistLogic::request(caller(), kind)
}

#[update]
pub fn vote_on_whitelist_request(id: u64, vote: Vote) -> CanisterResult<WhitelistRequestEntry> {
    WhitelistLogic::vote_request(caller(), id, vote)
}
