use std::{cell::RefCell, collections::HashMap, time::Duration};

use candid::{CandidType, Principal};

use serde::Deserialize;

use crate::ledger_declarations::types::{
    AirdropRequestData, AirdropTransactionDetails, TokenStandard, TransactionRequestData, Votes,
    WhitelistRequestData,
};

pub static DAY_IN_NANOS: u64 = Duration::from_secs(1 * 24 * 60 * 60).as_nanos() as u64;

#[derive(Deserialize, CandidType)]
pub struct Store {
    pub owner: Principal,

    pub whitelist: Vec<Principal>,
    pub whitelist_request_id: u32,
    pub whitelist_requests: HashMap<u32, WhitelistRequestData>,
    pub whitelist_request_expiry: u64,

    pub tokens: HashMap<Principal, TokenStandard>,
    pub transaction_request_id: u32,
    pub transaction_requests: HashMap<u32, TransactionRequestData>,
    pub transaction_request_expiry: u64,

    pub airdrop_transactions: HashMap<u32, HashMap<Principal, AirdropTransactionDetails>>,
    pub airdrop_error: HashMap<u32, String>,
    pub airdrop_request_id: u32,
    pub airdrop_requests: HashMap<u32, AirdropRequestData>,
    pub airdrop_request_expiry: u64,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            owner: Principal::anonymous(),
            whitelist: Default::default(),
            whitelist_request_id: 0,
            whitelist_requests: Default::default(),
            whitelist_request_expiry: DAY_IN_NANOS,

            tokens: Default::default(),
            transaction_request_id: 0,
            transaction_requests: Default::default(),
            transaction_request_expiry: DAY_IN_NANOS,

            airdrop_transactions: Default::default(),
            airdrop_error: Default::default(),
            airdrop_request_id: 0,
            airdrop_requests: Default::default(),
            airdrop_request_expiry: DAY_IN_NANOS,
        }
    }
}

thread_local! {
    pub static DATA: RefCell<Store>  = RefCell::new(Store::default());
}

impl Store {
    pub fn init(owner: Principal) {
        DATA.with(|data| {
            let mut data = data.borrow_mut();
            data.owner = owner.clone();
            data.whitelist.push(owner);
        });
    }

    pub fn get_airdrop_error(caller: Principal, request_id: u32) -> Result<String, String> {
        DATA.with(|data| {
            let data = data.borrow();

            if !data.whitelist.contains(&caller) {
                return Err("Caller is not whitelisted".to_string());
            };

            let error = data.airdrop_error.get(&request_id).clone();
            match error {
                Some(error) => Ok(error.clone()),
                None => Err("No error found".to_string()),
            }
        })
    }

    pub fn get_token_list() -> Vec<(Principal, TokenStandard)> {
        DATA.with(|data| {
            let data = data.borrow();
            data.tokens
                .iter()
                .map(|(canister_id, standard)| (canister_id.clone(), standard.clone()))
                .collect()
        })
    }

    pub fn add_token_to_list(
        caller: Principal,
        canister_id: Principal,
        standard: TokenStandard,
    ) -> Result<(), String> {
        if let Err(err) = Self::is_whitelisted(&caller) {
            return Err(err);
        }

        DATA.with(|data| {
            let mut data = data.borrow_mut();
            data.tokens.insert(canister_id, standard);
        });
        Ok(())
    }

    pub fn remove_token_from_list(caller: Principal, canister_id: Principal) -> Result<(), String> {
        if let Err(err) = Self::is_whitelisted(&caller) {
            return Err(err);
        }

        DATA.with(|data| {
            let mut data = data.borrow_mut();
            data.tokens.remove(&canister_id);
        });
        Ok(())
    }

    pub fn is_whitelisted(caller: &Principal) -> Result<(), String> {
        DATA.with(|data| {
            let data = &data.borrow();
            if data.whitelist.contains(caller) || caller == &data.owner {
                Ok(())
            } else {
                Err("Caller is not whitelisted".to_string())
            }
        })
    }

    pub fn check_duplicate_vote(caller: &Principal, votes: &Votes) -> Result<(), String> {
        if votes.approvals.contains(caller) {
            return Err("Approval vote already cast".to_string());
        } else if votes.rejections.contains(caller) {
            return Err("Rejection vote already cast".to_string());
        } else {
            Ok(())
        }
    }
}
