use ic_stable_structures::memory_manager::MemoryId;

use types::models::WhitelistRequest;

use super::{
    RequestStorage, StaticStorageRef, Storage, StorageInsertable, StorageQueryable,
    StorageUpdateable, WHITELIST_REQUESTS, WHITELIST_REQUESTS_MEMORY_ID,
};

pub struct WhitelistRequestStorage;

impl Storage<u64, WhitelistRequest> for WhitelistRequestStorage {
    const NAME: &'static str = "whitelist_requests";

    fn storage() -> StaticStorageRef<u64, WhitelistRequest> {
        &WHITELIST_REQUESTS
    }

    fn memory_id() -> MemoryId {
        WHITELIST_REQUESTS_MEMORY_ID
    }
}

impl StorageQueryable<u64, WhitelistRequest> for WhitelistRequestStorage {}
impl StorageInsertable<WhitelistRequest> for WhitelistRequestStorage {}
impl StorageUpdateable<u64, WhitelistRequest> for WhitelistRequestStorage {}
impl RequestStorage<WhitelistRequest> for WhitelistRequestStorage {}
