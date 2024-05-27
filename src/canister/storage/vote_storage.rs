use ic_stable_structures::memory_manager::MemoryId;

use types::Votes;

use super::{
    StaticStorageRef, Storage, StorageInsertableByKey, StorageQueryable, StorageUpdateable, VOTES,
    VOTES_MEMORY_ID,
};

pub struct VoteStorage;

impl Storage<u64, Votes> for VoteStorage {
    const NAME: &'static str = "votes";

    fn storage() -> StaticStorageRef<u64, Votes> {
        &VOTES
    }

    fn memory_id() -> MemoryId {
        VOTES_MEMORY_ID
    }
}

impl StorageQueryable<u64, Votes> for VoteStorage {}
impl StorageInsertableByKey<u64, Votes> for VoteStorage {}
impl StorageUpdateable<u64, Votes> for VoteStorage {}
