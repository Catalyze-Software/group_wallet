use candid::{CandidType, Nat, Principal};
use ic_cdk::{api::time, caller};
use serde::Deserialize;

use super::icrc_declaration::TransferArg;

#[derive(CandidType, Deserialize, Clone)]
pub struct TransactionRequestData {
    pub args: TransferRequestType,
    pub canister_id: Principal,
    pub data: SharedData,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Dip20TransferArgs {
    pub to: Principal,
    pub amount: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub enum TransferRequestType {
    DIP20(Dip20TransferArgs),
    ICRC1(TransferArg),
}

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub enum TokenStandard {
    DIP20,
    ICRC1,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct WhitelistRequestData {
    pub request_type: WhitelistRequestType,
    pub data: SharedData,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct AirdropRequestData {
    pub tranfer_args: Vec<TransferRequestType>,
    pub canister_id: Principal,
    pub data: SharedData,
}

#[derive(CandidType, Deserialize, PartialEq, Eq)]
pub enum VoteType {
    Approve,
    Reject,
}

#[derive(CandidType, Deserialize, PartialEq, Eq)]
pub enum VoteResponse {
    Approve,
    Reject,
    Deadlock,
}

#[derive(CandidType, Deserialize, PartialEq, Eq, Clone)]
pub enum WhitelistRequestType {
    Add(Principal),
    Remove(Principal),
}

impl WhitelistRequestType {
    pub fn principal(&self) -> &Principal {
        match self {
            WhitelistRequestType::Add(principal) => principal,
            WhitelistRequestType::Remove(principal) => principal,
        }
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct SharedData {
    pub id: u32,
    pub status: Status,
    pub votes: Votes,
    pub requested_by: Principal,
    pub send_at: Option<u64>,
    pub created_at: u64,
}

impl SharedData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            status: Status::Pending,
            votes: Votes {
                approvals: Vec::new(),
                rejections: Vec::new(),
            },
            requested_by: caller(),
            send_at: None,
            created_at: time(),
        }
    }

    pub fn add_approve_vote(&mut self, caller: Principal) {
        if !self.votes.approvals.contains(&caller) && !self.votes.rejections.contains(&caller) {
            self.votes.approvals.push(caller);
        }
    }

    pub fn add_reject_vote(&mut self, caller: Principal) {
        if !self.votes.approvals.contains(&caller) && !self.votes.rejections.contains(&caller) {
            self.votes.rejections.push(caller);
        }
    }

    pub fn update_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn set_send_at(&mut self, send_at: u64) {
        self.send_at = Some(send_at);
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Votes {
    pub approvals: Vec<Principal>,
    pub rejections: Vec<Principal>,
}

#[derive(CandidType, Deserialize, PartialEq, Eq, Clone)]
pub enum Status {
    Pending,
    Approved,
    Rejected,
    Expired,
    Deadlock,
}

#[derive(CandidType, Deserialize, PartialEq, Eq, Clone)]
pub enum Amount {
    DIP20(u64),
    ICRC1(Nat),
}

#[derive(CandidType, Deserialize, Clone)]
pub struct AirdropTransactionDetails {
    pub status: Status,
    pub receiver: Principal,
    pub amount: Amount,
    pub canister_id: Principal,
    pub token_standard: TokenStandard,
}
