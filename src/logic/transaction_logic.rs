use candid::{Nat, Principal};
use ic_cdk::api::time;
use ic_cdk::id;

use crate::logic::store::{Store, DATA};

use crate::rust_declarations::dip20_declaration::Dip20Service;
use crate::rust_declarations::icrc_declaration::{Account, IcrcService, TransferArgs};
use crate::rust_declarations::types::{
    Dip20TransferArgs, SharedData, Status, TransactionRequestData, TransferRequestType,
    VoteResponse, VoteType, Votes,
};

use super::store::DAY_IN_NANOS;

impl Store {
    pub fn get_transaction_requests(status: Option<Status>) -> Vec<TransactionRequestData> {
        DATA.with(|data| {
            let data = data.borrow();
            let mut transaction_requests = data
                .transaction_requests
                .values()
                .filter(|request| {
                    if let Some(status) = status.clone() {
                        request.data.status == status
                    } else {
                        true
                    }
                })
                .cloned()
                .collect::<Vec<TransactionRequestData>>();
            transaction_requests.sort_by(|a, b| a.data.created_at.cmp(&b.data.created_at));
            transaction_requests
        })
    }

    pub async fn transaction_request(
        caller: Principal,
        canister_id: Principal,
        args: TransferRequestType,
    ) -> Result<String, String> {
        if let Err(err) = Self::is_whitelisted(&caller) {
            return Err(err);
        }

        let has_balance = match &args {
            TransferRequestType::DIP20(args) => {
                Self::balance_check_dip20(&Dip20Service(canister_id), args).await
            }
            TransferRequestType::ICRC1(args) => {
                Self::balance_check_icrc(&IcrcService(canister_id), args).await
            }
        };

        match has_balance {
            Err(err) => return Err(err),
            Ok(_) => {
                let id = DATA.with(|data| {
                    let mut data = data.borrow_mut();
                    let transaction_request_id = data.transaction_request_id;

                    let transaction_data = TransactionRequestData {
                        args,
                        canister_id,
                        data: SharedData {
                            id: transaction_request_id,
                            status: Status::Pending,
                            votes: Votes {
                                approvals: vec![],
                                rejections: vec![],
                            },
                            requested_by: caller,
                            created_at: time(),
                        },
                    };
                    data.transaction_request_id += 1;
                    data.transaction_requests
                        .insert(transaction_request_id.clone(), transaction_data.clone());
                    transaction_request_id
                });

                Self::vote_on_transaction_request(caller, id, VoteType::Approve).await
            }
        }
    }

    pub async fn vote_on_transaction_request(
        caller: Principal,
        request_id: u32,
        vote: VoteType,
    ) -> Result<String, String> {
        // expire whitelist requests
        if let Err(err) = Self::expire_transaction_requests(&request_id) {
            return Err(err);
        }

        let result = DATA.with(|data| {
            let mut data = data.borrow_mut();

            if !data.whitelist.contains(&caller) {
                return Err("Caller is not whitelisted".to_string());
            }

            let transactions_request = data
                .transaction_requests
                .get_mut(&request_id)
                .ok_or("Transaction request not found")?;

            if transactions_request.data.status != Status::Pending {
                return Err("Transaction request is not pending".to_string());
            }

            if let Err(err) = Self::check_duplicate_vote(&caller, &transactions_request.data.votes)
            {
                return Err(err);
            }

            if vote == VoteType::Approve {
                transactions_request.data.votes.approvals.push(caller);
                return Ok(VoteType::Approve);
            } else {
                transactions_request.data.votes.rejections.push(caller);
                return Ok(VoteType::Reject);
            }
        });

        match result {
            Ok(_) => {
                if let Ok(vote_type) = Self::get_transaction_request_majority(request_id) {
                    match vote_type {
                        VoteResponse::Approve => {
                            return Self::approve_transaction_request(request_id).await;
                        }
                        VoteResponse::Reject => {
                            return Self::reject_transaction_request(request_id, false);
                        }
                        VoteResponse::Deadlock => {
                            return Self::reject_transaction_request(request_id, true);
                        }
                    }
                } else {
                    return Err("No marjority reached".to_string());
                }
            }
            Err(err) => return Err(err),
        }
    }

    fn get_transaction_request_majority(request_id: u32) -> Result<VoteResponse, String> {
        DATA.with(|data| {
            let mut data = data.borrow_mut();

            let whitelist = data.whitelist.clone();

            let transaction_request = data
                .transaction_requests
                .get_mut(&request_id)
                .ok_or("Transaction request not found".to_string())?;

            let whitelist_count = whitelist.len() as f32;
            let approval_count = transaction_request.data.votes.approvals.len() as f32;
            let rejection_count = transaction_request.data.votes.rejections.len() as f32;

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

    async fn approve_transaction_request(request_id: u32) -> Result<String, String> {
        let request = DATA.with(|data| {
            let mut data = data.borrow_mut();
            let transaction_request = data.transaction_requests.get_mut(&request_id);

            match transaction_request {
                Some(_request) => {
                    _request.data.status = Status::Approved;
                    return Ok(_request.clone());
                }
                None => {
                    return Err("Transaction request not found".to_string());
                }
            }
        });

        match request {
            Err(err) => Err(err),
            Ok(_request) => match _request.args {
                TransferRequestType::DIP20(args) => {
                    if let Ok(_) = Self::transfer_dip20(_request.canister_id, args).await {
                        return Ok("DIP20 transaction send request approved".to_string());
                    } else {
                        return Err("DIP20 transaction send request failed".to_string());
                    }
                }
                TransferRequestType::ICRC1(args) => {
                    if let Ok(_) = Self::transfer_icrc(_request.canister_id, args).await {
                        return Ok("ICRC transaction send request approved".to_string());
                    } else {
                        return Err("ICRC transaction send request failed".to_string());
                    }
                }
            },
        }
    }

    fn reject_transaction_request(request_id: u32, deadlock: bool) -> Result<String, String> {
        DATA.with(|data| {
            let mut data = data.borrow_mut();
            let transaction_request = data
                .transaction_requests
                .get_mut(&request_id)
                .ok_or("Transaction request not found".to_string())?;

            if deadlock {
                transaction_request.data.status = Status::Deadlock;
                return Ok("Transaction request deadlocked".to_string());
            } else {
                transaction_request.data.status = Status::Rejected;
                return Ok("Transaction request rejected".to_string());
            }
        })
    }

    pub fn expire_transaction_requests(request_id: &u32) -> Result<(), String> {
        DATA.with(|data| {
            let mut data = data.borrow_mut();
            let transaction_request = data.transaction_requests.get_mut(request_id);

            match transaction_request {
                Some(_request) => {
                    if (_request.data.created_at + DAY_IN_NANOS) < time()
                        && _request.data.status == Status::Pending
                    {
                        _request.data.status = Status::Expired;
                        return Err("Transaction request expired".to_string());
                    } else {
                        Ok(())
                    }
                }
                None => {
                    return Err("Whitelist request not found".to_string());
                }
            }
        })
    }

    async fn transfer_dip20(canister_id: Principal, args: Dip20TransferArgs) -> Result<(), String> {
        let actor = Dip20Service(canister_id);
        match Self::balance_check_dip20(&actor, &args).await {
            Err(err) => Err(err),
            Ok(()) => {
                let result = actor.transfer(args.to, Nat::from(args.amount)).await;
                match result {
                    Ok(_) => Ok(()),
                    Err((_, err)) => Err(err),
                }
            }
        }
    }

    async fn balance_check_dip20(
        actor: &Dip20Service,
        args: &Dip20TransferArgs,
    ) -> Result<(), String> {
        let balance = actor.balance_of(id()).await;

        match balance {
            Err((_, err)) => Err(err),
            Ok((balance,)) => {
                if balance < args.amount {
                    return Err("Insufficient balance".to_string());
                }

                Ok(())
            }
        }
    }

    async fn balance_check_icrc(actor: &IcrcService, args: &TransferArgs) -> Result<(), String> {
        let balance = actor
            .icrc1_balance_of(Account {
                owner: id(),
                subaccount: None,
            })
            .await;

        match balance {
            Err((_, err)) => Err(err),
            Ok((balance,)) => {
                if balance < args.amount {
                    return Err("Insufficient balance".to_string());
                }

                Ok(())
            }
        }
    }

    async fn transfer_icrc(canister_id: Principal, args: TransferArgs) -> Result<(), String> {
        let actor = IcrcService(canister_id);

        let balance = actor
            .icrc1_balance_of(Account {
                owner: id(),
                subaccount: None,
            })
            .await;

        match balance {
            Err((_, err)) => Err(err),
            Ok((balance,)) => {
                if balance < args.amount {
                    return Err("Insufficient balance".to_string());
                }

                let result = actor.icrc1_transfer(args).await;

                match result {
                    Ok(_) => Ok(()),
                    Err((_, err)) => Err(err),
                }
            }
        }
    }
}
