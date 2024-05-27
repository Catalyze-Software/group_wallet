use candid::Principal;
use ic_cdk::trap;
use types::{Error, ValidateField, ValidationType, WhitelistEntry};

use crate::{
    helpers::validator::Validator,
    result::CanisterResult,
    storage::{StorageInsertable, StorageQueryable, StorageUpdateable, WhitelistStorage},
};

use super::{MAX_WHITELISTED, MIN_WHITELISTED};

pub struct WhitelistLogic;

impl WhitelistLogic {
    pub fn init(owner: Principal, whitelisted: Vec<Principal>) {
        let whitelisted = whitelisted
            .into_iter()
            .filter(|p| p != &owner)
            .collect::<Vec<_>>();

        let whitelisted_size = whitelisted.len() + 1;

        if whitelisted_size < MIN_WHITELISTED {
            trap(&format!(
                "At least {MIN_WHITELISTED} principals must be whitelisted."
            ));
        }
        if whitelisted_size > MAX_WHITELISTED {
            trap(&format!(
                "At most {MAX_WHITELISTED} principals can be whitelisted."
            ));
        }

        WhitelistStorage::set_owner(owner).expect("Failed to set owner");

        for principal in whitelisted {
            WhitelistStorage::insert(principal).expect("Failed to insert principal");
        }
    }

    pub fn get_whitelist() -> Vec<Principal> {
        WhitelistStorage::get_all()
            .into_iter()
            .map(|(_, v)| v)
            .collect()
    }

    pub fn get_owner() -> CanisterResult<WhitelistEntry> {
        WhitelistStorage::get_owner()
    }

    pub fn transfer_ownership(new_owner: Principal) -> CanisterResult<WhitelistEntry> {
        WhitelistStorage::set_owner(new_owner)
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

    pub fn switch_whitelisted(remove: Principal, add: Principal) -> CanisterResult<WhitelistEntry> {
        if [remove, add].contains(&Principal::anonymous()) {
            return Err(Error::bad_request().add_message("Cannot switch anonymous principal"));
        }

        let (id, _) = WhitelistStorage::find(|_, value| value == &remove)
            .ok_or(Error::not_found().add_message("Principal does not exist in whitelist"))?;

        if WhitelistStorage::contains(&add) {
            return Err(
                Error::bad_request().add_message("Add principal already exists in whitelist")
            );
        }

        WhitelistStorage::update(id, add)
    }
}
