//! Configuration of the Infobip client
use std::env::{self, VarError};

/// Holds the necessary configuration URL and authentication details of an Infobip client.
#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_url: String,
    pub basic_auth: Option<BasicAuth>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
}

/// Holds the details for authentication based on username and password.
#[derive(Debug, Clone)]
pub struct BasicAuth {
    pub username: String,
    pub password: Option<String>,
}

/// Holds the details for API key authentication.
#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl ApiKey {
    /// Reads `IB_API_KEY`, and optionally `IB_API_KEY_PREFIX`, variables from environment.
    pub fn from_env() -> Result<ApiKey, VarError> {
        Ok(ApiKey {
            key: env::var("IB_API_KEY")?,
            prefix: Some(env::var("IB_API_KEY_PREFIX").unwrap_or_else(|_| "App".to_string())),
        })
    }
}

impl Configuration {
    /// Reads API key details and IB_BASE_URL environment variable to build and return a
    /// `Configuration` instance.
    pub fn from_env_api_key() -> Result<Configuration, VarError> {
        Ok(Configuration {
            base_url: env::var("IB_BASE_URL")?,
            api_key: Some(ApiKey::from_env()?),
            basic_auth: None,
            bearer_access_token: None,
        })
    }

    // Builds and returns a `Configuration` instance set with an API key.
    pub fn with_api_key(api_key: ApiKey) -> Configuration {
        Configuration {
            base_url: "https://base.path".to_string(),
            api_key: Some(api_key),
            basic_auth: None,
            bearer_access_token: None,
        }
    }
}
