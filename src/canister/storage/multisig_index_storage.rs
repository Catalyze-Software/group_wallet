use candid::Principal;
use ic_stable_structures::memory_manager::MemoryId;

use super::{CellStorage, CellStorageRef, MULTISIG_INDEX, MULTISIG_INDEX_MEMORY_ID};

pub struct MultisigIndexStorage;

impl CellStorage<Principal> for MultisigIndexStorage {
    const NAME: &'static str = "multisig_index";

    fn storage() -> CellStorageRef<Principal> {
        &MULTISIG_INDEX
    }

    fn memory_id() -> MemoryId {
        MULTISIG_INDEX_MEMORY_ID
    }
}
