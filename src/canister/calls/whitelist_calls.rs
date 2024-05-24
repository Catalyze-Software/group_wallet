use crate::{helpers::guards::is_owner, result::CanisterResult};
use candid::Principal;
use ic_cdk::{query, update};
use types::WhitelistEntry;

use crate::logic::WhitelistLogic;

#[query]
pub fn get_whitelist() -> Vec<Principal> {
    WhitelistLogic::get_whitelist()
}

#[update(guard = "is_owner")]
pub fn add_whitelisted(principal: Principal) -> CanisterResult<WhitelistEntry> {
    WhitelistLogic::add(principal)
}

#[update(guard = "is_owner")]
pub fn remote_whitelisted(principal: Principal) -> CanisterResult<()> {
    WhitelistLogic::remove(principal)
}
