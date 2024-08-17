use {
    crate::error::Error,
    alloy::{
        network::Ethereum,
        providers::{
            fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
            Identity, Provider, ProviderBuilder, RootProvider,
        },
        pubsub::PubSubFrontend,
        transports::{
            http::{Client, Http},
            Transport,
        },
    },
    alloy_chains::Chain,
    alloy_rpc_client::WsConnect,
    async_trait::async_trait,
    url::Url,
};

pub type ProviderType<T> = FillProvider<
    JoinFill<JoinFill<JoinFill<Identity, GasFiller>, NonceFiller>, ChainIdFiller>,
    RootProvider<T, Ethereum>,
    T,
    Ethereum,
>;

pub type HttpTransport = Http<Client>;

pub type HttpProvider = ProviderType<HttpTransport>;

pub type WsTransport = PubSubFrontend;

pub type WsProvider = ProviderType<WsTransport>;

#[async_trait]
pub trait AbiProviderTrait<T>
where
    T: Transport + Clone,
{
    async fn provider(&self) -> Result<ProviderType<T>, Error>;
}

pub struct AbiProvider {
    pub url: String,
    pub chain: Chain,
}

impl AbiProvider {
    pub fn new(url: String, chain: Chain) -> Self {
        Self { url, chain }
    }
}

macro_rules! assert_chain_id {
    ($chain: expr, $provider: expr) => {
        let provider_chain_id = $provider.get_chain_id().await.expect("panic");
        if $chain.id() != provider_chain_id {
            let e = format!(
                "Configured chain_id ({}) does not match chain ({})",
                $chain.id(),
                provider_chain_id
            );
            return Err(Error::ChainIdError(e));
        }
    };
}

#[async_trait]
impl AbiProviderTrait<HttpTransport> for AbiProvider {
    async fn provider(&self) -> Result<HttpProvider, Error> {
        let provider = ProviderBuilder::new()
            .filler(GasFiller)
            .filler(NonceFiller::default())
            .filler(ChainIdFiller::new(Some(self.chain.id())))
            .on_http(Url::parse(&self.url)?);

        assert_chain_id!(self.chain, provider);

        Ok(provider)
    }
}

#[async_trait]
impl AbiProviderTrait<WsTransport> for AbiProvider {
    async fn provider(&self) -> Result<WsProvider, Error> {
        let provider = ProviderBuilder::new()
            .filler(GasFiller)
            .filler(NonceFiller::default())
            .filler(ChainIdFiller::new(Some(self.chain.id())))
            .on_ws(WsConnect {
                url: self.url.clone(),
                auth: None,
            })
            .await?;

        assert_chain_id!(self.chain, provider);

        Ok(provider)
    }
}
