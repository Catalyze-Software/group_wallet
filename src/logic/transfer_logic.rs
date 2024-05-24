// use std::time::Duration;

// use candid::{Nat, Principal};
// use ic_cdk::api::time;
// use ic_cdk::id;
// use ic_cdk_timers::set_timer;

// use crate::logic::store::{Store, DATA};

// use crate::models::dip20_declaration::Dip20Service;
// use crate::models::icrc_declaration::{Account, IcrcService, TransferArg};
// use crate::models::types::{
//     Dip20TransferArgs, SharedData, Status, TransactionRequest, TransferRequestType, VoteResponse,
//     VoteType,
// };

// use super::store::DAY_IN_NANOS;

use candid::{Nat, Principal};
use ic_cdk::{api::time, id};

use crate::{
    helpers::votes::get_request_majority,
    models::{
        Account, Error, IcrcService, Request, Status, TransferArg, TransferRequest,
        TransferRequestEntry, Vote, VoteResponse,
    },
    result::CanisterResult,
    storage::{RequestStorage, StorageQueryable, TransferRequestStorage, WhitelistStorage},
};

pub struct TransferLogic;

impl TransferLogic {
    pub fn get_requests(status: Option<Status>) -> Vec<TransferRequestEntry> {
        TransferRequestStorage::get_requests_by_status(status)
    }

    pub fn request(
        caller: Principal,
        canister_id: Principal,
        args: TransferArg,
    ) -> CanisterResult<TransferRequestEntry> {
        // TODO: add whitelist guard
        // TODO: add Canister balance guard

        //         let has_balance = match &args {
        //             TransferRequestType::DIP20(args) => {
        //                 Self::balance_check_dip20(canister_id, &args.amount).await
        //             }
        //             TransferRequestType::ICRC1(args) => {
        //                 Self::balance_check_icrc(canister_id, &args.amount).await
        //             }
        //         };

        TransferRequestStorage::new_request(caller, TransferRequest::new(canister_id, args))
    }

    pub fn vote_request(
        caller: Principal,
        id: u64,
        vote: Vote,
    ) -> CanisterResult<TransferRequestEntry> {
        // TODO: Add whitelist guard
        let (_, req) = TransferRequestStorage::vote_request(caller, id, vote)?;

        match get_request_majority(WhitelistStorage::get_all(), &req.details.votes) {
            VoteResponse::Approve => TransferRequestStorage::approve_request(id),
            VoteResponse::Reject => TransferRequestStorage::reject_request(id, false),
            VoteResponse::Deadlock => TransferRequestStorage::reject_request(id, true),
            VoteResponse::NotReached => Ok((id, req)),
        }
    }

    pub async fn transfer(canister_id: Principal, args: TransferArg) -> CanisterResult<()> {
        Self::check_balance(canister_id, &args.amount).await?;

        IcrcService(canister_id)
            .icrc1_transfer(args)
            .await
            .map_err(|(_, e)| Error::internal().add_message(&format!("transfer failed: {e}")))?;
        Ok(())
    }

    pub async fn execute_request(_caller: Principal, id: u64) -> CanisterResult<()> {
        // TODO: Add whitelist guard
        let (_, req) = TransferRequestStorage::get(id)?;

        if req.details().sent_at.is_some() {
            return Err(Error::bad_request().add_message("Request already executed"));
        }

        let (_, req) = TransferRequestStorage::set_sent_at(id, time())?;
        Self::transfer(req.canister_id, req.args).await
    }

    pub async fn check_balance(canister_id: Principal, amount: &Nat) -> CanisterResult<()> {
        let (balance,) = IcrcService(canister_id)
            .icrc1_balance_of(Account {
                owner: id(),
                subaccount: None,
            })
            .await
            .map_err(|(_, e)| {
                Error::internal().add_message(&format!("balance check failed: {e}"))
            })?;

        if &balance < amount {
            return Err(Error::insufficient_balance());
        }

        Ok(())
    }
}
