use ic_stable_structures::memory_manager::MemoryId;

use crate::models::AirdropTransfers;

use super::{
    StaticStorageRef, Storage, StorageInsertableByKey, StorageQueryable, StorageUpdateable,
    AIRDROP_TRANSFERS, AIRDROP_TRANSFERS_MEMORY_ID,
};

pub struct AirdropTransferStorage;

impl Storage<u64, AirdropTransfers> for AirdropTransferStorage {
    const NAME: &'static str = "airdrop_transfer";

    fn storage() -> StaticStorageRef<u64, AirdropTransfers> {
        &AIRDROP_TRANSFERS
    }

    fn memory_id() -> MemoryId {
        AIRDROP_TRANSFERS_MEMORY_ID
    }
}

impl StorageQueryable<u64, AirdropTransfers> for AirdropTransferStorage {}
impl StorageInsertableByKey<u64, AirdropTransfers> for AirdropTransferStorage {}
impl StorageUpdateable<u64, AirdropTransfers> for AirdropTransferStorage {}
