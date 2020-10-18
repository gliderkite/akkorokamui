use hmac::{Hmac, Mac, NewMac};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256, Sha512};
use std::{
    convert::TryInto,
    fmt,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{api::Body, Api, Credentials, Error, Result};

/// The asynchronous HTTP client used to query the Kraken servers.
///
/// # Note
/// The default client will only able to query public APIs. In order to query
/// private APIs you need to construct the client with your private credentials.
#[derive(Clone)]
pub struct Client {
    /// The HTTP asynchronous client.
    client: reqwest::Client,
    /// The credentials to use for private APIs.
    credentials: Option<Credentials>,
    /// The User-Agent header used for each request.
    user_agent: HeaderValue,
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} <{}>",
            self.user_agent.to_str().unwrap_or_default(),
            user_agent()
        )
    }
}

impl Client {
    /// Sends the request to the Kraken servers.
    pub async fn send<T: DeserializeOwned>(&self, mut api: Api) -> Result<T> {
        log::trace!("{}", api);

        let user_agent = self.user_agent.to_owned();
        api.inner.headers.append(USER_AGENT, user_agent);

        if api.is_public() {
            self.get(api).await
        } else {
            self.post(api).await
        }
    }

    /// Sends a GET request using the given API.
    async fn get<T: DeserializeOwned>(&self, api: Api) -> Result<T> {
        let resp = self
            .client
            .get(&api.url())
            .headers(api.inner.headers)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    /// Sends a POST request using the given API.
    async fn post<T: DeserializeOwned>(&self, api: Api) -> Result<T> {
        let nonce = self.nonce()?;
        let url = api.url();
        let uri_path = api.inner.uri_path();

        debug_assert!(!api.is_public());
        let body = Body::with_params(nonce, api.inner.params);
        let body = body.urlencode();

        let mut headers: HeaderMap = api.inner.headers;
        if let Some(credentials) = &self.credentials {
            let api_sign = self.api_sign(uri_path, nonce, &body)?;
            headers.insert("API-Key", credentials.api_key().to_owned());
            headers.insert("API-Sign", api_sign);
        }

        let resp = self
            .client
            .post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    /// Gets a new increasing nonce value.
    fn nonce(&self) -> Result<u64> {
        let elapsed = SystemTime::now().duration_since(UNIX_EPOCH)?;
        Ok(elapsed.as_millis() as u64)
    }

    /// Generates the API-Sign header value.
    fn api_sign(
        &self,
        uri_path: String,
        nonce: u64,
        body: &str,
    ) -> Result<HeaderValue> {
        if let Some(credentials) = &self.credentials {
            type HmacSha512 = Hmac<Sha512>;

            // API-Sign = Message signature using HMAC-SHA512 of (URI path +
            // SHA256(nonce + POST data)) and base64 decoded secret API key
            let sha_body = format!("{}{}", nonce, body);
            let sha = Sha256::digest(sha_body.as_bytes());

            let private_key = credentials.private_key()?;
            let mut mac = HmacSha512::new_varkey(&private_key)?;
            let mut hmac_data = uri_path.into_bytes();
            hmac_data.append(&mut sha.to_vec());
            mac.update(&hmac_data);

            let b64 = base64::encode(mac.finalize().into_bytes());
            HeaderValue::from_str(&b64).map_err(Error::internal)
        } else {
            Err(Error::Unauthorized)
        }
    }
}

/// Creates a new Client builder with the given user agent.
pub fn with_user_agent(user_agent: impl fmt::Display) -> ClientBuilder {
    ClientBuilder::with_user_agent(user_agent)
}

/// Client builder.
pub struct ClientBuilder {
    /// The User-Agent header used for each request.
    user_agent: String,
    /// The credentials to use for private APIs.
    credentials: Option<Credentials>,
}

impl ClientBuilder {
    /// Creates a new Client builder with the given user agent.
    fn with_user_agent(user_agent: impl fmt::Display) -> Self {
        Self {
            user_agent: user_agent.to_string(),
            credentials: None,
        }
    }

    /// Sets the client credentials.
    pub fn with_credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }
}

impl TryInto<Client> for ClientBuilder {
    type Error = Error;

    fn try_into(self) -> Result<Client> {
        let user_agent =
            self.user_agent.try_into().map_err(Error::invalid_agent)?;
        Ok(Client {
            client: reqwest::Client::default(),
            credentials: self.credentials,
            user_agent,
        })
    }
}

/// Gets the client User Agent.
const fn user_agent() -> &'static str {
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"))
}

#[cfg(test)]
impl Default for Client {
    fn default() -> Self {
        Self {
            client: reqwest::Client::default(),
            credentials: None,
            user_agent: HeaderValue::from_static(user_agent()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::tests::DummyCredentials;
    use anyhow::Result;
    use std::convert::TryInto;

    const USER_AGENT: &str = user_agent();

    #[test]
    fn client_builder() -> Result<()> {
        let client: Client = with_user_agent(USER_AGENT).try_into()?;
        assert_eq!(client.user_agent.to_str()?, USER_AGENT);
        assert!(client.credentials.is_none());
        Ok(())
    }

    #[test]
    fn client_builder_with_credentials() -> Result<()> {
        let dummy = DummyCredentials::new()?;

        let credentials = Credentials::read(&dummy.path)?;
        let client: Client = ClientBuilder::with_user_agent(USER_AGENT)
            .with_credentials(credentials)
            .try_into()?;
        assert_eq!(client.user_agent.to_str()?, USER_AGENT);
        assert!(client.credentials.is_some());

        Ok(())
    }
}
