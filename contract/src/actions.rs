use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Uint128,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    config as configure, config_read as configure_read, creator_profiles, Config, CreatorProfile,
};

/*
 * Create a new profile for a creator
*/
fn create_creator_profile(
    deps: DepsMut,
    info: MessageInfo,
    stake: Uint128,
    viewing_key: String,
) -> StdResult<Response> {
    let state: Config = configure_read(deps.storage).load()?;

    if stake < state.creator_base_stake {
        return Err(StdError::generic_err(
            "Stake does not meet the base requirement",
        ));
    }

    // TODO: Generate an anonymous ID for the creator
    let anonymous_id = String::from("1");

    // Save the creator profile
    let profile = CreatorProfile {
        anonymous_id,
        addr: info.sender.clone(),
        stake,
        reputation: Some(0), // Starting reputation
        warnings_received: 0,
    };

    // creator_profiles(deps.storage).save(info.sender.as_bytes())?;

    Ok(Response::new()
        .add_attribute("method", "create_creator_profile")
        .add_attribute("creator", info.sender))
}
