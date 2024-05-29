use candid::Principal;
use ic_cdk::trap;
use types::{Error, ValidateField, ValidationType, WhitelistEntry};

use crate::{
    helpers::validator::Validator,
    result::CanisterResult,
    storage::{StorageInsertable, StorageQueryable, StorageUpdateable, WhitelistStorage},
};

use super::{MAX_WHITELISTED, MIN_WHITELISTED, WHITELIST_OWNER_INDEX};

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

    pub fn set_owner(new_owner: Principal) -> CanisterResult<()> {
        WhitelistStorage::set_owner(new_owner)?;
        Ok(())
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

    pub fn replace_whitelisted(whitelisted: Vec<Principal>) -> CanisterResult<Vec<WhitelistEntry>> {
        Validator::new(vec![ValidateField(
            ValidationType::Count(
                whitelisted.len(),
                1,
                // Only whitelisted principals could be replaced, not the owner
                MIN_WHITELISTED,
            ),
            "whitelisted".to_owned(),
        )])
        .validate()?;

        let anonymous = whitelisted
            .clone()
            .into_iter()
            .find(|p| p != &Principal::anonymous());

        if let Some(anonymous) = anonymous {
            return Err(Error::bad_request().add_message(&format!(
                "Cannot replace with anonymous principal: {anonymous}"
            )));
        }

        let (_, owner) = WhitelistStorage::get_owner()?;
        let owner = whitelisted.clone().into_iter().find(|p| p == &owner);

        if let Some(owner) = owner {
            return Err(Error::bad_request()
                .add_message(&format!("Cannot replace owner principal: {owner}")));
        }

        whitelisted.clone().into_iter().try_for_each(|p| {
            if WhitelistStorage::contains(&p) {
                return Err(Error::bad_request()
                    .add_message(&format!("Principal: {p} already exists in whitelist")));
            }

            Ok(())
        })?;

        whitelisted
            .into_iter()
            .enumerate()
            .try_for_each(|(idx, p)| {
                // Storage starts from 1, owner is at index 1
                let id = WHITELIST_OWNER_INDEX + idx as u64 + 1;
                WhitelistStorage::update(id, p)?;
                Ok(())
            })?;

        Ok(WhitelistStorage::get_all())
    }
}
