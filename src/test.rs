use aurora_engine::{
    fungible_token::FungibleTokenMetadata,
    parameters::{InitCallArgs, PauseEthConnectorCallArgs, SetContractDataCallArgs}, xcc::AddressVersionUpdateArgs,
};
use aurora_engine_types::{self, types::Address};
use aurora_workspace::{
    contract::{EthProverConfig}, types::{SecretKey, KeyType}
};
use aurora_workspace_demo::common;
use aurora_workspace_types::AccountId;
use std::str::FromStr;

pub const OWNER_ACCOUNT_ID: &str = "owner.test.near";
const PROVER_ACCOUNT_ID: &str = "prover.test.near";

#[tokio::test]
async fn test_new_eth_connector() {
    // 1. Create a sandbox environment.
    let worker = workspaces::sandbox().await.unwrap();

    worker.fast_forward(1).await.unwrap();

    // 2. deploy the Aurora EVM in sandbox with initial call to setup admin account from sender
    let (evm, _sk) = common::init_and_deploy_contract_with_path(
        &worker,
        "./res/aurora-testnet-feat-change-admin.wasm",
    )
    .await
    .unwrap();

    let sk2 = SecretKey::from_random(KeyType::ED25519);
    let owner = worker
    .create_tla(AccountId::from_str(OWNER_ACCOUNT_ID).unwrap(), sk2.clone())
    .await.unwrap()
    .into_result().unwrap();

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
async fn test_set_eth_connector_contract_data() {
    // 1. Create a sandbox environment.
    let worker = workspaces::sandbox().await.unwrap();

    worker.fast_forward(1).await.unwrap();

    // 2. deploy the Aurora EVM in sandbox.
    let (evm, _sk) = common::init_and_deploy_contract_with_path(
        &worker,
        "./res/aurora-testnet-feat-change-admin.wasm",
    )
    .await
    .unwrap();

    let sk2 = SecretKey::from_random(KeyType::ED25519);
    let owner = worker
    .create_tla(AccountId::from_str(OWNER_ACCOUNT_ID).unwrap(), sk2.clone())
    .await.unwrap()
    .into_result().unwrap();

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
    let (evm, _sk) = common::init_and_deploy_contract_with_path(
        &worker,
        "./res/aurora-testnet-feat-change-admin.wasm",
    )
    .await
    .unwrap();

    

    worker.fast_forward(1).await.unwrap();

    let args = PauseEthConnectorCallArgs { paused_mask: 0 };
     
    let sk2 = SecretKey::from_random(KeyType::ED25519);
    let owner = worker
    .create_tla(AccountId::from_str(OWNER_ACCOUNT_ID).unwrap(), sk2.clone())
    .await.unwrap()
    .into_result().unwrap();

    let result = owner
        .call(evm.as_account().id(), "set_paused_flags")
        .args_borsh(args)
        .transact()
        .await
        .unwrap();
    println!("result: {:?}", result);
    assert!(result.is_success());
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct SetOwnerArgs {
    pub new_owner: AccountId,
}

#[tokio::test]
async fn test_set_owner() {
    // 1. Create a sandbox environment.
    let worker = workspaces::sandbox().await.unwrap();

    worker.fast_forward(1).await.unwrap();

    // 2. deploy the Aurora EVM in sandbox.
    let (evm, sk, owner) = common::init_and_deploy_contract_with_path_on_admin_change(
        &worker,
        "./res/aurora-testnet-set-owner.wasm",
    )
    .await
    .unwrap();

    worker.fast_forward(1).await.unwrap();

    let args = SetOwnerArgs { new_owner: AccountId::from_str("newowner.test.near").unwrap() };

    let result = owner
        .call(evm.as_account().id(), "set_owner")
        .args_borsh(args)
        .transact()
        .await
        .unwrap();
    println!("result: {:?}", result);
    assert!(result.is_success());

    // get owner account id
    let result = owner
        .view(evm.as_account().id(), "get_owner")
        .args_borsh(())
        .await
        .unwrap();

    println!("result: {:?}", AccountId::from(result.result));
}
