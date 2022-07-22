use thiserror::Error;
use validator;

#[cfg(feature = "sms")]
pub mod sms;

#[derive(Error, Debug)]
pub enum SdkError {
    #[error("request body has field errors")]
    Validation(#[from] validator::ValidationErrors),

    #[error("error calling endpoint")]
    Reqwest(#[from] reqwest::Error),

    #[error("serialization error")]
    Serde(#[from] serde_json::Error),

    #[error("io error")]
    Io(#[from] std::io::Error),
}

pub struct ServiceException {
    message_id: String,
    text: String,
    validation_errors: String,
}

pub struct RequestError {
    service_exception: ServiceException,
}

pub struct SdkResponse<T> {
    pub response_body: T,
    pub status: reqwest::StatusCode,
}

