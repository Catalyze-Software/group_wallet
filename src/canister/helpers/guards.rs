use candid::Principal;
use ic_cdk::caller;

use types::Error;

use crate::storage::{StorageQueryable, WhitelistStorage};

pub fn is_authorized() -> Result<(), String> {
    if caller() != Principal::anonymous() {
        return Ok(());
    }

    Err(Error::unauthorized()
        .add_message("Anonymous principal")
        .to_string())
}

pub fn is_whitelisted() -> Result<(), String> {
    is_authorized()?;

    if WhitelistStorage::contains(&caller()) {
        return Ok(());
    }

    Err(Error::unauthorized()
        .add_message("Principal is not whitelisted")
        .to_string())
}

pub fn is_owner() -> Result<(), String> {
    is_authorized()?;
    is_whitelisted()?;

    let (_, owner) = WhitelistStorage::get_owner().map_err(|_| "Failed to get owner")?;
    if caller() == owner {
        return Ok(());
    }

    Err(Error::unauthorized()
        .add_message("Principal is not the owner")
        .to_string())
}
