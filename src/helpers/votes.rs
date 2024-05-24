use crate::models::{VoteResponse, Votes, WhitelistEntry};

pub fn get_request_majority(whitelist: Vec<WhitelistEntry>, votes: &Votes) -> VoteResponse {
    let whitelist_count = whitelist.len() as f32;
    let approval_count = votes.approvals.len() as f32;
    let rejection_count = votes.rejections.len() as f32;

    let approval_percentage = (approval_count / whitelist_count) * 100.0;
    let rejection_percentage = (rejection_count / whitelist_count) * 100.0;

    if approval_percentage > 50.0 {
        return VoteResponse::Approve;
    }
    if rejection_percentage > 50.0 {
        return VoteResponse::Reject;
    }
    if approval_percentage == 50.0 && rejection_percentage == 50.0 {
        return VoteResponse::Deadlock;
    }

    VoteResponse::NotReached
}
