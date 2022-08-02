use std::collections::HashMap;

use validator::Validate;

use crate::api::{
    build_api_error, send_blocking_valid_json_request, send_no_body_request,
    send_valid_json_request, ApiError, SdkError, SdkResponse,
};
use crate::model::sms::{
    GetDeliveryReportsQueryParameters, GetDeliveryReportsResponseBody, SendRequestBody,
    SendResponseBody,
};
use crate::{
    configuration::Configuration,
    model::sms::{PreviewRequestBody, PreviewResponseBody},
};

pub const PATH_DELIVERY_REPORTS: &str = "/sms/1/reports";
pub const PATH_PREVIEW: &str = "/sms/1/preview";
pub const PATH_SEND: &str = "/sms/2/text/advanced";

/// Main asynchronous client for the Infobip SMS channel.
#[derive(Clone, Debug)]
pub struct SmsClient {
    configuration: Configuration,
    client: reqwest::Client,
}

impl SmsClient {
    /// Builds and returns a new asynchronous `SmsClient` with specified configuration.
    pub fn with_configuration(configuration: Configuration) -> SmsClient {
        SmsClient {
            configuration,
            client: reqwest::Client::new(),
        }
    }

    /// Check how different message configurations will affect your message text, number of
    /// characters and message parts.
    pub async fn preview(
        &self,
        request_body: PreviewRequestBody,
    ) -> Result<SdkResponse<PreviewResponseBody>, SdkError> {
        let response = send_valid_json_request(
            &self.client,
            &self.configuration,
            request_body,
            HashMap::new(),
            reqwest::Method::POST,
            PATH_PREVIEW,
        )
        .await?;

        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            Ok(SdkResponse {
                response_body: serde_json::from_str(&text)?,
                status,
            })
        } else {
            Err(build_api_error(status, &text))
        }
    }

    /// Get delivery reports for recently sent SMS messages.
    ///
    /// If you are for any reason unable to receive real-time delivery reports on your webhook
    /// endpoint, you can use this API method to learn if and when the message has been delivered
    /// to the recipient. Each request will return a batch of delivery reports - only once.
    /// This API request will return only new reports that arrived since the last API
    /// request in the last 48 hours.
    pub async fn get_delivery_reports(
        &self,
        query_parameters: GetDeliveryReportsQueryParameters,
    ) -> Result<SdkResponse<GetDeliveryReportsResponseBody>, SdkError> {
        query_parameters.validate()?;

        let mut parameters_map = HashMap::<String, String>::new();
        if let Some(bulk_id) = query_parameters.bulk_id {
            parameters_map.insert("bulkId".to_string(), bulk_id);
        }
        if let Some(message_id) = query_parameters.message_id {
            parameters_map.insert("messageId".to_string(), message_id);
        }
        if let Some(limit) = query_parameters.limit {
            parameters_map.insert("limit".to_string(), limit.to_string());
        }

        let response = send_no_body_request(
            &self.client,
            &self.configuration,
            parameters_map,
            reqwest::Method::GET,
            PATH_DELIVERY_REPORTS,
        )
        .await?;

        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            Ok(SdkResponse {
                response_body: serde_json::from_str(&text)?,
                status,
            })
        } else {
            Err(build_api_error(status, &text))
        }
    }

    /// Send a single, or multiple SMS messages to one or many destinations.
    ///
    /// Everything from sending a simple single message to a single destination, up to batch
    /// sending of personalized messages to the thousands of recipients with a single API request.
    /// Language, transliteration, scheduling and every advanced feature you can think of is
    /// supported.
    pub async fn send(
        &self,
        request_body: SendRequestBody,
    ) -> Result<SdkResponse<SendResponseBody>, SdkError> {
        let response = send_valid_json_request(
            &self.client,
            &self.configuration,
            request_body,
            HashMap::new(),
            reqwest::Method::POST,
            PATH_SEND,
        )
        .await?;

        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            Ok(SdkResponse {
                response_body: serde_json::from_str(&text)?,
                status,
            })
        } else {
            Err(build_api_error(status, &text))
        }
    }
}

/// Blocking client for the Infobip SMS channel.
pub struct BlockingSmsClient {
    configuration: Configuration,
    client: reqwest::blocking::Client,
}

impl BlockingSmsClient {
    /// Builds and returns a new `BlockingSmsClient` with specified configuration.
    pub fn with_configuration(configuration: Configuration) -> BlockingSmsClient {
        BlockingSmsClient {
            configuration,
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Check how different message configurations will affect your message text, number of
    /// characters and message parts. This is the blocking version.
    pub fn preview(
        &self,
        request_body: PreviewRequestBody,
    ) -> Result<SdkResponse<PreviewResponseBody>, SdkError> {
        let response = send_blocking_valid_json_request(
            &self.client,
            &self.configuration,
            request_body,
            reqwest::Method::POST,
            PATH_PREVIEW,
        )?;

        let status = response.status();
        let text = response.text()?;

        if status.is_success() {
            Ok(SdkResponse {
                response_body: serde_json::from_str(&text)?,
                status,
            })
        } else {
            let api_error = ApiError {
                details: serde_json::from_str(&text)?,
                status,
            };

            Err(SdkError::ApiRequestError(api_error))
        }
    }
}
