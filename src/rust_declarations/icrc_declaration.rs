// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.

use ic_cdk::api::call::CallResult;
use ic_cdk::export::candid::{self, CandidType, Deserialize};

type Subaccount = Vec<u8>;
#[derive(CandidType, Deserialize)]
pub struct Account {
    owner: candid::Principal,
    subaccount: Option<Subaccount>,
}

#[derive(CandidType, Deserialize)]
pub enum Value {
    Int(candid::Int),
    Nat(candid::Nat),
    Blob(Vec<u8>),
    Text(String),
}

#[derive(CandidType, Deserialize)]
pub struct Icrc1SupportedStandardsRet0Inner {
    url: String,
    name: String,
}

pub type Timestamp = u64;
#[derive(CandidType, Deserialize)]
pub struct TransferArgs {
    to: Account,
    fee: Option<candid::Nat>,
    memo: Option<Vec<u8>>,
    from_subaccount: Option<Subaccount>,
    created_at_time: Option<Timestamp>,
    amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum TransferError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    BadBurn {
        min_burn_amount: candid::Nat,
    },
    Duplicate {
        duplicate_of: candid::Nat,
    },
    BadFee {
        expected_fee: candid::Nat,
    },
    CreatedInFuture {
        ledger_time: Timestamp,
    },
    TooOld,
    InsufficientFunds {
        balance: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub enum Icrc1TransferRet0 {
    Ok(candid::Nat),
    Err(TransferError),
}

pub struct SERVICE(candid::Principal);
impl SERVICE {
    pub async fn icrc1_balance_of(&self, arg0: Account) -> CallResult<(candid::Nat,)> {
        ic_cdk::call(self.0, "icrc1_balance_of", (arg0,)).await
    }
    pub async fn icrc1_decimals(&self) -> CallResult<(u8,)> {
        ic_cdk::call(self.0, "icrc1_decimals", ()).await
    }
    pub async fn icrc1_fee(&self) -> CallResult<(candid::Nat,)> {
        ic_cdk::call(self.0, "icrc1_fee", ()).await
    }
    pub async fn icrc1_metadata(&self) -> CallResult<(Vec<(String, Value)>,)> {
        ic_cdk::call(self.0, "icrc1_metadata", ()).await
    }
    pub async fn icrc1_minting_account(&self) -> CallResult<(Option<Account>,)> {
        ic_cdk::call(self.0, "icrc1_minting_account", ()).await
    }
    pub async fn icrc1_name(&self) -> CallResult<(String,)> {
        ic_cdk::call(self.0, "icrc1_name", ()).await
    }
    pub async fn icrc1_supported_standards(
        &self,
    ) -> CallResult<(Vec<Icrc1SupportedStandardsRet0Inner>,)> {
        ic_cdk::call(self.0, "icrc1_supported_standards", ()).await
    }
    pub async fn icrc1_symbol(&self) -> CallResult<(String,)> {
        ic_cdk::call(self.0, "icrc1_symbol", ()).await
    }
    pub async fn icrc1_total_supply(&self) -> CallResult<(candid::Nat,)> {
        ic_cdk::call(self.0, "icrc1_total_supply", ()).await
    }
    pub async fn icrc1_transfer(&self, arg0: TransferArgs) -> CallResult<(Icrc1TransferRet0,)> {
        ic_cdk::call(self.0, "icrc1_transfer", (arg0,)).await
    }
}
