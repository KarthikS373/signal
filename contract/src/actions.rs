use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Uint128,
};

use crate::helpers::generate_anonymous_id;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    config as configure, config_read as configure_read, creator_profiles, Config, CreatorProfile,
    NewsItem, ValidatorProfile, CREATOR_PROFILES, NEWS_ITEMS, VALIDATOR_PROFILES,
};

/*
 * Create a new profile for a creator
*/
pub fn create_creator_profile(deps: DepsMut, env: &Env, info: &MessageInfo) -> StdResult<Response> {
    let state: Config = configure_read(deps.storage).load()?;
    let anonymous_id = generate_anonymous_id(env, info, state.entropy.as_bytes());

    let profile = CreatorProfile {
        anonymous_id: anonymous_id.clone(),
        addr: info.sender.clone(),
        stake: Uint128::zero(),
        reputation: Some(0),
        warnings_received: 0,
    };

    CREATOR_PROFILES
        .add_suffix(info.sender.as_bytes())
        .save(deps.storage, &profile)?;

    // if stake < state.creator_base_stake {
    //     return Err(StdError::generic_err(
    //         "Stake does not meet the base requirement",
    //     ));
    // }

    Ok(Response::new()
        .add_attribute("method", "create_creator_profile")
        .add_attribute("creator", anonymous_id.to_string()))
}

/*
 * Create a new profile for a validator
*/
pub fn create_validator_profile(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
) -> StdResult<Response> {
    let state: Config = configure_read(deps.storage).load()?;
    let anonymous_id = generate_anonymous_id(env, info, state.entropy.as_bytes());

    let profile = ValidatorProfile {
        anonymous_id: anonymous_id.clone(),
        addr: info.sender.clone(),
        reputation_score: Uint128::zero(),

        stake: Uint128::zero(),
        validated_content_count: None,
        last_validation_timestamp: None,
    };

    VALIDATOR_PROFILES
        .add_suffix(info.sender.as_bytes())
        .save(deps.storage, &profile)?;

    // if stake < state.creator_base_stake {
    //     return Err(StdError::generic_err(
    //         "Stake does not meet the base requirement",
    //     ));
    // }

    Ok(Response::new()
        .add_attribute("method", "create_validator_profile")
        .add_attribute("validator", anonymous_id.to_string()))
}

/*
 * Create a news
*/
pub fn create_news_entry(deps: DepsMut, env: &Env, info: &MessageInfo) -> StdResult<Response> {
    let user_exists = CREATOR_PROFILES
        .add_suffix(info.sender.as_bytes())
        .may_load(deps.storage);

    match user_exists {
        Ok(Some(_)) => {
            // Pass
        }
        Ok(None) => {
            return Err(StdError::generic_err(
                "User does not have a creator profile",
            ));
        }
        Err(_) => {
            return Err(StdError::generic_err("Error checking user profile"));
        }
    }

    let index = NEWS_ITEMS
        .add_suffix(info.sender.as_bytes())
        .get_len(deps.storage)?;

    let news = NewsItem {
        id: (index + 1).to_string(),
        // TODO: Add IPFS hash
        content: "".to_string(),
        creator: info.sender.clone(),
        validated: false,
    };

    NEWS_ITEMS
        .add_suffix(info.sender.as_bytes())
        .insert(deps.storage, &index, &news)?;

    Ok(Response::new()
        .add_attribute("method", "create_news_entry")
        .add_attribute("news_id", index.to_string()))
}
