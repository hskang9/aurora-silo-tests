use aurora_workspace::{
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

#[tokio::test]
async fn test_factory_update_address_version() {
     // 1. Create a sandbox environment.
     let worker = workspaces::sandbox().await.unwrap();

     worker.fast_forward(1).await.unwrap();
 
     // 2. deploy the Aurora EVM in sandbox.
     let (evm, sk) =
         common::init_and_deploy_contract_with_path(&worker, "./res/aurora-testnet-silo-admin-methods.wasm")
             .await.unwrap();
 
     worker.fast_forward(1).await.unwrap();
}

#[tokio::test]
async fn test_new_eth_connector() {
    // 1. Create a sandbox environment.

    // 2. deploy the Aurora EVM in sandbox with initial call to setup admin account from sender

    // 3. new_eth_connector method should not be called from other NEAR account as this is an admin method.
}

#[tokio::test]
async fn test_set_eth_connector_contract_data() {
    // 1. Create a sandbox environment.

    // 2. deploy the Aurora EVM in sandbox with initial call to setup admin account from sender

    // 3. set_eth_connector_contract_data method should not be called from other NEAR account as this is an admin method.
}

#[tokio::test]
async fn test_set_paused_flags() {
    // 1. Create a sandbox environment.

    // 2. deploy the Aurora EVM in sandbox with initial call to setup admin account from sender

    // 3. set_paused_flags method should not be called from other NEAR account as this is an admin method.
}
