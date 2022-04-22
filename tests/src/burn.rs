use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    DEFAULT_RUN_GENESIS_REQUEST,
};
use casper_types::{runtime_args, system::mint, ContractHash, Key, RuntimeArgs, U256};

use crate::utility::{
    constants::{
        ACCOUNT_USER_1, ARG_KEY_NAME, ARG_NFT_CONTRACT_HASH, ARG_TOKEN_ID, ARG_TOKEN_META_DATA,
        ARG_TOKEN_OWNER, ARG_TOKEN_URI, BALANCES, BURNT_TOKENS, CONTRACT_NAME, ENTRY_POINT_BURN,
        MINT_SESSION_WASM, NFT_CONTRACT_WASM, OWNED_TOKENS, OWNED_TOKENS_DICTIONARY_KEY,
        TEST_META_DATA, TEST_URI,
    },
    installer_request_builder::{InstallerRequestBuilder, OwnershipMode},
    support::{self, get_nft_contract_hash},
};

#[test]
fn should_burn_minted_token() {
    const TOKEN_ID: U256 = U256::zero();
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let install_request_builder =
        InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
            .with_total_token_supply(U256::from(100u64))
            .with_ownership_mode(OwnershipMode::TransferableUnchecked)
            .build();

    builder
        .exec(install_request_builder)
        .expect_success()
        .commit();

    let installing_account = builder.get_expected_account(*DEFAULT_ACCOUNT_ADDR);
    let nft_contract_key = installing_account
        .named_keys()
        .get(CONTRACT_NAME)
        .expect("must have key in named keys");

    let nft_contract_hash = get_nft_contract_hash(&builder);

    let mint_session_call = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        MINT_SESSION_WASM,
        runtime_args! {
            ARG_NFT_CONTRACT_HASH => nft_contract_hash,
            ARG_KEY_NAME => Some(OWNED_TOKENS_DICTIONARY_KEY.to_string()),
            ARG_TOKEN_OWNER => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_TOKEN_META_DATA => TEST_META_DATA.to_string(),
            ARG_TOKEN_URI => TEST_URI.to_string(),
        },
    )
    .build();

    builder.exec(mint_session_call).expect_success().commit();

    let actual_owned_tokens = support::get_dictionary_value_from_key::<Vec<U256>>(
        &builder,
        nft_contract_key,
        OWNED_TOKENS,
        &DEFAULT_ACCOUNT_ADDR.clone().to_string(),
    );

    let expected_owned_tokens = vec![U256::zero()];
    assert_eq!(expected_owned_tokens, actual_owned_tokens);
    let actual_balance_before_burn = support::get_dictionary_value_from_key::<U256>(
        &builder,
        nft_contract_key,
        BALANCES,
        &DEFAULT_ACCOUNT_ADDR.clone().to_string(),
    );

    let expected_balance_before_burn = U256::one();
    assert_eq!(actual_balance_before_burn, expected_balance_before_burn);

    let burn_request = ExecuteRequestBuilder::contract_call_by_name(
        *DEFAULT_ACCOUNT_ADDR,
        CONTRACT_NAME,
        ENTRY_POINT_BURN,
        runtime_args! {
            ARG_TOKEN_ID => U256::zero(),
        },
    )
    .build();
    builder.exec(burn_request).expect_success().commit();

    //This will error of token is not registered as burnt.
    let _ = support::get_dictionary_value_from_key::<()>(
        &builder,
        nft_contract_key,
        BURNT_TOKENS,
        &TOKEN_ID.to_string(),
    );

    // This will error of token is not registered as
    let actual_balance = support::get_dictionary_value_from_key::<U256>(
        &builder,
        nft_contract_key,
        BALANCES,
        &DEFAULT_ACCOUNT_ADDR.clone().to_string(),
    );

    let expected_balance = U256::zero();
    assert_eq!(actual_balance, expected_balance);
}

#[test]
fn should_not_burn_previously_burnt_token() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let install_request_builder =
        InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
            .with_total_token_supply(U256::from(100u64))
            .with_ownership_mode(OwnershipMode::TransferableUnchecked)
            .build();

    builder
        .exec(install_request_builder)
        .expect_success()
        .commit();

    let installing_account = builder.get_expected_account(*DEFAULT_ACCOUNT_ADDR);
    let nft_contract_key = installing_account
        .named_keys()
        .get(CONTRACT_NAME)
        .expect("must have key in named keys");

    let nft_contract_hash = get_nft_contract_hash(&builder);
    let mint_session_call = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        MINT_SESSION_WASM,
        runtime_args! {
            ARG_NFT_CONTRACT_HASH => nft_contract_hash,
            ARG_KEY_NAME => Some(OWNED_TOKENS_DICTIONARY_KEY.to_string()),
            ARG_TOKEN_OWNER => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_TOKEN_META_DATA => TEST_META_DATA.to_string(),
            ARG_TOKEN_URI => TEST_URI.to_string(),
        },
    )
    .build();

    builder.exec(mint_session_call).expect_success().commit();

    let actual_owned_tokens = support::get_dictionary_value_from_key::<Vec<U256>>(
        &builder,
        nft_contract_key,
        OWNED_TOKENS,
        &DEFAULT_ACCOUNT_ADDR.clone().to_string(),
    );

    let expected_owned_tokens = vec![U256::zero()];
    assert_eq!(expected_owned_tokens, actual_owned_tokens);

    let burn_request = ExecuteRequestBuilder::contract_call_by_name(
        *DEFAULT_ACCOUNT_ADDR,
        CONTRACT_NAME,
        ENTRY_POINT_BURN,
        runtime_args! {
            ARG_TOKEN_ID => U256::zero(),
        },
    )
    .build();

    builder.exec(burn_request).expect_success().commit();

    let re_burn_request = ExecuteRequestBuilder::contract_call_by_name(
        *DEFAULT_ACCOUNT_ADDR,
        CONTRACT_NAME,
        ENTRY_POINT_BURN,
        runtime_args! {
            ARG_TOKEN_ID => U256::zero(),
        },
    )
    .build();

    builder.exec(re_burn_request).expect_failure();

    let actual_error = builder.get_error().expect("must have error");
    support::assert_expected_error(
        actual_error,
        42u16,
        "should disallow burning of previously burnt token",
    );
}

#[test]
fn should_return_expected_error_when_burning_non_existing_token() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let install_request_builder =
        InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
            .with_total_token_supply(U256::from(100u64))
            .with_ownership_mode(OwnershipMode::TransferableUnchecked)
            .build();

    builder
        .exec(install_request_builder)
        .expect_success()
        .commit();

    let burn_request = ExecuteRequestBuilder::contract_call_by_name(
        *DEFAULT_ACCOUNT_ADDR,
        CONTRACT_NAME,
        ENTRY_POINT_BURN,
        runtime_args! {
            ARG_TOKEN_ID => U256::zero(),
        },
    )
    .build();

    builder.exec(burn_request).expect_failure();

    let actual_error = builder.get_error().expect("must have error");
    support::assert_expected_error(
        actual_error,
        28u16,
        "should return InvalidTokenID error when trying to burn a non_existing token",
    );
}

#[test]
fn should_return_expected_error_burning_of_others_users_token() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let install_request_builder =
        InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
            .with_total_token_supply(U256::from(100u64))
            .with_ownership_mode(OwnershipMode::TransferableUnchecked)
            .build();

    builder
        .exec(install_request_builder)
        .expect_success()
        .commit();

    let installing_account = builder.get_expected_account(*DEFAULT_ACCOUNT_ADDR);
    let nft_contract_key = installing_account
        .named_keys()
        .get(CONTRACT_NAME)
        .expect("must have key in named keys");
    let nft_contract_hash = nft_contract_key
        .into_hash()
        .expect("must convert to hash addr");

    let (_, account_user_1) = support::create_dummy_key_pair(ACCOUNT_USER_1);

    let transfer_to_account_1 = ExecuteRequestBuilder::transfer(
        *DEFAULT_ACCOUNT_ADDR,
        runtime_args! {
            mint::ARG_AMOUNT => 100_000_000_000_000u64,
            mint::ARG_TARGET => account_user_1.to_account_hash(),
            mint::ARG_ID => Option::<u64>::None,
        },
    )
    .build();

    builder
        .exec(transfer_to_account_1)
        .expect_success()
        .commit();

    let mint_session_call = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        MINT_SESSION_WASM,
        runtime_args! {
            ARG_NFT_CONTRACT_HASH => nft_contract_hash,
            ARG_KEY_NAME => Option::<String>::None,
            ARG_TOKEN_OWNER => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_TOKEN_META_DATA => TEST_META_DATA.to_string(),
            ARG_TOKEN_URI => TEST_URI.to_string(),
        },
    )
    .build();

    builder.exec(mint_session_call).expect_success().commit();

    let actual_owned_tokens = support::get_dictionary_value_from_key::<Vec<U256>>(
        &builder,
        nft_contract_key,
        OWNED_TOKENS,
        &DEFAULT_ACCOUNT_ADDR.clone().to_string(),
    );

    let expected_owned_tokens = vec![U256::zero()];
    assert_eq!(expected_owned_tokens, actual_owned_tokens);

    let incorrect_burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        account_user_1.to_account_hash(),
        ContractHash::new(nft_contract_hash),
        ENTRY_POINT_BURN,
        runtime_args! {
            ARG_TOKEN_ID => U256::zero(),
        },
    )
    .build();

    builder.exec(incorrect_burn_request).expect_failure();

    let error = builder.get_error().expect("must have error");

    support::assert_expected_error(error, 6u16, "should disallow burning of other users' token");

    // TODO is this really diffferent than should_return_expected_error_when_burning_not_owned_token() ???
}

#[test]
fn should_return_expected_error_when_burning_not_owned_token() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let install_request_builder =
        InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
            .with_total_token_supply(U256::from(100u64))
            .with_ownership_mode(OwnershipMode::TransferableUnchecked)
            .build();

    builder
        .exec(install_request_builder)
        .expect_success()
        .commit();

    let installing_account = builder.get_expected_account(*DEFAULT_ACCOUNT_ADDR);
    let nft_contract_key = installing_account
        .named_keys()
        .get(CONTRACT_NAME)
        .expect("must have key in named keys");

    let nft_contract_hash = nft_contract_key
        .into_hash()
        .expect("must convert to hash addr");

    let (_, account_user_1) = support::create_dummy_key_pair(ACCOUNT_USER_1);

    let transfer_to_account_1 = ExecuteRequestBuilder::transfer(
        *DEFAULT_ACCOUNT_ADDR,
        runtime_args! {
            mint::ARG_AMOUNT => 100_000_000_000_000u64,
            mint::ARG_TARGET => account_user_1.to_account_hash(),
            mint::ARG_ID => Option::<u64>::None,
        },
    )
    .build();

    builder
        .exec(transfer_to_account_1)
        .expect_success()
        .commit();

    let mint_session_call = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        MINT_SESSION_WASM,
        runtime_args! {
            ARG_NFT_CONTRACT_HASH => nft_contract_hash,
            ARG_KEY_NAME => Some(OWNED_TOKENS_DICTIONARY_KEY.to_string()),
            ARG_TOKEN_OWNER => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_TOKEN_META_DATA => TEST_META_DATA.to_string(),
            ARG_TOKEN_URI => TEST_URI.to_string(),
        },
    )
    .build();

    builder.exec(mint_session_call).expect_success().commit();

    let actual_owned_tokens = support::get_dictionary_value_from_key::<Vec<U256>>(
        &builder,
        nft_contract_key,
        OWNED_TOKENS,
        &DEFAULT_ACCOUNT_ADDR.clone().to_string(),
    );

    let expected_owned_tokens = vec![U256::zero()];
    assert_eq!(expected_owned_tokens, actual_owned_tokens);

    let incorrect_burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        account_user_1.to_account_hash(),
        ContractHash::new(nft_contract_hash),
        ENTRY_POINT_BURN,
        runtime_args! {
            ARG_TOKEN_ID => U256::zero()
        },
    )
    .build();

    builder.exec(incorrect_burn_request).expect_failure();

    let actual_error = builder.get_error().expect("must get error");
    support::assert_expected_error(
        actual_error,
        6u16,
        "should disallow burning on mismatch of owner key",
    );
}
