use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Hex error: {0}")]
    AddressParseError(#[from] rustc_hex::FromHexError),
    #[error("ChainId error: {0}")]
    ChainIdError(String),
    #[error("Error: {0}")]
    Error(String),
    #[error("Rpc error: {0}")]
    RpcError(#[from] alloy::transports::RpcError<alloy::transports::TransportErrorKind>),
    #[error("Url parse Error: {0}")]
    UrlParseError(#[from] url::ParseError),
}
