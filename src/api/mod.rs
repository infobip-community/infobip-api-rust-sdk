//! Endpoint functions and base response and error types
use std::fmt;

use reqwest::StatusCode;
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "sms")]
pub mod sms;

#[derive(Error, Debug)]
pub struct ApiError {
    pub source: RequestError,
    pub status: StatusCode,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "API request error: status: {} {}",
            self.status, self.source
        )
    }
}

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceException {
    pub message_id: String,
    pub text: String,
    pub validation_errors: String,
}

#[derive(Clone, Debug, Error, PartialEq, Serialize, Deserialize)]
pub struct RequestError {
    pub service_exception: ServiceException,
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("error deserializing request error")
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SdkResponse<T> {
    pub response_body: T,
    pub status: reqwest::StatusCode,
}
