use crate::{
    helpers::guards::{is_authorized, is_owner, is_wallet_index},
    result::CanisterResult,
};
use candid::Principal;
use ic_cdk::{query, update};
use types::WhitelistEntry;

use crate::logic::WhitelistLogic;

#[query(guard = "is_authorized")]
pub fn get_whitelist() -> Vec<Principal> {
    WhitelistLogic::get_whitelist()
}

#[query(guard = "is_authorized")]
pub fn get_owner() -> CanisterResult<WhitelistEntry> {
    WhitelistLogic::get_owner()
}

#[update(guard = "is_owner")]
pub fn add_whitelisted(principal: Principal) -> CanisterResult<WhitelistEntry> {
    WhitelistLogic::add(principal)
}

#[update(guard = "is_owner")]
pub fn remove_whitelisted(principal: Principal) -> CanisterResult<()> {
    WhitelistLogic::remove(principal)
}

#[update(guard = "is_owner")]
pub fn switch_whitelisted(from: Principal, to: Principal) -> CanisterResult<WhitelistEntry> {
    WhitelistLogic::switch_whitelisted(from, to)
}

#[update(guard = "is_wallet_index")]
pub fn set_owner(new_owner: Principal) -> CanisterResult<Principal> {
    WhitelistLogic::set_owner(new_owner)?;
    Ok(ic_cdk::id())
}
