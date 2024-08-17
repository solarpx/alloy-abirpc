pub mod error;
pub mod provider;
pub mod registry;

pub mod prelude {
    pub use {
        crate::{
            abirpc, address_from,
            error::Error,
            provider::{
                AbiProvider, AbiProviderTrait, HttpProvider, HttpTransport, WsProvider, WsTransport,
            },
        },
        alloy::{
            eips::BlockNumberOrTag,
            network::EthereumWallet,
            primitives::{fixed_bytes, FixedBytes, U256},
            providers::{fillers::WalletFiller, Provider},
            signers::local::PrivateKeySigner,
            sol,
        },
        alloy_chains::{Chain, NamedChain},
    };
}
