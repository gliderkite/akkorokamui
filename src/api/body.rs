use percent_encoding::{utf8_percent_encode, CONTROLS};
use serde::Serialize;
use std::{collections::HashMap, fmt};

/// The body of a POST request.
#[derive(Debug, Serialize)]
pub struct Body {
    nonce: u64,
    otp: Option<String>,
    params: HashMap<String, String>,
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in &self.params {
            write!(f, "{}={}&", key, value)?;
        }

        write!(f, "nonce={}", self.nonce)?;

        if let Some(otp) = &self.otp {
            write!(f, "&otp={}", otp)?;
        }

        Ok(())
    }
}

impl Body {
    /// Constructs a new Body with the given nonce and parameters.
    pub fn with_params(nonce: u64, params: HashMap<String, String>) -> Self {
        Self {
            nonce,
            otp: None,
            params,
        }
    }

    /// Returns the URL encoded string representation of this Body.
    pub fn urlencode(&self) -> String {
        utf8_percent_encode(&self.to_string(), CONTROLS).to_string()
    }
}
