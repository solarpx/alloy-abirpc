use alloy_abirpc::prelude::*;

sol!(
    #[sol(rpc)]
    Erc20Token,
    "./tests/abi/Erc20Token.json"
);
abirpc!(Erc20Token);

const TEST_HTTP_PROVIDER: &str = "https://ethereum.publicnode.com";
const TEST_WS_PROVIDER: &str = "wss://ethereum-rpc.publicnode.com";

const TEST_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"; // WETH

#[tokio::test]
async fn test_ws() -> Result<(), Box<dyn std::error::Error>> {
    let registry =
        Erc20TokenRegistry::<WsTransport>::new(TEST_WS_PROVIDER.into(), Chain::from_id(1));
    let provider = registry.provider().await?;
    let instance = registry.register(provider.clone(), address_from!(TEST_ADDRESS)?);

    let _res = instance.decimals().call().await?._0;

    Ok(())
}

#[tokio::test]
async fn test_http() -> Result<(), Box<dyn std::error::Error>> {
    let registry =
        Erc20TokenRegistry::<HttpTransport>::new(TEST_HTTP_PROVIDER.into(), Chain::from_id(1));
    let provider = registry.provider().await?;
    let instance = registry.register(provider.clone(), address_from!(TEST_ADDRESS)?);

    let _res = instance.decimals().call().await?._0;

    Ok(())
}

#[tokio::test]
async fn get_logs() -> Result<(), Box<dyn std::error::Error>> {
    let registry =
        Erc20TokenRegistry::<WsTransport>::new(TEST_WS_PROVIDER.into(), Chain::from_id(1));
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address_from!(TEST_ADDRESS)?);

    let sig: &str = "Transfer(address,address,uint256)";

    let _res = instance
        .get_logs(sig, BlockNumberOrTag::Latest, BlockNumberOrTag::Latest)
        .await?;

    Ok(())
}
