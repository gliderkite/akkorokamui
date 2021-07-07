use hmac::{Hmac, Mac, NewMac};
use reqwest::header::{HeaderMap, HeaderValue};
use sha2::{Digest, Sha256, Sha512};
use std::{
    fmt,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{api::Body, Api, Credentials, Error, Result};

pub use r#async::Client;

pub mod r#async;
pub mod blocking;
pub(crate) mod builder;

/// The HTTP client used to query the Kraken servers.
///
/// # Note
/// The default client will only able to query public APIs. In order to query
/// private APIs you need to construct the client with your private credentials.
#[derive(Clone)]
pub struct HttpClient<T> {
    /// The HTTP client implementation.
    client: T,
    /// The credentials to use for private APIs.
    credentials: Option<Credentials>,
    /// The User-Agent header used for each request.
    user_agent: HeaderValue,
}

impl<T> fmt::Display for HttpClient<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} <{}>",
            self.user_agent.to_str().unwrap_or("User-Agent N/A"),
            user_agent()
        )
    }
}

impl<T> HttpClient<T> {
    /// Builds the POST request headers and body.
    fn make_req_args(&self, api: Api) -> Result<(HeaderMap, String)> {
        let nonce = self.nonce()?;
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

        Ok((headers, body))
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
            let mut mac = HmacSha512::new_from_slice(&private_key)?;
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

/// Gets the client User Agent.
pub(crate) const fn user_agent() -> &'static str {
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"))
}
