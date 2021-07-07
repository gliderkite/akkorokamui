use std::{convert::TryInto, fmt};

use crate::{
    client::{blocking, Client},
    Credentials, Error, Result,
};

/// Client builder.
pub struct ClientBuilder {
    /// The User-Agent header used for each request.
    user_agent: String,
    /// The credentials to use for private APIs.
    credentials: Option<Credentials>,
}

impl ClientBuilder {
    /// Creates a new Client builder with the given user agent.
    pub fn with_user_agent(user_agent: impl fmt::Display) -> Self {
        Self {
            user_agent: user_agent.to_string(),
            credentials: None,
        }
    }

    /// Sets the client credentials.
    pub fn with_credentials(
        mut self,
        credentials: impl Into<Credentials>,
    ) -> Self {
        self.credentials = Some(credentials.into());
        self
    }

    /// Consumes the client builder to build a new blocking Client.
    pub fn build_blocking(self) -> Result<blocking::Client> {
        Ok(blocking::Client {
            client: reqwest::blocking::Client::default(),
            credentials: self.credentials,
            user_agent: self
                .user_agent
                .try_into()
                .map_err(Error::invalid_agent)?,
        })
    }

    /// Consumes the client builder to build a new asynchronous Client.
    pub fn build_async(self) -> Result<Client> {
        Ok(Client {
            client: reqwest::Client::default(),
            credentials: self.credentials,
            user_agent: self
                .user_agent
                .try_into()
                .map_err(Error::invalid_agent)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{auth::tests::DummyCredentials, client};
    use anyhow::Result;

    #[test]
    fn client_builder_with_credentials() -> Result<()> {
        let dummy = DummyCredentials::new()?;

        let credentials = Credentials::read(&dummy.path)?;
        let client = ClientBuilder::with_user_agent(client::user_agent())
            .with_credentials(credentials)
            .build_blocking()?;
        assert_eq!(client.user_agent.to_str()?, client::user_agent());
        assert!(client.credentials.is_some());

        Ok(())
    }
}
