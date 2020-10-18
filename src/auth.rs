use reqwest::header::HeaderValue;
use std::{fs, path::Path};

use crate::{Error, Result};

/// Public-Private key pair to be used by the API.
#[derive(Clone)]
pub struct Credentials {
    api_key: HeaderValue,
    private_key: HeaderValue,
}

impl Credentials {
    /// Reads the given file where the first line contains the public API key
    /// and the second line contains the private key.
    pub fn read(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path).map_err(Error::invalid_key)?;
        let mut lines: Vec<&str> = content.lines().collect();

        let private_key = lines
            .pop()
            .map(|k| HeaderValue::from_str(k))
            .transpose()
            .map_err(Error::invalid_key)?;

        let api_key = lines
            .pop()
            .map(|k| HeaderValue::from_str(k))
            .transpose()
            .map_err(Error::invalid_key)?;

        match (api_key, private_key) {
            (Some(api_key), Some(private_key)) => Ok(Self {
                api_key,
                private_key,
            }),
            _ => Err(Error::invalid_key("key not found")),
        }
    }

    /// Gets the API public key.
    pub(crate) fn api_key(&self) -> &HeaderValue {
        &self.api_key
    }

    /// Gets the private key decoded as Base64.
    pub(crate) fn private_key(&self) -> Result<Vec<u8>> {
        base64::decode(&self.private_key).map_err(Error::invalid_key)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use anyhow::Result;
    use std::{env, path::PathBuf};
    use uuid::Uuid;

    #[derive(Debug)]
    pub struct DummyCredentials {
        pub api_key: String,
        pub private_key: String,
        pub path: PathBuf,
    }

    impl DummyCredentials {
        /// Creates a new file with dummy credentials.
        pub fn new() -> Result<Self> {
            let path = env::temp_dir().join(Uuid::new_v4().to_string());

            let api_key = "<api_key>".to_string();
            let private_key = "<private_key>".to_string();
            fs::write(&path, format!("{}\n{}", api_key, private_key))?;

            Ok(Self {
                api_key,
                private_key,
                path,
            })
        }
    }

    impl Drop for DummyCredentials {
        /// Deletes the dummy credentials file.
        fn drop(&mut self) {
            fs::remove_file(&self.path)
                .expect("Cannot remove dummy credentials");
        }
    }

    #[test]
    fn read_keys() -> Result<()> {
        let dummy = DummyCredentials::new()?;

        let credentials = Credentials::read(&dummy.path)?;
        assert_eq!(dummy.api_key, credentials.api_key.to_str()?);
        assert_eq!(dummy.private_key, credentials.private_key.to_str()?);

        Ok(())
    }
}
