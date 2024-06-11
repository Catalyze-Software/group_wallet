use ic_stable_structures::memory_manager::MemoryId;
use types::Metadata;

use super::{CellStorage, CellStorageRef, METADATA, METADATA_MEMORY_ID};

pub struct MetadataStorage;

impl CellStorage<Metadata> for MetadataStorage {
    const NAME: &'static str = "metadata";

    fn storage() -> CellStorageRef<Metadata> {
        &METADATA
    }

    fn memory_id() -> MemoryId {
        METADATA_MEMORY_ID
    }
}
