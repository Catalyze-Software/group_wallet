use candid::Principal;
use ic_cdk::caller;

use types::Error;

use crate::storage::{
    metadata_storage::MetadataStorage, CellStorage, OwnerStorage, StorageQueryable,
    WhitelistStorage,
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

    let metadata = MetadataStorage::get().map_err(|_| "Failed to get metadata")?;
    if caller() == metadata.index_canister {
        return Ok(());
    }

    Err(Error::unauthorized()
        .add_message("Principal is not the wallet index")
        .to_string())
}
