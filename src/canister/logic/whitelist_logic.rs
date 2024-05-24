use candid::Principal;
use types::{Error, ValidateField, ValidationType, WhitelistEntry};

use crate::{
    helpers::validator::Validator,
    result::CanisterResult,
    storage::{StorageInsertable, StorageQueryable, WhitelistStorage},
};

use super::{MAX_WHITELISTED, MIN_WHITELISTED};

pub struct WhitelistLogic;

impl WhitelistLogic {
    pub fn get_whitelist() -> Vec<Principal> {
        WhitelistStorage::get_all()
            .into_iter()
            .map(|(_, v)| v)
            .collect()
    }

    pub fn add(principal: Principal) -> CanisterResult<WhitelistEntry> {
        if WhitelistStorage::contains(&principal) {
            return Err(Error::bad_request().add_message("Principal already exists in whitelist"));
        }

        Validator::new(vec![ValidateField(
            ValidationType::Count(
                WhitelistStorage::get_all().len(),
                MIN_WHITELISTED,
                MAX_WHITELISTED,
            ),
            "whitelisted".to_owned(),
        )])
        .validate()?;

        WhitelistStorage::insert(principal)
    }

    pub fn remove(principal: Principal) -> CanisterResult<()> {
        let (id, _) = WhitelistStorage::find(|_, value| value == &principal)
            .ok_or(Error::not_found().add_message("Principal does not exist in whitelist"))?;

        Validator::new(vec![ValidateField(
            ValidationType::Count(
                WhitelistStorage::get_all().len() - 1,
                MIN_WHITELISTED,
                MAX_WHITELISTED,
            ),
            "whitelisted".to_owned(),
        )])
        .validate()?;

        WhitelistStorage::remove(id);
        Ok(())
    }
}
