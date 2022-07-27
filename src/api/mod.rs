//! Endpoint functions and base response and error types
use std::fmt;

use crate::configuration::Configuration;
use reqwest;
use reqwest::StatusCode;
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "sms")]
pub mod sms;

#[derive(Error, Debug)]
pub enum SdkError {
    #[error("request body has field errors")]
    Validation(#[from] validator::ValidationErrors),

    #[error("client error calling endpoint")]
    Reqwest(#[from] reqwest::Error),

    #[error("serialization error")]
    Serde(#[from] serde_json::Error),

    #[error("api request error")]
    ApiRequestError(#[from] ApiError),
}

#[derive(Error, Debug)]
pub struct ApiError {
    pub details: ApiErrorDetails,
    pub status: StatusCode,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "API request error: status: {} {}",
            self.status, self.details
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceException {
    #[serde(rename = "messageId")]
    pub message_id: Option<String>,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "validationErrors", skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestError {
    #[serde(rename = "serviceException")]
    pub service_exception: ServiceException,
}

#[derive(Clone, Debug, Error, PartialEq, Serialize, Deserialize)]
pub struct ApiErrorDetails {
    #[serde(rename = "requestError")]
    pub request_error: RequestError,
}

impl fmt::Display for ApiErrorDetails {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "API request error: {}",
            serde_json::to_string(self).expect("error deserializing request error")
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SdkResponse<T> {
    pub response_body: T,
    pub status: StatusCode,
}

pub fn add_auth(
    mut builder: reqwest::RequestBuilder,
    configuration: &Configuration,
) -> reqwest::RequestBuilder {
    if let Some(api_key) = &configuration.api_key {
        let key = api_key.key.to_owned();
        let prefix = api_key
            .prefix
            .to_owned()
            .unwrap_or_else(|| "App".to_string());

        builder = builder.header("Authorization", format!("{}{}", prefix, key));
    } else if let Some(basic_auth) = &configuration.basic_auth {
        builder = builder.basic_auth(
            basic_auth.username.to_owned(),
            basic_auth.password.to_owned(),
        );
    } else if let Some(token) = &configuration.bearer_access_token {
        builder = builder.bearer_auth(token);
    };

    builder
}
