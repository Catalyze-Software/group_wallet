use candid::Principal;
use ic_cdk::api::time;

use crate::logic::store::{Store, DATA};
use crate::rust_declarations::types::{
    SharedData, Status, TransactionRequestData, TransferRequestType, Votes,
};

impl Store {
    pub fn transfer_request(caller: Principal, args: TransferRequestType) {
        DATA.with(|data| {
            let mut data = data.borrow_mut();
            let transaction_id = data.transaction_request_id;

            let transaction_data = TransactionRequestData {
                args,
                data: SharedData {
                    status: Status::Pending,
                    votes: Votes {
                        approvals: vec![caller],
                        rejections: vec![],
                    },
                    requested_by: caller,
                    created_at: time(),
                    id: transaction_id,
                },
            };
            data.transaction_request_id += 1;
            data.transaction_requests
                .insert(transaction_id, transaction_data);
        });
    }
}
