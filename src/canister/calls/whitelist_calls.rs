use crate::{
    helpers::guards::{is_authorized, is_owner},
    result::CanisterResult,
};
use candid::Principal;
use ic_cdk::{query, update};

use crate::logic::WhitelistLogic;

#[query(guard = "is_authorized")]
pub fn get_whitelist() -> CanisterResult<Vec<Principal>> {
    WhitelistLogic::get_whitelist()
}

#[update(guard = "is_owner")]
pub fn replace_whitelisted(whitelisted: Vec<Principal>) -> CanisterResult<Vec<Principal>> {
    WhitelistLogic::replace_whitelisted(whitelisted)
}
