use candid::Principal;
use ic_cdk::api::time;

use crate::logic::store::{Store, DATA};

use crate::rust_declarations::types::{
    SharedData, Status, VoteType, Votes, WhitelistRequestData, WhitelistRequestType,
};

impl Store {
    pub fn get_whitelist() -> Vec<Principal> {
        DATA.with(|data| {
            let data = data.borrow();
            data.whitelist.clone()
        })
    }

    pub fn get_whitelist_requests(status: Option<Status>) -> Vec<WhitelistRequestData> {
        DATA.with(|data| {
            let data = data.borrow();
            let mut whitelist_requests = data
                .whitelist_requests
                .values()
                .filter(|request| {
                    if let Some(status) = status.clone() {
                        request.data.status == status
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
        if let Err(err) = Self::is_whitelisted(&caller) {
            return Err(err);
        }

        if let Err(err) = Self::check_duplicate_whitelist_request(&request_type) {
            return Err(err);
        }

        let id = DATA.with(|data| {
            let mut data = data.borrow_mut();
            let whitelist_request_id = data.whitelist_request_id;
            data.whitelist_request_id += 1;

            let whitelist_data = WhitelistRequestData {
                request_type,
                data: SharedData {
                    status: Status::Pending,
                    votes: Votes {
                        approvals: vec![],
                        rejections: vec![],
                    },
                    requested_by: caller,
                    created_at: time(),
                },
            };
            data.whitelist_requests
                .insert(whitelist_request_id.clone(), whitelist_data.clone());
            whitelist_request_id
        });

        Self::vote_on_whitelist_request(caller, id, VoteType::Approve)
    }

    pub fn vote_on_whitelist_request(
        caller: Principal,
        request_id: u32,
        vote: VoteType,
    ) -> Result<String, String> {
        let result = DATA.with(|data| {
            let mut data = data.borrow_mut();

            if !data.whitelist.contains(&caller) {
                return Err("Caller is not whitelisted".to_string());
            }

            let whitelist_request = data
                .whitelist_requests
                .get_mut(&request_id)
                .ok_or("Whitelist request not found")?;

            if whitelist_request.data.status != Status::Pending {
                return Err("Whitelist request is not pending".to_string());
            }

            if let Err(err) = Self::check_duplicate_vote(&caller, &whitelist_request.data.votes) {
                return Err(err);
            }

            if vote == VoteType::Approve {
                whitelist_request.data.votes.approvals.push(caller);
                return Ok(VoteType::Approve);
            } else {
                whitelist_request.data.votes.rejections.push(caller);
                return Ok(VoteType::Reject);
            }
        });

        match result {
            Ok(_) => {
                if let Ok(vote_type) = Self::get_request_majority(request_id) {
                    if vote_type == VoteType::Approve {
                        return Self::approve_whitelist_request(request_id);
                    } else {
                        return Self::reject_whitelist_request(request_id);
                    }
                } else {
                    return Ok("No marjority reached yet".to_string());
                }
            }
            Err(err) => return Err(err),
        }
    }

    fn get_request_majority(request_id: u32) -> Result<VoteType, String> {
        DATA.with(|data| {
            let mut data = data.borrow_mut();

            let whitelist = data.whitelist.clone();

            let whitelist_request = data
                .whitelist_requests
                .get_mut(&request_id)
                .ok_or("Whitelist request not found".to_string())?;

            let whitelist_count = whitelist.len() as u32;
            let approval_count = whitelist_request.data.votes.approvals.len() as u32;
            let rejection_count = whitelist_request.data.votes.rejections.len() as u32;

            let majority = (whitelist_count / 2) + 1;

            if approval_count >= majority {
                return Ok(VoteType::Approve);
            } else if rejection_count >= majority {
                return Ok(VoteType::Reject);
            } else {
                return Err("No marjority reached".to_string());
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
                    return Ok(request_type);
                }
                None => {
                    return Err("Whitelist request not found".to_string());
                }
            }
        });

        DATA.with(|data| {
            let mut data = data.borrow_mut();
            match request_type {
                Ok(WhitelistRequestType::Add(principal)) => {
                    data.whitelist.push(principal);
                    return Ok("Whitelist request approved".to_string());
                }
                Ok(WhitelistRequestType::Remove(principal)) => {
                    let index = data.whitelist.iter().position(|x| *x == principal);
                    match index {
                        Some(i) => {
                            data.whitelist.remove(i);
                            return Ok("Whitelist request approved".to_string());
                        }
                        None => {
                            return Err("Principal not found in whitelist".to_string());
                        }
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        })
    }

    fn reject_whitelist_request(request_id: u32) -> Result<String, String> {
        DATA.with(|data| {
            let mut data = data.borrow_mut();
            let whitelist_request = data
                .whitelist_requests
                .get_mut(&request_id)
                .ok_or("Whitelist request not found".to_string())?;

            whitelist_request.data.status = Status::Rejected;
            Ok("Whitelist request rejected".to_string())
        })
    }

    fn check_duplicate_whitelist_request(request: &WhitelistRequestType) -> Result<(), String> {
        DATA.with(|data| {
            let data = &data.borrow();
            let whitelist_requests = &data.whitelist_requests;

            let has_pending_add_request = whitelist_requests.iter().any(|(_, _request)| {
                &_request.request_type == request && _request.data.status == Status::Pending
            });

            if has_pending_add_request {
                return Err("Already a pending add request".to_string());
            }

            let has_pending_remove_request = whitelist_requests.iter().any(|(_, _request)| {
                &_request.request_type == request && _request.data.status == Status::Pending
            });

            if has_pending_remove_request {
                return Err("Already a pending remove request".to_string());
            }

            return Ok(());
        })
    }
}
