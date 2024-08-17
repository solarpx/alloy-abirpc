use alloy_abirpc::prelude::*;

sol!(
    #[sol(rpc)]
    Erc20Token,
    "./tests/abi/Erc20Token.json"
);
abirpc!(Erc20Token);

const WS_PROVIDER: &str = "wss://polygon-bor-rpc.publicnode.com";
const WETH_ADDRESS: &str = "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619";

const LOCAL_WALLET: FixedBytes<32> =
    fixed_bytes!("380eb0f3d505f087e438eca80bc4df9a7faa24f868e69fc0440261a0fc0567dc");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry =
        Erc20TokenRegistry::<WsTransport>::new(String::from(WS_PROVIDER), Chain::from_id(137));
    let provider = registry.provider().await?;

    let weth_address = address_from!(WETH_ADDRESS)?;
    let instance = registry.register(provider.clone(), weth_address);

    let signer = PrivateKeySigner::from_bytes(&LOCAL_WALLET)?;
    let weth_amount = U256::from(10u64.pow(15));
    let tx = instance
        .approve(signer.address(), weth_amount)
        .value(U256::from(0_u64))
        .into_transaction_request();

    let wallet = EthereumWallet::from(signer);
    let tx = provider
        .join_with(WalletFiller::new(wallet))
        .fill(tx)
        .await?;

    println!("{:?}", tx);

    Ok(())
}
