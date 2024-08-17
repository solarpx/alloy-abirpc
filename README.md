# alloy-abirpc

`alloy-abirpc` allows developers to efficiently manage multiple smart contract instances across multiple blockchains within the same application context via a simple API. This crate provides an implementation of the [`ethers-abirpc`](https://crates.io/crates/ethers-abirpc) crate for the [`alloy-rs`](https://github.com/alloy-rs) developer community.

## Overview 

The crate defines the `abirpc!` macro along with several other utilities for alloy provider encapsulation. The `abirpc!` macro is implemented as an extension of the `sol!` macro as shown in the example below. 

```rust
use alloy_abirpc::prelude::*;

sol!(
    #[sol(rpc)]
    Erc20Token,
    "./tests/abi/Erc20Token.json"
);
abirpc!(Erc20Token);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = address_from!("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")?; // WETH
    let registry = Erc20TokenRegistry::<WsTransport>::new(
    	String::from("wss://ethereum-rpc.publicnode.com"), 
    	Chain::from(NamedChain::Mainnet)
    );
    let provider = registry.provider().await?;
    let instance = registry.register(provider, address);

    let _ = instance.decimals().call().await?._0;; // Query contract abi

    Ok(())
}
```

In this example, the `abirpc!(Erc20Token)` call generates the `Erc20TokenRegistry` type which implements RPC provider encapsulation, and the preceding `sol!` call generates the underlying `Erc20TokenInstance` type which contains the required rust bindings for the contract ABI.

## Network management

Network implementation is provided by the [`alloy_chains`](https://crates.io/crates/alloy-chains) crate.

```rust
let chain = Chain::from(NamedChain::Mainnet);
// OR
let chain = Chain::from_id(1);
```

If the chain `Id` does not match the on-chain configuration, initialization will fail.

```rust
let registry = Erc20TokenRegistry::<WsTransport>::new(
	String::from("wss://ethereum-rpc.publicnode.com"), 
	Chain::from_id(10) // Incorrect ChainId
);
let provider = registry.provider().await?; // Error 
```

## Provider management

The crate also includes a wrapper for direct initialization of `alloy-rs` providers. This is helpful for interactions not requiring an ABI.

```rust
let provider: WsProvider = AbiProvider::new(
    String::from("wss://ethereum-rpc.publicnode.com"),
    Chain::from_id(1),
)
.provider()
.await?;

let sub = provider.subscribe_blocks().await?;
let mut stream = sub.into_stream();
if let Some(block) = stream.next().await {
    println!("{:?}", block)
}
```

## ABI management

ABI files can be located anywhere on the system, and multiple ABIs can be initialized within the same `.rs` file.

```rust
use alloy_abirpc::prelude::*;

sol!(
    #[sol(rpc)]
    Erc20Token,
    "./tests/abi/Erc20Token.json"
);
abirpc!(Erc20Token);

sol!(
    #[sol(rpc)]
    Erc721Token,
    "./tests/abi/Erc721Token.json"
);
abirpc!(Erc721Token);
```

## Release notes

- 0.1.0: Stabilized API. `SemVer` forward compatibility
