use std::time::Duration;

use candid::Principal;
use ic_cdk_timers::set_timer;
use ic_stable_structures::memory_manager::MemoryId;
use types::{Error, Proposal, ProposalEntry, Status, Vote};

use crate::{logic::DAY_IN_NANOS, result::CanisterResult};

use super::{
    StaticStorageRef, Storage, StorageInsertable, StorageQueryable, StorageUpdateable, PROPOSALS,
    PROPOSALS_MEMORY_ID,
};

pub struct ProposalStorage;

impl Storage<u64, Proposal> for ProposalStorage {
    const NAME: &'static str = "proposals";

    fn storage() -> StaticStorageRef<u64, Proposal> {
        &PROPOSALS
    }

    fn memory_id() -> MemoryId {
        PROPOSALS_MEMORY_ID
    }
}

impl StorageQueryable<u64, Proposal> for ProposalStorage {}
impl StorageInsertable<Proposal> for ProposalStorage {}
impl StorageUpdateable<u64, Proposal> for ProposalStorage {}

impl ProposalStorage {
    pub fn new_proposal(caller: Principal, proposal: Proposal) -> CanisterResult<ProposalEntry> {
        let mut prop = proposal;
        prop.add_vote(caller, Vote::Approve);

        let (id, req) = Self::insert(prop)?;

        set_timer(Duration::from_nanos(DAY_IN_NANOS), move || {
            Self::expire(id);
        });

        Ok((id, req))
    }

    pub fn vote_proposal(caller: Principal, id: u64, vote: Vote) -> CanisterResult<ProposalEntry> {
        let (id, mut req) = Self::get(id)?;

        if req.status != Status::Pending {
            return Err(Error::bad_request().add_message("Proposal is not pending"));
        }

        if req.votes.approvals.contains(&caller) {
            return Err(Error::bad_request().add_message("Approval vote already cast"));
        }

        if req.votes.rejections.contains(&caller) {
            return Err(Error::bad_request().add_message("Rejection vote already cast"));
        }

        req.add_vote(caller, vote);
        Self::update(id, req.clone())
    }

    pub fn expire(id: u64) {
        let proposal = Self::get_opt(id);
        if proposal.is_none() {
            return;
        }

        let (_, mut proposal) = proposal.unwrap();

        if proposal.status() != Status::Pending {
            return;
        }

        proposal.update_status(Status::Expired);
        Self::update(id, proposal).expect("expected proposal exist");
    }

    pub fn reject(id: u64, deadlock: bool) -> CanisterResult<ProposalEntry> {
        let (_, mut proposal) = Self::get(id)?;

        match deadlock {
            true => proposal.update_status(Status::Deadlock),
            false => proposal.update_status(Status::Rejected),
        }

        Self::update(id, proposal)
    }

    pub fn approve(id: u64) -> CanisterResult<ProposalEntry> {
        let (_, mut proposal) = Self::get(id)?;
        proposal.update_status(Status::Approved);
        Self::update(id, proposal)
    }

    pub fn set_sent_at(id: u64, sent_at: u64) -> CanisterResult<ProposalEntry> {
        let (_, mut proposal) = Self::get(id)?;
        proposal.set_sent_at(sent_at);
        Self::update(id, proposal)
    }

    pub fn get_by_status(status: Option<Status>) -> Vec<ProposalEntry> {
        let mut proposals = Self::filter(|_, proposal| {
            if let Some(status) = status.clone() {
                return proposal.status() == status;
            }

            true
        });

        proposals.sort_by(|(_, a), (_, b)| a.created_at.cmp(&b.created_at));
        proposals
    }
}
