use cosmwasm_std::{Uint128, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub entropy: String,
    pub creator_base_stake: Uint128,
    pub validator_base_stake: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateCreatorProfile {
        stake: Uint128,
        viewing_key: String,
    },
    CreateValidatorProfile {
        stake: Uint128,
        viewing_key: String,
    },
    PostNews {
        content: String,
        anonymous_id: String,
    },
    ValidateNews {
        news_id: u64,
        approved: bool,
        anonymous_id: String,
    },
    // UpdateStake {
    //     new_stake: Uint128,
    //     anonymous_id: String,
    // },
    // UpdateReputation {
    //     new_reputation: u64,
    // },
    // WarnCreator {
    //     anonymous_id: String,
    // },
    // RemoveWarning {
    //     anonymous_id: String,
    // },
    // RemoveValidator {
    //     anonymous_id: String,
    // },
    // RemoveCreator {
    //     anonymous_id: String,
    // },
    // RemoveNews {
    //     news_id: u64,
    // },
    // TransferStake {
    //     recipient_anonymous_id: String,
    //     amount: Uint128,
    //     viewing_key: String,
    // },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetProfileWithViewingKey { viewing_key: String },
    GetNewsItem { news_id: Uint64 },
    GetValidations { news_id: Uint64 },
    GetConfig {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Custom {}
