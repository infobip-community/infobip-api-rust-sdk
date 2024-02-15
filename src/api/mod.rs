//! Endpoint functions and base response and error types
use crate::configuration::{ApiKey, Configuration};
use reqwest;
use reqwest::{RequestBuilder, Response, StatusCode};
use serde::Deserialize;
use serde_derive::Serialize;
use std::{collections::HashMap, fmt};
use thiserror::Error;
use validator::Validate;

#[cfg(feature = "email")]
pub mod email;

#[cfg(feature = "sms")]
pub mod sms;

#[cfg(feature = "whatsapp")]
pub mod whatsapp;

/// Holds the possible errors that can happen when calling the Infobip API.
#[derive(Error, Debug)]
pub enum SdkError {
    #[error("request body has field errors")]
    Validation(#[from] validator::ValidationErrors),

    #[error("HTTP client error")]
    Reqwest(#[from] reqwest::Error),

    #[error("serialization error")]
    Serde(#[from] serde_json::Error),

    #[error("API request error")]
    ApiRequestError(#[from] ApiError),

    #[error("IO error")]
    Io(#[from] std::io::Error),
}

/// Holds the status code and error details when a 4xx or 5xx response is received.
#[derive(Error, Clone, Debug)]
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

/// Holds information about a server-side error.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceException {
    /// Identifier of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// Detailed error description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Map of validation errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<HashMap<String, Vec<String>>>,
}

/// Holds the exception produced by a server-side error.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestError {
    #[serde(rename = "serviceException")]
    pub service_exception: ServiceException,
}

/// Holds the details about a 4xx/5xx server-side error.
#[derive(Clone, Debug, Error, PartialEq, Eq, Serialize, Deserialize)]
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

/// Holds the status code and the response body of a successful API call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SdkResponse<T> {
    pub body: T,
    pub status: StatusCode,
}

fn api_key_authorization_value(api_key: &ApiKey) -> String {
    let key = api_key.key.to_owned();
    let prefix = api_key
        .prefix
        .to_owned()
        .unwrap_or_else(|| "App".to_string());

    format!("{} {}", prefix, key)
}

// Async version of add_auth, uses async request builder.
fn add_auth(mut builder: RequestBuilder, configuration: &Configuration) -> RequestBuilder {
    if let Some(api_key) = &configuration.api_key() {
        builder = builder.header("Authorization", api_key_authorization_value(api_key));
    } else if let Some(basic_auth) = &configuration.basic_auth() {
        builder = builder.basic_auth(
            basic_auth.username.to_owned(),
            basic_auth.password.to_owned(),
        );
    } else if let Some(token) = &configuration.bearer_access_token() {
        builder = builder.bearer_auth(token);
    };

    builder
}

#[inline]
fn user_agent() -> &'static str {
    include!("../../version.txt")
}

// Adds user agent to the request builder. Synchronous version.
fn add_user_agent(builder: RequestBuilder) -> RequestBuilder {
    builder.header("User-Agent", user_agent())
}

// Adds user agent to the request builder. Synchronous version.
#[cfg(feature = "sms")]
fn add_user_agent_blocking(
    builder: reqwest::blocking::RequestBuilder,
) -> reqwest::blocking::RequestBuilder {
    builder.header("User-Agent", user_agent())
}

// Blocking version of add_auth, uses blocking request builder.
#[cfg(feature = "sms")]
fn add_auth_blocking(
    mut builder: reqwest::blocking::RequestBuilder,
    configuration: &Configuration,
) -> reqwest::blocking::RequestBuilder {
    if let Some(api_key) = &configuration.api_key() {
        builder = builder.header("Authorization", api_key_authorization_value(api_key));
    } else if let Some(basic_auth) = &configuration.basic_auth() {
        builder = builder.basic_auth(
            basic_auth.username.to_owned(),
            basic_auth.password.to_owned(),
        );
    } else if let Some(token) = &configuration.bearer_access_token() {
        builder = builder.bearer_auth(token);
    };

    builder
}

fn build_api_error(status: StatusCode, text: &str) -> SdkError {
    match serde_json::from_str(text) {
        Ok(details) => SdkError::ApiRequestError(ApiError { details, status }),
        Err(e) => SdkError::Serde(e),
    }
}

async fn send_no_body_request(
    client: &reqwest::Client,
    configuration: &Configuration,
    query_parameters: HashMap<String, String>,
    method: reqwest::Method,
    path: &str,
) -> Result<Response, SdkError> {
    let url = format!("{}{}", configuration.base_url(), path);
    let mut builder = client.request(method, url).query(&query_parameters);

    builder = add_auth(builder, configuration);
    builder = add_user_agent(builder);

    Ok(builder.send().await?)
}

async fn send_valid_json_request<T: Validate + serde::Serialize>(
    client: &reqwest::Client,
    configuration: &Configuration,
    request_body: T,
    query_parameters: HashMap<String, String>,
    method: reqwest::Method,
    path: &str,
) -> Result<Response, SdkError> {
    request_body.validate()?;

    let url = format!("{}{}", configuration.base_url(), path);
    let mut builder = client
        .request(method, url)
        .json(&request_body)
        .query(&query_parameters);

    builder = add_auth(builder, configuration);
    builder = add_user_agent(builder);

    Ok(builder.send().await?)
}

#[cfg(feature = "email")]
async fn send_multipart_request(
    client: &reqwest::Client,
    configuration: &Configuration,
    form: reqwest::multipart::Form,
    method: reqwest::Method,
    path: &str,
) -> Result<Response, SdkError> {
    let url = format!("{}{}", configuration.base_url(), path);
    let mut builder = client.request(method, url);

    builder = add_auth(builder, configuration);
    builder = add_user_agent(builder);

    Ok(builder.multipart(form).send().await?)
}

#[cfg(feature = "sms")]
fn send_blocking_valid_json_request<T: Validate + serde::Serialize>(
    client: &reqwest::blocking::Client,
    configuration: &Configuration,
    request_body: T,
    method: reqwest::Method,
    path: &str,
) -> Result<reqwest::blocking::Response, SdkError> {
    request_body.validate()?;

    let url = format!("{}{}", configuration.base_url(), path);
    let mut builder = client.request(method, url);

    builder = add_auth_blocking(builder, configuration);
    builder = add_user_agent_blocking(builder);

    Ok(builder.json(&request_body).send()?)
}

mod tests;
