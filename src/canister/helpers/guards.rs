use candid::Principal;
use ic_cdk::caller;

use types::Error;

use crate::storage::{CellStorage, OwnerStorage, StorageQueryable, WhitelistStorage, MULTISIG_INDEX};

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

    if caller() == OwnerStorage::get().map_err(|_| "Failed to get owner")? {
        return Ok(());
    }

    Err(Error::unauthorized()
        .add_message("Principal is not whitelisted")
        .to_string())
}

pub fn is_owner() -> Result<(), String> {
    is_authorized()?;
    is_whitelisted()?;

    let owner = OwnerStorage::get().map_err(|_| "Failed to get owner")?;
    if caller() == owner {
        return Ok(());
    }

    Err(Error::unauthorized()
        .add_message("Principal is not the owner")
        .to_string())
}

pub fn is_wallet_index() -> Result<(), String> {
    is_authorized()?;

    let wallet_index = MULTISIG_INDEX
        .with(|w| {
            w.borrow()
                .get()
                .ok_or_else(|| Error::internal().add_message("Wallet index not set"))
        })
        .map_err(|e| e.to_string())?;

    if caller() == wallet_index {
        return Ok(());
    }

    Err(Error::unauthorized()
        .add_message("Principal is not the wallet index")
        .to_string())
}
