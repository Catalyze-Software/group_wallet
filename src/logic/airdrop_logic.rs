use std::time::Duration;

use candid::{Nat, Principal};
use ic_cdk::api::time;
use ic_cdk_timers::set_timer;

use crate::logic::store::{Store, DATA};

use crate::rust_declarations::types::{
    AirdropRequestData, AirdropTransactionDetails, Amount, SharedData, Status, TokenStandard,
    TransferRequestType, VoteResponse, VoteType, Votes,
};

use super::store::DAY_IN_NANOS;

impl Store {
    pub fn get_airdrops() -> Vec<AirdropRequestData> {
        DATA.with(|data| {
            let data = data.borrow();
            data.airdrop_requests
                .values()
                .cloned()
                .collect::<Vec<AirdropRequestData>>()
        })
    }

    pub fn get_airdrop_requests(status: Option<Status>) -> Vec<AirdropRequestData> {
        DATA.with(|data| {
            let data = data.borrow();
            let mut airdrop_requests = data
                .airdrop_requests
                .values()
                .filter(|request| {
                    if let Some(status) = status.clone() {
                        request.data.status == status
                    } else {
                        true
                    }
                })
                .cloned()
                .collect::<Vec<AirdropRequestData>>();
            airdrop_requests.sort_by(|a, b| a.data.created_at.cmp(&b.data.created_at));
            airdrop_requests
        })
    }

    pub async fn airdrop_request(
        caller: Principal,
        transfer_args: Vec<TransferRequestType>,
        canister_id: Principal,
    ) -> Result<String, String> {
        if let Err(err) = Self::is_whitelisted(&caller) {
            return Err(err);
        }

        if let Err(err) = Self::check_balance(canister_id, transfer_args.clone()).await {
            return Err(err);
        }

        let id = DATA.with(|data| {
            let mut data = data.borrow_mut();
            let airdrop_request_id = data.airdrop_request_id;

            let airdrop_data = AirdropRequestData {
                tranfer_args: transfer_args.clone(),
                data: SharedData {
                    id: airdrop_request_id,
                    status: Status::Pending,
                    votes: Votes {
                        approvals: vec![],
                        rejections: vec![],
                    },
                    requested_by: caller,
                    created_at: time(),
                },
                canister_id,
            };
            data.airdrop_request_id += 1;
            data.airdrop_requests
                .insert(airdrop_request_id.clone(), airdrop_data.clone());

            airdrop_request_id
        });

        set_timer(Duration::from_nanos(DAY_IN_NANOS), move || {
            Self::expire_airdrop_request(&id);
        });

        Self::vote_on_whitelist_request(caller, id, VoteType::Approve)
    }

    pub fn get_airdrop_transactions(
        caller: Principal,
        request_id: u32,
    ) -> Result<Vec<AirdropTransactionDetails>, String> {
        DATA.with(|data| {
            let data = data.borrow();

            if !data.whitelist.contains(&caller) {
                return Err("Caller is not whitelisted".to_string());
            };

            let result = data
                .airdrop_transactions
                .get(&request_id)
                .cloned()
                .unwrap_or(vec![]);

            Ok(result)
        })
    }

    pub async fn vote_on_airdrop_request(
        caller: Principal,
        request_id: u32,
        vote: VoteType,
    ) -> Result<String, String> {
        let result = DATA.with(|data| {
            let mut data = data.borrow_mut();

            if !data.whitelist.contains(&caller) {
                return Err("Caller is not whitelisted".to_string());
            }

            let airdrop_request = data
                .airdrop_requests
                .get_mut(&request_id)
                .ok_or("Airdrop request not found")?;

            if airdrop_request.data.status != Status::Pending {
                return Err("Airdrop request is not pending".to_string());
            }

            if let Err(err) = Self::check_duplicate_vote(&caller, &airdrop_request.data.votes) {
                return Err(err);
            }

            if vote == VoteType::Approve {
                airdrop_request.data.votes.approvals.push(caller);
                return Ok(VoteType::Approve);
            } else {
                airdrop_request.data.votes.rejections.push(caller);
                return Ok(VoteType::Reject);
            }
        });

        match result {
            Ok(_) => {
                if let Ok(vote_type) = Self::get_airdrop_request_majority(request_id) {
                    match vote_type {
                        VoteResponse::Approve => {
                            let result = Self::approve_airdrop_request(request_id).await;
                            match result {
                                Ok(transactions) => DATA.with(|d| {
                                    d.borrow_mut()
                                        .airdrop_transactions
                                        .insert(request_id, transactions);
                                    return Ok("Airdrop request approved".to_string());
                                }),
                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        }
                        VoteResponse::Reject => {
                            return Self::reject_airdrop_request(request_id, false);
                        }
                        VoteResponse::Deadlock => {
                            return Self::reject_airdrop_request(request_id, true);
                        }
                    }
                } else {
                    return Err("No marjority reached".to_string());
                }
            }
            Err(err) => return Err(err),
        }
    }

    fn get_airdrop_request_majority(request_id: u32) -> Result<VoteResponse, String> {
        DATA.with(|data| {
            let mut data = data.borrow_mut();

            let whitelist = data.whitelist.clone();

            let airdrop_request = data
                .airdrop_requests
                .get_mut(&request_id)
                .ok_or("Whitelist request not found".to_string())?;

            let whitelist_count = whitelist.len() as f32;
            let approval_count = airdrop_request.data.votes.approvals.len() as f32;
            let rejection_count = airdrop_request.data.votes.rejections.len() as f32;

            let approval_percentage = (approval_count / whitelist_count) * 100.0;
            let rejection_percentage = (rejection_count / whitelist_count) * 100.0;

            if approval_percentage > 50.0 {
                return Ok(VoteResponse::Approve);
            } else if rejection_percentage > 50.0 {
                return Ok(VoteResponse::Reject);
            } else if approval_percentage == 50.0 && rejection_percentage == 50.0 {
                return Ok(VoteResponse::Deadlock);
            } else {
                return Err("No marjority reached".to_string());
            }
        })
    }

    async fn approve_airdrop_request(
        request_id: u32,
    ) -> Result<Vec<AirdropTransactionDetails>, String> {
        let request = DATA.with(|data| {
            let mut data = data.borrow_mut();
            let airdrop_request = data.airdrop_requests.get_mut(&request_id);

            match airdrop_request {
                Some(_request) => {
                    _request.data.status = Status::Approved;
                    return Ok(_request.clone());
                }
                None => {
                    return Err("Whitelist request not found".to_string());
                }
            }
        });

        match request {
            Err(err) => Err(err),
            Ok(_request) => {
                let mut transaction_results: Vec<AirdropTransactionDetails> = vec![];

                for args in _request.tranfer_args {
                    match args {
                        TransferRequestType::DIP20(_args) => {
                            if let Ok(_) =
                                Store::transfer_dip20(_request.canister_id, _args.clone()).await
                            {
                                transaction_results.push(AirdropTransactionDetails {
                                    status: Status::Approved,
                                    receiver: _args.to,
                                    amount: Amount::DIP20(_args.amount),
                                    canister_id: _request.canister_id,
                                    token_standard: TokenStandard::DIP20,
                                });
                            } else {
                                transaction_results.push(AirdropTransactionDetails {
                                    status: Status::Rejected,
                                    receiver: _args.to,
                                    amount: Amount::DIP20(_args.amount),
                                    canister_id: _request.canister_id,
                                    token_standard: TokenStandard::DIP20,
                                });
                            }
                        }
                        TransferRequestType::ICRC1(_args) => {
                            if let Ok(_) =
                                Self::transfer_icrc(_request.canister_id, _args.clone()).await
                            {
                                transaction_results.push(AirdropTransactionDetails {
                                    status: Status::Approved,
                                    receiver: _args.to.owner,
                                    amount: Amount::ICRC1(_args.amount),
                                    canister_id: _request.canister_id,
                                    token_standard: TokenStandard::ICRC1,
                                });
                            } else {
                                transaction_results.push(AirdropTransactionDetails {
                                    status: Status::Rejected,
                                    receiver: _args.to.owner,
                                    amount: Amount::ICRC1(_args.amount),
                                    canister_id: _request.canister_id,
                                    token_standard: TokenStandard::ICRC1,
                                });
                            }
                        }
                    }
                }
                return Ok(transaction_results);
            }
        }
    }

    fn reject_airdrop_request(request_id: u32, deadlock: bool) -> Result<String, String> {
        DATA.with(|data| {
            let mut data = data.borrow_mut();
            let whitelist_request = data
                .whitelist_requests
                .get_mut(&request_id)
                .ok_or("Whitelist request not found".to_string())?;

            if deadlock {
                whitelist_request.data.status = Status::Deadlock;
                return Ok("Whitelist request deadlocked".to_string());
            } else {
                whitelist_request.data.status = Status::Rejected;
                return Ok("Whitelist request rejected".to_string());
            }
        })
    }

    async fn check_balance(
        canister_id: Principal,
        transfer_args: Vec<TransferRequestType>,
    ) -> Result<(), String> {
        let mut dip20total = 0;
        let mut icrc1total: Nat = Nat::from(0);

        for args in transfer_args {
            match args {
                TransferRequestType::DIP20(_args) => {
                    dip20total += _args.amount;
                }
                TransferRequestType::ICRC1(_args) => {
                    icrc1total += _args.amount;
                }
            }
        }

        if dip20total > 0 {
            let balance = Store::balance_check_dip20(canister_id, &dip20total).await;
            match balance {
                Ok(_) => {}
                Err(err) => return Err(err),
            }
        }

        if icrc1total > Nat::from(0) {
            let balance = Store::balance_check_icrc(canister_id, &icrc1total).await;
            match balance {
                Ok(_) => {}
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    pub fn expire_airdrop_request(request_id: &u32) {
        DATA.with(|data| {
            let mut data = data.borrow_mut();
            let whitelist_request = data.airdrop_requests.get_mut(request_id);

            match whitelist_request {
                Some(_request) => {
                    if _request.data.status == Status::Pending {
                        _request.data.status = Status::Expired;
                    }
                }
                None => {}
            }
        })
    }
}
