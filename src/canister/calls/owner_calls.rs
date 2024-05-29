use candid::Principal;
use ic_cdk::{query, update};

use crate::{
    helpers::guards::{is_authorized, is_wallet_index},
    logic::OwnerLogic,
    result::CanisterResult,
};

#[query(guard = "is_authorized")]
pub fn get_owner() -> CanisterResult<Principal> {
    OwnerLogic::get()
}

#[update(guard = "is_wallet_index")]
pub fn set_owner(new_owner: Principal) -> CanisterResult<Principal> {
    OwnerLogic::set(new_owner)?;
    Ok(ic_cdk::id())
}
