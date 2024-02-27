use cosmwasm_std::{
    coins, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128,
};

use crate::helpers::{generate_anonymous_id, generate_random_number};
use crate::state::{
    config_read as configure_read, Config, CreatorProfile, NewsItem, NewsItemWithValidations,
    Validation, ValidatorProfile, ANONID_CREATORADDRESS, ANONID_VALIDATORADDRESS, CREATOR_PROFILES,
    NEWS_ITEMS, NEWS_VALIDATIONS, VALIDATOR_PROFILES,
};

/// POST ACTIONS

/// Create a new profile for a creator
pub struct CreateCreatorProfileArgs {}
pub fn create_creator_profile(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    args: CreateCreatorProfileArgs,
) -> StdResult<Response> {
    let state = configure_read(deps.storage).load()?;
    let anonymous_id = generate_anonymous_id(env, info, state.entropy.as_bytes());

    let profile = CreatorProfile {
        anonymous_id: anonymous_id.clone(),
        stake: Uint128::zero(), // Initial stake is zero
        reputation: Some(0),    // Initial reputation is zero
        warnings_received: 0,   // Initial warnings received is zero
    };

    ANONID_CREATORADDRESS.insert(deps.storage, &anonymous_id, &info.sender)?;

    CREATOR_PROFILES
        .add_suffix(info.sender.as_bytes())
        .save(deps.storage, &profile)?;

    Ok(Response::new()
        .add_attribute("method", "create_creator_profile")
        .add_attribute("creator", anonymous_id.to_string()))
}

/// Create a new profile for a validator
pub struct CreateValidatorProfileArgs {}
pub fn create_validator_profile(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    args: CreateValidatorProfileArgs,
) -> StdResult<Response> {
    let state: Config = configure_read(deps.storage).load()?;
    let anonymous_id = generate_anonymous_id(env, info, state.entropy.as_bytes());

    let profile = ValidatorProfile {
        anonymous_id: anonymous_id.clone(),
        reputation_score: Uint128::zero(),

        validated_content_count: None, // Initial validated content count is zero
        last_validation_timestamp: None, // Timestamp of the last validation
    };

    ANONID_VALIDATORADDRESS.insert(deps.storage, &anonymous_id, &info.sender)?;

    VALIDATOR_PROFILES
        .add_suffix(info.sender.as_bytes())
        .save(deps.storage, &profile)?;

    Ok(Response::new()
        .add_attribute("method", "create_validator_profile")
        .add_attribute("validator", anonymous_id.to_string()))
}

/// Create a news
pub struct CreateNewsArgs {
    pub content: String,
}
pub fn create_news_entry(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    args: CreateNewsArgs,
) -> StdResult<Response> {
    let user_exists = CREATOR_PROFILES
        .add_suffix(info.sender.as_bytes())
        .may_load(deps.storage)?;

    match user_exists {
        Some(_) => {
            // User exists - pass
        }
        None => {
            return Err(StdError::generic_err(
                "User does not have a creator profile",
            ));
        }
    }

    let state: Config = configure_read(deps.storage).load()?;

    let user_exists = user_exists.unwrap();

    // Check if user has staked the required amount of SCRT
    let stake = user_exists.stake.u128();
    let base_stake = state.creator_base_stake.u128();

    if stake < base_stake - 1 || stake > base_stake + 1 {
        return Err(StdError::generic_err(
            "Stake does not meet the base requirement",
        ));
    }

    let index = NEWS_ITEMS.get_len(deps.storage)?;
    let content = args.content;

    let news = NewsItem {
        id: (index + 1).to_string(),
        content: content.clone(),
        creator: user_exists.anonymous_id.clone(),
    };

    NEWS_ITEMS.insert(deps.storage, &index, &news)?;

    Ok(Response::new()
        .add_attribute("method", "create_news_entry")
        .add_attribute("creator", user_exists.anonymous_id)
        .add_attribute("news_id", index.to_string())
        .add_attribute("content", content))
}

/// Find a random validator to validate a news with a given ID
pub struct FindRandomValidatorAndAssignValidateNewsArgs {
    pub news_id: u32,
}
pub fn find_random_validator_and_assign_validate_news(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    args: FindRandomValidatorAndAssignValidateNewsArgs,
) -> StdResult<Response> {
    let news_id = args.news_id;

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
    // news.validator = Some(random_validator_anon_id.clone());

    NEWS_ITEMS.insert(deps.storage, &news_id, &news)?;

    Ok(Response::new()
        .add_attribute("method", "find_random_validator_and_assign_validate_news")
        .add_attribute("validator", random_validator_anon_id))
}

/// Validate a news
pub struct ValidateNewsArgs {
    pub news_id: u64,
    pub vote: bool,
    pub comment: String,
}
pub fn validate_news_entry(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    args: ValidateNewsArgs,
) -> StdResult<Response> {
    let news_id = args.news_id as u32;

    let validator_exists = VALIDATOR_PROFILES
        .add_suffix(info.sender.as_bytes())
        .may_load(deps.storage)?;

    // Check if comment has a length of at least 300 characters
    if args.comment.len() < 300 {
        return Err(StdError::generic_err(
            "Comment is too short (min 300 characters)",
        ));
    }

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

    let mut user_exists = validator_exists.unwrap();
    let news = NEWS_ITEMS.get(deps.storage, &news_id).unwrap();

    let validation = Validation {
        news_id: news.id.clone(),
        validator: user_exists.anonymous_id.clone(),
        vote: args.vote,
        comment: args.comment.clone(),
    };

    NEWS_VALIDATIONS
        .add_suffix(&news.creator.as_bytes())
        .add_suffix(&news_id.to_be_bytes())
        .insert(deps.storage, &user_exists.anonymous_id, &validation)?;

    // Update the reputation of the validator
    user_exists.reputation_score += Uint128::new(1);
    VALIDATOR_PROFILES
        .add_suffix(info.sender.as_bytes())
        .save(deps.storage, &user_exists)?;

    Ok(Response::new()
        .add_attribute("method", "validate_news_entry")
        .add_attribute("news_id", news_id.to_string())
        .add_attribute("validator", user_exists.anonymous_id)
        .add_attribute("vote", args.vote.to_string())
        .add_attribute("comment", args.comment))
}

/// GET QUERY

/// Get the configuration
pub fn get_config(deps: Deps, env: &Env) -> StdResult<Binary> {
    let config = configure_read(deps.storage).load()?;
    to_binary(&config)
}

// /// Get a creator profile
// pub fn get_creator_profile(deps: DepsMut, _env: &Env, info: &MessageInfo) -> StdResult<Binary> {
//     let profile = CREATOR_PROFILES
//         .add_suffix(info.sender.as_bytes())
//         .load(deps.storage)?;

//     to_binary(&profile)
// }

// /// Get a validator profile
// pub fn get_validator_profile(deps: DepsMut, _env: &Env, info: &MessageInfo) -> StdResult<Binary> {
//     let profile = VALIDATOR_PROFILES
//         .add_suffix(info.sender.as_bytes())
//         .load(deps.storage)?;

//     to_binary(&profile)
// }

/// Get a news item
pub struct GetNewsItemArgs {
    pub news_id: u32,
}
pub fn get_news_item(deps: Deps, env: &Env, args: GetNewsItemArgs) -> StdResult<Binary> {
    let news_id = args.news_id;
    let news = NEWS_ITEMS.get(deps.storage, &news_id).unwrap();

    // Get All the related validations
    let binding = NEWS_VALIDATIONS
        .add_suffix(news.creator.as_bytes())
        .add_suffix(news.id.as_bytes());
    let iter = binding.iter(deps.storage).unwrap();

    let mut validations = vec![];
    for item in iter {
        let validation = item?;
        validations.push(Validation {
            news_id: validation.1.news_id,
            validator: validation.0,
            vote: validation.1.vote,
            comment: validation.1.comment,
        });
    }

    let news_with_validations = NewsItemWithValidations {
        news: news,
        validations: validations,
    };

    to_binary(&news_with_validations)
}

/// Get all news items
pub fn get_all_news_items(deps: Deps, env: &Env) -> StdResult<Binary> {
    let news_iter = NEWS_ITEMS.iter(deps.storage);

    let iter = news_iter.unwrap();

    let mut news_items = vec![];

    for item in iter {
        let item = item?;
        news_items.push(item.1);
    }

    to_binary(&news_items)
}

/// Get all news of a creator
pub struct GetNewsOfCreatorArgs {
    pub creator_anonymous_id: String,
}
pub fn get_news_of_creator(deps: Deps, env: &Env, args: GetNewsOfCreatorArgs) -> StdResult<Binary> {
    let creator_anonymous_id = args.creator_anonymous_id;

    let binding = NEWS_ITEMS.add_suffix(creator_anonymous_id.as_bytes());
    let iter = binding.iter(deps.storage).unwrap();

    let mut news_items = vec![];

    for item in iter {
        let item = item?;

        if item.1.creator == creator_anonymous_id {
            news_items.push(item.1);
        }
    }

    to_binary(&news_items)
}

/// Payment actions

/// Update the stake of a creator
pub fn update_creator_stake(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    new_stake: Uint128,
) -> StdResult<Response> {
    let state = configure_read(deps.storage).load()?;
    let sent_amount = info
        .funds
        .iter()
        .find(|f| f.denom == "uscrt")
        .map(|f| f.amount);

    let amount = sent_amount.ok_or_else(|| "No SCRT sent");
    let base_stake = state.validator_base_stake.u128();

    match amount {
        Ok(amount) => {
            let stake = amount.u128();
            if stake > base_stake + 1 || stake < base_stake - 1 {
                return Err(StdError::generic_err(
                    "Stake does not meet the base requirement",
                ));
            }
        }
        Err(_) => {
            return Err(StdError::generic_err("No SCRT sent"));
        }
    }

    let mut profile = CREATOR_PROFILES
        .add_suffix(info.sender.as_bytes())
        .load(deps.storage)?;

    // If already staked before, reject the transaction as we don't support more than one stake
    if profile.stake > Uint128::zero() {
        return Err(StdError::generic_err("Stake already exists"));
    }

    let contract_address = env.contract.address.clone();
    BankMsg::Send {
        to_address: contract_address.to_string(),
        amount: coins(32, "scrt"),
    };

    profile.stake += new_stake;

    CREATOR_PROFILES
        .add_suffix(info.sender.as_bytes())
        .save(deps.storage, &profile)?;

    Ok(Response::new()
        .add_attribute("method", "update_creator_stake")
        .add_attribute("creator", profile.anonymous_id))
}

/// Withdraw the stake of a creator
pub fn withdraw_creator_stake(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    amount: Uint128,
) -> StdResult<Response> {
    let mut profile = CREATOR_PROFILES
        .add_suffix(info.sender.as_bytes())
        .load(deps.storage)?;

    if profile.stake < amount {
        return Err(StdError::generic_err("Insufficient stake"));
    }

    // let contract_address = env.contract.address.clone();
    BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: coins(32, "scrt"),
    };

    profile.stake -= amount;

    CREATOR_PROFILES
        .add_suffix(info.sender.as_bytes())
        .save(deps.storage, &profile)?;

    Ok(Response::new()
        .add_attribute("method", "withdraw_creator_stake")
        .add_attribute("creator", profile.anonymous_id))
}
