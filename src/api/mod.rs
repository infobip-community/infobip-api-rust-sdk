//! Endpoint functions and base response and error types
use std::collections::HashMap;
use std::fmt;

use reqwest;
use reqwest::{RequestBuilder, Response, StatusCode};
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

use crate::configuration::Configuration;

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

#[derive(Clone, Debug, PartialEq)]
pub struct QueryParameter {
    pub key: String,
    pub value: String,
}

fn add_auth(mut builder: RequestBuilder, configuration: &Configuration) -> RequestBuilder {
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

fn add_auth_blocking(
    mut builder: reqwest::blocking::RequestBuilder,
    configuration: &Configuration,
) -> reqwest::blocking::RequestBuilder {
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
    let url = format!("{}/{}", configuration.base_path, path);
    let mut builder = client
        .request(method, url)
        .query(&query_parameters);

    builder = add_auth(builder, configuration);

    Ok(builder.send().await?)
}

async fn send_json_request<T: Validate + serde::Serialize>(
    client: &reqwest::Client,
    configuration: &Configuration,
    request_body: T,
    query_parameters: HashMap<String, String>,
    method: reqwest::Method,
    path: &str,
) -> Result<Response, SdkError> {
    let url = format!("{}/{}", configuration.base_path, path);
    let mut builder = client
        .request(method, url)
        .json(&request_body)
        .query(&query_parameters);

    builder = add_auth(builder, configuration);

    Ok(builder.send().await?)
}

async fn _send_multipart_request(
    client: &reqwest::Client,
    configuration: &Configuration,
    form: reqwest::multipart::Form,
    method: reqwest::Method,
    path: &str,
) -> Result<Response, SdkError> {
    let url = format!("{}/{}", configuration.base_path, path);
    let mut builder = client.request(method, url);

    builder = add_auth(builder, configuration);

    Ok(builder.multipart(form).send().await?)
}

fn send_blocking_request<T: Validate + serde::Serialize>(
    client: &reqwest::blocking::Client,
    configuration: &Configuration,
    request_body: T,
    _query_parameters: Option<Vec<QueryParameter>>,
    method: reqwest::Method,
    path: &str,
) -> Result<reqwest::blocking::Response, SdkError> {
    request_body.validate()?;

    let url = format!("{}/{}", configuration.base_path, path);
    let mut builder = client.request(method, url);

    builder = add_auth_blocking(builder, configuration);

    Ok(builder.json(&request_body).send()?)
}
