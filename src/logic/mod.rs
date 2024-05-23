// use candid::{CandidType, Principal};

// use serde::Deserialize;

// use crate::models::types::{
//     AirdropRequest, AirdropTransaction, TransactionRequest, Votes, WhitelistRequest,
// };

pub mod airdrop_logic;
pub mod consts;
pub mod transaction_logic;
pub mod whitelist_logic;

// pub use airdrop_logic::AirdropLogic;
pub use consts::*;
// pub use transaction_logic::TransactionLogic;
pub use whitelist_logic::WhitelistLogic;
// #[derive(Deserialize, CandidType)]
// pub struct Store {
//     pub owner: Principal,

//     pub whitelist: Vec<Principal>,
//     pub whitelist_request_id: u32,
//     pub whitelist_requests: HashMap<u32, WhitelistRequest>,
//     pub whitelist_request_expiry: u64,

//     pub transaction_request_id: u32,
//     pub transaction_requests: HashMap<u32, TransactionRequest>,
//     pub transaction_request_expiry: u64,

//     // pub airdrop_transactions: HashMap<u32, Vec<AirdropTransactionDetails>>,
//     pub airdrop_transactions: HashMap<u32, HashMap<Principal, AirdropTransaction>>,
//     pub airdrop_error: HashMap<u32, String>,
//     pub airdrop_request_id: u32,
//     pub airdrop_requests: HashMap<u32, AirdropRequest>,
//     pub airdrop_request_expiry: u64,
// }

// impl Default for Store {
//     fn default() -> Self {
//         Self {
//             owner: Principal::anonymous(),
//             whitelist: Default::default(),
//             whitelist_request_id: 0,
//             whitelist_requests: Default::default(),
//             whitelist_request_expiry: DAY_IN_NANOS,

//             transaction_request_id: 0,
//             transaction_requests: Default::default(),
//             transaction_request_expiry: DAY_IN_NANOS,

//             airdrop_transactions: Default::default(),
//             airdrop_error: Default::default(),
//             airdrop_request_id: 0,
//             airdrop_requests: Default::default(),
//             airdrop_request_expiry: DAY_IN_NANOS,
//         }
//     }
// }

// thread_local! {
//     pub static DATA: RefCell<Store>  = RefCell::new(Store::default());
// }

// impl Store {
//     pub fn init(owner: Principal) {
//         DATA.with(|data| {
//             let mut data = data.borrow_mut();
//             data.owner = owner;
//             data.whitelist.push(owner);
//         });
//     }

//     pub fn get_airdrop_error(caller: Principal, request_id: u32) -> Result<String, String> {
//         DATA.with(|data| {
//             let data = data.borrow();

//             if !data.whitelist.contains(&caller) {
//                 return Err("Caller is not whitelisted".to_string());
//             };

//             match data.airdrop_error.get(&request_id) {
//                 Some(error) => Ok(error.clone()),
//                 None => Err("No error found".to_string()),
//             }
//         })
//     }

//     pub fn is_whitelisted(caller: &Principal) -> Result<(), String> {
//         DATA.with(|data| {
//             let data = &data.borrow();
//             if data.whitelist.contains(caller) || caller == &data.owner {
//                 Ok(())
//             } else {
//                 Err("Caller is not whitelisted".to_string())
//             }
//         })
//     }

//     pub fn check_duplicate_vote(caller: &Principal, votes: &Votes) -> Result<(), String> {
//         if votes.approvals.contains(caller) {
//             Err("Approval vote already cast".to_string())
//         } else if votes.rejections.contains(caller) {
//             return Err("Rejection vote already cast".to_string());
//         } else {
//             Ok(())
//         }
//     }
// }
