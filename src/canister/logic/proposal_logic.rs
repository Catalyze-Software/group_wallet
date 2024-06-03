use std::time::Duration;

use candid::Principal;
use ic_cdk::api::time;
use ic_cdk_timers::set_timer;
use types::{
    Content, Error, Proposal, ProposalEntry, ProposalResponse, Status, TallyResult, Vote, VoteKind,
    Votes, VotesEntry,
};

use crate::{
    result::CanisterResult,
    storage::{
        ProposalStorage, StorageInsertable, StorageInsertableByKey, StorageQueryable,
        StorageUpdateable, VoteStorage, WhitelistStorage,
    },
};

use super::{AirdropLogic, TransferLogic, DAY_IN_NANOS};

pub struct ProposalLogic;

impl ProposalLogic {
    pub fn get_proposals(status: Option<Status>) -> Vec<ProposalResponse> {
        ProposalStorage::get_by_status(status)
    }

    pub fn get_votes(id: u64, kind: Option<VoteKind>) -> CanisterResult<VotesEntry> {
        let (_, votes) = VoteStorage::get(id)?;

        let votes = match kind {
            Some(kind) => Votes(votes.0.iter().filter(|v| v.kind == kind).cloned().collect()),
            None => votes,
        };

        Ok((id, votes))
    }

    pub async fn propose(
        caller: Principal,
        content: Content,
        voting_period: Option<u64>,
    ) -> CanisterResult<ProposalEntry> {
        match content.clone() {
            Content::Transfer(content) => {
                TransferLogic::check_balance(content.canister_id, &content.args.amount).await?
            }
            Content::Airdrop(content) => {
                AirdropLogic::check_balance(content.canister_id, content.args).await?
            }
        }

        let voting_period = voting_period.unwrap_or(DAY_IN_NANOS);

        let (id, proposal) =
            ProposalStorage::insert(Proposal::new(caller, content, voting_period))?;

        set_timer(Duration::from_nanos(voting_period), move || {
            ProposalStorage::expire(id);
        });

        VoteStorage::insert_by_key(id, Votes(vec![Vote::new(caller, VoteKind::Approve)]))?;

        Ok((id, proposal))
    }

    pub fn vote(caller: Principal, id: u64, vote: VoteKind) -> CanisterResult<ProposalEntry> {
        let (id, proposal) = ProposalStorage::get(id)?;

        if proposal.status != Status::Pending {
            return Err(Error::bad_request().add_message("Proposal is not pending"));
        }

        let (_, mut votes) = VoteStorage::get(id)?;

        match votes.voted(&caller) {
            true => votes.update(caller, vote),
            false => votes.add(Vote::new(caller, vote)),
        }

        let (_, votes) = VoteStorage::update(id, votes)?;

        match Self::get_tally_result(&votes) {
            TallyResult::Approve => ProposalStorage::approve(id),
            TallyResult::Reject => ProposalStorage::reject(id, false),
            TallyResult::Deadlock => ProposalStorage::reject(id, true),
            TallyResult::NotReached => Ok((id, proposal)),
        }
    }

    pub async fn execute(id: u64) -> CanisterResult<()> {
        let (_, proposal) = ProposalStorage::get(id)?;

        if proposal.status != Status::Approved {
            return Err(Error::bad_request().add_message("Proposal is not approved"));
        }

        if proposal.sent_at.is_some() {
            return Err(Error::bad_request().add_message("Proposal already executed"));
        }

        let (id, proposal) = ProposalStorage::set_sent_at(id, time())?;

        match proposal.content {
            Content::Transfer(content) => TransferLogic::execute_transfer(content).await,
            Content::Airdrop(content) => AirdropLogic::execute_airdrop(id, content).await,
        }
    }

    fn get_tally_result(votes: &Votes) -> TallyResult {
        let whitelist = WhitelistStorage::get_all();
        // Plus one for the owner
        let whitelist_count = (whitelist.len() + 1) as f32;
        let approval_count = votes.approvals() as f32;
        let rejection_count = votes.rejections() as f32;

        let approval_percentage = (approval_count / whitelist_count) * 100.0;
        let rejection_percentage = (rejection_count / whitelist_count) * 100.0;

        if approval_percentage > 50.0 {
            return TallyResult::Approve;
        }
        if rejection_percentage > 50.0 {
            return TallyResult::Reject;
        }
        if approval_percentage == 50.0 && rejection_percentage == 50.0 {
            return TallyResult::Deadlock;
        }

        TallyResult::NotReached
    }
}
