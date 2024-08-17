use alloy_abirpc::prelude::*;

sol!(
    #[sol(rpc)]
    Erc20Token,
    "./tests/abi/Erc20Token.json"
);
abirpc!(Erc20Token);

#[tokio::test]
async fn test_abirpc() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
