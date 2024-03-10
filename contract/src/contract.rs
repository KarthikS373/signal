use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::actions::{
    create_creator_profile, create_news_entry, create_validator_profile, get_all_news_items,
    get_config, get_news_item, get_news_of_creator, lock_funds, tip_creator, update_creator_stake,
    validate_news_entry, withdraw_creator_stake, withdraw_locked_funds, withdraw_tip,
    CreateCreatorProfileArgs, CreateNewsArgs, CreateValidatorProfileArgs, GetNewsItemArgs,
    GetNewsOfCreatorArgs, LockFundsArgs, TipCreatorArgs, UpdateCreatorStakeArgs, ValidateNewsArgs,
    WithdrawLockedFundsArgs, WithdrawStakeArgs, WithdrawTipArgs,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
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
        /*
         * route: create_creator_profile
         * args: {}
         */
        ExecuteMsg::CreateCreatorProfile {} => {
            deps.api.debug("create_creator_profile");
            create_creator_profile(deps, &env, &info, CreateCreatorProfileArgs {})
        }
        /*
         * route: create_validator_profile
         * args: {}
         */
        ExecuteMsg::CreateValidatorProfile {} => {
            deps.api.debug("create_validator_profile");
            create_validator_profile(deps, &env, &info, CreateValidatorProfileArgs {})
        }
        /*
         */
        ExecuteMsg::DepositStake {} => {
            deps.api.debug("deposit_stake");
            update_creator_stake(deps, &env, &info, UpdateCreatorStakeArgs {})
        }
        ExecuteMsg::WithdrawStake {} => {
            deps.api.debug("withdraw_stake");
            withdraw_creator_stake(deps, &env, &info, WithdrawStakeArgs {})
        }
        /*
         * route: create_news_entry
         * args: {
         *   content: String --> IPFS hash with the news content
         * }
         */
        ExecuteMsg::PostNews { content } => {
            deps.api.debug("create_news_entry");
            create_news_entry(deps, &env, &info, CreateNewsArgs { content })
        }
        /*
         * route: validate_news_entry
         * args: {
         *   news_id: Uint64 --> ID of the news to validate
         *   vote: bool --> Vote for the news (true = approve, false = reject)
         *   comment: String --> Comment for the news (min 300 characters, max 1000 characters)
         * }
         */
        ExecuteMsg::ValidateNews {
            news_id,
            comment,
            vote,
        } => {
            deps.api.debug("validate_news");
            validate_news_entry(
                deps,
                &env,
                &info,
                ValidateNewsArgs {
                    news_id: news_id.u64(),
                    vote,
                    comment,
                },
            )
        }
        /*
         * route: tip_creator
         * args: {
         *  creator: String --> Anonymous ID of the creator to tip
         *  amount: Uint128 --> Amount to tip
         * }
         */
        ExecuteMsg::TipCreator {
            creator_anonymous_id,
        } => {
            deps.api.debug("tip_creator");
            tip_creator(
                deps,
                &env,
                &info,
                TipCreatorArgs {
                    creator_anonymous_id,
                },
            )
        }
        /*
         * route: withdraw_tip
         * args: {
         *   amount: Uint128 --> Amount to withdraw
         * }
         */
        ExecuteMsg::WithdrawTip { amount } => {
            deps.api.debug("withdraw_tip");
            withdraw_tip(deps, &env, &info, WithdrawTipArgs { amount })
        }
        /*
         * route: lock_funds
         * args: {
         *   amount: Uint128 --> Amount to lock
         * }
         */
        ExecuteMsg::LockFunds {} => {
            deps.api.debug("lock_funds");
            lock_funds(deps, &env, &info, LockFundsArgs {})
        }
        /*
         * route: unlock_funds
         * args: {
         *   amount: Uint128 --> Amount to unlock
         * }
         */
        ExecuteMsg::UnlockFunds {} => {
            deps.api.debug("unlock_funds");
            withdraw_locked_funds(deps, &env, &info, WithdrawLockedFundsArgs {})
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        /*
         * route: get_config
         * args: {}
         */
        QueryMsg::GetConfig {} => {
            deps.api.debug("get_config");
            get_config(deps, &env)
        }
        /*
         * route: get_news_item
         * args: {
         *   news_id: Uint64 --> ID of the news to retrieve
         * }
         */
        QueryMsg::GetNewsItem { news_id } => {
            deps.api.debug("get_news_item");
            get_news_item(
                deps,
                &env,
                GetNewsItemArgs {
                    news_id: news_id.u64() as u32,
                },
            )
        }
        /*
         * route: get_all_news_items
         * args: {}
         */
        QueryMsg::GetAllNewsItems {} => {
            deps.api.debug("get_all_news_items");
            get_all_news_items(deps, &env)
        }
        /*
         * route: get_all_news_items_by_creator
         */
        QueryMsg::GetAllNewsItemsByCreator { creator } => {
            deps.api.debug("get_all_news_items_by_creator");
            get_news_of_creator(
                deps,
                &env,
                GetNewsOfCreatorArgs {
                    creator_anonymous_id: creator,
                },
            )
        }
    }
}
