use candid::{Nat, Principal};
use ic_cdk::api::time;
use icrc_ledger_types::icrc1::transfer::TransferArg;

use types::{
    AirdropRequest, AirdropRequestEntry, AirdropTransfer, AirdropTransfers, Error, Request, Status,
    Vote, VoteResponse,
};

use crate::{
    helpers::votes::get_request_majority,
    result::CanisterResult,
    storage::{
        AirdropRequestStorage, AirdropTransferStorage, RequestStorage, StorageInsertableByKey,
        StorageQueryable, WhitelistStorage,
    },
};

use super::transfer_logic::TransferLogic;

pub struct AirdropLogic;

impl AirdropLogic {
    pub fn get_requests(status: Option<Status>) -> Vec<AirdropRequestEntry> {
        AirdropRequestStorage::get_requests_by_status(status)
    }

    pub async fn request(
        caller: Principal,
        canister_id: Principal,
        transfer_args: Vec<TransferArg>,
    ) -> CanisterResult<AirdropRequestEntry> {
        Self::check_balance(canister_id, transfer_args.clone()).await?;

        AirdropRequestStorage::new_request(caller, AirdropRequest::new(canister_id, transfer_args))
    }

    pub fn get_transfers(_caller: Principal, id: u64) -> CanisterResult<AirdropTransfers> {
        let (_, txs) = AirdropTransferStorage::get(id)?;
        Ok(txs)
    }

    pub fn vote_request(
        caller: Principal,
        id: u64,
        vote: Vote,
    ) -> CanisterResult<AirdropRequestEntry> {
        let (_, req) = AirdropRequestStorage::vote_request(caller, id, vote)?;

        match get_request_majority(WhitelistStorage::get_all(), &req.details.votes) {
            VoteResponse::Approve => AirdropRequestStorage::approve_request(id),
            VoteResponse::Reject => AirdropRequestStorage::reject_request(id, false),
            VoteResponse::Deadlock => AirdropRequestStorage::reject_request(id, true),
            VoteResponse::NotReached => Ok((id, req)),
        }
    }

    pub async fn execute_request(id: u64) -> CanisterResult<()> {
        let (_, req) = AirdropRequestStorage::get(id)?;

        if req.status() != Status::Approved {
            return Err(Error::bad_request().add_message("Request is not approved"));
        }

        if req.details().sent_at.is_some() {
            return Err(Error::bad_request().add_message("Request already executed"));
        }

        let (_, req) = AirdropRequestStorage::set_sent_at(id, time())?;

        let mut transfers = Vec::<AirdropTransfer>::default();

        for args in req.transfer_args {
            let is_ok = TransferLogic::transfer(req.canister_id, args.clone())
                .await
                .is_ok();

            let status = match is_ok {
                true => Status::Approved,
                false => Status::Rejected,
            };

            let details = AirdropTransfer {
                status,
                receiver: args.to.owner,
                amount: args.amount,
                canister_id: req.canister_id,
            };

            transfers.push(details);
        }

        // ID equals to the request id
        AirdropTransferStorage::insert_by_key(id, AirdropTransfers(transfers))?;

        Ok(())
    }

    async fn check_balance(
        canister_id: Principal,
        transfer_args: Vec<TransferArg>,
    ) -> CanisterResult<()> {
        let total = transfer_args
            .iter()
            .fold(Nat::from(0u32), |acc, arg| acc + arg.amount.clone());

        if total > 0u32 {
            return TransferLogic::check_balance(canister_id, &total).await;
        }

        Ok(())
    }
}
