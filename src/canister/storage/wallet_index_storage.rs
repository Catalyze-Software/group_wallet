use candid::Principal;
use ic_stable_structures::memory_manager::MemoryId;

use super::{CellStorage, CellStorageRef, WALLET_INDEX, WALLET_INDEX_MEMORY_ID};

pub struct WalletIndexStorage;

impl CellStorage<Principal> for WalletIndexStorage {
    const NAME: &'static str = "wallet_index";

    fn storage() -> CellStorageRef<Principal> {
        &WALLET_INDEX
    }

    fn memory_id() -> MemoryId {
        WALLET_INDEX_MEMORY_ID
    }
}
