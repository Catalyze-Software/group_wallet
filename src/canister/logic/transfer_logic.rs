use candid::{Nat, Principal};
use ic_cdk::id;
use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};

use types::{Error, TransferProposalContent};

use crate::{
    helpers::icrc::{icrc1_balance_of, icrc1_transfer},
    result::CanisterResult,
};

pub struct TransferLogic;

impl TransferLogic {
    pub async fn transfer(canister_id: Principal, args: TransferArg) -> CanisterResult<()> {
        Self::check_balance(canister_id, &args.amount).await?;

        icrc1_transfer(canister_id, args)
            .await
            .map_err(|(_, e)| Error::internal().add_message(&format!("transfer failed: {e}")))?;
        Ok(())
    }

    pub async fn execute_transfer(content: TransferProposalContent) -> CanisterResult<()> {
        Self::transfer(content.canister_id, content.args).await
    }

    pub async fn check_balance(ledger_canister: Principal, amount: &Nat) -> CanisterResult<()> {
        let (balance,) = icrc1_balance_of(
            ledger_canister,
            Account {
                owner: id(),
                subaccount: None,
            },
        )
        .await
        .map_err(|(_, e)| Error::internal().add_message(&format!("balance check failed: {e}")))?;

        if &balance < amount {
            return Err(Error::insufficient_balance());
        }

        Ok(())
    }
}
