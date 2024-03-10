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
    CreateCreatorProfile {},
    CreateValidatorProfile {},
    PostNews {
        content: String,
    },
    ValidateNews {
        news_id: Uint64,
        vote: bool,
        comment: String,
    },
    DepositStake {},
    WithdrawStake {},
    TipCreator {
        creator_anonymous_id: String,
    },
    WithdrawTip {
        amount: u128,
    },
    LockFunds {},
    UnlockFunds {},
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
    GetConfig {},
    // GetCreatorProfile {},   // Only the creator can view their profile
    // GetValidatorProfile {}, // Only the validator can view their profile
    GetNewsItem { news_id: Uint64 },
    GetAllNewsItems {},
    GetAllNewsItemsByCreator { creator: String },
}
