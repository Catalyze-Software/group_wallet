use candid::Principal;

use crate::{
    result::CanisterResult,
    storage::{CellStorage, OwnerStorage},
};

pub struct OwnerLogic;

impl OwnerLogic {
    pub fn get() -> CanisterResult<Principal> {
        OwnerStorage::get()
    }

    pub fn set(new_owner: Principal) -> CanisterResult<Principal> {
        OwnerStorage::set(new_owner)
    }
}
