use std::collections::HashMap;

use reqwest::{Method, Response};
use serde::Serialize;
use validator::Validate;

use crate::api::{build_api_error, send_valid_json_request, SdkError, SdkResponse};
use crate::configuration::Configuration;
use crate::model::whatsapp::{SendTextRequestBody, SendTextResponseBody};

pub const PATH_SEND_TEXT: &str = "/whatsapp/1/message/text";

/// Main asynchronous client for the Infobip WhatsApp channel.
#[derive(Clone, Debug)]
pub struct WhatsappClient {
    configuration: Configuration,
    client: reqwest::Client,
}

impl WhatsappClient {
    /// Builds and returns a new asynchronous `WhatsappClient` with specified configuration.
    pub fn with_configuration(configuration: Configuration) -> Self {
        WhatsappClient {
            configuration,
            client: reqwest::Client::new(),
        }
    }

    async fn send_request<T: Validate + Serialize>(
        &self,
        request_body: T,
        parameters: HashMap<String, String>,
        method: Method,
        path: &str,
    ) -> Result<Response, SdkError> {
        send_valid_json_request(
            &self.client,
            &self.configuration,
            request_body,
            parameters,
            method,
            path,
        )
        .await
    }

    /// Send a text message to a single recipient. Text messages can only be successfully delivered
    /// if the recipient has contacted the business within the last 24 hours, otherwise template
    /// message should be used.
    pub async fn send_text(
        &self,
        request_body: SendTextRequestBody,
    ) -> Result<SdkResponse<SendTextResponseBody>, SdkError> {
        let response = self
            .send_request(
                request_body,
                HashMap::new(),
                reqwest::Method::POST,
                PATH_SEND_TEXT,
            )
            .await?;

        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            Ok(SdkResponse {
                body: serde_json::from_str(&text)?,
                status,
            })
        } else {
            Err(build_api_error(status, &text))
        }
    }
}
