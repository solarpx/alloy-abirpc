use {alloy_abirpc::prelude::*, futures::StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider: WsProvider = AbiProvider::new(
        String::from("wss://polygon-bor-rpc.publicnode.com"),
        Chain::from_id(137),
    )
    .provider()
    .await?;

    let sub = provider.subscribe_blocks().await?;
    let mut stream = sub.into_stream();
    if let Some(block) = stream.next().await {
        println!("{:?}", block)
    }

    Ok(())
}
