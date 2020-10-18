use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Crate error enumeration.
#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
    #[error("invalid key: {0}")]
    InvalidKey(String),
    #[error("invalid user agent: {0}")]
    InvalidUserAgent(String),
    #[error("internal error: {0}")]
    Internal(String),
    #[error("request failed: {err}")]
    Request { err: String, status: Option<u16> },
    #[error("not authorized")]
    Unauthorized,
}

impl Error {
    /// Constructs an internal error.
    pub(crate) fn internal(message: impl fmt::Display) -> Self {
        Self::Internal(message.to_string())
    }

    /// Constructs an invalid key error.
    pub(crate) fn invalid_key(message: impl fmt::Display) -> Self {
        Self::InvalidKey(message.to_string())
    }

    /// Constructs an invalid user agent error.
    pub(crate) fn invalid_agent(message: impl fmt::Display) -> Self {
        Self::InvalidUserAgent(message.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Request {
            err: e.to_string(),
            status: e.status().map(|c| c.as_u16()),
        }
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(e: std::time::SystemTimeError) -> Self {
        Self::internal(e)
    }
}

impl From<hmac::crypto_mac::InvalidKeyLength> for Error {
    fn from(e: hmac::crypto_mac::InvalidKeyLength) -> Self {
        Self::invalid_key(e)
    }
}
