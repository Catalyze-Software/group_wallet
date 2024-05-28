use ic_stable_structures::memory_manager::MemoryId;
use types::{Proposal, ProposalEntry, ProposalResponse, Status, Votes};

use crate::result::CanisterResult;

use super::{
    StaticStorageRef, Storage, StorageInsertable, StorageQueryable, StorageUpdateable, VoteStorage,
    PROPOSALS, PROPOSALS_MEMORY_ID,
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

    pub fn get_by_status(status: Option<Status>) -> Vec<ProposalResponse> {
        let mut proposals = Self::filter(|_, proposal| {
            if let Some(status) = status.clone() {
                return proposal.status() == status;
            }

            true
        });

        proposals.sort_by(|(_, a), (_, b)| a.created_at.cmp(&b.created_at));
        proposals.into_iter().map(Self::map_to_response).collect()
    }

    fn map_to_response(data: (u64, Proposal)) -> ProposalResponse {
        let (id, proposal) = data;
        let votes: Votes = VoteStorage::get(id)
            .map(|(_, votes)| votes)
            .unwrap_or_else(|_| Votes(vec![]));
        ProposalResponse {
            id,
            proposal,
            votes,
        }
    }
}
