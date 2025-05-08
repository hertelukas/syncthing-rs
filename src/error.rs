//! syncthing-rs specific Error and Result

/// Main library error type
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    NetworkError(#[from] reqwest::Error),

    #[error("failed to send event (no receivers)")]
    SendEventError,

    #[error("device ID was not set in response header")]
    HeaderDeviceIDError,

    #[error("could not convert header to string")]
    HeaderParseError,

    #[error("folder already in the configuration")]
    DuplicateFolderError,

    #[error("device already in the configuration")]
    DuplicateDeviceError,

    #[error("folder does not exist")]
    UnknownFolderError,

    #[error("device does not exist")]
    UnknownDeviceError,
}

impl From<tokio::sync::broadcast::error::SendError<crate::types::events::Event>> for Error {
    fn from(_: tokio::sync::broadcast::error::SendError<crate::types::events::Event>) -> Self {
        Self::SendEventError
    }
}

pub type Result<T> = std::result::Result<T, Error>;
