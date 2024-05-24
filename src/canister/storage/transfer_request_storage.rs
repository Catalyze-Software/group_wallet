use ic_stable_structures::memory_manager::MemoryId;

use types::models::TransferRequest;

use super::{
    RequestStorage, StaticStorageRef, Storage, StorageInsertable, StorageQueryable,
    StorageUpdateable, TRANSFER_REQUESTS, TRANSFER_REQUESTS_MEMORY_ID,
};

pub struct TransferRequestStorage;

impl Storage<u64, TransferRequest> for TransferRequestStorage {
    const NAME: &'static str = "transfer_requests";

    fn storage() -> StaticStorageRef<u64, TransferRequest> {
        &TRANSFER_REQUESTS
    }

    fn memory_id() -> MemoryId {
        TRANSFER_REQUESTS_MEMORY_ID
    }
}

impl StorageQueryable<u64, TransferRequest> for TransferRequestStorage {}
impl StorageInsertable<TransferRequest> for TransferRequestStorage {}
impl StorageUpdateable<u64, TransferRequest> for TransferRequestStorage {}
impl RequestStorage<TransferRequest> for TransferRequestStorage {}
