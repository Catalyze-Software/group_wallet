use candid::Principal;
use ic_cdk::caller;

use crate::{
    models::Error,
    storage::{StorageQueryable, WhitelistStorage},
};

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

pub fn unsupported() -> Result<(), String> {
    Err(Error::unsupported()
        .add_message("This call is unsupported")
        .to_string())
}
