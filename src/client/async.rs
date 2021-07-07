use reqwest::header::USER_AGENT;
use serde::de::DeserializeOwned;
use std::fmt;

use crate::{
    client::{self, builder::ClientBuilder},
    Api, Credentials, Response, Result,
};

/// The asynchronous HTTP client used to query the Kraken servers.
///
/// # Note
/// The default client will only able to query public APIs. In order to query
/// private APIs you need to construct the client with your private credentials.
pub type Client = client::HttpClient<reqwest::Client>;

impl Client {
    /// Constructs a new asynchronous Client that can only be used for public APIs.
    pub fn new(user_agent: impl fmt::Display) -> Result<Self> {
        ClientBuilder::with_user_agent(user_agent).build_async()
    }

    /// Constructs a new asynchronous Client with the given credentials.
    pub fn with_credentials(
        user_agent: impl fmt::Display,
        credentials: impl Into<Credentials>,
    ) -> Result<Self> {
        ClientBuilder::with_user_agent(user_agent)
            .with_credentials(credentials)
            .build_async()
    }

    /// Sends the request to the Kraken servers.
    pub async fn send<Req: Into<Api>, Resp: DeserializeOwned>(
        &self,
        api: Req,
    ) -> Result<Response<Resp>> {
        let mut api = api.into();
        log::trace!("Sending request {}", api);

        let user_agent = self.user_agent.to_owned();
        api.inner.headers.append(USER_AGENT, user_agent);

        let resp = if api.is_public() {
            self.get(api).await?
        } else {
            self.post(api).await?
        };

        let status = resp.status();
        let mut resp: Response<Resp> = resp.json().await?;
        resp.status_code = status.as_u16();

        Ok(resp)
    }

    /// Sends a GET request using the given API.
    async fn get(&self, api: Api) -> Result<reqwest::Response> {
        let resp = self
            .client
            .get(&api.url())
            .headers(api.inner.headers)
            .send()
            .await?;
        Ok(resp)
    }

    /// Sends a POST request using the given API.
    async fn post(&self, api: Api) -> Result<reqwest::Response> {
        let url = api.url();
        let (headers, body) = self.make_req_args(api)?;
        let resp = self
            .client
            .post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client;
    use anyhow::Result;

    #[test]
    fn client_builder() -> Result<()> {
        let client = Client::new(client::user_agent())?;
        assert_eq!(client.user_agent.to_str()?, client::user_agent());
        assert!(client.credentials.is_none());
        Ok(())
    }
}
