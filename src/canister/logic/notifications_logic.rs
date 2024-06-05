use ic_cdk::{api::call::CallResult, call, id};

use crate::storage::{CellStorage, MultisigIndexStorage};

use super::WhitelistLogic;

pub struct NotificationLogic;

impl NotificationLogic {
    pub async fn send_whitelist_notice() {
        if let Ok(whitelist) = WhitelistLogic::get_whitelist() {
            if let Ok(index) = MultisigIndexStorage::get() {
                let _: CallResult<((),)> = call(
                    index,
                    "multisig_whitelist_notice_notification",
                    (whitelist, id()),
                )
                .await;
            }
        }
    }

    pub async fn send_accept_proposal(proposal_id: u64) {
        if let Ok(whitelist) = WhitelistLogic::get_whitelist() {
            if let Ok(index) = MultisigIndexStorage::get() {
                let _: CallResult<((),)> = call(
                    index,
                    "multisig_proposal_accept_notification",
                    (whitelist, id(), proposal_id),
                )
                .await;
            }
        }
    }

    pub async fn send_decline_proposal(proposal_id: u64) {
        if let Ok(whitelist) = WhitelistLogic::get_whitelist() {
            if let Ok(index) = MultisigIndexStorage::get() {
                let _: CallResult<((),)> = call(
                    index,
                    "multisig_proposal_decline_notification",
                    (whitelist, id(), proposal_id),
                )
                .await;
            }
        }
    }

    pub async fn send_update_proposal(proposal_id: u64) {
        if let Ok(whitelist) = WhitelistLogic::get_whitelist() {
            if let Ok(index) = MultisigIndexStorage::get() {
                let _: CallResult<((),)> = call(
                    index,
                    "multisig_proposal_status_update_notification",
                    (whitelist, id(), proposal_id),
                )
                .await;
            }
        }
    }

    pub async fn send_new_proposal(proposal_id: u64) {
        if let Ok(whitelist) = WhitelistLogic::get_whitelist() {
            if let Ok(index) = MultisigIndexStorage::get() {
                let _: CallResult<((),)> = call(
                    index,
                    "multisig_new_proposal_notification",
                    (whitelist, id(), proposal_id),
                )
                .await;
            }
        }
    }
}
