use candid::{CandidType, Principal};
use ic_cdk::api::call::CallResult as Result;
use icrc_ledger_types::icrc1::{
    account::Account,
    transfer::{TransferArg, TransferError},
};
use serde::Deserialize;

pub async fn icrc1_balance_of(canister_id: Principal, arg0: Account) -> Result<(candid::Nat,)> {
    ic_cdk::call(canister_id, "icrc1_balance_of", (arg0,)).await
}

pub async fn icrc1_transfer(canister_id: Principal, arg0: TransferArg) -> Result<(Result3,)> {
    ic_cdk::call(canister_id, "icrc1_transfer", (arg0,)).await
}

#[derive(CandidType, Deserialize)]
pub enum Result3 {
    Ok(candid::Nat),
    Err(TransferError),
}
