use ic_cdk::{caller, query, update};

use types::{Content, ProposalEntry, Status, Vote};

use crate::{
    helpers::guards::{is_owner, is_whitelisted},
    logic::ProposalLogic,
    result::CanisterResult,
};

#[query]
pub fn get_proposals(status: Option<Status>) -> Vec<ProposalEntry> {
    ProposalLogic::get_proposals(status)
}

#[update(guard = "is_owner")]
pub async fn propose(content: Content) -> CanisterResult<ProposalEntry> {
    ProposalLogic::propose(caller(), content).await
}

#[update(guard = "is_whitelisted")]
pub fn vote_proposal(id: u64, vote: Vote) -> CanisterResult<ProposalEntry> {
    ProposalLogic::vote(caller(), id, vote)
}
