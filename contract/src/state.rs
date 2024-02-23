use schemars::JsonSchema;
use secret_toolkit::storage::{Item, Keymap};
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage, Uint128};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

use crate::constants::{
    COMMENT_KEY, CONFIG_KEY, CREATOR_KEY, CREATOR_PROFILES_KEY, NEWS_ITEMS_KEY, NEWS_ITEM_KEY,
    VALIDATION_KEY, VALIDATOR_KEY, VALIDATOR_PROFILES_KEY,
};

// Config for the contract
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Config {
    // Randomness
    pub entropy: String,

    // Staking parameters
    pub creator_base_stake: Uint128,
    pub validator_base_stake: Uint128,
}

pub fn config(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG_KEY)
}

// Keymaps
pub static CREATOR_PROFILES: Item<CreatorProfile> = Item::new(CREATOR_PROFILES_KEY);
pub static VALIDATOR_PROFILES: Item<ValidatorProfile> = Item::new(VALIDATOR_PROFILES_KEY);

pub static NEWS_ITEMS: Keymap<u32, NewsItem> = Keymap::new(NEWS_ITEMS_KEY);

// Profile for content creators
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct CreatorProfile {
    pub anonymous_id: String,

    pub addr: Addr,
    pub stake: Uint128,
    pub reputation: Option<u64>,

    pub warnings_received: u32,
}

pub fn creator_profiles(storage: &mut dyn Storage) -> Singleton<CreatorProfile> {
    singleton(storage, CREATOR_KEY)
}

pub fn creator_profiles_read(storage: &dyn Storage) -> ReadonlySingleton<CreatorProfile> {
    singleton_read(storage, CREATOR_KEY)
}

// Profile for validators
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct ValidatorProfile {
    pub anonymous_id: String,

    pub addr: Addr,
    pub stake: Uint128,
    pub reputation_score: Uint128,
    pub validated_content_count: Option<Uint128>,
    pub last_validation_timestamp: Option<String>,
}

pub fn validator_profiles(storage: &mut dyn Storage) -> Singleton<ValidatorProfile> {
    singleton(storage, VALIDATOR_KEY)
}

pub fn validator_profiles_read(storage: &dyn Storage) -> ReadonlySingleton<ValidatorProfile> {
    singleton_read(storage, VALIDATOR_KEY)
}

// Represents a news item
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct NewsItem {
    pub id: String,
    pub creator: Addr,
    pub content: String, // IPFS hash
    pub validated: bool,
}

pub fn news_items(storage: &mut dyn Storage) -> Singleton<NewsItem> {
    singleton(storage, NEWS_ITEM_KEY)
}

pub fn news_items_read(storage: &dyn Storage) -> ReadonlySingleton<NewsItem> {
    singleton_read(storage, NEWS_ITEM_KEY)
}

// Record of a validation action by a reader
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Validation {
    pub news_id: String,
    pub validator: Addr,
    pub approved: bool,
}

pub fn validations(storage: &mut dyn Storage) -> Singleton<Validation> {
    singleton(storage, VALIDATION_KEY)
}

pub fn validations_read(storage: &dyn Storage) -> ReadonlySingleton<Validation> {
    singleton_read(storage, VALIDATION_KEY)
}
