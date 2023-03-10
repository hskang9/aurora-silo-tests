use aurora_workspace::contract::EthProverConfig;
use aurora_workspace::{types::AccountId, EvmContract, InitConfig};
use aurora_engine::parameters::NewCallArgs;
use std::str::FromStr;
use workspaces::network::Sandbox;
use workspaces::types::{KeyType, SecretKey};
use workspaces::{Worker, Account};

pub const EVM_ACCOUNT_ID: &str = "aurora.test.near";
const AURORA_LOCAL_CHAIN_ID: u64 = 1313161556;
pub const OWNER_ACCOUNT_ID: &str = "owner.test.near";
const PROVER_ACCOUNT_ID: &str = "prover.test.near";

pub async fn create_account(worker: &Worker<Sandbox>, id: &str, sk: Option<SecretKey>) -> anyhow::Result<Account> {
    let secret = sk.unwrap_or_else(|| SecretKey::from_random(KeyType::ED25519));
    let account = worker
        .create_tla(AccountId::from_str(id)?, secret)
        .await?
        .into_result()?;
    Ok(account) 
}

pub async fn init_and_deploy_contract_with_path(worker: &Worker<Sandbox>, path: &str) -> anyhow::Result<(EvmContract, SecretKey)> {
    let sk = SecretKey::from_random(KeyType::ED25519);
    let evm_account = worker
        .create_tla(AccountId::from_str(EVM_ACCOUNT_ID)?, sk.clone())
        .await?
        .into_result()?;
    let eth_prover_config = EthProverConfig::default();
    let init_config = InitConfig {
        owner_id: AccountId::from_str(OWNER_ACCOUNT_ID)?,
        prover_id: AccountId::from_str(PROVER_ACCOUNT_ID)?,
        eth_prover_config: Some(eth_prover_config),
        chain_id: AURORA_LOCAL_CHAIN_ID.into(),
    };
    let wasm = std::fs::read(path)?;
    // create contract
    let contract = EvmContract::deploy_and_init(evm_account.clone(), init_config, wasm).await?;

    Ok((contract, sk))
}

pub async fn init_and_deploy_contract_with_path_on_admin_change(worker: &Worker<Sandbox>, path: &str) -> anyhow::Result<(EvmContract, SecretKey, Account)> {
    let sk = SecretKey::from_random(KeyType::ED25519);
    let evm_account = worker
        .create_tla(AccountId::from_str(OWNER_ACCOUNT_ID)?, sk.clone())
        .await?
        .into_result()?;
    let eth_prover_config = EthProverConfig::default();
    let init_config = InitConfig {
        owner_id: AccountId::from_str(OWNER_ACCOUNT_ID)?,
        prover_id: AccountId::from_str(PROVER_ACCOUNT_ID)?,
        eth_prover_config: Some(eth_prover_config),
        chain_id: AURORA_LOCAL_CHAIN_ID.into(),
    };
    let wasm = std::fs::read(path)?;
    // create contract
    let contract = EvmContract::deploy_and_init(evm_account.clone(), init_config, wasm).await?;

    Ok((contract, sk, evm_account))
}

pub async fn init_and_deploy_contract(worker: &Worker<Sandbox>) -> anyhow::Result<EvmContract> {
    let sk = SecretKey::from_random(KeyType::ED25519);
    let evm_account = worker
        .create_tla(AccountId::from_str(EVM_ACCOUNT_ID)?, sk)
        .await?
        .into_result()?;
    let eth_prover_config = EthProverConfig::default();
    let init_config = InitConfig {
        owner_id: AccountId::from_str(OWNER_ACCOUNT_ID)?,
        prover_id: AccountId::from_str(PROVER_ACCOUNT_ID)?,
        eth_prover_config: Some(eth_prover_config),
        chain_id: AURORA_LOCAL_CHAIN_ID.into(),
    };
    let wasm = std::fs::read("./res/aurora-testnet.wasm")?;
    // create contract
    let contract = EvmContract::deploy_and_init(evm_account, init_config, wasm).await?;

    Ok(contract)
}

pub async fn init_and_deploy_sputnik(worker: &Worker<Sandbox>) -> anyhow::Result<EvmContract> {
    let sk = SecretKey::from_random(KeyType::ED25519);
    let evm_account = worker
        .create_tla(AccountId::from_str(EVM_ACCOUNT_ID)?, sk)
        .await?
        .into_result()?;
    let eth_prover_config = EthProverConfig::default();
    let init_config = InitConfig {
        owner_id: AccountId::from_str(OWNER_ACCOUNT_ID)?,
        prover_id: AccountId::from_str(PROVER_ACCOUNT_ID)?,
        eth_prover_config: Some(eth_prover_config),
        chain_id: AURORA_LOCAL_CHAIN_ID.into(),
    };
    let wasm = std::fs::read("./res/aurora-testnet.wasm")?;
    // create contract
    let contract = EvmContract::deploy_and_init(evm_account, init_config, wasm).await?;

    Ok(contract)
}
