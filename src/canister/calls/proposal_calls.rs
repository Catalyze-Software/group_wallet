use ic_cdk::{caller, query, update};

use types::{Content, ProposalEntry, ProposalResponse, Status, VoteKind, VotesEntry};

use crate::{
    helpers::guards::{is_authorized, is_owner, is_whitelisted},
    logic::ProposalLogic,
    result::CanisterResult,
};

#[query(guard = "is_authorized")]
pub fn get_proposals(status: Option<Status>) -> Vec<ProposalResponse> {
    ProposalLogic::get_proposals(status)
}

#[query(guard = "is_authorized")]
pub fn get_votes(id: u64, kind: Option<VoteKind>) -> CanisterResult<VotesEntry> {
    ProposalLogic::get_votes(id, kind)
}

#[query(guard = "is_authorized")]
pub fn get_voting_period() -> CanisterResult<u64> {
    ProposalLogic::get_voting_period()
}

#[query(guard = "is_owner")]
pub fn set_voting_period(nanos_period: u64) -> CanisterResult<u64> {
    ProposalLogic::set_voting_period(nanos_period)
}

#[update(guard = "is_owner")]
pub async fn propose(content: Content) -> CanisterResult<ProposalEntry> {
    ProposalLogic::propose(caller(), content).await
}

#[update(guard = "is_whitelisted")]
pub fn vote_proposal(id: u64, vote: VoteKind) -> CanisterResult<ProposalEntry> {
    ProposalLogic::vote(caller(), id, vote)
}

#[update(guard = "is_whitelisted")]
pub async fn execute_proposal(id: u64) -> CanisterResult<()> {
    ProposalLogic::execute(id).await
}
