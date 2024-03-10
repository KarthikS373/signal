// Unit tests
#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies, mock_env, mock_info},
        Uint128,
    };

    use crate::{
        contract::{execute, instantiate},
        msg::{ExecuteMsg, InstantiateMsg},
        state::{ANONID_CREATORADDRESS, ANONID_VALIDATORADDRESS, CREATOR_PROFILES},
    };

    #[test]
    // Test 01: The proper initialization of the contract
    fn test_proper_initialization() {
        let mut binding = mock_dependencies();

        let deps = binding.as_mut();
        let env = mock_env();

        let info = mock_info("owner", &[]);
        // let info = mock_info("owner", &coins(1000, "uscrt"));

        // Declare variables
        let init_msg = InstantiateMsg {
            entropy: "random".to_owned(),
            creator_base_stake: Uint128::one(),
            validator_base_stake: Uint128::one(),
        };

        let msg = init_msg.clone();

        // Check if the contract can be instantiated properly
        let res = instantiate(deps, env, info, msg).unwrap();

        // Response attributes
        let method = res.attributes[0].clone();
        let creator_base_stake = res.attributes[1].clone();

        // Check 01: check if res has attribute method and value is instantiate
        assert_eq!(
            method,
            ("method", "instantiate"),
            "Check if method is instantiate"
        );

        // Check 02: check if res has attribute creator_base_stake and value is specified in the dummy message
        assert_eq!(
            creator_base_stake,
            (
                "creator_base_stake",
                init_msg.creator_base_stake.to_string()
            ),
            "Check if creator_base_stake is same as the one in the message"
        );
    }

    // Test 02: Check if the contract can create a creator profile
    #[test]
    fn test_create_creator_profile() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);

        // Initialise
        let init_msg = InstantiateMsg {
            entropy: "test_entropy".to_string(),
            creator_base_stake: Uint128::new(100),
            validator_base_stake: Uint128::new(50),
        };
        let init_res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg);

        // Pre check 01: check if the contract can be instantiated properly
        assert!(
            init_res.is_ok(),
            "Contract initialization failed: {:?}",
            init_res
        );

        // Check if the contract can create a creator profile
        let res = execute(
            deps.as_mut(),
            env,
            info.clone(),
            ExecuteMsg::CreateCreatorProfile {},
        );

        // Check 01: check if the contract can create a creator profile
        assert!(res.is_ok(), "Failed to create profile: {:?}", res);

        let res = res.unwrap();

        // Response attributes
        let method = res.attributes[0].clone();
        let creator = res.attributes[1].clone();

        // Check storage
        let stored_anonid_creator_addr = ANONID_CREATORADDRESS
            .get(&deps.storage, &creator.value)
            .unwrap();

        let stored_creator_profile = CREATOR_PROFILES
            .add_suffix(info.sender.as_bytes())
            .load(&deps.storage)
            .unwrap();

        // Check 02: check if res has attribute method and value is create_creator_profile
        assert_eq!(
            method,
            ("method", "create_creator_profile"),
            "Check if method is create_creator_profile"
        );

        // Check 03: check if res has attribute creator
        assert_eq!(
            creator.key, "creator",
            "Check if creator anon ID is the key in the response"
        );

        // Check 04: check the storage if the creator profile address is stored anonymously
        assert_eq!(
            stored_anonid_creator_addr.to_string(),
            info.sender.to_string(),
            "Check if the creator profile address is stored anonymously"
        );

        // Check 05: check the storage if the creator profile is stored
        assert_eq!(
            stored_creator_profile.anonymous_id, creator.value,
            "Check if the creator profile is stored"
        );

        // Check 06: check the storage if the creator profile stake is stored
        assert_eq!(
            stored_creator_profile.stake,
            Uint128::zero(),
            "Check if the creator profile stake is stored and is zero (intial value)"
        );

        // Check 07: check the storage if the creator profile reputation is stored
        assert_eq!(
            stored_creator_profile.reputation, 0,
            "Check if the creator profile reputation is stored and is None (intial value)"
        );

        // Check 08: check the storage if the creator profile warnings received is stored
        assert_eq!(
            stored_creator_profile.warnings_received, 0,
            "Check if the creator profile warnings received is stored and is zero (intial value)"
        );
    }

    // Test 03: Check if the contract can create multiple creator profiles
    #[test]
    fn test_create_multiple_creator_profile() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let init_info = mock_info("creator", &[]);

        // Initialise the contract
        let init_msg = InstantiateMsg {
            entropy: "test_entropy".to_string(),
            creator_base_stake: Uint128::new(100),
            validator_base_stake: Uint128::new(50),
        };

        let init_res = instantiate(deps.as_mut(), env.clone(), init_info.clone(), init_msg);

        // Pre check 01: check if the contract can be instantiated properly
        assert!(
            init_res.is_ok(),
            "Contract initialization failed: {:?}",
            init_res
        );

        // Define creators and their corresponding info
        // let creators = vec!["creator1", "creator2"];
        // let creators = vec!["creator1", "creator2", "creator3", "creator4", "creator5"];
        let creators = vec!["creator", "creator"];
        let mut anon_ids = vec![]; // Populate with anon_ids as they are created and check if they are unique

        // Iterate over creators and create profiles
        for creator_name in creators {
            let creator_info = mock_info(creator_name, &[]);

            // Create creator profile
            let res = execute(
                deps.as_mut(),
                env.clone(),
                creator_info.clone(),
                ExecuteMsg::CreateCreatorProfile {},
            );

            // Check 01: check if the contract can create a creator profile
            assert!(
                res.is_ok(),
                "Failed to create profile for {}: {:?}",
                creator_name,
                res
            );

            let res = res.unwrap();

            // Check response attributes
            let method = res.attributes[0].clone();
            let creator = res.attributes[1].clone();

            anon_ids.push(creator.value.clone());

            // Check if the creator profile is stored
            let stored_anonid_creator_addr = ANONID_CREATORADDRESS
                .get(&deps.storage, &creator.value)
                .unwrap();
            let stored_creator_profile = CREATOR_PROFILES
                .add_suffix(creator_info.sender.as_bytes())
                .load(&deps.storage)
                .unwrap();

            // Check 02: check if res has attribute method and value is create_creator_profile
            assert_eq!(
                method,
                ("method", "create_creator_profile"),
                "Check if method is create_creator_profile"
            );

            // Check 03: check if res has attribute creator
            assert_eq!(
                stored_anonid_creator_addr.to_string(),
                creator_info.sender.to_string(),
                "Check if the creator profile address is stored anonymously"
            );

            // Check 04: check the storage if the creator profile is stored
            assert_eq!(
                stored_creator_profile.anonymous_id, creator.value,
                "Check if the creator profile is stored"
            );

            // Check 05: check the storage if the creator profile stake is stored
            assert_eq!(
                stored_creator_profile.stake,
                Uint128::zero(),
                "Check if the creator profile stake is stored and is zero (intial value)"
            );

            // Check 06: check the storage if the creator profile reputation is stored
            assert_eq!(
                stored_creator_profile.reputation, 0,
                "Check if the creator profile reputation is stored and is None (intial value)"
            );

            // Check 07: check the storage if the creator profile warnings received is stored
            assert_eq!(
                stored_creator_profile.warnings_received, 0,
                "Check if the creator profile warnings received is stored and is zero (intial value)"
            );
        }

        // Check 08: check if the anon_ids are unique
        let unique_anon_ids: Vec<&String> = anon_ids.iter().collect();
        assert_eq!(
            unique_anon_ids.len(),
            anon_ids.len(),
            "Check if the anon_ids are unique"
        );
    }

    // Test 04: Check if the contract can create a validator profile
    #[test]
    fn test_create_validator_profile() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let init_info = mock_info("validator", &[]);

        // Initialise the contract
        let init_msg = InstantiateMsg {
            entropy: "test_entropy".to_string(),
            creator_base_stake: Uint128::new(100),
            validator_base_stake: Uint128::new(50),
        };

        let init_res = instantiate(deps.as_mut(), env.clone(), init_info.clone(), init_msg);

        // Pre check 01: check if the contract can be instantiated properly
        assert!(
            init_res.is_ok(),
            "Contract initialization failed: {:?}",
            init_res
        );

        // Create validator profile
        let res = execute(
            deps.as_mut(),
            env.clone(),
            init_info.clone(),
            ExecuteMsg::CreateValidatorProfile {},
        );

        // Check 01: check if the contract can create a validator profile
        assert!(res.is_ok(), "Failed to create profile: {:?}", res);

        let res = res.unwrap();

        // Response attributes
        let method = res.attributes[0].clone();
        let validator = res.attributes[1].clone();

        // Check storage
        let stored_anonid_validator_addr = ANONID_VALIDATORADDRESS
            .get(&deps.storage, &validator.value)
            .unwrap();

        // Check 02: check if res has attribute method and value is create_validator_profile
        assert_eq!(
            method,
            ("method", "create_validator_profile"),
            "Check if method is create_validator_profile"
        );

        // Check 03: check if res has attribute validator
        assert_eq!(
            validator.key, "validator",
            "Check if validator anon ID is the key in the response"
        );

        // Check 04: check the storage if the validator profile address is stored anonymously
        assert_eq!(
            stored_anonid_validator_addr.to_string(),
            init_info.sender.to_string(),
            "Check if the validator profile address is stored anonymously"
        );
    }

    // Test 05-a: Creator stake (more than the base requirement)
    // - Creator: 1000
    // - Base requirement: 100
    #[test]
    #[should_panic(expected = "Stake does not meet the base requirement")]
    fn test_creator_stake_more_than_req() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let creator_info = mock_info("creator", &[]);

        // Initialise
        let init_msg = InstantiateMsg {
            entropy: "test_entropy".to_string(),
            creator_base_stake: Uint128::new(100),
            validator_base_stake: Uint128::new(50),
        };
        let init_res = instantiate(deps.as_mut(), env.clone(), creator_info.clone(), init_msg);

        // Pre check 01: check if the contract can be instantiated properly
        assert!(
            init_res.is_ok(),
            "Contract initialization failed: {:?}",
            init_res
        );

        // Create creator profile
        let res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            ExecuteMsg::CreateCreatorProfile {},
        );

        // Pre check 02: check if the contract can create a creator profile
        assert!(res.is_ok(), "Failed to create profile: {:?}", res);

        let stored_creator_profile = CREATOR_PROFILES
            .add_suffix(creator_info.sender.as_bytes())
            .load(&deps.storage)
            .unwrap();

        // Check 01: check if the creator profile stake is stored
        assert_eq!(
            stored_creator_profile.stake,
            Uint128::zero(),
            "Check if the creator profile stake is stored and is zero (intial value)"
        );

        let creator_info_with_stake = mock_info("creator", &coins(1000, "uscrt"));
        // Update creator profile stake
        let res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info_with_stake.clone(),
            ExecuteMsg::DepositStake {},
        );

        // Check 02: check if the creator profile stake is updated
        assert!(
            res.is_ok(),
            "Failed to update creator profile stake: {:?}",
            res
        );

        let stored_creator_profile = CREATOR_PROFILES
            .add_suffix(creator_info.sender.as_bytes())
            .load(&deps.storage)
            .unwrap();

        // Check 03: check if the creator profile stake is updated
        assert_eq!(
            stored_creator_profile.stake,
            Uint128::new(100),
            "Check if the creator profile stake is updated"
        );
    }

    // Test 05-b: Creator stake (less than the base requirement)
    // - Creator: 10
    // - Base requirement: 100
    #[test]
    #[should_panic(expected = "Stake does not meet the base requirement")]
    fn test_creator_stake_less_than_req() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let creator_info = mock_info("creator", &[]);

        // Initialise
        let init_msg = InstantiateMsg {
            entropy: "test_entropy".to_string(),
            creator_base_stake: Uint128::new(100),
            validator_base_stake: Uint128::new(50),
        };
        let init_res = instantiate(deps.as_mut(), env.clone(), creator_info.clone(), init_msg);

        // Pre check 01: check if the contract can be instantiated properly
        assert!(
            init_res.is_ok(),
            "Contract initialization failed: {:?}",
            init_res
        );

        // Create creator profile
        let res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            ExecuteMsg::CreateCreatorProfile {},
        );

        // Pre check 02: check if the contract can create a creator profile
        assert!(res.is_ok(), "Failed to create profile: {:?}", res);

        let stored_creator_profile = CREATOR_PROFILES
            .add_suffix(creator_info.sender.as_bytes())
            .load(&deps.storage)
            .unwrap();

        // Check 01: check if the creator profile stake is stored
        assert_eq!(
            stored_creator_profile.stake,
            Uint128::zero(),
            "Check if the creator profile stake is stored and is zero (intial value)"
        );

        let creator_info_with_stake = mock_info("creator", &coins(10, "uscrt"));
        // Update creator profile stake
        let res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info_with_stake.clone(),
            ExecuteMsg::DepositStake {},
        );

        // Check 02: check if the creator profile stake is updated
        assert!(
            res.is_ok(),
            "Failed to update creator profile stake: {:?}",
            res
        );

        let stored_creator_profile = CREATOR_PROFILES
            .add_suffix(creator_info.sender.as_bytes())
            .load(&deps.storage)
            .unwrap();

        // Check 03: check if the creator profile stake is updated
        assert_eq!(
            stored_creator_profile.stake,
            Uint128::new(100),
            "Check if the creator profile stake is updated"
        );
    }

    // Test 05-c: Creator stake (wrong token)
    // - Creator: 100 meme
    // - Base requirement: 100 uscrt
    #[test]
    #[should_panic(expected = "No SCRT sent")]
    fn test_creator_stake_wrong_denom() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let creator_info = mock_info("creator", &[]);

        // Initialise
        let init_msg = InstantiateMsg {
            entropy: "test_entropy".to_string(),
            creator_base_stake: Uint128::new(100),
            validator_base_stake: Uint128::new(50),
        };
        let init_res = instantiate(deps.as_mut(), env.clone(), creator_info.clone(), init_msg);

        // Pre check 01: check if the contract can be instantiated properly
        assert!(
            init_res.is_ok(),
            "Contract initialization failed: {:?}",
            init_res
        );

        // Create creator profile
        let res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            ExecuteMsg::CreateCreatorProfile {},
        );

        // Pre check 02: check if the contract can create a creator profile
        assert!(res.is_ok(), "Failed to create profile: {:?}", res);

        let stored_creator_profile = CREATOR_PROFILES
            .add_suffix(creator_info.sender.as_bytes())
            .load(&deps.storage)
            .unwrap();

        // Check 01: check if the creator profile stake is stored
        assert_eq!(
            stored_creator_profile.stake,
            Uint128::zero(),
            "Check if the creator profile stake is stored and is zero (intial value)"
        );

        let creator_info_with_stake = mock_info("creator", &coins(100, "meme"));
        // Update creator profile stake
        let res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info_with_stake.clone(),
            ExecuteMsg::DepositStake {},
        );

        // Check 02: check if the creator profile stake is updated
        assert!(
            res.is_ok(),
            "Failed to update creator profile stake: {:?}",
            res
        );

        let stored_creator_profile = CREATOR_PROFILES
            .add_suffix(creator_info.sender.as_bytes())
            .load(&deps.storage)
            .unwrap();

        // Check 03: check if the creator profile stake is updated
        assert_eq!(
            stored_creator_profile.stake,
            Uint128::new(100),
            "Check if the creator profile stake is updated"
        );
    }

    // Test 05-c: Creator stake (wrong token)
    // - Creator: 100 meme
    // - Base requirement: 100 uscrt
    #[test]
    fn test_creator_stake() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let creator_info = mock_info("creator", &[]);

        // Initialise
        let init_msg = InstantiateMsg {
            entropy: "test_entropy".to_string(),
            creator_base_stake: Uint128::new(100),
            validator_base_stake: Uint128::new(50),
        };
        let init_res = instantiate(deps.as_mut(), env.clone(), creator_info.clone(), init_msg);

        // Pre check 01: check if the contract can be instantiated properly
        assert!(
            init_res.is_ok(),
            "Contract initialization failed: {:?}",
            init_res
        );

        // Create creator profile
        let res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            ExecuteMsg::CreateCreatorProfile {},
        );

        // Pre check 02: check if the contract can create a creator profile
        assert!(res.is_ok(), "Failed to create profile: {:?}", res);

        let stored_creator_profile = CREATOR_PROFILES
            .add_suffix(creator_info.sender.as_bytes())
            .load(&deps.storage)
            .unwrap();

        // Check 01: check if the creator profile stake is stored
        assert_eq!(
            stored_creator_profile.stake,
            Uint128::zero(),
            "Check if the creator profile stake is stored and is zero (intial value)"
        );

        let creator_info_with_stake = mock_info("creator", &coins(100, "uscrt"));
        // Update creator profile stake
        let res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info_with_stake.clone(),
            ExecuteMsg::DepositStake {},
        );

        // Check 02: check if the creator profile stake is updated
        assert!(
            res.is_ok(),
            "Failed to update creator profile stake: {:?}",
            res
        );

        let stored_creator_profile = CREATOR_PROFILES
            .add_suffix(creator_info.sender.as_bytes())
            .load(&deps.storage)
            .unwrap();

        // Check 03: check if the creator profile stake is updated
        assert_eq!(
            stored_creator_profile.stake,
            Uint128::new(100),
            "Check if the creator profile stake is updated"
        );

        // Check 04: check balance of contract address
        let contract_address = env.contract.address.clone();
        // let contract_current_balance = deps.querier.(&contract_address).unwrap();

        // assert_eq!(
        //     contract_current_balance[0].amount,
        //     Uint128::new(100),
        //     "Check if the contract balance is updated"
        // );
    }

    // Test 06-a: Check creating a news entry (without creating a creator profile)
    #[test]
    #[should_panic(expected = "User does not have a creator profile")]
    fn test_create_news_entry() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("creator", &[]);

        // Initialise
        let init_msg = InstantiateMsg {
            entropy: "test_entropy".to_string(),
            creator_base_stake: Uint128::new(100),
            validator_base_stake: Uint128::new(50),
        };
        let init_res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg);

        // Pre check 01: check if the contract can be instantiated properly
        assert!(
            init_res.is_ok(),
            "Contract initialization failed: {:?}",
            init_res
        );

        // Create news entry - should panic as only creator can create news
        let res = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::PostNews {
                content: "test_content".to_string(),
            },
        );

        // Check 01: check if the contract can create a news entry
        assert!(res.is_ok(), "Failed to create news entry: {:?}", res);

        let res = res.unwrap();

        // Response attributes
        let method = res.attributes[0].clone();
        let news_id = res.attributes[1].clone();

        // Check 02: check if res has attribute method and value is create_news_entry
        assert_eq!(
            method,
            ("method", "create_news_entry"),
            "Check if method is create_news_entry"
        );

        // Check 03: check if res has attribute news_id
        assert_eq!(
            news_id.key, "news_id",
            "Check if news_id is the key in the response"
        );
    }

    // Test 06-b: Check creating a news entry (with a creator profile without stake)
    #[test]
    #[should_panic(expected = "Stake does not meet the base requirement")]
    fn test_create_news_entry_with_creator_profile() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let creator_info = mock_info("creator", &[]);

        // Initialise
        let init_msg = InstantiateMsg {
            entropy: "test_entropy".to_string(),
            creator_base_stake: Uint128::new(100),
            validator_base_stake: Uint128::new(50),
        };
        let init_res = instantiate(deps.as_mut(), env.clone(), creator_info.clone(), init_msg);

        // Pre check 01: check if the contract can be instantiated properly
        assert!(
            init_res.is_ok(),
            "Contract initialization failed: {:?}",
            init_res
        );

        // Create creator profile
        let res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            ExecuteMsg::CreateCreatorProfile {},
        );

        // Pre check 02: check if the contract can create a creator profile
        assert!(res.is_ok(), "Failed to create profile: {:?}", res);

        // Create news entry
        let res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            ExecuteMsg::PostNews {
                content: "test_content".to_string(),
            },
        );

        // Check 01: check if the contract can create a news entry
        assert!(res.is_ok(), "Failed to create news entry: {:?}", res);

        let res = res.unwrap();

        // Response attributes
        let method = res.attributes[0].clone();
        let news_id = res.attributes[1].clone();

        // Check 04: check if res has attribute method and value is create_news_entry
        assert_eq!(
            method,
            ("method", "create_news_entry"),
            "Check if method is create_news_entry"
        );

        // Check 05: check if res has attribute news_id
        assert_eq!(
            news_id.key, "news_id",
            "Check if news_id is the key in the response"
        );
    }
}
