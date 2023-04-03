//! Module with client and endpoint functions for the Email channel.

use std::collections::HashMap;
use std::io;

use reqwest::multipart::Form;
use reqwest::multipart::Part;
use tokio::io::AsyncReadExt;
use validator::Validate;

use crate::api::{
    build_api_error, send_multipart_request, send_no_body_request, send_valid_json_request,
    SdkError, SdkResponse,
};
use crate::configuration::Configuration;
use crate::model::email::{
    AddDomainRequestBody, AddDomainResponseBody, GetBulksQueryParameters, GetBulksResponseBody,
    GetDeliveryReportsQueryParameters, GetDeliveryReportsResponseBody, GetDomainResponseBody,
    GetDomainsQueryParameters, GetDomainsResponseBody, GetLogsQueryParameters, GetLogsResponseBody,
    GetScheduledStatusQueryParameters, GetScheduledStatusResponseBody, RescheduleQueryParameters,
    RescheduleRequestBody, RescheduleResponseBody, SendRequestBody, SendResponseBody,
    UpdateScheduledStatusQueryParameters, UpdateScheduledStatusRequestBody,
    UpdateScheduledStatusResponseBody, UpdateTrackingRequestBody, UpdateTrackingResponseBody,
    ValidateAddressRequestBody, ValidateAddressResponseBody,
};

pub const PATH_ADD_DOMAIN: &str = "/email/1/domains";
pub const PATH_DELETE_DOMAIN: &str = "/email/1/domains/{domainName}";
pub const PATH_GET_BULKS: &str = "/email/1/bulks";
pub const PATH_GET_DELIVERY_REPORTS: &str = "/email/1/reports";
pub const PATH_GET_DOMAIN: &str = "/email/1/domains/{domainName}";
pub const PATH_GET_DOMAINS: &str = "/email/1/domains";
pub const PATH_GET_LOGS: &str = "/email/1/logs";
pub const PATH_GET_SCHEDULED_STATUS: &str = "/email/1/bulks/status";
pub const PATH_RESCHEDULE: &str = "/email/1/bulks";
pub const PATH_SEND: &str = "/email/3/send";
pub const PATH_UPDATE_SCHEDULED_STATUS: &str = "/email/1/bulks/status";
pub const PATH_UPDATE_TRACKING: &str = "/email/1/domains/{domainName}/tracking";
pub const PATH_VALIDATE: &str = "/email/2/validation";
pub const PATH_VERIFY_DOMAIN: &str = "/email/1/domains/{domainName}/verify";

async fn get_file_part(file_name: String) -> io::Result<Part> {
    let mut file = tokio::fs::File::open(file_name.clone()).await?;
    let mut buffer = Vec::new();
    let count = file.read_to_end(&mut buffer).await?;

    Ok(Part::stream_with_length(buffer, count as u64).file_name(file_name))
}

async fn build_form(request_body: SendRequestBody) -> io::Result<Form> {
    let mut form = Form::new().text("to", request_body.to.clone());

    if let Some(from) = request_body.from {
        form = form.text("from", from);
    }
    if let Some(cc) = request_body.cc {
        form = form.text("cc", cc);
    }
    if let Some(bcc) = request_body.bcc {
        form = form.text("bcc", bcc);
    }
    if let Some(subject) = request_body.subject {
        form = form.text("subject", subject);
    }
    if let Some(text) = request_body.text {
        form = form.text("text", text);
    }
    if let Some(html) = request_body.html {
        form = form.text("html", html);
    }
    if let Some(amp_html) = request_body.amp_html {
        form = form.text("ampHtml", amp_html);
    }
    if let Some(template_id) = request_body.template_id {
        form = form.text("templateId", template_id.to_string());
    }
    if let Some(attachment) = request_body.attachment {
        form = form.part("attachment", get_file_part(attachment).await?);
    }
    if let Some(inline_image) = request_body.inline_image {
        form = form.part("inlineImage", get_file_part(inline_image).await?);
    }
    if let Some(intermediate_report) = request_body.intermediate_report {
        form = form.text("intermediateReport", intermediate_report.to_string());
    }
    if let Some(notify_url) = request_body.notify_url {
        form = form.text("notifyUrl", notify_url);
    }
    if let Some(notify_content_type) = request_body.notify_content_type {
        form = form.text("notifyContentType", notify_content_type);
    }
    if let Some(callback_data) = request_body.callback_data {
        form = form.text("callbackData", callback_data);
    }
    if let Some(track) = request_body.track {
        form = form.text("track", track.to_string());
    }
    if let Some(track_clicks) = request_body.track_clicks {
        form = form.text("trackClicks", track_clicks.to_string());
    }
    if let Some(track_opens) = request_body.track_opens {
        form = form.text("trackOpens", track_opens.to_string());
    }
    if let Some(tracking_url) = request_body.tracking_url {
        form = form.text("trackingUrl", tracking_url);
    }
    if let Some(bulk_id) = request_body.bulk_id {
        form = form.text("bulkId", bulk_id);
    }
    if let Some(message_id) = request_body.message_id {
        form = form.text("messageId", message_id);
    }
    if let Some(reply_to) = request_body.reply_to {
        form = form.text("replyTo", reply_to);
    }
    if let Some(default_placeholders) = request_body.default_placeholders {
        form = form.text("defaultPlaceholders", default_placeholders);
    }
    if let Some(preserve_recipients) = request_body.preserve_recipients {
        form = form.text("preserveRecipients", preserve_recipients.to_string());
    }
    if let Some(send_at) = request_body.send_at {
        form = form.text("sendAt", send_at);
    }
    if let Some(landing_page_placeholders) = request_body.landing_page_placeholders {
        form = form.text("landingPagePlaceholders", landing_page_placeholders);
    }
    if let Some(landing_page_id) = request_body.landing_page_id {
        form = form.text("landingPageId", landing_page_id);
    }

    Ok(form)
}

/// Main asynchronous client for the Infobip Email channel.
#[derive(Clone, Debug)]
pub struct EmailClient {
    configuration: Configuration,
    client: reqwest::Client,
}

impl EmailClient {
    /// Builds and returns a new asynchronous `EmailClient` with a specified configuration.
    pub fn with_configuration(configuration: Configuration) -> Self {
        EmailClient {
            configuration,
            client: reqwest::Client::new(),
        }
    }

    /// Send an email or multiple emails to a recipient or multiple recipients with CC/BCC enabled.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::email::EmailClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::email::SendRequestBody;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EmailClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let mut request_body = SendRequestBody::new("someone@domain.com");
    /// request_body.from = Some("someone@company.com".to_string());
    /// request_body.subject = Some("Test subject".to_string());
    /// request_body.text = Some("Hello world!".to_string());
    ///
    /// let response = client.send(request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(
        &self,
        request_body: SendRequestBody,
    ) -> Result<SdkResponse<SendResponseBody>, SdkError> {
        request_body.validate()?;

        let form = build_form(request_body).await?;

        let response = send_multipart_request(
            &self.client,
            &self.configuration,
            form,
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

    /// See the scheduled time of your Email messages.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::email::EmailClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::email::GetBulksQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EmailClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_parameters = GetBulksQueryParameters::new("some-bulk-id");
    ///
    /// let response = client.get_bulks(query_parameters).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bulks(
        &self,
        query_parameters: GetBulksQueryParameters,
    ) -> Result<SdkResponse<GetBulksResponseBody>, SdkError> {
        query_parameters.validate()?;

        let parameters_map = HashMap::from([("bulkId".to_string(), query_parameters.bulk_id)]);

        let response = send_no_body_request(
            &self.client,
            &self.configuration,
            parameters_map,
            reqwest::Method::GET,
            PATH_GET_BULKS,
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

    /// Change the date and time for sending scheduled messages.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::email::EmailClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::email::{RescheduleQueryParameters, RescheduleRequestBody};
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EmailClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_params = RescheduleQueryParameters::new("test-bulk-id-rust-003");
    /// let request_body = RescheduleRequestBody::new("2022-10-05T17:29:52Z");
    ///
    /// let response = client.reschedule(query_params, request_body).await?;
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

        let parameters_map = HashMap::from([("bulkId".to_string(), query_parameters.bulk_id)]);

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

    /// See the status of scheduled email messages.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::email::EmailClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::email::GetScheduledStatusQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EmailClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_params = GetScheduledStatusQueryParameters::new("some-bulk-id");
    ///
    /// let response = client.get_scheduled_status(query_params).await?;
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

        let parameters_map = HashMap::from([("bulkId".to_string(), query_parameters.bulk_id)]);

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

    /// Change status or completely cancel sending of scheduled messages.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::email::EmailClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::email::{BulkStatus, UpdateScheduledStatusQueryParameters, UpdateScheduledStatusRequestBody};
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EmailClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_params = UpdateScheduledStatusQueryParameters::new("some-bulk-id");
    /// let request_body = UpdateScheduledStatusRequestBody::new(BulkStatus::CANCELED);
    ///
    /// let response = client.update_scheduled_status(query_params, request_body).await?;
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

        let parameters_map = HashMap::from([("bulkId".to_string(), query_parameters.bulk_id)]);

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

    /// Get one-time delivery reports for all sent emails.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::email::EmailClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::email::GetDeliveryReportsQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EmailClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_params = GetDeliveryReportsQueryParameters::default();
    ///
    /// let response = client.get_delivery_reports(query_params).await?;
    ///
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

    /// Get email logs of sent Email messagesId for request. Email logs
    /// are available for the last 48 hours.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::email::EmailClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::email::GetLogsQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EmailClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_params = GetLogsQueryParameters::default();
    ///
    /// let response = client.get_logs(query_params).await?;
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
        if let Some(message_id) = query_parameters.message_id {
            parameters_map.insert("messageId".to_string(), message_id);
        }
        if let Some(from) = query_parameters.from {
            parameters_map.insert("from".to_string(), from);
        }
        if let Some(to) = query_parameters.to {
            parameters_map.insert("to".to_string(), to);
        }
        if let Some(bulk_id) = query_parameters.bulk_id {
            parameters_map.insert("bulkId".to_string(), bulk_id);
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

    /// Run validation to identify poor quality emails to clean up your recipient list.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::email::EmailClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::email::ValidateAddressRequestBody;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EmailClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let request_body = ValidateAddressRequestBody::new("someone@somewhere.com");
    ///
    /// let response = client.validate_address(request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn validate_address(
        &self,
        request_body: ValidateAddressRequestBody,
    ) -> Result<SdkResponse<ValidateAddressResponseBody>, SdkError> {
        let response = send_valid_json_request(
            &self.client,
            &self.configuration,
            request_body,
            HashMap::new(),
            reqwest::Method::POST,
            PATH_VALIDATE,
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

    /// Get all domains associated with the account. It also provides details of the
    /// retrieved domain like the DNS records, tracking details, active/blocked status, etc.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::email::EmailClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::email::GetDomainsQueryParameters;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EmailClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let query_params = GetDomainsQueryParameters::default();
    ///
    /// let response = client.get_domains(query_params).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_domains(
        &self,
        query_parameters: GetDomainsQueryParameters,
    ) -> Result<SdkResponse<GetDomainsResponseBody>, SdkError> {
        query_parameters.validate()?;

        let mut parameters_map = HashMap::<String, String>::new();
        if let Some(size) = query_parameters.size {
            parameters_map.insert("size".to_string(), size.to_string());
        }
        if let Some(page) = query_parameters.page {
            parameters_map.insert("page".to_string(), page.to_string());
        }

        let response = send_no_body_request(
            &self.client,
            &self.configuration,
            parameters_map,
            reqwest::Method::GET,
            PATH_GET_DOMAINS,
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

    /// This method allows you to add new domains with a limit to create a maximum of 1000 domains
    /// in a day.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::email::EmailClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::email::AddDomainRequestBody;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EmailClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let request_body = AddDomainRequestBody::new("example.com");
    ///
    /// let response = client.add_domain(request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_domain(
        &self,
        request_body: AddDomainRequestBody,
    ) -> Result<SdkResponse<AddDomainResponseBody>, SdkError> {
        let response = send_valid_json_request(
            &self.client,
            &self.configuration,
            request_body,
            HashMap::new(),
            reqwest::Method::POST,
            PATH_ADD_DOMAIN,
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

    /// Get the details of the domain like the DNS records, tracking details, active/blocked
    /// status, etc.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::email::EmailClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EmailClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let response = client.get_domain("example.com").await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_domain(
        &self,
        domain_name: &str,
    ) -> Result<SdkResponse<GetDomainResponseBody>, SdkError> {
        let path = PATH_GET_DOMAIN.replace("{domainName}", domain_name);

        let response = send_no_body_request(
            &self.client,
            &self.configuration,
            HashMap::new(),
            reqwest::Method::GET,
            path.as_str(),
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

    /// This method allows you to delete an existing domain.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::email::EmailClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EmailClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let status = client.delete_domain("example.com").await?;
    ///
    /// assert_eq!(status, StatusCode::NO_CONTENT);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_domain(
        &self,
        domain_name: &str,
    ) -> Result<reqwest::StatusCode, SdkError> {
        let path = PATH_DELETE_DOMAIN.replace("{domainName}", domain_name);

        let response = send_no_body_request(
            &self.client,
            &self.configuration,
            HashMap::new(),
            reqwest::Method::DELETE,
            path.as_str(),
        )
        .await?;

        let status = response.status();

        if status.is_success() {
            Ok(status)
        } else {
            let text = response.text().await?;
            Err(build_api_error(status, &text))
        }
    }

    /// Update tracking events for the provided domain. Tracking events can be updated only for
    /// CLICKS, OPENS and UNSUBSCRIBES.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::email::EmailClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::email::UpdateTrackingRequestBody;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EmailClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let mut request_body = UpdateTrackingRequestBody::default();
    /// request_body.clicks = Some(true);
    ///
    /// let response = client.update_tracking("domain.com", request_body).await?;
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_tracking(
        &self,
        domain_name: &str,
        request_body: UpdateTrackingRequestBody,
    ) -> Result<SdkResponse<UpdateTrackingResponseBody>, SdkError> {
        let path = PATH_UPDATE_TRACKING.replace("{domainName}", domain_name);

        let response = send_valid_json_request(
            &self.client,
            &self.configuration,
            request_body,
            HashMap::new(),
            reqwest::Method::PUT,
            path.as_str(),
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

    /// Verify records(TXT, MX, DKIM) associated with the provided domain.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::email::EmailClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use reqwest::StatusCode;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = EmailClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let status = client.verify_domain("example.com").await?;
    ///
    /// assert_eq!(status, StatusCode::ACCEPTED);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn verify_domain(
        &self,
        domain_name: &str,
    ) -> Result<reqwest::StatusCode, SdkError> {
        let path = PATH_VERIFY_DOMAIN.replace("{domainName}", domain_name);

        let response = send_no_body_request(
            &self.client,
            &self.configuration,
            HashMap::new(),
            reqwest::Method::POST,
            path.as_str(),
        )
        .await?;

        let status = response.status();

        if status.is_success() {
            Ok(status)
        } else {
            let text = response.text().await?;
            Err(build_api_error(status, &text))
        }
    }
}
