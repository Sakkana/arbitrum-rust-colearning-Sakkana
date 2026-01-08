use alloy::providers::Provider as AlloyProvider;
use alloy::providers::ProviderBuilder;
use alloy::primitives::Address as AlloyAddress;
use alloy::sol;

use ethers::prelude::*;
use ethers::providers::{Provider as EthersProvider, Http};
use ethers::types::{Address as EthAddress, TransactionRequest};
use ethers::types::U256;
use ethers::utils::{format_ether, parse_ether};
use ethers::middleware::{Middleware, SignerMiddleware};
use ethers::signers::Signer;
use ethers::prelude::LocalWallet;

use std::error::Error;
use std::convert::TryFrom;
use std::env;
use std::sync::Arc;
use eyre::eyre;
use eyre::Result;

sol! {
    #[sol(rpc)]
    contract HelloWorldWeb3 {
        function hello_web3() pure public returns (string memory);
    }
}

// task 1：和链上合约交互
async fn task_1() -> Result<(), Box<dyn Error>> {
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;

    let provider = ProviderBuilder::new().connect_http(rpc_url);

    let latest_block = provider.get_block_number().await?;
    println!("latest block: {:?}", latest_block);
    println!("Hello, world, Web3!");

    // 远程部署好的合约
    let contract_address: AlloyAddress =
        "0x3f1f78ED98Cd180794f1346F5bD379D5Ec47DE90".parse()?;

    let contract = HelloWorldWeb3::new(contract_address, provider);

    let result = contract.hello_web3().call().await?;
    println!("return of the contract: {}", result);

    Ok(())
}

// task2: 查询自己的余额
async fn task_2() -> Result<(), Box<dyn Error>> {
    // arb sepolia rpc
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com";
    let provider: EthersProvider<Http> = EthersProvider::<Http>::try_from(rpc_url)?;

    // 查询余额
    let address_str = env::var("MY_ARB_ADDRESS")?;
    let address: EthAddress = address_str.parse()?;
    let balance_wei = provider.get_balance(address, None).await?;

    // wei -> ETH
    let balance_eth = format_ether(balance_wei);
    println!("Address: {:?}", address);
    println!("Balance: {} ETH", balance_eth);

    Ok(())
}


// task 3: 计算 arbitrum gas 费用
async fn task_3() -> Result<(), Box<dyn Error>> {
    // Gas Fee = Gas Price × Gas Limit

    // Arbitrum Sepolia RPC
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com";
    let provider = EthersProvider::<Http>::try_from(rpc_url)?;

    // 动态获取当前 Gas Price - 单位：wei
    let gas_price: U256 = provider.get_gas_price().await?;
    println!("Current Gas Price: {} wei", gas_price);

    // 基础 ETH 转账 Gas Limit（行业通用值）
    let gas_limit: U256 = U256::from(21_000u64);

    // 3计算 Gas 费用
    let gas_fee_wei = gas_price * gas_limit;
    let gas_fee_eth = format_ether(gas_fee_wei);

    println!("Gas Limit (transfer): {}", gas_limit);
    println!("Estimated Gas Fee: {} wei", gas_fee_wei);
    println!("Estimated Gas Fee: {} ETH", gas_fee_eth);

    Ok(())
}

async fn task_4() -> Result<()> {
    // 1. 初始化 RPC 提供者
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com";
    let provider: Provider<Http> = Provider::<Http>::try_from(rpc_url)?
        .interval(std::time::Duration::from_millis(500));

    // 2. 从环境变量读取私钥和目标地址
    let private_key = env::var("ARB_PRIVATE_KEY")
        .map_err(|e| eyre!("Failed to read ARB_PRIVATE_KEY: {}", e))?;
    let another_address_str = env::var("ANOTHER_ARB_ADDRESS")
        .map_err(|e| eyre!("Failed to read ANOTHER_ARB_ADDRESS: {}", e))?;

    // 3. 初始化钱包和签名客户端
    let chain_id = 421614u64;
    let wallet: LocalWallet = private_key.parse::<LocalWallet>()
        .map_err(|e| eyre!("Invalid private key: {}", e))?
        .with_chain_id(chain_id);
    let client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>> =
        Arc::new(SignerMiddleware::new(provider.clone(), wallet));

    // 4. 准备转账参数
    let from = client.address();
    let to: Address = another_address_str.parse()
        .map_err(|e| eyre!("Invalid target address: {}", e))?;
    let to = NameOrAddress::Address(to);

    // 5. 检查余额
    let balance = client.get_balance(from, None).await?;
    let value = parse_ether("0.01")?;
    if balance < value {
        return Err(eyre!("Insufficient balance: have {}, need {}", balance, value));
    }

    // 6. 计算 gas 参数（EIP-1559）
    let gas_price = client.get_gas_price().await?;
    let max_priority_fee = U256::from(1_000_000_000u64); // 1 Gwei
    let max_fee = gas_price + max_priority_fee;

    // 7. 构建并发送交易
    let tx = Eip1559TransactionRequest {
        to: Some(to),
        value: Some(value),
        gas: Some(21_000.into()), // 普通转账固定 gas 21000
        max_priority_fee_per_gas: Some(max_priority_fee),
        max_fee_per_gas: Some(max_fee),
        ..Default::default()
    };

    let pending_tx = client.send_transaction(tx, None).await?;
    println!("Tx sent! Hash: {:?}", pending_tx.tx_hash());

    // 8. 等待交易上链并打印结果
    let receipt = pending_tx.await?;
    match receipt {
        Some(r) => println!("Tx mined in block: {:?}", r.block_number),
        None => println!("Tx was dropped or not mined yet"),
    }

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    task_1().await?;
    task_2().await?;
    task_3().await?;
    task_4().await?;
    Ok(())
}