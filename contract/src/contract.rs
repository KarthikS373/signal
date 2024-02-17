use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config as configure, config_read as configure_read, Config};

// Excecutables
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
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
        ExecuteMsg::CreateCreatorProfile { stake, viewing_key } => {}
        ExecuteMsg::CreateValidatorProfile { stake, viewing_key } => {}
        _ => unimplemented!(),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetProfileWithViewingKey { viewing_key } => {}
        QueryMsg::GetNewsItem { news_id } => {}
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
    use super::*;
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{from_binary, Coin, StdError, Uint128};

    #[test]
    fn proper_initialization() {
        
    }
}
