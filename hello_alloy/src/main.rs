use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::Address;
use std::error::Error;
use alloy::sol;

sol! {
    #[sol(rpc)]
    contract HelloWorldWeb3 {
        function hello_web3() pure public returns (string memory);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;

    let provider = ProviderBuilder::new().connect_http(rpc_url);

    let latest_block = provider.get_block_number().await?;

    println!("latest block: {:?}", latest_block);
    println!("Hello, world, Web3!");

    // 远程部署好的合约
    let contarct_address: Address = "0x3f1f78ED98Cd180794f1346F5bD379D5Ec47DE90".parse()?;

    // 这个合约里的一个函数
    let contract = HelloWorldWeb3::new(contarct_address, provider);

    let result = contract.hello_web3().call().await?;
    println!("return of the contract: {}", result);

    Ok(())
}
