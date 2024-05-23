// use std::collections::HashMap;
// use std::time::Duration;

// use candid::{Nat, Principal};
// use ic_cdk_timers::set_timer;

// use crate::logic::store::{Store, DATA};

// use crate::models::types::{
//     AirdropRequest, AirdropTransaction, Amount, SharedData, Status, TokenStandard,
//     TransferRequestType, VoteResponse, VoteType,
// };

// use super::store::DAY_IN_NANOS;

// impl Store {
//     pub fn get_airdrops() -> Vec<AirdropRequest> {
//         DATA.with(|data| {
//             data.borrow()
//                 .airdrop_requests
//                 .values()
//                 .cloned()
//                 .collect::<Vec<AirdropRequest>>()
//         })
//     }

//     pub fn get_airdrop_requests(status: Option<Status>) -> Vec<AirdropRequest> {
//         DATA.with(|data| {
//             let mut airdrop_requests = data
//                 .borrow()
//                 .airdrop_requests
//                 .values()
//                 .filter(|request| {
//                     if let Some(status) = status.clone() {
//                         request.data.status == status
//                     } else {
//                         true
//                     }
//                 })
//                 .cloned()
//                 .collect::<Vec<AirdropRequest>>();

//             airdrop_requests.sort_by(|a, b| a.data.created_at.cmp(&b.data.created_at));
//             airdrop_requests
//         })
//     }

//     pub async fn airdrop_request(
//         caller: Principal,
//         transfer_args: Vec<TransferRequestType>,
//         canister_id: Principal,
//     ) -> Result<String, String> {
//         Self::is_whitelisted(&caller)?;
//         Self::check_balance(canister_id, transfer_args.clone()).await?;

//         let id = DATA.with(|data| {
//             let mut data = data.borrow_mut();
//             let airdrop_request_id = data.airdrop_request_id;

//             let airdrop_data = AirdropRequest {
//                 tranfer_args: transfer_args.clone(),
//                 data: SharedData::new(airdrop_request_id),
//                 canister_id,
//             };
//             data.airdrop_request_id += 1;
//             data.airdrop_requests
//                 .insert(airdrop_request_id, airdrop_data.clone());

//             airdrop_request_id
//         });

//         set_timer(Duration::from_nanos(DAY_IN_NANOS), move || {
//             Self::expire_airdrop_request(&id);
//         });

//         ic_cdk::spawn(Self::_vote_on_airdrop_request(
//             caller,
//             id,
//             VoteType::Approve,
//         ));
//         Ok("Airdrop request created".to_string())
//     }

//     pub fn get_airdrop_transactions(
//         caller: Principal,
//         request_id: u32,
//     ) -> Result<Vec<AirdropTransaction>, String> {
//         DATA.with(|data| {
//             let data = data.borrow();

//             if !data.whitelist.contains(&caller) {
//                 return Err("Caller is not whitelisted".to_string());
//             };

//             let result = data
//                 .airdrop_transactions
//                 .get(&request_id)
//                 .cloned()
//                 .unwrap_or(HashMap::default())
//                 .values()
//                 .cloned()
//                 .collect::<Vec<AirdropTransaction>>();

//             Ok(result)
//         })
//     }

//     pub async fn _vote_on_airdrop_request(caller: Principal, request_id: u32, vote: VoteType) {
//         let _ = Self::vote_on_airdrop_request(caller, request_id, vote).await;
//     }

//     pub async fn vote_on_airdrop_request(
//         caller: Principal,
//         request_id: u32,
//         vote: VoteType,
//     ) -> Result<String, String> {
//         DATA.with(|data| {
//             let mut data = data.borrow_mut();

//             if !data.whitelist.contains(&caller) {
//                 return Err("Caller is not whitelisted".to_string());
//             }

//             let airdrop_request = data
//                 .airdrop_requests
//                 .get_mut(&request_id)
//                 .ok_or("Airdrop request not found")?;

//             if airdrop_request.data.status != Status::Pending {
//                 return Err("Airdrop request is not pending".to_string());
//             }

//             Self::check_duplicate_vote(&caller, &airdrop_request.data.votes)?;

//             if vote == VoteType::Approve {
//                 airdrop_request.data.add_approve_vote(caller);
//                 Ok(VoteType::Approve)
//             } else {
//                 airdrop_request.data.add_reject_vote(caller);
//                 Ok(VoteType::Reject)
//             }
//         })?;

//         let majority = Self::get_airdrop_request_majority(request_id)?;

//         use VoteResponse::*;
//         match majority {
//             Approve => {
//                 ic_cdk::spawn(Self::approve_airdrop_request(request_id));
//                 Ok("Airdrop request approved".to_string())
//             }
//             Reject => Self::reject_airdrop_request(request_id, false),
//             Deadlock => Self::reject_airdrop_request(request_id, true),
//         }
//     }

//     fn get_airdrop_request_majority(request_id: u32) -> Result<VoteResponse, String> {
//         DATA.with(|data| {
//             let mut data = data.borrow_mut();

//             let whitelist = data.whitelist.clone();

//             let airdrop_request = data
//                 .airdrop_requests
//                 .get_mut(&request_id)
//                 .ok_or("Airdrop request not found".to_string())?;

//             let whitelist_count = whitelist.len() as f32;
//             let approval_count = airdrop_request.data.votes.approvals.len() as f32;
//             let rejection_count = airdrop_request.data.votes.rejections.len() as f32;

//             let approval_percentage = (approval_count / whitelist_count) * 100.0;
//             let rejection_percentage = (rejection_count / whitelist_count) * 100.0;

//             if approval_percentage > 50.0 {
//                 Ok(VoteResponse::Approve)
//             } else if rejection_percentage > 50.0 {
//                 Ok(VoteResponse::Reject)
//             } else if approval_percentage == 50.0 && rejection_percentage == 50.0 {
//                 Ok(VoteResponse::Deadlock)
//             } else {
//                 Err("No majority reached".to_string())
//             }
//         })
//     }

//     async fn approve_airdrop_request(request_id: u32) {
//         let request = DATA.with(|data| {
//             let mut data = data.borrow_mut();

//             match data.airdrop_requests.get_mut(&request_id) {
//                 Some(_request) => {
//                     _request.data.status = Status::Approved;
//                     Ok(_request.clone())
//                 }
//                 None => Err("Airdrop request not found".to_string()),
//             }
//         });

//         match request {
//             Err(err) => {
//                 let _ = DATA.with(|data| data.borrow_mut().airdrop_error.insert(request_id, err));
//             }
//             Ok(_request) => {
//                 for args in _request.tranfer_args {
//                     match args {
//                         TransferRequestType::DIP20(_args) => {
//                             if (Store::transfer_dip20(_request.canister_id, _args.clone()).await)
//                                 .is_ok()
//                             {
//                                 let details = AirdropTransaction {
//                                     status: Status::Approved,
//                                     receiver: _args.to,
//                                     amount: Amount::DIP20(_args.amount),
//                                     canister_id: _request.canister_id,
//                                     token_standard: TokenStandard::DIP20,
//                                 };

//                                 DATA.with(|_data| {
//                                     _data
//                                         .borrow_mut()
//                                         .airdrop_transactions
//                                         .entry(request_id)
//                                         .or_default()
//                                         .insert(_args.to, details);
//                                 });
//                             } else {
//                                 let details = AirdropTransaction {
//                                     status: Status::Rejected,
//                                     receiver: _args.to,
//                                     amount: Amount::DIP20(_args.amount),
//                                     canister_id: _request.canister_id,
//                                     token_standard: TokenStandard::DIP20,
//                                 };

//                                 DATA.with(|_data| {
//                                     _data
//                                         .borrow_mut()
//                                         .airdrop_transactions
//                                         .entry(request_id)
//                                         .or_default()
//                                         .insert(_args.to, details);
//                                 });
//                             }
//                         }
//                         TransferRequestType::ICRC1(_args) => {
//                             if (Self::transfer_icrc(_request.canister_id, _args.clone()).await)
//                                 .is_ok()
//                             {
//                                 let details = AirdropTransaction {
//                                     status: Status::Approved,
//                                     receiver: _args.to.owner,
//                                     amount: Amount::ICRC1(_args.amount.clone()),
//                                     canister_id: _request.canister_id,
//                                     token_standard: TokenStandard::DIP20,
//                                 };

//                                 DATA.with(|_data| {
//                                     _data
//                                         .borrow_mut()
//                                         .airdrop_transactions
//                                         .entry(request_id)
//                                         .or_default()
//                                         .insert(_args.to.owner, details);
//                                 });
//                             } else {
//                                 let details = AirdropTransaction {
//                                     status: Status::Rejected,
//                                     receiver: _args.to.owner,
//                                     amount: Amount::ICRC1(_args.amount.clone()),
//                                     canister_id: _request.canister_id,
//                                     token_standard: TokenStandard::DIP20,
//                                 };

//                                 DATA.with(|_data| {
//                                     _data
//                                         .borrow_mut()
//                                         .airdrop_transactions
//                                         .entry(request_id)
//                                         .or_default()
//                                         .insert(_args.to.owner, details);
//                                 });
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     fn reject_airdrop_request(request_id: u32, deadlock: bool) -> Result<String, String> {
//         DATA.with(|data| {
//             let mut data = data.borrow_mut();
//             let airdrop_request = data
//                 .airdrop_requests
//                 .get_mut(&request_id)
//                 .ok_or("Airdrop request not found".to_string())?;

//             if deadlock {
//                 airdrop_request.data.status = Status::Deadlock;
//                 Ok("Airdrop request deadlocked".to_string())
//             } else {
//                 airdrop_request.data.status = Status::Rejected;
//                 Ok("Airdrop request rejected".to_string())
//             }
//         })
//     }

//     async fn check_balance(
//         canister_id: Principal,
//         transfer_args: Vec<TransferRequestType>,
//     ) -> Result<(), String> {
//         let mut dip20total = 0;
//         let mut icrc1total: Nat = Nat::from(0u32);

//         for args in transfer_args {
//             match args {
//                 TransferRequestType::DIP20(_args) => {
//                     dip20total += _args.amount;
//                 }
//                 TransferRequestType::ICRC1(_args) => {
//                     icrc1total += _args.amount;
//                 }
//             }
//         }

//         if dip20total > 0 {
//             let balance = Store::balance_check_dip20(canister_id, &dip20total).await;
//             match balance {
//                 Ok(_) => {}
//                 Err(err) => return Err(err),
//             }
//         }

//         if icrc1total > 0u32 {
//             let balance = Store::balance_check_icrc(canister_id, &icrc1total).await;
//             match balance {
//                 Ok(_) => {}
//                 Err(err) => return Err(err),
//             }
//         }

//         Ok(())
//     }

//     pub fn expire_airdrop_request(request_id: &u32) {
//         DATA.with(|data| {
//             let mut data = data.borrow_mut();
//             let airdrop_request = data.airdrop_requests.get_mut(request_id);

//             if let Some(_request) = airdrop_request {
//                 if _request.data.status == Status::Pending {
//                     _request.data.status = Status::Expired;
//                 }
//             }
//         })
//     }
// }
