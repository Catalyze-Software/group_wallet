use candid::Principal;

use crate::helpers::votes::get_request_majority;
use crate::models::{
    Error, Status, Vote, VoteResponse, WhitelistRequest, WhitelistRequestEntry,
    WhitelistRequestKind,
};
use crate::result::CanisterResult;
use crate::storage::{
    RequestStorage, StorageInsertable, StorageQueryable, StorageUpdateable,
    WhitelistRequestStorage, WhitelistStorage,
};

pub struct WhitelistLogic;

impl WhitelistLogic {
    pub fn get_whitelist() -> Vec<Principal> {
        WhitelistStorage::get_all()
            .into_iter()
            .map(|(_, v)| v)
            .collect()
    }

    pub fn get_requests(status: Option<Status>) -> Vec<WhitelistRequestEntry> {
        WhitelistRequestStorage::get_requests_by_status(status)
    }

    pub fn request(
        caller: Principal,
        kind: WhitelistRequestKind,
    ) -> CanisterResult<WhitelistRequestEntry> {
        let whitelisted = WhitelistStorage::contains(&caller);
        let duplicate = WhitelistRequestStorage::find(|_, req| {
            req.kind.principal() == &caller && req.details.status == Status::Pending
        });

        match kind {
            WhitelistRequestKind::Add(_) => {
                if duplicate.is_some() {
                    return Err(Error::duplicate().add_message("Already a pending add request"));
                }
                if whitelisted {
                    return Err(Error::duplicate().add_message("Principal already whitelisted"));
                }
            }
            WhitelistRequestKind::Remove(_) => {
                if duplicate.is_some() {
                    return Err(Error::duplicate().add_message("Already a pending remove request"));
                }
                if !whitelisted {
                    return Err(Error::not_found().add_message("Principal not whitelisted"));
                }
            }
        };

        WhitelistRequestStorage::new_request(caller, WhitelistRequest::new(kind))
    }

    pub fn vote_request(
        caller: Principal,
        id: u64,
        vote: Vote,
    ) -> CanisterResult<WhitelistRequestEntry> {
        let (_, req) = WhitelistRequestStorage::vote_request(caller, id, vote)?;

        match Self::get_request_majority(id)? {
            VoteResponse::Approve => Self::approve_request(id),
            VoteResponse::Reject => WhitelistRequestStorage::reject_request(id, false),
            VoteResponse::Deadlock => WhitelistRequestStorage::reject_request(id, true),
            VoteResponse::NotReached => Ok((id, req)),
        }
    }

    fn get_request_majority(id: u64) -> CanisterResult<VoteResponse> {
        let (_, req) = WhitelistRequestStorage::get(id)?;

        let mut whitelist = WhitelistStorage::get_all();

        if let WhitelistRequestKind::Remove(principal) = req.kind {
            whitelist.retain(|(_, p)| p != &principal);
        }

        Ok(get_request_majority(whitelist, &req.details.votes))
    }

    fn approve_request(id: u64) -> CanisterResult<WhitelistRequestEntry> {
        let (_, req) = WhitelistRequestStorage::approve_request(id)?;

        match req.kind {
            WhitelistRequestKind::Add(principal) => {
                WhitelistStorage::insert(principal)?;
            }
            WhitelistRequestKind::Remove(principal) => {
                WhitelistStorage::remove_by_value(&principal)?;
            }
        }

        Ok((id, req))
    }
}
