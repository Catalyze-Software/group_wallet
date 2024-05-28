use candid::Principal;
use ic_stable_structures::memory_manager::MemoryId;
use types::WhitelistEntry;

use crate::{logic::WHITELIST_OWNER_INDEX, result::CanisterResult};

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

impl WhitelistStorage {
    pub fn set_owner(owner: Principal) -> CanisterResult<WhitelistEntry> {
        WhitelistStorage::upsert(WHITELIST_OWNER_INDEX, owner)
    }

    pub fn get_owner() -> CanisterResult<WhitelistEntry> {
        WhitelistStorage::get(WHITELIST_OWNER_INDEX)
    }

    pub fn remove(id: u64) -> bool {
        Self::storage().with(|data| data.borrow_mut().remove(&id).is_some())
    }
}
