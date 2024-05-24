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

    pub async fn request(
        caller: Principal,
        canister_id: Principal,
        args: TransferArg,
    ) -> CanisterResult<TransferRequestEntry> {
        Self::check_balance(canister_id, &args.amount).await?;
        TransferRequestStorage::new_request(caller, TransferRequest::new(canister_id, args))
    }

    pub fn vote_request(
        caller: Principal,
        id: u64,
        vote: Vote,
    ) -> CanisterResult<TransferRequestEntry> {
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

    pub async fn execute_request(id: u64) -> CanisterResult<()> {
        let (_, req) = TransferRequestStorage::get(id)?;

        if req.status() != Status::Approved {
            return Err(Error::bad_request().add_message("Request is not approved"));
        }

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
