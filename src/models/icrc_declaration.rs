// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
use candid::{self, CandidType, Deserialize, Principal};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub struct GetBlocksRequest {
    start: candid::Nat,
    length: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum VecItem {
    Int(candid::Int),
    Map(Vec<(String, Box<Value>)>),
    Nat(candid::Nat),
    Nat64(u64),
    Blob(serde_bytes::ByteBuf),
    Text(String),
    Array(Box<Vec<VecItem>>),
}

#[derive(CandidType, Deserialize)]
pub enum Value {
    Int(candid::Int),
    Map(Vec<(String, Box<Value>)>),
    Nat(candid::Nat),
    Nat64(u64),
    Blob(serde_bytes::ByteBuf),
    Text(String),
    Array(Box<Vec<VecItem>>),
}

#[derive(CandidType, Deserialize)]
pub struct ArchivedRangeCallbackRet0 {
    blocks: Vec<Value>,
}

candid::define_function!(pub ArchivedRangeCallback : (GetBlocksRequest) -> (
    ArchivedRangeCallbackRet0,
  ) query);
#[derive(CandidType, Deserialize)]
pub struct ArchivedRange {
    callback: ArchivedRangeCallback,
    start: candid::Nat,
    length: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct GetBlocksResponse {
    certificate: Option<serde_bytes::ByteBuf>,
    first_index: candid::Nat,
    blocks: Vec<Value>,
    chain_length: u64,
    archived_blocks: Vec<ArchivedRange>,
}

#[derive(CandidType, Deserialize)]
pub struct DataCertificate {
    certificate: Option<serde_bytes::ByteBuf>,
    hash_tree: serde_bytes::ByteBuf,
}

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct Burn {
    from: Account,
    memo: Option<serde_bytes::ByteBuf>,
    created_at_time: Option<u64>,
    amount: candid::Nat,
    spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Mint {
    to: Account,
    memo: Option<serde_bytes::ByteBuf>,
    created_at_time: Option<u64>,
    amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct Approve {
    fee: Option<candid::Nat>,
    from: Account,
    memo: Option<serde_bytes::ByteBuf>,
    created_at_time: Option<u64>,
    amount: candid::Nat,
    expected_allowance: Option<candid::Nat>,
    expires_at: Option<u64>,
    spender: Account,
}

#[derive(CandidType, Deserialize)]
pub struct Transfer {
    to: Account,
    fee: Option<candid::Nat>,
    from: Account,
    memo: Option<serde_bytes::ByteBuf>,
    created_at_time: Option<u64>,
    amount: candid::Nat,
    spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Transaction {
    burn: Option<Burn>,
    kind: String,
    mint: Option<Mint>,
    approve: Option<Approve>,
    timestamp: u64,
    transfer: Option<Transfer>,
}

#[derive(CandidType, Deserialize)]
pub struct ArchivedRange1CallbackRet0 {
    transactions: Vec<Transaction>,
}

candid::define_function!(pub ArchivedRange1Callback : (GetBlocksRequest) -> (
    ArchivedRange1CallbackRet0,
  ) query);
#[derive(CandidType, Deserialize)]
pub struct ArchivedRange1 {
    callback: ArchivedRange1Callback,
    start: candid::Nat,
    length: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionsResponse {
    first_index: candid::Nat,
    log_length: candid::Nat,
    transactions: Vec<Transaction>,
    archived_transactions: Vec<ArchivedRange1>,
}

#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    url: String,
    method: String,
    body: serde_bytes::ByteBuf,
    headers: Vec<(String, String)>,
}

#[derive(CandidType, Deserialize)]
pub struct HttpResponse {
    body: serde_bytes::ByteBuf,
    headers: Vec<(String, String)>,
    status_code: u16,
}

#[derive(CandidType, Deserialize)]
pub enum MetadataValue {
    Int(candid::Int),
    Nat(candid::Nat),
    Blob(serde_bytes::ByteBuf),
    Text(String),
}

#[derive(CandidType, Deserialize)]
pub struct StandardRecord {
    url: String,
    name: String,
}

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct TransferArg {
    pub to: Account,
    pub fee: Option<candid::Nat>,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub from_subaccount: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
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
        ledger_time: u64,
    },
    TooOld,
    InsufficientFunds {
        balance: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub struct AllowanceArgs {
    account: Account,
    spender: Account,
}

#[derive(CandidType, Deserialize)]
pub struct Allowance {
    allowance: candid::Nat,
    expires_at: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct ApproveArgs {
    fee: Option<candid::Nat>,
    memo: Option<serde_bytes::ByteBuf>,
    from_subaccount: Option<serde_bytes::ByteBuf>,
    created_at_time: Option<u64>,
    amount: candid::Nat,
    expected_allowance: Option<candid::Nat>,
    expires_at: Option<u64>,
    spender: Account,
}

#[derive(CandidType, Deserialize)]
pub enum ApproveError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    Duplicate {
        duplicate_of: candid::Nat,
    },
    BadFee {
        expected_fee: candid::Nat,
    },
    AllowanceChanged {
        current_allowance: candid::Nat,
    },
    CreatedInFuture {
        ledger_time: u64,
    },
    TooOld,
    Expired {
        ledger_time: u64,
    },
    InsufficientFunds {
        balance: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub enum Result1 {
    Ok(candid::Nat),
    Err(ApproveError),
}

#[derive(CandidType, Deserialize)]
pub struct TransferFromArgs {
    to: Account,
    fee: Option<candid::Nat>,
    spender_subaccount: Option<serde_bytes::ByteBuf>,
    from: Account,
    memo: Option<serde_bytes::ByteBuf>,
    created_at_time: Option<u64>,
    amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum TransferFromError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    InsufficientAllowance {
        allowance: candid::Nat,
    },
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
        ledger_time: u64,
    },
    TooOld,
    InsufficientFunds {
        balance: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub enum Result2 {
    Ok(candid::Nat),
    Err(TransferFromError),
}

#[derive(CandidType, Deserialize)]
pub enum Result3 {
    Ok(candid::Nat),
    Err(TransferError),
}

pub struct IcrcService(pub Principal);
impl IcrcService {
    pub async fn get_blocks(&self, arg0: GetBlocksRequest) -> Result<(GetBlocksResponse,)> {
        ic_cdk::call(self.0, "get_blocks", (arg0,)).await
    }
    pub async fn get_data_certificate(&self) -> Result<(DataCertificate,)> {
        ic_cdk::call(self.0, "get_data_certificate", ()).await
    }
    pub async fn get_transactions(
        &self,
        arg0: GetBlocksRequest,
    ) -> Result<(GetTransactionsResponse,)> {
        ic_cdk::call(self.0, "get_transactions", (arg0,)).await
    }
    pub async fn http_request(&self, arg0: HttpRequest) -> Result<(HttpResponse,)> {
        ic_cdk::call(self.0, "http_request", (arg0,)).await
    }
    pub async fn icrc1_balance_of(&self, arg0: Account) -> Result<(candid::Nat,)> {
        ic_cdk::call(self.0, "icrc1_balance_of", (arg0,)).await
    }
    pub async fn icrc1_decimals(&self) -> Result<(u8,)> {
        ic_cdk::call(self.0, "icrc1_decimals", ()).await
    }
    pub async fn icrc1_fee(&self) -> Result<(candid::Nat,)> {
        ic_cdk::call(self.0, "icrc1_fee", ()).await
    }
    pub async fn icrc1_metadata(&self) -> Result<(Vec<(String, MetadataValue)>,)> {
        ic_cdk::call(self.0, "icrc1_metadata", ()).await
    }
    pub async fn icrc1_minting_account(&self) -> Result<(Option<Account>,)> {
        ic_cdk::call(self.0, "icrc1_minting_account", ()).await
    }
    pub async fn icrc1_name(&self) -> Result<(String,)> {
        ic_cdk::call(self.0, "icrc1_name", ()).await
    }
    pub async fn icrc1_supported_standards(&self) -> Result<(Vec<StandardRecord>,)> {
        ic_cdk::call(self.0, "icrc1_supported_standards", ()).await
    }
    pub async fn icrc1_symbol(&self) -> Result<(String,)> {
        ic_cdk::call(self.0, "icrc1_symbol", ()).await
    }
    pub async fn icrc1_total_supply(&self) -> Result<(candid::Nat,)> {
        ic_cdk::call(self.0, "icrc1_total_supply", ()).await
    }
    pub async fn icrc1_transfer(&self, arg0: TransferArg) -> Result<(Result3,)> {
        ic_cdk::call(self.0, "icrc1_transfer", (arg0,)).await
    }
    pub async fn icrc2_allowance(&self, arg0: AllowanceArgs) -> Result<(Allowance,)> {
        ic_cdk::call(self.0, "icrc2_allowance", (arg0,)).await
    }
    pub async fn icrc2_approve(&self, arg0: ApproveArgs) -> Result<(Result1,)> {
        ic_cdk::call(self.0, "icrc2_approve", (arg0,)).await
    }
    pub async fn icrc2_transfer_from(&self, arg0: TransferFromArgs) -> Result<(Result2,)> {
        ic_cdk::call(self.0, "icrc2_transfer_from", (arg0,)).await
    }
}
