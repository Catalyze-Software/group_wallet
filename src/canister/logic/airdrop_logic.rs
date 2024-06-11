use candid::{Nat, Principal};
use icrc_ledger_types::icrc1::transfer::TransferArg;

use types::{AirdropProposalContent, AirdropTransfer, AirdropTransfers, Status};

use crate::{
    result::CanisterResult,
    storage::{AirdropTransferStorage, StorageInsertableByKey, StorageQueryable},
};

use super::transfer_logic::TransferLogic;

pub struct AirdropLogic;

impl AirdropLogic {
    pub fn get_transfers(_caller: Principal, id: u64) -> CanisterResult<AirdropTransfers> {
        let (_, txs) = AirdropTransferStorage::get(id)?;
        Ok(txs)
    }

    pub async fn execute_airdrop(id: u64, content: AirdropProposalContent) -> CanisterResult<()> {
        let mut transfers = Vec::<AirdropTransfer>::default();

        for args in content.args {
            let is_ok = TransferLogic::transfer(content.canister_id, args.clone())
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
                canister_id: content.canister_id,
            };

            transfers.push(details);
        }

        // ID tied to the proposal id
        AirdropTransferStorage::insert_by_key(id, AirdropTransfers(transfers))?;

        Ok(())
    }

    pub async fn check_balance(
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
