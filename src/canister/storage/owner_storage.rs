use candid::Principal;
use ic_stable_structures::memory_manager::MemoryId;

use super::{CellStorage, CellStorageRef, OWNER, OWNER_MEMORY_ID};

pub struct OwnerStorage;

impl CellStorage<Principal> for OwnerStorage {
    const NAME: &'static str = "owner";

    fn storage() -> CellStorageRef<Principal> {
        &OWNER
    }

    fn memory_id() -> MemoryId {
        OWNER_MEMORY_ID
    }
}
