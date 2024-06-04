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

#[update(guard = "is_owner")]
pub async fn propose(
    content: Content,
    voting_period: Option<u64>,
) -> CanisterResult<ProposalEntry> {
    ProposalLogic::propose(caller(), content, voting_period).await
}

#[update(guard = "is_whitelisted")]
pub fn vote_proposal(id: u64, vote: VoteKind) -> CanisterResult<ProposalEntry> {
    ProposalLogic::vote(caller(), id, vote)
}
