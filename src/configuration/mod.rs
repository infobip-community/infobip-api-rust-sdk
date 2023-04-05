//! Configuration of the Infobip client
use std::env::{self, VarError};

/// Holds the necessary configuration URL and authentication details of an Infobip client.
#[derive(Debug, Clone)]
pub struct Configuration {
    base_url: String,
    basic_auth: Option<BasicAuth>,
    bearer_access_token: Option<String>,
    api_key: Option<ApiKey>,
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
    pub fn with_api_key(base_url: String, api_key: ApiKey) -> Configuration {
        Configuration {
            base_url,
            api_key: Some(api_key),
            basic_auth: None,
            bearer_access_token: None,
        }
    }

    /// Returns the base URL of the Configuration.
    pub fn base_url(&self) -> &String {
        &self.base_url
    }

    /// Returns the API key of the Configuration.
    pub fn api_key(&self) -> Option<&ApiKey> {
        self.api_key.as_ref()
    }

    /// Returns the basic authentication of the Configuration.
    pub fn basic_auth(&self) -> Option<&BasicAuth> {
        self.basic_auth.as_ref()
    }

    /// Returns the bearer access token of the Configuration.
    pub fn bearer_access_token(&self) -> Option<&String> {
        self.bearer_access_token.as_ref()
    }
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
    /// Creates a new `ApiKey`.
    pub fn new(key: String) -> ApiKey {
        ApiKey {
            key,
            prefix: Some("App".to_string()),
        }
    }

    /// Reads `IB_API_KEY`, and optionally `IB_API_KEY_PREFIX`, variables from environment.
    pub fn from_env() -> Result<ApiKey, VarError> {
        Ok(ApiKey {
            key: env::var("IB_API_KEY")?,
            prefix: Some(env::var("IB_API_KEY_PREFIX").unwrap_or_else(|_| "App".to_string())),
        })
    }
}
