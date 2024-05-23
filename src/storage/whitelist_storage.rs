use candid::Principal;
use ic_stable_structures::memory_manager::MemoryId;

use super::{
    StaticStorageRef, Storage, StorageInsertable, StorageQueryable, StorageUpdateable, WHITELIST,
    WHITELIST_MEMORY_ID,
};

pub struct WhitelistStorage;

impl Storage<u64, Principal> for WhitelistStorage {
    const NAME: &'static str = "whitelist";

    fn storage() -> StaticStorageRef<u64, Principal> {
        &WHITELIST
    }

    fn memory_id() -> MemoryId {
        WHITELIST_MEMORY_ID
    }
}

impl StorageQueryable<u64, Principal> for WhitelistStorage {}
impl StorageInsertable<Principal> for WhitelistStorage {}
impl StorageUpdateable<u64, Principal> for WhitelistStorage {}
