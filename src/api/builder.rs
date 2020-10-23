use reqwest::header::HeaderMap;
use std::{collections::HashMap, fmt};

use crate::{
    api::{private::PrivateMethod, public::PublicMethod, ApiKind},
    KRAKEN_DOMAIN,
};

/// API builder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiBuilder {
    pub(crate) kind: ApiKind,
    /// Kraken domain.
    pub(crate) domain: String,
    /// API version.
    pub(crate) version: String,
    /// Public/Private API path.
    pub(crate) path: String,
    /// API method.
    pub(crate) method: String,
    /// API parameters.
    pub(crate) params: HashMap<String, String>,
    /// API headers map.
    pub(crate) headers: HeaderMap,
}

impl fmt::Display for ApiBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url())?;

        // if the API is public the parameters are already included in the URL query
        if self.kind == ApiKind::Private && !self.params.is_empty() {
            write!(f, "?{}", self.params())?;
        }

        Ok(())
    }
}

impl ApiBuilder {
    /// Creates new API components for the given (public/private) path and method.
    fn with_method(kind: ApiKind, method: impl fmt::Display) -> Self {
        Self {
            kind,
            domain: KRAKEN_DOMAIN.into(),
            version: "0".into(),
            path: kind.to_string(),
            method: method.to_string(),
            params: HashMap::default(),
            headers: HeaderMap::default(),
        }
    }

    /// Adds a new parameter to the API.
    pub fn with(
        mut self,
        key: impl fmt::Display,
        value: impl fmt::Display,
    ) -> Self {
        self.params.insert(key.to_string(), value.to_string());
        self
    }

    /// Adds a new parameter to the API.
    pub fn with_mut(
        &mut self,
        key: impl fmt::Display,
        value: impl fmt::Display,
    ) -> &mut Self {
        self.params.insert(key.to_string(), value.to_string());
        self
    }

    /// Constructs the default API components for a public method.
    pub(crate) fn public(method: PublicMethod) -> Self {
        Self::with_method(ApiKind::Public, method)
    }

    /// Constructs the default API components for a private method.
    pub(crate) fn private(method: PrivateMethod) -> Self {
        Self::with_method(ApiKind::Private, method)
    }

    /// Gets the API URI path used for the Sign-API header.
    pub(crate) fn uri_path(&self) -> String {
        format!("/{}/{}/{}", self.version, self.path, self.method)
    }

    /// Gets the API URL.
    pub(crate) fn url(&self) -> String {
        let mut url = format!("{}{}", self.domain, self.uri_path());

        if self.kind == ApiKind::Public && !self.params.is_empty() {
            url.push_str(&format!("?{}", self.params()));
        }

        url
    }

    /// Gets the API list of parameters.
    fn params(&self) -> String {
        let mut params = String::new();
        for (key, value) in &self.params {
            params.push_str(&format!("{}={}&", key, value));
        }
        params.pop();
        params
    }
}
