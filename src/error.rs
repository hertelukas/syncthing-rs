//! syncthing-rs specific Error and Result

/// Main library error type
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    NetworkError(#[from] reqwest::Error),

    #[error(transparent)]
    SendEventError(#[from] tokio::sync::mpsc::error::SendError<crate::types::events::Event>),

    #[error("device ID was not set in response header")]
    HeaderDeviceIDError,

    #[error("could not convert header to string")]
    HeaderParseError,
}

pub type Result<T> = std::result::Result<T, Error>;
