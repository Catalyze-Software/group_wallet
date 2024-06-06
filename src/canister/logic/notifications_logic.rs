use ic_cdk::{api::call::CallResult, call, caller};

use crate::storage::{metadata_storage::MetadataStorage, CellStorage};

use super::WhitelistLogic;

pub struct NotificationLogic;

impl NotificationLogic {
    pub async fn send_whitelist_notice() {
        if let Ok(mut whitelist) = WhitelistLogic::get_whitelist() {
            whitelist.retain(|x| x != &caller());
            if let Ok(metadata) = MetadataStorage::get() {
                let _: CallResult<((),)> = call(
                    metadata.index_canister,
                    "multisig_whitelist_notice_notification",
                    (whitelist, metadata.group_id),
                )
                .await;
            }
        }
    }

    pub async fn send_accept_proposal(proposal_id: u64) {
        if let Ok(mut whitelist) = WhitelistLogic::get_whitelist() {
            whitelist.retain(|x| x != &caller());
            if let Ok(metadata) = MetadataStorage::get() {
                let _: CallResult<((),)> = call(
                    metadata.index_canister,
                    "multisig_proposal_accept_notification",
                    (whitelist, proposal_id, metadata.group_id),
                )
                .await;
            }
        }
    }

    pub async fn send_decline_proposal(proposal_id: u64) {
        if let Ok(mut whitelist) = WhitelistLogic::get_whitelist() {
            whitelist.retain(|x| x != &caller());
            if let Ok(metadata) = MetadataStorage::get() {
                let _: CallResult<((),)> = call(
                    metadata.index_canister,
                    "multisig_proposal_decline_notification",
                    (whitelist, proposal_id, metadata.group_id),
                )
                .await;
            }
        }
    }

    pub async fn send_update_proposal(proposal_id: u64) {
        if let Ok(mut whitelist) = WhitelistLogic::get_whitelist() {
            whitelist.retain(|x| x != &caller());
            if let Ok(metadata) = MetadataStorage::get() {
                let _: CallResult<((),)> = call(
                    metadata.index_canister,
                    "multisig_proposal_status_update_notification",
                    (whitelist, proposal_id, metadata.group_id),
                )
                .await;
            }
        }
    }

    pub async fn send_new_proposal(proposal_id: u64) {
        if let Ok(mut whitelist) = WhitelistLogic::get_whitelist() {
            whitelist.retain(|x| x != &caller());

            if let Ok(metadata) = MetadataStorage::get() {
                let _: CallResult<((),)> = call(
                    metadata.index_canister,
                    "multisig_new_proposal_notification",
                    (whitelist, proposal_id, metadata.group_id),
                )
                .await;
            }
        }
    }
}
