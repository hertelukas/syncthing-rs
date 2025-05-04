//! syncthing-rs specific Error and Result

/// Main library error type
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    Network(#[from] reqwest::Error),

    #[error(transparent)]
    SendEventError(#[from] tokio::sync::mpsc::error::SendError<crate::types::events::Event>),
}

pub type Result<T> = std::result::Result<T, Error>;
