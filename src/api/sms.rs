use std::collections::HashMap;

use validator::Validate;

use crate::api::{
    build_api_error, send_blocking_valid_json_request, send_no_body_request,
    send_valid_json_request, ApiError, SdkError, SdkResponse,
};
use crate::model::sms::{
    GetDeliveryReportsQueryParameters, GetDeliveryReportsResponseBody,
    GetInboundReportsQueryParameters, GetInboundReportsResponseBody, GetLogsQueryParameters,
    GetLogsResponseBody, GetScheduledQueryParameters, GetScheduledResponseBody,
    GetScheduledStatusQueryParameters, GetScheduledStatusResponseBody, RescheduleQueryParameters,
    RescheduleRequestBody, RescheduleResponseBody, SendBinaryRequestBody, SendBinaryResponseBody,
    SendOverQueryParametersQueryParameters, SendOverQueryParametersResponseBody, SendRequestBody,
    SendResponseBody, UpdateScheduledStatusQueryParameters, UpdateScheduledStatusRequestBody,
    UpdateScheduledStatusResponseBody,
};
use crate::{
    configuration::Configuration,
    model::sms::{PreviewRequestBody, PreviewResponseBody},
};

pub const PATH_GET_DELIVERY_REPORTS: &str = "/sms/1/reports";
pub const PATH_GET_INBOUND: &str = "/sms/1/inbox/reports";
pub const PATH_GET_LOGS: &str = "/sms/1/logs";
pub const PATH_GET_SCHEDULED: &str = "/sms/1/bulks";
pub const PATH_RESCHEDULE: &str = "/sms/1/bulks";
pub const PATH_GET_SCHEDULED_STATUS: &str = "/sms/1/bulks/status";
pub const PATH_UPDATE_SCHEDULED_STATUS: &str = "/sms/1/bulks/status";
pub const PATH_PREVIEW: &str = "/sms/1/preview";
pub const PATH_SEND: &str = "/sms/2/text/advanced";
pub const PATH_SEND_BINARY: &str = "/sms/2/binary/advanced";
pub const PATH_SEND_OVER_QUERY_PARAMS: &str = "/sms/1/text/query";

/// Main asynchronous client for the Infobip SMS channel.
#[derive(Clone, Debug)]
pub struct SmsClient {
    configuration: Configuration,
    client: reqwest::Client,
}

impl SmsClient {
    /// Builds and returns a new asynchronous `SmsClient` with specified configuration.
    pub fn with_configuration(configuration: Configuration) -> Self {
        SmsClient {
            configuration,
            client: reqwest::Client::new(),
        }
    }

    /// Check how different message configurations will affect your message text, number of
    /// characters, and message parts.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::model::sms::PreviewRequestBody;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let mut request_body = PreviewRequestBody::new("Some text to preview".to_string());
    /// request_body.transliteration = Some("GREEK".to_string());
    ///
    /// let response = sms_client.preview(request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
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
                body: serde_json::from_str(&text)?,
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
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::GetDeliveryReportsQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_parameters = GetDeliveryReportsQueryParameters::new();
    ///
    /// let response = sms_client.get_delivery_reports(query_parameters).await?;
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
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
            PATH_GET_DELIVERY_REPORTS,
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

    /// Send a single, or multiple SMS messages to one or many destinations.
    ///
    /// Everything from sending a simple single message to a single destination, up to batch
    /// sending of personalized messages to the thousands of recipients with a single API request.
    /// Language, transliteration, scheduling and every advanced feature you can think of is
    /// supported.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::{Destination, Message, SendRequestBody};
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let mut message = Message::new(vec![Destination::new("555555555555".to_string())]);
    /// message.text = Some("Hello Rustacean!".to_string());
    /// message.from = Some("Infobip".to_string());
    ///
    /// let request_body = SendRequestBody::new(vec![message]);
    ///
    /// let response = sms_client.send(request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
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
                body: serde_json::from_str(&text)?,
                status,
            })
        } else {
            Err(build_api_error(status, &text))
        }
    }

    /// Send single or multiple binary messages to one or more destination addresses.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::{Destination, BinaryData, BinaryMessage, SendBinaryRequestBody};
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let mut message = BinaryMessage::new(vec![Destination::new("555555555555".to_string())]);
    /// message.binary = Some(BinaryData::new("0f c2 4a bf 34 13 ba".to_string()));
    ///
    /// let request_body = SendBinaryRequestBody::new(vec![message]);
    ///
    /// let response = sms_client.send_binary(request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_binary(
        &self,
        request_body: SendBinaryRequestBody,
    ) -> Result<SdkResponse<SendBinaryResponseBody>, SdkError> {
        let response = send_valid_json_request(
            &self.client,
            &self.configuration,
            request_body,
            HashMap::new(),
            reqwest::Method::POST,
            PATH_SEND_BINARY,
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

    /// See all scheduled messages and their scheduled date and time. To schedule a message, use
    /// the sendAt field when sending a message.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::GetScheduledQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_parameters = GetScheduledQueryParameters::new("dummy-rust-sdk-bulk-id".to_string());
    ///
    /// let response = sms_client.get_scheduled(query_parameters).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// }
    /// ```
    pub async fn get_scheduled(
        &self,
        query_parameters: GetScheduledQueryParameters,
    ) -> Result<SdkResponse<GetScheduledResponseBody>, SdkError> {
        query_parameters.validate()?;

        let parameters_map =
            HashMap::<String, String>::from([("bulkId".to_string(), query_parameters.bulk_id)]);

        let response = send_no_body_request(
            &self.client,
            &self.configuration,
            parameters_map,
            reqwest::Method::GET,
            PATH_GET_SCHEDULED,
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

    /// Use this method for displaying logs for example in the user interface. Available are the
    /// logs for the last 48 hours and you can only retrieve maximum of 1000 logs per call.
    /// See `get_delivery_reports` if your use case is to verify message delivery.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::GetLogsQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_parameters = GetLogsQueryParameters::new();
    ///
    /// let response = sms_client.get_logs(query_parameters).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_logs(
        &self,
        query_parameters: GetLogsQueryParameters,
    ) -> Result<SdkResponse<GetLogsResponseBody>, SdkError> {
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
            PATH_GET_LOGS,
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

    /// If for some reason you are unable to receive incoming SMS to the endpoint of your choice
    /// in real time, you can use this API call to fetch messages. Each request will return a
    /// batch of received messages - only once. The API request will only return new messages
    /// that arrived since the last API request.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::GetInboundReportsQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_parameters = GetInboundReportsQueryParameters::new();
    ///
    /// let response = sms_client.get_inbound_reports(query_parameters).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_inbound_reports(
        &self,
        query_parameters: GetInboundReportsQueryParameters,
    ) -> Result<SdkResponse<GetInboundReportsResponseBody>, SdkError> {
        query_parameters.validate()?;

        let mut parameters_map = HashMap::<String, String>::new();
        if let Some(limit) = query_parameters.limit {
            parameters_map.insert("limit".to_string(), limit.to_string());
        }

        let response = send_no_body_request(
            &self.client,
            &self.configuration,
            parameters_map,
            reqwest::Method::GET,
            PATH_GET_INBOUND,
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

    /// All message parameters of the message can be defined in the query string. Use this method
    /// only if Send SMS message is not an option for your use case!
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::SendOverQueryParametersQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let destinations = vec!["31612345678".to_string(), "31698765432".to_string()];
    /// let query_parameters = SendOverQueryParametersQueryParameters::new(
    ///     "username".to_string(),
    ///     "password".to_string(),
    ///     destinations
    /// );
    ///
    /// let response = sms_client.send_over_query_parameters(query_parameters).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_over_query_parameters(
        &self,
        query_parameters: SendOverQueryParametersQueryParameters,
    ) -> Result<SdkResponse<SendOverQueryParametersResponseBody>, SdkError> {
        query_parameters.validate()?;

        let mut parameters_map = HashMap::<String, String>::new();
        parameters_map.insert("username".to_string(), query_parameters.username);
        parameters_map.insert("password".to_string(), query_parameters.password);
        parameters_map.insert("to".to_string(), query_parameters.to.join(","));

        if let Some(bulk_id) = query_parameters.bulk_id {
            parameters_map.insert("bulkId".to_string(), bulk_id);
        }
        if let Some(from) = query_parameters.from {
            parameters_map.insert("from".to_string(), from);
        }
        if let Some(text) = query_parameters.text {
            parameters_map.insert("text".to_string(), text);
        }
        if let Some(flash) = query_parameters.flash {
            parameters_map.insert("flash".to_string(), flash.to_string());
        }
        if let Some(transliteration) = query_parameters.transliteration {
            parameters_map.insert("transliteration".to_string(), transliteration);
        }
        if let Some(language_code) = query_parameters.language_code {
            parameters_map.insert("languageCode".to_string(), language_code);
        }
        if let Some(intermediate_report) = query_parameters.intermediate_report {
            parameters_map.insert(
                "intermediateReport".to_string(),
                intermediate_report.to_string(),
            );
        }
        if let Some(notify_url) = query_parameters.notify_url {
            parameters_map.insert("notifyUrl".to_string(), notify_url);
        }
        if let Some(notify_content_type) = query_parameters.notify_content_type {
            parameters_map.insert("notifyContentType".to_string(), notify_content_type);
        }
        if let Some(callback_data) = query_parameters.callback_data {
            parameters_map.insert("callbackData".to_string(), callback_data);
        }
        if let Some(validity_period) = query_parameters.validity_period {
            parameters_map.insert("validityPeriod".to_string(), validity_period.to_string());
        }
        if let Some(send_at) = query_parameters.send_at {
            parameters_map.insert("sendAt".to_string(), send_at);
        }
        if let Some(track) = query_parameters.track {
            parameters_map.insert("track".to_string(), track);
        }
        if let Some(process_key) = query_parameters.process_key {
            parameters_map.insert("processKey".to_string(), process_key);
        }
        if let Some(tracking_type) = query_parameters.tracking_type {
            parameters_map.insert("trackingType".to_string(), tracking_type);
        }
        if let Some(india_dlt_content_template_id) = query_parameters.india_dlt_content_template_id
        {
            parameters_map.insert(
                "indiaDltContentTemplateId".to_string(),
                india_dlt_content_template_id,
            );
        }
        if let Some(india_dlt_principal_entity_id) = query_parameters.india_dlt_principal_entity_id
        {
            parameters_map.insert(
                "indiaDltPrincipalEntityId".to_string(),
                india_dlt_principal_entity_id,
            );
        }

        let response = send_no_body_request(
            &self.client,
            &self.configuration,
            parameters_map,
            reqwest::Method::GET,
            PATH_SEND_OVER_QUERY_PARAMS,
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

    /// Change the date and time of already scheduled messages. To schedule a message, use the
    /// sendAt field when sending a message.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::{RescheduleQueryParameters, RescheduleRequestBody};
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_parameters = RescheduleQueryParameters::new("some-bulk-id".to_string());
    /// let request_body = RescheduleRequestBody::new("2020-01-01T00:00:00".to_string());
    ///
    /// let response = sms_client.reschedule(query_parameters, request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn reschedule(
        &self,
        query_parameters: RescheduleQueryParameters,
        request_body: RescheduleRequestBody,
    ) -> Result<SdkResponse<RescheduleResponseBody>, SdkError> {
        query_parameters.validate()?;

        let parameters_map =
            HashMap::<String, String>::from([("bulkId".to_string(), query_parameters.bulk_id)]);

        let response = send_valid_json_request(
            &self.client,
            &self.configuration,
            request_body,
            parameters_map,
            reqwest::Method::PUT,
            PATH_RESCHEDULE,
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

    /// Get the status of scheduled messages.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::GetScheduledStatusQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_parameters = GetScheduledStatusQueryParameters::new("some-bulk-id".to_string());
    ///
    /// let response = sms_client.get_scheduled_status(query_parameters).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_scheduled_status(
        &self,
        query_parameters: GetScheduledStatusQueryParameters,
    ) -> Result<SdkResponse<GetScheduledStatusResponseBody>, SdkError> {
        query_parameters.validate()?;

        let parameters_map =
            HashMap::<String, String>::from([("bulkId".to_string(), query_parameters.bulk_id)]);

        let response = send_no_body_request(
            &self.client,
            &self.configuration,
            parameters_map,
            reqwest::Method::GET,
            PATH_GET_SCHEDULED_STATUS,
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

    /// Change status or completely cancel sending of scheduled messages. To schedule a message,
    /// use the sendAt field when sending a message.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::{UpdateScheduledStatusQueryParameters, UpdateScheduledStatusRequestBody, ScheduledStatus};
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_parameters = UpdateScheduledStatusQueryParameters::new("some-bulk-id".to_string());
    /// let request_body = UpdateScheduledStatusRequestBody::new(ScheduledStatus::CANCELED);
    ///
    /// let response = sms_client.update_scheduled_status(query_parameters, request_body).await?;
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_scheduled_status(
        &self,
        query_parameters: UpdateScheduledStatusQueryParameters,
        request_body: UpdateScheduledStatusRequestBody,
    ) -> Result<SdkResponse<UpdateScheduledStatusResponseBody>, SdkError> {
        query_parameters.validate()?;

        let parameters_map =
            HashMap::<String, String>::from([("bulkId".to_string(), query_parameters.bulk_id)]);

        let response = send_valid_json_request(
            &self.client,
            &self.configuration,
            request_body,
            parameters_map,
            reqwest::Method::PUT,
            PATH_UPDATE_SCHEDULED_STATUS,
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

/// Blocking client for the Infobip SMS channel.
pub struct BlockingSmsClient {
    configuration: Configuration,
    client: reqwest::blocking::Client,
}

impl BlockingSmsClient {
    /// Builds and returns a new `BlockingSmsClient` with a specified configuration.
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
                body: serde_json::from_str(&text)?,
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
