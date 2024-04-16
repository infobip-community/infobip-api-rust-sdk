//! Module with client and endpoint functions for the SMS channel.

use std::collections::HashMap;

use validator::Validate;

use crate::api::{
    build_api_error, send_blocking_valid_json_request, send_no_body_request,
    send_valid_json_request, ApiError, SdkError, SdkResponse,
};
use crate::model::sms::{
    CreateTfaApplicationRequestBody, CreateTfaApplicationResponseBody,
    CreateTfaMessageTemplateRequestBody, CreateTfaMessageTemplateResponseBody,
    DeliveryReportsQueryParameters, DeliveryReportsResponseBody, InboundReportsQueryParameters,
    InboundReportsResponseBody, LogsQueryParameters, LogsResponseBody, RescheduleQueryParameters,
    RescheduleRequestBody, RescheduleResponseBody, ResendPinOverSmsRequestBody,
    ResendPinOverSmsResponseBody, ResendPinOverVoiceRequestBody, ResendPinOverVoiceResponseBody,
    ScheduledQueryParameters, ScheduledResponseBody, ScheduledStatusQueryParameters,
    ScheduledStatusResponseBody, SendBinaryRequestBody, SendBinaryResponseBody,
    SendOverQueryParametersQueryParameters, SendOverQueryParametersResponseBody,
    SendPinOverSmsQueryParameters, SendPinOverSmsRequestBody, SendPinOverSmsResponseBody,
    SendPinOverVoiceRequestBody, SendPinOverVoiceResponseBody, SendRequestBody, SendResponseBody,
    TfaApplicationResponseBody, TfaApplicationsResponseBody, TfaMessageTemplateResponseBody,
    TfaMessageTemplatesResponseBody, TfaVerificationStatusQueryParameters,
    TfaVerificationStatusResponseBody, UpdateScheduledStatusQueryParameters,
    UpdateScheduledStatusRequestBody, UpdateScheduledStatusResponseBody,
    UpdateTfaApplicationRequestBody, UpdateTfaApplicationResponseBody,
    UpdateTfaMessageTemplateRequestBody, UpdateTfaMessageTemplateResponseBody,
    VerifyPhoneNumberRequestBody, VerifyPhoneNumberResponseBody,
};
use crate::{
    configuration::Configuration,
    model::sms::{PreviewRequestBody, PreviewResponseBody},
};

pub const PATH_GET_DELIVERY_REPORTS: &str = "/sms/1/reports";
pub const PATH_GET_INBOUND: &str = "/sms/1/inbox/reports";
pub const PATH_GET_LOGS: &str = "/sms/1/logs";
pub const PATH_GET_SCHEDULED: &str = "/sms/1/bulks";
pub const PATH_GET_SCHEDULED_STATUS: &str = "/sms/1/bulks/status";
pub const PATH_PREVIEW: &str = "/sms/1/preview";
pub const PATH_RESCHEDULE: &str = "/sms/1/bulks";
pub const PATH_SEND: &str = "/sms/2/text/advanced";
pub const PATH_SEND_BINARY: &str = "/sms/2/binary/advanced";
pub const PATH_SEND_OVER_QUERY_PARAMS: &str = "/sms/1/text/query";
pub const PATH_UPDATE_SCHEDULED_STATUS: &str = "/sms/1/bulks/status";
pub const PATH_GET_TFA_APPLICATIONS: &str = "/2fa/2/applications";
pub const PATH_CREATE_TFA_APPLICATION: &str = "/2fa/2/applications";
pub const PATH_GET_TFA_APPLICATION: &str = "/2fa/2/applications/{appId}";
pub const PATH_UPDATE_TFA_APPLICATION: &str = "/2fa/2/applications/{appId}";
pub const PATH_GET_TFA_MESSAGE_TEMPLATES: &str = "/2fa/2/applications/{appId}/messages";
pub const PATH_CREATE_TFA_MESSAGE_TEMPLATE: &str = "/2fa/2/applications/{appId}/messages";
pub const PATH_GET_TFA_MESSAGE_TEMPLATE: &str = "/2fa/2/applications/{appId}/messages/{msgId}";
pub const PATH_UPDATE_TFA_MESSAGE_TEMPLATE: &str = "/2fa/2/applications/{appId}/messages/{msgId}";
pub const PATH_SEND_PIN_OVER_SMS: &str = "/2fa/2/pin";
pub const PATH_RESEND_PIN_OVER_SMS: &str = "/2fa/2/pin/{pinId}/resend";
pub const PATH_SEND_PIN_OVER_VOICE: &str = "/2fa/2/pin/voice";
pub const PATH_RESEND_PIN_OVER_VOICE: &str = "/2fa/2/pin/{pinId}/resend/voice";
pub const PATH_VERIFY_PHONE_NUMBER: &str = "/2fa/2/pin/{pinId}/verify";
pub const PATH_GET_TFA_VERIFICATION_STATUS: &str = "/2fa/2/applications/{appId}/verifications";

/// Main asynchronous client for the Infobip SMS channel.
#[derive(Clone, Debug)]
pub struct SmsClient {
    pub configuration: Configuration,
    pub http_client: reqwest::Client,
}

impl SmsClient {
    /// Builds and returns a new asynchronous `SmsClient` with specified configuration.
    pub fn with_configuration(configuration: Configuration) -> Self {
        SmsClient {
            configuration,
            http_client: reqwest::Client::new(),
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
    /// let mut request_body = PreviewRequestBody::new("Some text to preview");
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
            &self.http_client,
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

    ///  delivery reports for recently sent SMS messages.
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
    /// # use infobip_sdk::model::sms::DeliveryReportsQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_parameters = DeliveryReportsQueryParameters::new();
    ///
    /// let response = sms_client.delivery_reports(query_parameters).await?;
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delivery_reports(
        &self,
        query_parameters: DeliveryReportsQueryParameters,
    ) -> Result<SdkResponse<DeliveryReportsResponseBody>, SdkError> {
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
            &self.http_client,
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
    /// let message = Message {
    ///     destinations: Some(vec![Destination::new("555555555555")]),
    ///     text: Some("Hello Rustacean!".into()),
    ///     from: Some("Infobip".into()),
    ///     ..Default::default()
    /// };
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
            &self.http_client,
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
    /// let message = BinaryMessage {
    ///     destinations: Some(vec![Destination::new("555555555555")]),
    ///     binary: Some(BinaryData::new("0f c2 4a bf 34 13 ba")),
    ///     ..Default::default()
    /// };
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
            &self.http_client,
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
    /// # use infobip_sdk::model::sms::ScheduledQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_parameters = ScheduledQueryParameters::new("dummy-bulk-id");
    ///
    /// let response = sms_client.scheduled(query_parameters).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// }
    /// ```
    pub async fn scheduled(
        &self,
        query_parameters: ScheduledQueryParameters,
    ) -> Result<SdkResponse<ScheduledResponseBody>, SdkError> {
        query_parameters.validate()?;

        let parameters_map =
            HashMap::<String, String>::from([("bulkId".to_string(), query_parameters.bulk_id)]);

        let response = send_no_body_request(
            &self.http_client,
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
    /// See `delivery_reports` if your use case is to verify message delivery.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::LogsQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_parameters = LogsQueryParameters::new();
    ///
    /// let response = sms_client.logs(query_parameters).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn logs(
        &self,
        query_parameters: LogsQueryParameters,
    ) -> Result<SdkResponse<LogsResponseBody>, SdkError> {
        query_parameters.validate()?;

        let mut parameters_map = HashMap::<String, String>::new();
        if let Some(from) = query_parameters.from {
            parameters_map.insert("from".to_string(), from);
        }
        if let Some(to) = query_parameters.to {
            parameters_map.insert("to".to_string(), to);
        }
        if let Some(bulk_id) = query_parameters.bulk_id {
            parameters_map.insert("bulkId".to_string(), bulk_id);
        }
        if let Some(message_id) = query_parameters.message_id {
            parameters_map.insert("messageId".to_string(), message_id);
        }
        if let Some(general_status) = query_parameters.general_status {
            parameters_map.insert("generalStatus".to_string(), general_status);
        }
        if let Some(sent_since) = query_parameters.sent_since {
            parameters_map.insert("sentSince".to_string(), sent_since);
        }
        if let Some(sent_until) = query_parameters.sent_until {
            parameters_map.insert("sentUntil".to_string(), sent_until);
        }
        if let Some(limit) = query_parameters.limit {
            parameters_map.insert("limit".to_string(), limit.to_string());
        }
        if let Some(mcc) = query_parameters.mcc {
            parameters_map.insert("mcc".to_string(), mcc);
        }
        if let Some(mnc) = query_parameters.mnc {
            parameters_map.insert("mnc".to_string(), mnc);
        }

        let response = send_no_body_request(
            &self.http_client,
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
    /// # use infobip_sdk::model::sms::InboundReportsQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_parameters = InboundReportsQueryParameters::new();
    ///
    /// let response = sms_client.inbound_reports(query_parameters).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn inbound_reports(
        &self,
        query_parameters: InboundReportsQueryParameters,
    ) -> Result<SdkResponse<InboundReportsResponseBody>, SdkError> {
        query_parameters.validate()?;

        let mut parameters_map = HashMap::<String, String>::new();
        if let Some(limit) = query_parameters.limit {
            parameters_map.insert("limit".to_string(), limit.to_string());
        }

        let response = send_no_body_request(
            &self.http_client,
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
    ///     "username",
    ///     "password",
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
            &self.http_client,
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
    /// let query_parameters = RescheduleQueryParameters::new("some-bulk-id");
    /// let request_body = RescheduleRequestBody::new("2020-01-01T00:00:00");
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
            &self.http_client,
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

    ///  the status of scheduled messages.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::ScheduledStatusQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let sms_client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_parameters = ScheduledStatusQueryParameters::new("some-bulk-id");
    ///
    /// let response = sms_client.scheduled_status(query_parameters).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn scheduled_status(
        &self,
        query_parameters: ScheduledStatusQueryParameters,
    ) -> Result<SdkResponse<ScheduledStatusResponseBody>, SdkError> {
        query_parameters.validate()?;

        let parameters_map =
            HashMap::<String, String>::from([("bulkId".to_string(), query_parameters.bulk_id)]);

        let response = send_no_body_request(
            &self.http_client,
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
    /// let query_parameters = UpdateScheduledStatusQueryParameters::new("some-bulk-id");
    /// let request_body = UpdateScheduledStatusRequestBody::new(ScheduledStatus::Canceled);
    ///
    /// let response = sms_client.update_scheduled_status(query_parameters, request_body).await?;
    ///
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
            &self.http_client,
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

    ///  a list of your 2FA applications.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let response = client.tfa_applications().await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    pub async fn tfa_applications(
        &self,
    ) -> Result<SdkResponse<TfaApplicationsResponseBody>, SdkError> {
        let response = send_no_body_request(
            &self.http_client,
            &self.configuration,
            HashMap::new(),
            reqwest::Method::GET,
            PATH_GET_TFA_APPLICATIONS,
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

    /// Create and configure a new 2FA application.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::CreateTfaApplicationRequestBody;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    /// let request_body = CreateTfaApplicationRequestBody::new("some-name");
    ///
    /// let response = client.create_tfa_application(request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::CREATED);
    /// # Ok(())
    /// # }
    pub async fn create_tfa_application(
        &self,
        request_body: CreateTfaApplicationRequestBody,
    ) -> Result<SdkResponse<CreateTfaApplicationResponseBody>, SdkError> {
        let response = send_valid_json_request(
            &self.http_client,
            &self.configuration,
            request_body,
            HashMap::new(),
            reqwest::Method::POST,
            PATH_CREATE_TFA_APPLICATION,
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

    ///  a single 2FA application to see its configuration details.
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let application_id = "02CC3CAAFD733136AA15DFAC720A0C42";
    /// let response = client.tfa_application(application_id).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn tfa_application(
        &self,
        application_id: &str,
    ) -> Result<SdkResponse<TfaApplicationResponseBody>, SdkError> {
        let path = &PATH_GET_TFA_APPLICATION.replace("{appId}", application_id);

        let response = send_no_body_request(
            &self.http_client,
            &self.configuration,
            HashMap::new(),
            reqwest::Method::GET,
            path,
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

    /// Change configuration options for your existing 2FA application.
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::UpdateTfaApplicationRequestBody;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let application_id = "02CC3CAAFD733136AA15DFAC720A0C42";
    /// let request_body = UpdateTfaApplicationRequestBody::new("some-new-name");
    /// let response = client.update_tfa_application(application_id, request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_tfa_application(
        &self,
        application_id: &str,
        request_body: UpdateTfaApplicationRequestBody,
    ) -> Result<SdkResponse<UpdateTfaApplicationResponseBody>, SdkError> {
        let path = &PATH_UPDATE_TFA_APPLICATION.replace("{appId}", application_id);

        let response = send_valid_json_request(
            &self.http_client,
            &self.configuration,
            request_body,
            HashMap::new(),
            reqwest::Method::PUT,
            path,
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

    ///  all message templates in a 2FA application.
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let application_id = "02CC3CAAFD733136AA15DFAC720A0C42";
    /// let response = client.tfa_message_templates(application_id).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn tfa_message_templates(
        &self,
        application_id: &str,
    ) -> Result<SdkResponse<TfaMessageTemplatesResponseBody>, SdkError> {
        let path = &PATH_GET_TFA_MESSAGE_TEMPLATES.replace("{appId}", application_id);

        let response = send_no_body_request(
            &self.http_client,
            &self.configuration,
            HashMap::new(),
            reqwest::Method::GET,
            path,
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

    /// Create one or more message templates where your PIN will be dynamically included when you send the PIN message.
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::CreateTfaMessageTemplateRequestBody;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use infobip_sdk::model::sms::PinType::Numeric;
    /// let client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let application_id = "02CC3CAAFD733136AA15DFAC720A0C42";
    /// let request_body = CreateTfaMessageTemplateRequestBody::new("some-name", Numeric, 4);
    /// let response = client.create_tfa_message_template(application_id, request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_tfa_message_template(
        &self,
        application_id: &str,
        request_body: CreateTfaMessageTemplateRequestBody,
    ) -> Result<SdkResponse<CreateTfaMessageTemplateResponseBody>, SdkError> {
        let path = &PATH_CREATE_TFA_MESSAGE_TEMPLATE.replace("{appId}", application_id);

        let response = send_valid_json_request(
            &self.http_client,
            &self.configuration,
            request_body,
            HashMap::new(),
            reqwest::Method::POST,
            path,
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

    ///  a single 2FA message template from an application to see its configuration details.
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let application_id = "02CC3CAAFD733136AA15DFAC720A0C42";
    /// let template_id = "02CC3CAAFD733136AA15DFAC720A0C42";
    /// let response = client.tfa_message_template(application_id, template_id).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn tfa_message_template(
        &self,
        application_id: &str,
        template_id: &str,
    ) -> Result<SdkResponse<TfaMessageTemplateResponseBody>, SdkError> {
        let path = &PATH_GET_TFA_MESSAGE_TEMPLATE
            .replace("{appId}", application_id)
            .replace("{msgId}", template_id);

        let response = send_no_body_request(
            &self.http_client,
            &self.configuration,
            HashMap::new(),
            reqwest::Method::GET,
            path,
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

    /// Change configuration options for your existing 2FA application message template.
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::UpdateTfaMessageTemplateRequestBody;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use infobip_sdk::model::sms::PinType::Numeric;
    /// let client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let application_id = "02CC3CAAFD733136AA15DFAC720A0C42";
    /// let template_id = "02CC3CAAFD733136AA15DFAC720A0C42";
    /// let request_body = UpdateTfaMessageTemplateRequestBody::new("some-text", Numeric, 4);
    ///
    /// let response = client.update_tfa_message_template(application_id, template_id, request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_tfa_message_template(
        &self,
        application_id: &str,
        template_id: &str,
        request_body: UpdateTfaMessageTemplateRequestBody,
    ) -> Result<SdkResponse<UpdateTfaMessageTemplateResponseBody>, SdkError> {
        let path = &PATH_UPDATE_TFA_MESSAGE_TEMPLATE
            .replace("{appId}", application_id)
            .replace("{msgId}", template_id);

        let response = send_valid_json_request(
            &self.http_client,
            &self.configuration,
            request_body,
            HashMap::new(),
            reqwest::Method::PUT,
            path,
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

    /// Send a PIN code over SMS using a previously created message template.
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::SendPinOverSmsQueryParameters;
    /// # use infobip_sdk::model::sms::SendPinOverSmsRequestBody;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_parameters = SendPinOverSmsQueryParameters::default();
    /// let request_body = SendPinOverSmsRequestBody::new("some-application-id", "some-template-id", "555555555555");
    ///
    /// let response = client.send_pin_over_sms(query_parameters, request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_pin_over_sms(
        &self,
        query_parameters: SendPinOverSmsQueryParameters,
        request_body: SendPinOverSmsRequestBody,
    ) -> Result<SdkResponse<SendPinOverSmsResponseBody>, SdkError> {
        query_parameters.validate()?;
        let mut parameters_map = HashMap::new();
        if let Some(nc_needed) = query_parameters.nc_needed {
            parameters_map.insert("ncNeeded".to_string(), nc_needed.to_string());
        }

        let response = send_valid_json_request(
            &self.http_client,
            &self.configuration,
            request_body,
            parameters_map,
            reqwest::Method::POST,
            PATH_SEND_PIN_OVER_SMS,
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

    /// Resend the same (previously sent) PIN code over SMS.
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::ResendPinOverSmsRequestBody;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let pin_id = "02CC3CAAFD733136AA15DFAC720A0C42";
    /// let request_body = ResendPinOverSmsRequestBody::default();
    ///
    /// let response = client.resend_pin_over_sms(pin_id, request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn resend_pin_over_sms(
        &self,
        pin_id: &str,
        request_body: ResendPinOverSmsRequestBody,
    ) -> Result<SdkResponse<ResendPinOverSmsResponseBody>, SdkError> {
        let path = &PATH_RESEND_PIN_OVER_SMS.replace("{pinId}", pin_id);

        let response = send_valid_json_request(
            &self.http_client,
            &self.configuration,
            request_body,
            HashMap::new(),
            reqwest::Method::POST,
            path,
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

    /// Send a PIN code over Voice using previously created message template.
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::SendPinOverVoiceRequestBody;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let request_body = SendPinOverVoiceRequestBody::new("some-application-id", "some-template-id", "555555555555");
    ///
    /// let response = client.send_pin_over_voice(request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_pin_over_voice(
        &self,
        request_body: SendPinOverVoiceRequestBody,
    ) -> Result<SdkResponse<SendPinOverVoiceResponseBody>, SdkError> {
        let response = send_valid_json_request(
            &self.http_client,
            &self.configuration,
            request_body,
            HashMap::new(),
            reqwest::Method::POST,
            PATH_SEND_PIN_OVER_VOICE,
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

    /// Resend the same (previously sent) PIN code over Voice.
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::ResendPinOverVoiceRequestBody;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let pin_id = "02CC3CAAFD733136AA15DFAC720A0C42";
    /// let request_body = ResendPinOverVoiceRequestBody::default();
    ///
    /// let response = client.resend_pin_over_voice(pin_id, request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn resend_pin_over_voice(
        &self,
        pin_id: &str,
        request_body: ResendPinOverVoiceRequestBody,
    ) -> Result<SdkResponse<ResendPinOverVoiceResponseBody>, SdkError> {
        let path = &PATH_RESEND_PIN_OVER_VOICE.replace("{pinId}", pin_id);

        let response = send_valid_json_request(
            &self.http_client,
            &self.configuration,
            request_body,
            HashMap::new(),
            reqwest::Method::POST,
            path,
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

    /// Verify a phone number to confirm successful 2FA authentication.
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::VerifyPhoneNumberRequestBody;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use infobip_sdk::model::sms::VerifyPhoneNumberResponseBody;
    /// let client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let pin_id = "02CC3CAAFD733136AA15DFAC720A0C42";
    /// let request_body = VerifyPhoneNumberRequestBody::new("123456");
    /// let response = client.verify_phone_number(pin_id, request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn verify_phone_number(
        &self,
        pin_id: &str,
        request_body: VerifyPhoneNumberRequestBody,
    ) -> Result<SdkResponse<VerifyPhoneNumberResponseBody>, SdkError> {
        let path = &PATH_VERIFY_PHONE_NUMBER.replace("{pinId}", pin_id);

        let response = send_valid_json_request(
            &self.http_client,
            &self.configuration,
            request_body,
            HashMap::new(),
            reqwest::Method::POST,
            path,
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

    /// Check if a phone number is already verified for a specific 2FA application.
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::sms::SmsClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::sms::{TfaVerificationStatusQueryParameters,
    /// #         TfaVerificationStatusResponseBody};
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SmsClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_parameters = TfaVerificationStatusQueryParameters::new("555555555555");
    /// let response = client.tfa_verification_status("some-application-id", query_parameters).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn tfa_verification_status(
        &self,
        app_id: &str,
        query_parameters: TfaVerificationStatusQueryParameters,
    ) -> Result<SdkResponse<TfaVerificationStatusResponseBody>, SdkError> {
        let path = &PATH_GET_TFA_VERIFICATION_STATUS.replace("{appId}", app_id);

        query_parameters.validate()?;
        let mut parameters_map = HashMap::new();
        parameters_map.insert("msisdn".to_string(), query_parameters.msisdn);
        if let Some(verified) = query_parameters.verified {
            parameters_map.insert("verified".to_string(), verified.to_string());
        }
        if let Some(sent) = query_parameters.sent {
            parameters_map.insert("sent".to_string(), sent.to_string());
        }

        let response = send_no_body_request(
            &self.http_client,
            &self.configuration,
            HashMap::new(),
            reqwest::Method::GET,
            path,
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
