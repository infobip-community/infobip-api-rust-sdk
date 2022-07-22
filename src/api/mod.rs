#[cfg(feature = "sms")]
pub mod sms;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum ApiError {
    // TODO: add necessary types depending on return types from reqwest/json/serde
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}
