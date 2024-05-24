use ic_stable_structures::memory_manager::MemoryId;

use crate::models::AirdropRequest;

use super::{
    StaticStorageRef, Storage, StorageInsertable, StorageQueryable, RequestStorage,
    StorageUpdateable, AIRDROP_REQUESTS, AIRDROP_REQUESTS_MEMORY_ID,
};

pub struct AirdropRequestStorage;

impl Storage<u64, AirdropRequest> for AirdropRequestStorage {
    const NAME: &'static str = "airdrop_requests";

    fn storage() -> StaticStorageRef<u64, AirdropRequest> {
        &AIRDROP_REQUESTS
    }

    fn memory_id() -> MemoryId {
        AIRDROP_REQUESTS_MEMORY_ID
    }
}

impl StorageQueryable<u64, AirdropRequest> for AirdropRequestStorage {}
impl StorageInsertable<AirdropRequest> for AirdropRequestStorage {}
impl StorageUpdateable<u64, AirdropRequest> for AirdropRequestStorage {}
impl RequestStorage<AirdropRequest> for AirdropRequestStorage {}
