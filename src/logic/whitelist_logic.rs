use std::time::Duration;

use candid::Principal;
use ic_cdk_timers::set_timer;

use crate::helpers::votes::check_duplicate_vote;
use crate::models::{
    Error, Status, Vote, VoteResponse, WhitelistRequest, WhitelistRequestEntry,
    WhitelistRequestKind,
};
use crate::result::CanisterResult;
use crate::storage::{
    StorageInsertable, StorageQueryable, StorageUpdateable, WhitelistRequestStorage,
    WhitelistStorage,
};

use super::consts::DAY_IN_NANOS;

pub struct WhitelistLogic;

impl WhitelistLogic {
    pub fn get_whitelist() -> Vec<Principal> {
        WhitelistStorage::get_all()
            .into_iter()
            .map(|(_, v)| v)
            .collect()
    }

    pub fn get_requests(status: Option<Status>) -> Vec<WhitelistRequest> {
        let mut resp = WhitelistRequestStorage::filter(|_, request| {
            if let Some(status) = &status {
                &request.details.status == status
            } else {
                true
            }
        })
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<WhitelistRequest>>();

        resp.sort_by(|a, b| a.details.created_at.cmp(&b.details.created_at));
        resp
    }

    pub fn request(
        caller: Principal,
        kind: WhitelistRequestKind,
    ) -> CanisterResult<WhitelistRequestEntry> {
        // TODO: Add whitelist guard
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

        let (id, _) = WhitelistRequestStorage::insert(WhitelistRequest::new(kind))?;

        set_timer(Duration::from_nanos(DAY_IN_NANOS), move || {
            Self::expire_request(id)
        });

        Self::vote_request(caller, id, Vote::Approve)
    }

    pub fn vote_request(
        caller: Principal,
        id: u64,
        vote: Vote,
    ) -> CanisterResult<WhitelistRequestEntry> {
        // TODO: Add whitelist guard

        let (id, mut req) = WhitelistRequestStorage::get(id)?;

        if req.details.status != Status::Pending {
            return Err(Error::bad_request().add_message("Whitelist request is not pending"));
        }

        check_duplicate_vote(&caller, &req.details.votes)?;

        req.add_vote(caller, vote);
        WhitelistRequestStorage::update(id, req.clone())?;

        match Self::get_request_majority(id)? {
            VoteResponse::Approve => Self::approve_request(id),
            VoteResponse::Reject => Self::reject_request(id, false),
            VoteResponse::Deadlock => Self::reject_request(id, true),
            VoteResponse::NotReached => Ok((id, req)),
        }
    }

    fn get_request_majority(id: u64) -> CanisterResult<VoteResponse> {
        let (_, req) = WhitelistRequestStorage::get(id)?;

        let mut whitelist = WhitelistStorage::get_all();

        if let WhitelistRequestKind::Remove(principal) = req.kind {
            whitelist.retain(|(_, p)| p != &principal);
        }

        let whitelist_count = whitelist.len() as f32;
        let approval_count = req.details.votes.approvals.len() as f32;
        let rejection_count = req.details.votes.rejections.len() as f32;

        let approval_percentage = (approval_count / whitelist_count) * 100.0;
        let rejection_percentage = (rejection_count / whitelist_count) * 100.0;

        if approval_percentage > 50.0 {
            return Ok(VoteResponse::Approve);
        }
        if rejection_percentage > 50.0 {
            return Ok(VoteResponse::Reject);
        }
        if approval_percentage == 50.0 && rejection_percentage == 50.0 {
            return Ok(VoteResponse::Deadlock);
        }

        Ok(VoteResponse::NotReached)
    }

    fn approve_request(id: u64) -> CanisterResult<WhitelistRequestEntry> {
        let (_, mut req) = WhitelistRequestStorage::get(id)?;
        req.update_status(Status::Approved);
        let (_, req) = WhitelistRequestStorage::insert(req)?;

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

    fn reject_request(id: u64, deadlock: bool) -> CanisterResult<WhitelistRequestEntry> {
        let (_, mut req) = WhitelistRequestStorage::get(id)?;

        if deadlock {
            req.update_status(Status::Deadlock);
        } else {
            req.update_status(Status::Rejected);
        }

        WhitelistRequestStorage::insert(req)
    }

    pub fn expire_request(id: u64) {
        let req = WhitelistRequestStorage::get_opt(id);
        if req.is_none() {
            return;
        }

        let (_, mut req) = req.unwrap();

        if req.details.status != Status::Pending {
            return;
        }

        req.update_status(Status::Expired);
        WhitelistRequestStorage::update(id, req).expect("expected request exist");
    }
}
