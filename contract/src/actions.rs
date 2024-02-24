use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Uint128,
};

use crate::helpers::{generate_anonymous_id, generate_random_number};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    config as configure, config_read as configure_read, creator_profiles, Config, CreatorProfile,
    NewsItem, ValidatorProfile, ANONID_CREATORADDRESS, ANONID_VALIDATORADDRESS, CREATOR_PROFILES,
    NEWS_ITEMS, VALIDATOR_PROFILES,
};

/// POST ACTIONS

/// Create a new profile for a creator
pub fn create_creator_profile(deps: DepsMut, env: &Env, info: &MessageInfo) -> StdResult<Response> {
    let state: Config = configure_read(deps.storage).load()?;
    let anonymous_id = generate_anonymous_id(env, info, state.entropy.as_bytes());

    let profile = CreatorProfile {
        anonymous_id: anonymous_id.clone(),
        stake: Uint128::zero(),
        reputation: Some(0),
        warnings_received: 0,
    };

    ANONID_CREATORADDRESS.insert(deps.storage, &anonymous_id, &info.sender)?;

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

/// Create a new profile for a validator
pub fn create_validator_profile(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
) -> StdResult<Response> {
    let state: Config = configure_read(deps.storage).load()?;
    let anonymous_id = generate_anonymous_id(env, info, state.entropy.as_bytes());

    let profile = ValidatorProfile {
        anonymous_id: anonymous_id.clone(),
        reputation_score: Uint128::zero(),

        stake: Uint128::zero(),
        validated_content_count: None,
        last_validation_timestamp: None,
    };

    ANONID_VALIDATORADDRESS.insert(deps.storage, &anonymous_id, &info.sender)?;

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

/// Create a news
pub fn create_news_entry(deps: DepsMut, env: &Env, info: &MessageInfo) -> StdResult<Response> {
    let user_exists = CREATOR_PROFILES
        .add_suffix(info.sender.as_bytes())
        .may_load(deps.storage)?;

    match user_exists {
        Some(_) => {
            // Pass
        }
        None => {
            return Err(StdError::generic_err(
                "User does not have a creator profile",
            ));
        }
    }

    let user_exists = user_exists.unwrap();

    let index = NEWS_ITEMS.get_len(deps.storage)?;

    let news = NewsItem {
        id: (index + 1).to_string(),
        // TODO: Add IPFS hash
        content: "".to_string(),
        creator: user_exists.anonymous_id.clone(),

        validated: false,
        approved: false,

        validator: None,
    };

    NEWS_ITEMS.insert(deps.storage, &index, &news)?;

    Ok(Response::new()
        .add_attribute("method", "create_news_entry")
        .add_attribute("news_id", index.to_string()))
}

/// Find a random validator to validate a news with a given ID
pub fn find_random_validator_and_assign_validate_news(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    news_id: u32,
) -> StdResult<Response> {
    let state: Config = configure_read(deps.storage).load()?;
    let validators_count = ANONID_VALIDATORADDRESS.get_len(deps.storage)?;

    let random_index = generate_random_number(
        env,
        info,
        state.entropy.as_bytes(),
        u64::from(validators_count),
    );

    let mut iter = ANONID_VALIDATORADDRESS.iter(deps.storage)?;

    let random_validator = iter.nth(random_index as usize).unwrap()?;

    let random_validator_anon_id = random_validator.0;

    // TODO: Send a message to the validator to validate the news

    let mut news = NEWS_ITEMS.get(deps.storage, &news_id).unwrap();

    news.validator = Some(random_validator_anon_id.clone());

    NEWS_ITEMS.insert(deps.storage, &news_id, &news)?;

    Ok(Response::new()
        .add_attribute("method", "find_random_validator_and_assign_validate_news")
        .add_attribute("validator", random_validator_anon_id))
}

/// Validate a news
pub fn validate_news_entry(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    news_id: u32,
) -> StdResult<Response> {
    let validator_exists = VALIDATOR_PROFILES
        .add_suffix(info.sender.as_bytes())
        .may_load(deps.storage)?;

    match validator_exists {
        Some(_) => {
            // Pass
        }
        None => {
            return Err(StdError::generic_err(
                "User does not have a validator profile",
            ));
        }
    }

    let user_exists = validator_exists.unwrap();

    let news = NEWS_ITEMS.get(deps.storage, &news_id).unwrap();

    if news.validated {
        return Err(StdError::generic_err("News has already been validated"));
    }

    let mut news = news;
    news.validated = true;
    news.approved = true;
    news.validator = Some(user_exists.anonymous_id.clone());

    NEWS_ITEMS.insert(deps.storage, &news_id, &news)?;

    Ok(Response::new()
        .add_attribute("method", "validate_news_entry")
        .add_attribute("news_id", news_id.to_string()))
}

/// GET QUERY

/// Get a creator profile
pub fn get_creator_profile(deps: Deps, anonymous_id: String) -> StdResult<CreatorProfile> {
    // let profile = CREATOR_PROFILES.add_suffix()
    // .load(deps.storage, anonymous_id.as_bytes())
    // .unwrap();

    // Ok(profile)
    unimplemented!()
}

/// Get a validator profile
pub fn get_validator_profile(deps: Deps, anonymous_id: String) -> StdResult<ValidatorProfile> {
    // let profile = VALIDATOR_PROFILES
    //     .add_suffix(anonymous_id.as_bytes())
    //     .load(deps.storage)
    //     .unwrap();

    // Ok(profile)
    unimplemented!()
}

/// Get a news item
pub fn get_news_item(deps: Deps, news_id: u32) -> StdResult<NewsItem> {
    let news = NEWS_ITEMS.get(deps.storage, &news_id).unwrap();

    Ok(news)
}

/// Get all news items
pub fn get_all_news_items(deps: Deps) -> StdResult<Vec<NewsItem>> {
    // let news = NEWS_ITEMS.range(deps.storage, None, None, cosmwasm_std::Order::Ascending);

    // let mut news_items: Vec<NewsItem> = vec![];

    // for item in news {
    //     news_items.push(item.1);
    // }

    // Ok(news_items)
    unimplemented!()
}
