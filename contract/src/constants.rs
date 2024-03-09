pub static BLOCK_SIZE: usize = 256;
pub static SCRT_DENOM: &str = "uscrt";

// Storage keys
pub static CONFIG_KEY: &[u8] = b"config";
pub static CREATOR_KEY: &[u8] = b"creator";
pub static VALIDATOR_KEY: &[u8] = b"validator";
pub static NEWS_ITEM_KEY: &[u8] = b"news_item";
pub static COMMENT_KEY: &[u8] = b"comment";
pub static VALIDATION_KEY: &[u8] = b"validation";

// Keymap identifiers
pub static CREATOR_PROFILES_KEY: &[u8] = b"creator_profiles";
pub static VALIDATOR_PROFILES_KEY: &[u8] = b"validator_profiles";
pub static NEWS_ITEMS_KEY: &[u8] = b"news_items";

// Address mappings
pub static ANONID_CREATORADDRESS_KEY: &[u8] = b"anonid_to_creator_address";
pub static ANONID_VALIDATORADDRESS_KEY: &[u8] = b"aonid_to_validator_address";

// Tipping
pub static CREATOR_TIPS_KEY: &[u8] = b"creator_tips";
