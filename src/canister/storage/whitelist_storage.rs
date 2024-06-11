use candid::Principal;
use ic_stable_structures::{memory_manager::MemoryId, StableBTreeMap};

use crate::result::CanisterResult;

use super::{
    StaticStorageRef, Storage, StorageInsertable, StorageQueryable, StorageUpdateable,
    MEMORY_MANAGER, WHITELIST, WHITELIST_MEMORY_ID,
};

pub struct WhitelistStorage;

impl Storage<u64, Principal> for WhitelistStorage {
    const NAME: &'static str = "whitelist";

    fn storage() -> StaticStorageRef<u64, Principal> {
        &WHITELIST
    }

    fn memory_id() -> MemoryId {
        WHITELIST_MEMORY_ID
    }
}

impl StorageQueryable<u64, Principal> for WhitelistStorage {}
impl StorageInsertable<Principal> for WhitelistStorage {}
impl StorageUpdateable<u64, Principal> for WhitelistStorage {}

impl WhitelistStorage {
    pub fn remove(id: u64) -> bool {
        Self::storage().with(|data| data.borrow_mut().remove(&id).is_some())
    }

    pub fn replace(whitelisted: Vec<Principal>) -> CanisterResult<Vec<Principal>> {
        Self::storage().with(|n| {
            n.replace(StableBTreeMap::new(
                MEMORY_MANAGER.with(|m| m.borrow().get(Self::memory_id())),
            ))
        });

        whitelisted.into_iter().try_for_each(|p| {
            Self::insert(p)?;
            Ok(())
        })?;

        Ok(Self::get_all().iter().map(|(_, v)| *v).collect())
    }
}
