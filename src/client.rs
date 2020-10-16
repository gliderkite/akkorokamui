use hmac::{Hmac, Mac, NewMac};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256, Sha512};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{api::Body, Api, Credentials, Error, Result};

/// Gets the client User Agent.
pub fn user_agent() -> &'static str {
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"))
}

/// The asynchronous HTTP client used to query the Kraken servers.
///
/// # Note
/// The default client will only able to query public APIs. In order to query
/// private APIs you need to construct the client with your credentials.
#[derive(Default, Clone)]
pub struct Client {
    /// The HTTP asynchronous client.
    client: reqwest::Client,
    /// The credentials to use for private APIs.
    credentials: Option<Credentials>,
}

impl Client {
    /// Constructs a new Client with the given credentials.
    pub fn new(credentials: Credentials) -> Self {
        Self {
            client: reqwest::Client::default(),
            credentials: Some(credentials),
        }
    }

    /// Sends the request to the Kraken servers.
    pub async fn send<T: DeserializeOwned>(&self, api: Api) -> Result<T> {
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
