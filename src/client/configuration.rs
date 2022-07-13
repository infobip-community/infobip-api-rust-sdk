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

impl Configuration {
    pub fn _new() -> Configuration {
        Configuration {
            base_path: "https://base.path".to_string(),
            api_key: Some(ApiKey {
                prefix: Some("App ".to_string()),
                key: "aoeuaou".to_string(),
            }),
            basic_auth: None,
            bearer_access_token: None,
        }
    }

    pub fn _with_api_key(_api_key: ApiKey) -> Configuration {
        Configuration {
            base_path: "https://base.path".to_string(),
            api_key: Some(ApiKey {
                prefix: Some("App ".to_string()),
                key: "aoeuaou".to_string(),
            }),
            basic_auth: None,
            bearer_access_token: None,
        }
    }

    pub fn from_env() -> Configuration {
        Configuration {
            base_path: "https://base.path".to_string(),
            api_key: Some(ApiKey {
                prefix: Some("App ".to_string()),
                key: "aoeuaou".to_string(),
            }),
            basic_auth: None,
            bearer_access_token: None,
        }
    }
}
