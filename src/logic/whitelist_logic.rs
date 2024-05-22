use std::time::Duration;

use candid::Principal;
use ic_cdk_timers::set_timer;

use crate::logic::store::{Store, DATA};

use crate::ledger_declarations::types::{
    SharedData, Status, VoteResponse, VoteType, WhitelistRequestData, WhitelistRequestType,
};

use super::store::DAY_IN_NANOS;

impl Store {
    pub fn get_whitelist() -> Vec<Principal> {
        DATA.with(|data| data.borrow().whitelist.clone())
    }

    pub fn get_whitelist_requests(status: Option<Status>) -> Vec<WhitelistRequestData> {
        DATA.with(|data| {
            let data = data.borrow();

            let mut whitelist_requests = data
                .whitelist_requests
                .values()
                .filter(|request| {
                    if let Some(status) = &status {
                        &request.data.status == status
                    } else {
                        true
                    }
                })
                .cloned()
                .collect::<Vec<WhitelistRequestData>>();

            whitelist_requests.sort_by(|a, b| a.data.created_at.cmp(&b.data.created_at));
            whitelist_requests
        })
    }

    pub fn whitelist_request(
        caller: Principal,
        request_type: WhitelistRequestType,
    ) -> Result<String, String> {
        Self::is_whitelisted(&caller)?;
        Self::check_duplicate_whitelist_request(&request_type)?;

        use WhitelistRequestType::*;
        if let Add(principal) = &request_type {
            if Self::is_whitelisted(principal).is_ok() {
                return Err("Principal already whitelisted".to_string());
            }
        }

        if let Remove(principal) = &request_type {
            if Self::is_whitelisted(principal).is_err() {
                return Err("Principal not whitelisted".to_string());
            }
        }

        let id = DATA.with(|data| {
            let mut data = data.borrow_mut();
            let whitelist_request_id = data.whitelist_request_id;

            let whitelist_data = WhitelistRequestData {
                request_type,
                data: SharedData::new(whitelist_request_id),
            };
            data.whitelist_request_id += 1;
            data.whitelist_requests
                .insert(whitelist_request_id, whitelist_data.clone());

            whitelist_request_id
        });

        set_timer(Duration::from_nanos(DAY_IN_NANOS), move || {
            Self::expire_airdrop_request(&id);
        });

        Self::vote_on_whitelist_request(caller, id, VoteType::Approve)
    }

    pub fn vote_on_whitelist_request(
        caller: Principal,
        request_id: u32,
        vote: VoteType,
    ) -> Result<String, String> {
        Self::is_whitelisted(&caller)?;

        DATA.with(|data| {
            let mut data = data.borrow_mut();

            let whitelist_request = data
                .whitelist_requests
                .get_mut(&request_id)
                .ok_or("Whitelist request not found")?;

            if whitelist_request.data.status != Status::Pending {
                return Err("Whitelist request is not pending".to_string());
            }

            Self::check_duplicate_vote(&caller, &whitelist_request.data.votes)?;

            if vote == VoteType::Approve {
                whitelist_request.data.votes.approvals.push(caller);
                Ok(VoteType::Approve)
            } else {
                whitelist_request.data.votes.rejections.push(caller);
                Ok(VoteType::Reject)
            }
        })?;

        if let Ok(vote_type) = Self::get_whitelist_request_majority(request_id) {
            use VoteResponse::*;
            match vote_type {
                Approve => Self::approve_whitelist_request(request_id),
                Reject => Self::reject_whitelist_request(request_id, false),
                Deadlock => Self::reject_whitelist_request(request_id, true),
            }
        } else {
            Err("No majority reached".to_string())
        }
    }

    fn get_whitelist_request_majority(request_id: u32) -> Result<VoteResponse, String> {
        DATA.with(|data| {
            let mut data = data.borrow_mut();

            let mut whitelist = data.whitelist.clone();

            let whitelist_request = data
                .whitelist_requests
                .get_mut(&request_id)
                .ok_or("Whitelist request not found".to_string())?;

            if let WhitelistRequestType::Remove(principal) = whitelist_request.request_type {
                whitelist.retain(|p| p != &principal);
            }

            let whitelist_count = whitelist.len() as f32;
            let approval_count = whitelist_request.data.votes.approvals.len() as f32;
            let rejection_count = whitelist_request.data.votes.rejections.len() as f32;

            let approval_percentage = (approval_count / whitelist_count) * 100.0;
            let rejection_percentage = (rejection_count / whitelist_count) * 100.0;

            use VoteResponse::*;
            if approval_percentage > 50.0 {
                Ok(Approve)
            } else if rejection_percentage > 50.0 {
                Ok(Reject)
            } else if approval_percentage == 50.0 && rejection_percentage == 50.0 {
                Ok(Deadlock)
            } else {
                Err("No majority reached".to_string())
            }
        })
    }

    fn approve_whitelist_request(request_id: u32) -> Result<String, String> {
        let request_type = DATA.with(|data| {
            let mut data = data.borrow_mut();
            let whitelist_request = data.whitelist_requests.get_mut(&request_id);

            match whitelist_request {
                Some(_request) => {
                    let request_type = _request.request_type.clone();
                    _request.data.status = Status::Approved;
                    Ok(request_type)
                }
                None => Err("Whitelist request not found".to_string()),
            }
        });

        DATA.with(|data| {
            let mut data = data.borrow_mut();
            use WhitelistRequestType::*;
            match request_type {
                Ok(Add(principal)) => {
                    data.whitelist.push(principal);
                    Ok("Whitelist request approved".to_string())
                }
                Ok(Remove(principal)) => {
                    match data.whitelist.iter().position(|x| *x == principal) {
                        Some(i) => {
                            data.whitelist.remove(i);
                            Ok("Whitelist request approved".to_string())
                        }
                        None => Err("Principal not found in whitelist".to_string()),
                    }
                }
                Err(err) => Err(err),
            }
        })
    }

    fn reject_whitelist_request(request_id: u32, deadlock: bool) -> Result<String, String> {
        DATA.with(|data| {
            let mut data = data.borrow_mut();
            let whitelist_request = data
                .whitelist_requests
                .get_mut(&request_id)
                .ok_or("Whitelist request not found".to_string())?;

            if deadlock {
                whitelist_request.data.status = Status::Deadlock;
                Ok("Whitelist request deadlocked".to_string())
            } else {
                whitelist_request.data.status = Status::Rejected;
                Ok("Whitelist request rejected".to_string())
            }
        })
    }

    fn check_duplicate_whitelist_request(request: &WhitelistRequestType) -> Result<(), String> {
        DATA.with(|data| {
            let data = &data.borrow();
            let whitelist_requests = &data.whitelist_requests;

            match request {
                WhitelistRequestType::Add(principal) => {
                    if whitelist_requests.iter().any(|(_, _request)| {
                        _request.request_type.principal() == principal
                            && _request.data.status == Status::Pending
                    }) {
                        return Err("Already a pending add request".to_string());
                    }
                }
                WhitelistRequestType::Remove(principal) => {
                    if whitelist_requests.iter().any(|(_, _request)| {
                        _request.request_type.principal() == principal
                            && _request.data.status == Status::Pending
                    }) {
                        return Err("Already a pending remove request".to_string());
                    }
                }
            }

            Ok(())
        })
    }

    pub fn expire_whitelist_request(request_id: &u32) {
        DATA.with(|data| {
            let mut data = data.borrow_mut();

            if let Some(_request) = data.whitelist_requests.get_mut(request_id) {
                if _request.data.status == Status::Pending {
                    _request.data.status = Status::Expired;
                }
            }
        })
    }
}
