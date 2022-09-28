use std::fs::File;
use std::io;
use std::io::Read;

use reqwest::multipart::Form;
use reqwest::multipart::Part;

use crate::api::{build_api_error, send_multipart_request, SdkError, SdkResponse};
use crate::configuration::Configuration;
use crate::model::email::{SendRequestBody, SendResponseBody};

pub const PATH_SEND: &str = "/email/3/send";

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
    pub async fn send(
        &self,
        request_body: SendRequestBody,
    ) -> Result<SdkResponse<SendResponseBody>, SdkError> {
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
}

fn get_file_part(file_name: String) -> io::Result<Part> {
    let mut file = File::open(file_name.clone())?;
    let mut buffer = Vec::new();
    let count = file.read_to_end(&mut buffer)?;

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
    if let Some(template_id) = request_body.template_id {
        form = form.text("templateId", template_id.to_string());
    }
    if let Some(attachment) = request_body.attachment {
        form = form.part("attachment", get_file_part(attachment)?);
    }
    if let Some(inline_image) = request_body.inline_image {
        form = form.part("inlineImage", get_file_part(inline_image)?);
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
        form = form.text("defaultPlaceholders", default_placeholders.to_string());
    }
    if let Some(preserve_recipients) = request_body.preserve_recipients {
        form = form.text("preserveRecipients", preserve_recipients.to_string());
    }
    if let Some(send_at) = request_body.send_at {
        form = form.text("sendAt", send_at.to_string());
    }
    if let Some(landing_page_placeholders) = request_body.landing_page_placeholders {
        form = form.text(
            "landingPagePlaceholders",
            landing_page_placeholders.to_string(),
        );
    }
    if let Some(landing_page_id) = request_body.landing_page_id {
        form = form.text("landingPageId", landing_page_id);
    }

    Ok(form)
}
