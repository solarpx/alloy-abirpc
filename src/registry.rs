use {
    alloy::primitives::Address,
    alloy_chains::Chain,
    std::{
        clone::Clone,
        collections::HashMap,
        sync::{Arc, RwLock},
    },
};

#[derive(Debug)]
pub struct AbiRegistry<C> {
    pub url: String,
    pub chain: Chain,
    pub registry: Arc<RwLock<HashMap<Address, C>>>,
}

impl<C> AbiRegistry<C> {
    pub fn new(url: String, chain: Chain) -> Self {
        Self {
            url,
            chain,
            registry: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn entry_exists(&self, address: Address) -> bool {
        let arc_clone = Arc::clone(&self.registry);
        let registry = arc_clone.read().expect("Registry RwLock poisoned!");
        let entry_exists = registry.contains_key(&address);
        drop(registry);

        entry_exists
    }

    pub fn add_entry(&self, address: Address, contract: C) {
        let arc_clone = Arc::clone(&self.registry);
        let mut registry = arc_clone.write().expect("Registry RwLock poisoned!");
        registry.insert(address, contract);
        drop(registry);
    }
}

#[macro_export]
macro_rules! abirpc {
    ($abi:ident) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<$abi Registry>]<T>(
                $crate::registry::AbiRegistry<
                    $abi::[<$abi Instance>]<T, $crate::provider::ProviderType<T>>,
                >,
            )
            where
                T: ::alloy::transports::Transport + Clone;


            impl<T> [<$abi Registry>]<T>
            where
                T: ::alloy::transports::Transport + Clone,
            {
                pub fn new(url: String, chain: alloy_chains::Chain) -> Self {
                    let registry = $crate::registry::AbiRegistry::<
                        $abi::[<$abi Instance>]<T, $crate::provider::ProviderType<T>>,
                    >::new(url, chain);
                    Self(registry)
                }

                pub fn register(
                    &self,
                    provider: $crate::provider::ProviderType<T>,
                    address: ::alloy::primitives::Address,
                ) -> $abi::[<$abi Instance>]<T, $crate::provider::ProviderType<T>> {
                    if !self.0.entry_exists(address) {
                        let instance = $abi::[<$abi Instance>]::new(address, provider.into());
                        self.0.add_entry(address, instance)
                    }

                    let clone_lock = std::sync::Arc::clone(&self.0.registry);
                    let registry = clone_lock.read().expect("Registry RwLock poisoned!");
                    let instance = registry.get(&address).unwrap().clone();
                    drop(registry);

                    instance
                }
            }

            #[async_trait::async_trait]
            impl $crate::provider::AbiProviderTrait<$crate::provider::WsTransport>
                for [<$abi Registry>]<$crate::provider::WsTransport>
            {
                async fn provider(&self) -> Result<$crate::provider::WsProvider, $crate::error::Error> {
                    let provider: $crate::provider::WsProvider =
                        $crate::provider::AbiProvider::new(self.0.url.clone(), self.0.chain)
                            .provider()
                            .await?;

                    Ok(provider)
                }
            }

            #[async_trait::async_trait]
            impl $crate::provider::AbiProviderTrait<$crate::provider::HttpTransport>
                for [<$abi Registry>]<$crate::provider::HttpTransport>
            {
                async fn provider(
                    &self,
                ) -> Result<$crate::provider::HttpProvider, $crate::error::Error> {
                    let provider: $crate::provider::HttpProvider =
                        $crate::provider::AbiProvider::new(self.0.url.clone(), self.0.chain)
                            .provider()
                            .await?;

                    Ok(provider)
                }
            }

            impl<T> $abi::[<$abi Instance>]<T, $crate::provider::ProviderType<T>>
            where
                T: ::alloy::transports::Transport + Clone,
            {
                pub async fn get_logs(
                    &self,
                    sig: &str,
                    from_block: ::alloy::eips::BlockNumberOrTag,
                    to_block: ::alloy::eips::BlockNumberOrTag,
                ) -> Result<Vec<alloy::rpc::types::Log>, $crate::error::Error> {
                    let filter = ::alloy::rpc::types::Filter::new()
                        .event(sig)
                        .address(*self.address())
                        .from_block(from_block)
                        .to_block(to_block);

                    let res = self.provider().get_logs(&filter).await?;

                    Ok(res)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! address_from {
    ($address: expr) => {
        $address.parse::<alloy::primitives::Address>()
    };
}
