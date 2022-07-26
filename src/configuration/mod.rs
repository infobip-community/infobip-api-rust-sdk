//! Configuration of the Infobip client
use std::env::{self, VarError};

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub basic_auth: Option<BasicAuth>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl ApiKey {
    pub fn from_env() -> Result<ApiKey, VarError> {
        Ok(ApiKey {
            key: env::var("IB_API_KEY")?,
            prefix: Some("App ".to_string()),
        })
    }
}

impl Configuration {
    pub fn from_env_api_key() -> Result<Configuration, VarError> {
        Ok(Configuration {
            base_path: env::var("IB_BASE_URL")?,
            api_key: Some(ApiKey::from_env()?),
            basic_auth: None,
            bearer_access_token: None,
        })
    }

    pub fn with_api_key(api_key: ApiKey) -> Configuration {
        Configuration {
            base_path: "https://base.path".to_string(),
            api_key: Some(api_key),
            basic_auth: None,
            bearer_access_token: None,
        }
    }
}
