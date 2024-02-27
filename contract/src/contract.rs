use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::actions::{create_creator_profile, create_news_entry, create_validator_profile};
use crate::msg::{self, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config as configure, Config};

// Excecutables
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        entropy: msg.entropy,

        creator_base_stake: msg.creator_base_stake,
        validator_base_stake: msg.validator_base_stake,
    };

    deps.api
        .debug(format!("Contract was initialized by {}", info.sender).as_str());

    configure(deps.storage).save(&config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("creator_base_stake", msg.creator_base_stake.to_string())
        .add_attribute("validator_base_stake", msg.validator_base_stake.to_string()))
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateCreatorProfile {
            stake: _,
            viewing_key,
        } => {
            deps.api.debug("create_creator_profile");
            create_creator_profile(deps, &env, &info)
        }
        ExecuteMsg::CreateValidatorProfile {
            stake: _,
            viewing_key,
        } => {
            deps.api.debug("create_validator_profile");
            create_validator_profile(deps, &env, &info)
        }
        ExecuteMsg::PostNews {
            content,
            anonymous_id,
        } => {
            deps.api.debug("create_news_entry");
            create_news_entry(deps, &env, &info)
        }
        ExecuteMsg::ValidateNews {
            news_id,
            approved,
            anonymous_id,
        } => {
            deps.api.debug("validate_news");
            create_news_entry(deps, &env, &info)
        }
        _ => unimplemented!(),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // QueryMsg::GetProfileWithViewingKey { viewing_key } => {}
        // QueryMsg::GetNewsItem { news_id } => {}
        _ => unimplemented!(),
    }
}

// Actions
pub fn sample_fn(deps: DepsMut, _env: Env) -> StdResult<Response> {
    deps.api.debug("executed successfully");
    // Err(StdError::generic_err("Only the owner can reset count"))
    // let state = config_read(deps.storage).load()?;
    // Ok(CountResponse { count: state.count })
    Ok(Response::default())
}

// Unit tests
#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        StdError, Uint128,
    };

    use crate::{
        contract::{execute, instantiate},
        msg::{ExecuteMsg, InstantiateMsg},
    };

    // Tests the proper initialization of the contract
    #[test]
    fn test_proper_initialization() {
        // Declare variables
        let dummy_message = InstantiateMsg {
            entropy: "random".to_owned(),
            creator_base_stake: Uint128::one(),
            validator_base_stake: Uint128::one(),
        };

        let mut deps = mock_dependencies();
        let msg = dummy_message.clone();

        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // check if res has attribute method and value is instantiate
        assert_eq!(
            res.attributes[0],
            ("method", "instantiate"),
            "Check if method is instantiate"
        );

        // check if res has attribute creator_base_stake and value is 1
        assert_eq!(
            res.attributes[1],
            (
                "creator_base_stake",
                dummy_message.creator_base_stake.to_string()
            ),
            "Check if creator_base_stake is same as the one in the message"
        );

        // Check if res has attribute validator_base_stake and value is 1
        assert_eq!(
            res.attributes[2],
            (
                "validator_base_stake",
                dummy_message.validator_base_stake.to_string()
            ),
            "Check if validator_base_stake is same as the one in the message"
        );
    }

    // Check if the contract can create a creator profile
    #[test]
    fn test_create_creator_profile() {
        // Initialise
        test_proper_initialization();

        // Declare variables
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);
        let stake = Uint128::one();
        let viewing_key = "viewing_key".to_owned();

        let res = execute(
            deps.as_mut(),
            mock_env(),
            info.to_owned(),
            ExecuteMsg::CreateCreatorProfile { stake, viewing_key },
        );

        if res.is_err() {
            panic!("Failed to create profile: {:?}", res);
        }

        let res = res.unwrap();

        // Check if the function returns a response
        assert_eq!(
            res.attributes[0],
            ("method", "create_creator_profile"),
            "Check if method is create_creator_profile"
        );

        // Check if the creator is the same as the one in the message
        assert_ne!(
            res.attributes[1],
            ("creator", info.sender.to_string()),
            "Check if creator is the same as the one in the message"
        );
    }

    // Check if the contract can create multiple creator profiles
    #[test]
    fn test_create_multiple_creator_profile() {
        // Initialise
        test_proper_initialization();

        // Declare variables
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);
        let stake = Uint128::one();
        let viewing_key = "viewing_key".to_owned();

        // Try to create profile 1
        let res1 = execute(
            deps.as_mut(),
            mock_env(),
            info.to_owned(),
            ExecuteMsg::CreateCreatorProfile {
                stake,
                viewing_key: viewing_key.to_string(),
            },
        );

        if res1.is_err() {
            panic!("Failed to create profile: {:?}", res1);
        }

        let res1 = res1.unwrap();

        // Check if creating first profile returns a response
        assert_eq!(
            res1.attributes[0],
            ("method", "create_creator_profile"),
            "Check if creating profile 1 is returning a response create_creator_profile"
        );

        // Try to create profile 2
        let res2 = execute(
            deps.as_mut(),
            mock_env(),
            info.to_owned(),
            ExecuteMsg::CreateCreatorProfile {
                stake,
                viewing_key: viewing_key.to_string(),
            },
        );

        if res2.is_err() {
            panic!("Failed to create profile: {:?}", res2);
        }

        let res2 = res2.unwrap();

        // Check if creating second profile returns a response
        assert_eq!(
            res2.attributes[0],
            ("method", "create_creator_profile"),
            "Check if creating profile 2 is returning a response create_creator_profile"
        );

        // Check if both the IDs match or not
        assert_ne!(
            res1.attributes[2], res2.attributes[2],
            "Changing the profile reassign the ID to keep no track"
        );
    }
}
