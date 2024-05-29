use candid::Principal;
use ic_cdk::trap;
use types::{Error, ValidateField, ValidationType};

use crate::{
    helpers::validator::Validator,
    result::CanisterResult,
    storage::{CellStorage, OwnerStorage, StorageInsertable, StorageQueryable, WhitelistStorage},
};

use super::{MAX_WHITELISTED, MIN_WHITELISTED};

pub struct WhitelistLogic;

impl WhitelistLogic {
    pub fn init(owner: Principal, whitelisted: Vec<Principal>) {
        let mut deduped = whitelisted.clone();
        deduped.sort();
        deduped.dedup();

        if whitelisted.len() != deduped.len() {
            trap("Duplicate principals in whitelist");
        }

        // Filter out the owner from the whitelisted to ensure amount is correct
        let whitelisted = whitelisted
            .into_iter()
            .filter(|p| p != &owner)
            .collect::<Vec<_>>();

        // Plus one for the owner
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

        OwnerStorage::set(owner).expect("Failed to set owner");

        for principal in whitelisted {
            WhitelistStorage::insert(principal).expect("Failed to insert principal");
        }
    }

    pub fn get_whitelist() -> CanisterResult<Vec<Principal>> {
        let result = vec![OwnerStorage::get()?];
        let whitelisted = WhitelistStorage::get_all().into_iter().map(|(_, v)| v);
        Ok(result.into_iter().chain(whitelisted).collect())
    }

    pub fn add(principal: Principal) -> CanisterResult<Principal> {
        if principal == Principal::anonymous() {
            return Err(Error::bad_request().add_message("Cannot add anonymous principal"));
        }
        if principal == OwnerStorage::get()? {
            return Err(Error::bad_request().add_message("Cannot add owner principal"));
        }
        if WhitelistStorage::contains(&principal) {
            return Err(Error::bad_request().add_message("Principal already exists in whitelist"));
        }

        Validator::new(vec![ValidateField(
            ValidationType::Count(
                // Plus one for the owner
                WhitelistStorage::get_all().len() + 1,
                MIN_WHITELISTED,
                MAX_WHITELISTED,
            ),
            "whitelisted".to_owned(),
        )])
        .validate()?;

        WhitelistStorage::insert(principal).map(|(_, v)| v)
    }

    pub fn remove(principal: Principal) -> CanisterResult<()> {
        let (id, _) = WhitelistStorage::find(|_, value| value == &principal)
            .ok_or(Error::not_found().add_message("Principal does not exist in whitelist"))?;

        Validator::new(vec![ValidateField(
            ValidationType::Count(
                // whitelist_size + 1 for the owner - 1 for the removed principal == whitelist_size
                WhitelistStorage::get_all().len(),
                MIN_WHITELISTED,
                MAX_WHITELISTED,
            ),
            "whitelisted".to_owned(),
        )])
        .validate()?;

        WhitelistStorage::remove(id);
        Ok(())
    }

    pub fn replace_whitelisted(whitelisted: Vec<Principal>) -> CanisterResult<Vec<Principal>> {
        let mut deduped = whitelisted.clone();
        deduped.sort();
        deduped.dedup();

        if whitelisted.len() != deduped.len() {
            return Err(Error::bad_request().add_message("Duplicate principals in whitelist"));
        }

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

        if whitelisted.clone().contains(&Principal::anonymous()) {
            return Err(Error::bad_request().add_message("Cannot replace with anonymous principal"));
        }

        let owner = OwnerStorage::get()?;
        let owner = whitelisted.clone().into_iter().find(|p| p == &owner);

        if let Some(owner) = owner {
            return Err(Error::bad_request()
                .add_message(&format!("Cannot replace owner principal: {owner}")));
        }

        let whitelisted = WhitelistStorage::replace(whitelisted)?;
        let whitelisted = vec![OwnerStorage::get()?]
            .into_iter()
            .chain(whitelisted)
            .collect();
        Ok(whitelisted)
    }
}
