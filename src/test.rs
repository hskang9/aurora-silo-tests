use aurora_engine::{
    fungible_token::FungibleTokenMetadata,
    parameters::{InitCallArgs, PauseEthConnectorCallArgs, SetContractDataCallArgs}, xcc::AddressVersionUpdateArgs,
};
use aurora_engine_types::{self, types::Address};
use aurora_workspace::{
    contract::{EthProverConfig, Owner},
    types::{KeyType, SecretKey},
    EvmContract,
};
use aurora_workspace_demo::common;
use serde_json::json;
use std::str::FromStr;
use workspaces::AccountId;

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use std::io::{self, Write};

pub const EVM_ACCOUNT_ID: &str = "aurora.test.near";
const AURORA_LOCAL_CHAIN_ID: u64 = 1313161556;
pub const OWNER_ACCOUNT_ID: &str = "owner.test.near";
const PROVER_ACCOUNT_ID: &str = "prover.test.near";

#[tokio::test]
async fn test_new_eth_connector() {
    // 1. Create a sandbox environment.
    let worker = workspaces::sandbox().await.unwrap();

    worker.fast_forward(1).await.unwrap();

    // 2. deploy the Aurora EVM in sandbox with initial call to setup admin account from sender
    let (evm, sk, owner) = common::init_and_deploy_contract_with_path_on_admin_change(
        &worker,
        "./res/aurora-testnet-silo-admin-methods.wasm",
    )
    .await
    .unwrap();

    worker.fast_forward(1).await.unwrap();

    let eth_prover_config = EthProverConfig::default();

    let args = InitCallArgs {
        prover_account: aurora_engine_types::account_id::AccountId::from_str(
            eth_prover_config.account_id.as_str(),
        )
        .unwrap(),
        eth_custodian_address: eth_prover_config.evm_custodian_address,
        metadata: FungibleTokenMetadata::default(),
    };

    let result = owner
        .call(evm.as_account().id(), "new_eth_connector")
        .args_borsh(args)
        .transact()
        .await
        .unwrap();
    println!("result: {:?}", result);
    // new eth connector should fail as it can only be called once and it is already called in init method.
    // set_eth_connector_contract_data should be called instead.
    assert!(result.is_failure());
}

#[tokio::test]
async fn test_factory_update_address_version() {
    // 1. Create a sandbox environment.
    let worker = workspaces::sandbox().await.unwrap();

    worker.fast_forward(1).await.unwrap();

    // 2. deploy the Aurora EVM in sandbox.
    let (evm, sk, owner) = common::init_and_deploy_contract_with_path_on_admin_change(
        &worker,
        "./res/aurora-testnet-silo-admin-methods.wasm",
    )
    .await
    .unwrap();

    worker.fast_forward(1).await.unwrap();

    let args = AddressVersionUpdateArgs {
        address: Address::zero(),
        version: aurora_engine::xcc::CodeVersion(1),
    };

    let result = owner
        .call(evm.as_account().id(), "factory_update_address_version")
        .args_borsh(args)
        .transact()
        .await
        .unwrap();
    println!("result: {:?}", result);
    // factory update involves invalid access on async call, so it should fail
    assert!(result.is_failure());
}

#[tokio::test]
async fn test_set_eth_connector_contract_data() {
    // 1. Create a sandbox environment.
    let worker = workspaces::sandbox().await.unwrap();

    worker.fast_forward(1).await.unwrap();

    // 2. deploy the Aurora EVM in sandbox.
    let (evm, sk, owner) = common::init_and_deploy_contract_with_path_on_admin_change(
        &worker,
        "./res/aurora-testnet-silo-admin-methods.wasm",
    )
    .await
    .unwrap();

    worker.fast_forward(1).await.unwrap();
    let aur_prover_account: aurora_engine_types::account_id::AccountId =
        aurora_engine_types::account_id::AccountId::from_str(PROVER_ACCOUNT_ID).unwrap();

    let args = SetContractDataCallArgs {
        eth_custodian_address: "0000000000000000000000000000000000000000".to_string(),
        prover_account: aur_prover_account,
        metadata: FungibleTokenMetadata {
            spec: "ft-1.0.0".to_string(),
            name: "Aurora Testnet NEAR".to_string(),
            symbol: "aurora.test.near".to_string(),
            icon: None,
            reference: None,
            reference_hash: None,
            decimals: 24,
        },
    };

    let result = owner
        .call(evm.as_account().id(), "set_eth_connector_contract_data")
        .args_borsh(args)
        .transact()
        .await
        .unwrap();
    println!("result: {:?}", result);
    assert!(result.is_success());
}

#[tokio::test]
async fn test_set_paused_flags() {
    // 1. Create a sandbox environment.
    let worker = workspaces::sandbox().await.unwrap();

    worker.fast_forward(1).await.unwrap();

    // 2. deploy the Aurora EVM in sandbox.
    let (evm, sk, owner) = common::init_and_deploy_contract_with_path_on_admin_change(
        &worker,
        "./res/aurora-testnet-silo-admin-methods.wasm",
    )
    .await
    .unwrap();

    worker.fast_forward(1).await.unwrap();

    let args = PauseEthConnectorCallArgs { paused_mask: 0 };

    let result = owner
        .call(evm.as_account().id(), "set_paused_flags")
        .args_borsh(args)
        .transact()
        .await
        .unwrap();
    println!("result: {:?}", result);
    assert!(result.is_success());
}
