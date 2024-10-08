use crate::utils;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AccountAddresses {
    pub data: Vec<String>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccountAsset {
    pub amount: i64,
    pub unit: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccountAssets {
    pub data: Vec<AccountAsset>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StakeAccountHistoryItem {
    pub active_stake: i64,
    pub epoch_no: i64,
    pub pool_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StakeAccountHistory {
    pub data: Vec<StakeAccountHistoryItem>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccountInformation {
    pub delegated_pool: String,
    pub registered: bool,
    pub rewards_available: i64,
    pub stake_address: String,
    pub total_balance: i64,
    pub total_rewarded: i64,
    pub total_withdrawn: i64,
    pub utxo_balance: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StakeAccountInformation {
    pub data: AccountInformation,
    pub last_updated: utils::LastUpdated,
}

#[derive(Deserialize, Debug, Clone)]
pub enum StakeRewardType {
    Member,
    Leader,
    Refund,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StakeAccountReward {
    pub amount: i64,
    pub earned_epoch: i64,
    pub pool_id: String,
    pub spendable_epoch: i64,
    pub r#type: StakeRewardType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StakeAccountRewards {
    pub data: Vec<StakeAccountReward>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum StakeUpdateAction {
    Registration,
    Deregistration,
    Delegation,
    Withdrawal,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StakeAccountUpdate {
    pub absolute_slot: i64,
    pub action: StakeUpdateAction,
    pub epoch_no: i64,
    pub tx_hash: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StakeAccountUpdates {
    pub data: Vec<StakeAccountUpdate>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}
