use candid::Principal;
use ic_cdk::api::time;

use crate::{
    helpers::votes::get_request_majority,
    models::{
        AirdropRequest, AirdropRequestEntry, AirdropTransfer, AirdropTransfers, Error, Request,
        Status, TransferArg, Vote, VoteResponse,
    },
    result::CanisterResult,
    storage::{
        AirdropRequestStorage, AirdropTransferStorage, RequestStorage, StorageInsertableByKey,
        StorageQueryable, WhitelistStorage,
    },
};

use super::transfer_logic::TransferLogic;

// TODO: mark with unsupported guard
pub struct AirdropLogic;

impl AirdropLogic {
    pub fn get_requests(status: Option<Status>) -> Vec<AirdropRequestEntry> {
        AirdropRequestStorage::get_requests_by_status(status)
    }

    pub fn request(
        caller: Principal,
        canister_id: Principal,
        transfer_args: Vec<TransferArg>,
    ) -> CanisterResult<AirdropRequestEntry> {
        // TODO: whitelisted guard
        // TODO: add Canister balance guard

        AirdropRequestStorage::new_request(caller, AirdropRequest::new(canister_id, transfer_args))
    }

    pub fn get_transfers(_caller: Principal, id: u64) -> CanisterResult<AirdropTransfers> {
        // TODO: whitelisted guard
        let (_, txs) = AirdropTransferStorage::get(id)?;
        Ok(txs)
    }

    pub fn vote_request(
        caller: Principal,
        id: u64,
        vote: Vote,
    ) -> CanisterResult<AirdropRequestEntry> {
        // TODO: Add whitelist guard
        let (_, req) = AirdropRequestStorage::vote_request(caller, id, vote)?;

        match get_request_majority(WhitelistStorage::get_all(), &req.details.votes) {
            VoteResponse::Approve => AirdropRequestStorage::approve_request(id),
            VoteResponse::Reject => AirdropRequestStorage::reject_request(id, false),
            VoteResponse::Deadlock => AirdropRequestStorage::reject_request(id, true),
            VoteResponse::NotReached => Ok((id, req)),
        }
    }

    pub async fn execute_request(_caller: Principal, id: u64) -> CanisterResult<()> {
        // TODO: Add whitelist guard
        let (_, req) = AirdropRequestStorage::get(id)?;

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
}

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
// }
