use std::time::Duration;

use candid::Principal;
use ic_cdk_timers::set_timer;
use ic_stable_structures::Storable;

use crate::{
    logic::DAY_IN_NANOS,
    models::{Error, Request, Status, Vote},
    result::CanisterResult,
};

use super::{Storage, StorageInsertable, StorageQueryable, StorageUpdateable};

pub trait RequestStorage<V: 'static + Storable + Clone + PartialEq + Request>:
    Storage<u64, V> + StorageQueryable<u64, V> + StorageUpdateable<u64, V> + StorageInsertable<V>
{
    fn new_request(caller: Principal, request: V) -> CanisterResult<(u64, V)> {
        let mut req = request;
        req.add_vote(caller, Vote::Approve);

        let (id, req) = Self::insert(req)?;

        set_timer(Duration::from_nanos(DAY_IN_NANOS), move || {
            Self::expire_request(id);
        });

        Ok((id, req))
    }

    fn vote_request(caller: Principal, id: u64, vote: Vote) -> CanisterResult<(u64, V)> {
        let (id, mut req) = Self::get(id)?;

        if req.details().status != Status::Pending {
            return Err(Error::bad_request().add_message("Request is not pending"));
        }

        if req.details().votes.approvals.contains(&caller) {
            return Err(Error::bad_request().add_message("Approval vote already cast"));
        }

        if req.details().votes.rejections.contains(&caller) {
            return Err(Error::bad_request().add_message("Rejection vote already cast"));
        }

        req.add_vote(caller, vote);
        Self::update(id, req.clone())
    }

    fn expire_request(id: u64) {
        let req = Self::get_opt(id);
        if req.is_none() {
            return;
        }

        let (_, mut req) = req.unwrap();

        if req.status() != Status::Pending {
            return;
        }

        req.update_status(Status::Expired);
        Self::update(id, req).expect("expected request exist");
    }

    fn reject_request(id: u64, deadlock: bool) -> CanisterResult<(u64, V)> {
        let (_, mut req) = Self::get(id)?;

        match deadlock {
            true => req.update_status(Status::Deadlock),
            false => req.update_status(Status::Rejected),
        }

        Self::update(id, req)
    }

    fn approve_request(id: u64) -> CanisterResult<(u64, V)> {
        let (_, mut req) = Self::get(id)?;
        req.update_status(Status::Approved);
        Self::update(id, req)
    }

    fn set_sent_at(id: u64, sent_at: u64) -> CanisterResult<(u64, V)> {
        let (_, mut req) = Self::get(id)?;
        req.set_sent_at(sent_at);
        Self::update(id, req)
    }

    fn get_requests_by_status(status: Option<Status>) -> Vec<(u64, V)> {
        let mut requests = Self::filter(|_, request| {
            if let Some(status) = status.clone() {
                return request.status() == status;
            }

            true
        });

        requests.sort_by(|(_, a), (_, b)| a.details().created_at.cmp(&b.details().created_at));
        requests
    }
}
