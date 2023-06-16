use candid::{CandidType, Principal};
use serde::Deserialize;

use super::icrc_declaration::TransferArgs;

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
    ICRC1(TransferArgs),
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

#[derive(CandidType, Deserialize, Clone)]
pub struct SharedData {
    pub id: u32,
    pub status: Status,
    pub votes: Votes,
    pub requested_by: Principal,
    pub created_at: u64,
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
