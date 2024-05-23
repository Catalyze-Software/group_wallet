use candid::Principal;

use crate::{
    models::{Error, Votes},
    result::CanisterResult,
};

pub fn check_duplicate_vote(caller: &Principal, votes: &Votes) -> CanisterResult<()> {
    if votes.approvals.contains(caller) {
        return Err(Error::bad_request().add_message("Approval vote already cast"));
    }

    if votes.rejections.contains(caller) {
        return Err(Error::bad_request().add_message("Rejection vote already cast"));
    }

    Ok(())
}
