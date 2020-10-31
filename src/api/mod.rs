use serde::{Deserialize, Serialize};
use serde_json::{value::Index, Value};
use std::fmt;

pub(crate) use body::Body;
pub use builder::ApiBuilder;

pub mod private;
pub mod public;

mod body;
mod builder;

/// Kraken API response.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Response<T> {
    /// List of error messages.
    pub error: Vec<String>,
    /// Result of API call (may not be present if errors occur).
    pub result: Option<T>,
    /// The response HTTP status code.
    #[serde(skip)]
    pub status_code: u16,
}

/// Generic Kraken API response.
pub type ResponseValue = Response<Value>;

impl ResponseValue {
    /// Gets the object at the given index from the response result.
    pub fn get(&self, index: impl Index) -> Option<&Value> {
        self.result.as_ref().and_then(|r| r.get(index))
    }

    /// Gets a mutable reference of the object at the given index from the
    /// response result.
    pub fn get_mut(&mut self, index: impl Index) -> Option<&mut Value> {
        self.result.as_mut().and_then(|r| r.get_mut(index))
    }
}

impl<T> Response<T> {
    /// Returns true only if the Response doesn't contain any error and the
    /// HTTP status code is within [200, 299].
    pub fn is_success(&self) -> bool {
        self.error.is_empty()
            && self.status_code >= 200
            && self.status_code < 300
    }
}

/// A single Kraken API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Api {
    pub(crate) inner: ApiBuilder,
}

impl Api {
    /// Returns true only if this is a public API.
    pub fn is_public(&self) -> bool {
        self.inner.kind == ApiKind::Public
    }

    /// Returns true only if this is a private API.
    pub fn is_private(&self) -> bool {
        self.inner.kind == ApiKind::Private
    }

    /// Gets the API URL.
    pub fn url(&self) -> String {
        self.inner.url()
    }
}

impl From<ApiBuilder> for Api {
    fn from(inner: ApiBuilder) -> Self {
        Self { inner }
    }
}

impl fmt::Display for Api {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

/// The API kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ApiKind {
    Public,
    Private,
}

impl fmt::Display for ApiKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Public => write!(f, "public"),
            Self::Private => write!(f, "private"),
        }
    }
}
