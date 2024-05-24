use candid::Principal;
use ic_cdk::api::time;
use types::{Content, Error, Proposal, ProposalEntry, Status, TallyResult, Vote, Votes};

use crate::{
    result::CanisterResult,
    storage::{ProposalStorage, StorageQueryable, WhitelistStorage},
};

use super::{AirdropLogic, TransferLogic};

pub struct ProposalLogic;

impl ProposalLogic {
    pub fn get_proposals(status: Option<Status>) -> Vec<ProposalEntry> {
        ProposalStorage::get_by_status(status)
    }

    pub async fn propose(caller: Principal, content: Content) -> CanisterResult<ProposalEntry> {
        match content.clone() {
            Content::Transfer(content) => TransferLogic::check_balance(caller, &content.args.amount).await?,
            Content::Airdrop(content) => {
                AirdropLogic::check_balance(content.canister_id, content.args).await?
            }
        }

        ProposalStorage::new_proposal(caller, Proposal::new(caller, content))
    }

    pub fn vote(caller: Principal, id: u64, vote: Vote) -> CanisterResult<ProposalEntry> {
        let (id, proposal) = ProposalStorage::vote_proposal(caller, id, vote)?;

        match Self::get_tally_result(&proposal.votes) {
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
        let whitelist_count = whitelist.len() as f32;
        let approval_count = votes.approvals.len() as f32;
        let rejection_count = votes.rejections.len() as f32;

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
