use ic_stable_structures::memory_manager::MemoryId;

use super::{CellStorage, CellStorageRef, VOTING_PERIOD, VOTING_PERIOD_MEMORY_ID};

pub struct VotingPeriodStorage;

impl CellStorage<u64> for VotingPeriodStorage {
    const NAME: &'static str = "voting_period";

    fn storage() -> CellStorageRef<u64> {
        &VOTING_PERIOD
    }

    fn memory_id() -> MemoryId {
        VOTING_PERIOD_MEMORY_ID
    }
}
