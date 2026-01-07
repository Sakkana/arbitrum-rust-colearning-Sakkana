use alloy::providers::Provider as AlloyProvider;
use alloy::providers::ProviderBuilder;
use alloy::primitives::Address as AlloyAddress;
use alloy::sol;

use ethers::providers::{Provider as EthersProvider, Http};
use ethers::types::Address as EthAddress;
use ethers::utils::format_ether;
use ethers::middleware::Middleware;

use std::error::Error;
use std::convert::TryFrom;
use std::env;


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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    task_1().await?;
    task_2().await?;
    Ok(())
}