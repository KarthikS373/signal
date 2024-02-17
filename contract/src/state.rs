use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage, Uint128};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";
pub static CREATOR_KEY: &[u8] = b"creator";
pub static VALIDATOR_KEY: &[u8] = b"validator";
pub static NEWS_ITEM_KEY: &[u8] = b"news_item";
pub static COMMENT_KEY: &[u8] = b"comment";
pub static VALIDATION_KEY: &[u8] = b"validation";

// Config for the contract
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Config {
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
    pub validated_content_count: Uint128,
    pub last_validation_timestamp: String,
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

// Comments on a news item
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Comment {
    pub news_id: String,
    pub commenter: Addr,
    pub content: String,
}

pub fn comments(storage: &mut dyn Storage) -> Singleton<Comment> {
    singleton(storage, COMMENT_KEY)
}

pub fn comments_read(storage: &dyn Storage) -> ReadonlySingleton<Comment> {
    singleton_read(storage, COMMENT_KEY)
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
