use std::collections::HashMap;

use reqwest::{Method, Response};
use serde::Serialize;
use validator::Validate;

use crate::api::{
    build_api_error, send_no_body_request, send_valid_json_request, SdkError, SdkResponse,
};
use crate::configuration::Configuration;
use crate::model::whatsapp::{
    CreateTemplateRequestBody, CreateTemplateResponseBody, GetTemplatesResponseBody,
    SendAudioRequestBody, SendAudioResponseBody, SendContactRequestBody, SendContactResponseBody,
    SendDocumentRequestBody, SendDocumentResponseBody, SendImageRequestBody, SendImageResponseBody,
    SendInteractiveButtonsRequestBody, SendInteractiveButtonsResponseBody,
    SendInteractiveListRequestBody, SendInteractiveListResponseBody,
    SendInteractiveMultiproductRequestBody, SendInteractiveMultiproductResponseBody,
    SendInteractiveProductRequestBody, SendInteractiveProductResponseBody, SendLocationRequestBody,
    SendLocationResponseBody, SendStickerRequestBody, SendStickerResponseBody,
    SendTemplateRequestBody, SendTemplateResponseBody, SendTextRequestBody, SendTextResponseBody,
    SendVideoRequestBody, SendVideoResponseBody,
};

pub const PATH_CREATE_TEMPLATE: &str = "/whatsapp/2/senders/{sender}/templates";
pub const PATH_DELETE_TEMPLATE: &str = "/whatsapp/2/senders/{sender}/templates/{templateName}";
pub const PATH_GET_TEMPLATES: &str = "/whatsapp/2/senders/{sender}/templates";
pub const PATH_SEND_AUDIO: &str = "/whatsapp/1/message/audio";
pub const PATH_SEND_CONTACT: &str = "/whatsapp/1/message/contact";
pub const PATH_SEND_DOCUMENT: &str = "/whatsapp/1/message/document";
pub const PATH_SEND_IMAGE: &str = "/whatsapp/1/message/image";
pub const PATH_SEND_INTERACTIVE_BUTTONS: &str = "/whatsapp/1/message/interactive/buttons";
pub const PATH_SEND_INTERACTIVE_LIST: &str = "/whatsapp/1/message/interactive/list";
pub const PATH_SEND_INTERACTIVE_MULTIPRODUCT: &str =
    "/whatsapp/1/message/interactive/multi-product";
pub const PATH_SEND_INTERACTIVE_PRODUCT: &str = "/whatsapp/1/message/interactive/product";
pub const PATH_SEND_LOCATION: &str = "/whatsapp/1/message/location";
pub const PATH_SEND_STICKER: &str = "/whatsapp/1/message/sticker";
pub const PATH_SEND_TEMPLATE: &str = "/whatsapp/1/message/template";
pub const PATH_SEND_TEXT: &str = "/whatsapp/1/message/text";
pub const PATH_SEND_VIDEO: &str = "/whatsapp/1/message/video";

/// Main asynchronous client for the Infobip WhatsApp channel.
#[derive(Clone, Debug)]
pub struct WhatsappClient {
    configuration: Configuration,
    client: reqwest::Client,
}

impl WhatsappClient {
    /// Builds and returns a new asynchronous `WhatsappClient` with a specified configuration.
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
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::whatsapp::WhatsappClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::whatsapp::{SendTextRequestBody, TextContent};
    /// # use reqwest::StatusCode;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let wa_client = WhatsappClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let request_body = SendTextRequestBody::new(
    ///     "44444444444".to_string(),
    ///     "55555555555".to_string(),
    ///     TextContent::new("Hello, Rustacean!".to_string())
    /// );
    ///
    /// let response = wa_client.send_text(request_body).await.unwrap();
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_text(
        &self,
        request_body: SendTextRequestBody,
    ) -> Result<SdkResponse<SendTextResponseBody>, SdkError> {
        let response = self
            .send_request(request_body, HashMap::new(), Method::POST, PATH_SEND_TEXT)
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

    /// Send a document to a single recipient. Document messages can only be successfully delivered
    /// if the recipient has contacted the business within the last 24 hours, otherwise template
    /// message should be used.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::whatsapp::WhatsappClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::whatsapp::{SendDocumentRequestBody, DocumentContent};
    /// # use reqwest::StatusCode;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let wa_client = WhatsappClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let request_body = SendDocumentRequestBody::new(
    ///     "44444444444".to_string(),
    ///     "55555555555".to_string(),
    ///     DocumentContent::new("https://url.to/document.pdf".to_string())
    /// );
    ///
    /// let response = wa_client.send_document(request_body).await.unwrap();
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_document(
        &self,
        request_body: SendDocumentRequestBody,
    ) -> Result<SdkResponse<SendDocumentResponseBody>, SdkError> {
        let response = self
            .send_request(
                request_body,
                HashMap::new(),
                Method::POST,
                PATH_SEND_DOCUMENT,
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

    /// Send an image to a single recipient. Image messages can only be successfully delivered if
    /// the recipient has contacted the business within the last 24 hours, otherwise template
    /// message should be used.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::whatsapp::WhatsappClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::whatsapp::{SendImageRequestBody, ImageContent};
    /// # use reqwest::StatusCode;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let wa_client = WhatsappClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let request_body = SendImageRequestBody::new(
    ///     "44444444444".to_string(),
    ///     "55555555555".to_string(),
    ///     ImageContent::new("https://url.to/image.jpg".to_string())
    /// );
    ///
    /// let response = wa_client.send_image(request_body).await.unwrap();
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    pub async fn send_image(
        &self,
        request_body: SendImageRequestBody,
    ) -> Result<SdkResponse<SendImageResponseBody>, SdkError> {
        let response = self
            .send_request(request_body, HashMap::new(), Method::POST, PATH_SEND_IMAGE)
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

    /// Send an audio to a single recipient. Audio messages can only be successfully delivered if
    /// the recipient has contacted the business within the last 24 hours, otherwise template
    /// message should be used.
    pub async fn send_audio(
        &self,
        request_body: SendAudioRequestBody,
    ) -> Result<SdkResponse<SendAudioResponseBody>, SdkError> {
        let response = self
            .send_request(request_body, HashMap::new(), Method::POST, PATH_SEND_AUDIO)
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

    /// Send a video to a single recipient. Video messages can only be successfully delivered if
    /// the recipient has contacted the business within the last 24 hours, otherwise template
    /// message should be used.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::whatsapp::WhatsappClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::whatsapp::{SendVideoRequestBody, VideoContent};
    /// # use reqwest::StatusCode;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let wa_client = WhatsappClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let request_body = SendVideoRequestBody::new(
    ///     "44444444444".to_string(),
    ///     "55555555555".to_string(),
    ///     VideoContent::new("https://url.to/video.mp4".to_string())
    /// );
    ///
    /// let response = wa_client.send_video(request_body).await.unwrap();
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_video(
        &self,
        request_body: SendVideoRequestBody,
    ) -> Result<SdkResponse<SendVideoResponseBody>, SdkError> {
        let response = self
            .send_request(request_body, HashMap::new(), Method::POST, PATH_SEND_VIDEO)
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

    /// Send a sticker to a single recipient. Sticker messages can only be successfully delivered
    /// if the recipient has contacted the business within the last 24 hours, otherwise template
    /// message should be used.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::whatsapp::WhatsappClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::whatsapp::{SendStickerRequestBody, StickerContent};
    /// # use reqwest::StatusCode;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let wa_client = WhatsappClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let request_body = SendStickerRequestBody::new(
    ///     "44444444444".to_string(),
    ///     "55555555555".to_string(),
    ///     StickerContent::new("https://url.to/sticker.webp".to_string())
    /// );
    ///
    /// let response = wa_client.send_sticker(request_body).await.unwrap();
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_sticker(
        &self,
        request_body: SendStickerRequestBody,
    ) -> Result<SdkResponse<SendStickerResponseBody>, SdkError> {
        let response = self
            .send_request(
                request_body,
                HashMap::new(),
                Method::POST,
                PATH_SEND_STICKER,
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

    /// Send a location to a single recipient. Location messages can only be successfully
    /// delivered if the recipient has contacted the business within the last 24 hours, otherwise
    /// template message should be used.
    ///
    /// # Example
    /// ```no_run
    /// # use infobip_sdk::api::whatsapp::WhatsappClient;
    /// # use infobip_sdk::configuration::Configuration;
    /// # use infobip_sdk::model::whatsapp::{SendLocationRequestBody, LocationContent};
    /// # use reqwest::StatusCode;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let wa_client = WhatsappClient::with_configuration(Configuration::from_env_api_key()?);
    ///
    /// let request_body = SendLocationRequestBody::new(
    ///     "44444444444".to_string(),
    ///     "55555555555".to_string(),
    ///     LocationContent::new(0.0, 0.0)
    /// );
    ///
    /// let response = wa_client.send_location(request_body).await.unwrap();
    ///
    /// assert_eq!(response.status, StatusCode::OK);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_location(
        &self,
        request_body: SendLocationRequestBody,
    ) -> Result<SdkResponse<SendLocationResponseBody>, SdkError> {
        let response = self
            .send_request(
                request_body,
                HashMap::new(),
                Method::POST,
                PATH_SEND_LOCATION,
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

    /// Send a contact to a single recipient. Contact messages can only be successfully delivered
    /// if the recipient has contacted the business within the last 24 hours, otherwise template
    /// message should be used.
    pub async fn send_contact(
        &self,
        request_body: SendContactRequestBody,
    ) -> Result<SdkResponse<SendContactResponseBody>, SdkError> {
        let response = self
            .send_request(
                request_body,
                HashMap::new(),
                Method::POST,
                PATH_SEND_CONTACT,
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

    /// Send an interactive buttons message to a single recipient. Interactive buttons messages
    /// can only be successfully delivered if the recipient has contacted the business within the
    /// last 24 hours, otherwise template message should be used.
    pub async fn send_interactive_buttons(
        &self,
        request_body: SendInteractiveButtonsRequestBody,
    ) -> Result<SdkResponse<SendInteractiveButtonsResponseBody>, SdkError> {
        let response = self
            .send_request(
                request_body,
                HashMap::new(),
                Method::POST,
                PATH_SEND_INTERACTIVE_BUTTONS,
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

    /// Send an interactive list message to a single recipient. Interactive list messages can only
    /// be successfully delivered if the recipient has contacted the business within the last 24
    /// hours, otherwise template message should be used.
    pub async fn send_interactive_list(
        &self,
        request_body: SendInteractiveListRequestBody,
    ) -> Result<SdkResponse<SendInteractiveListResponseBody>, SdkError> {
        let response = self
            .send_request(
                request_body,
                HashMap::new(),
                Method::POST,
                PATH_SEND_INTERACTIVE_LIST,
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

    /// Send an interactive product message to a single recipient. Interactive product messages
    /// can only be successfully delivered if the recipient has contacted the business within the
    /// last 24 hours, otherwise template message should be used.
    pub async fn send_interactive_product(
        &self,
        request_body: SendInteractiveProductRequestBody,
    ) -> Result<SdkResponse<SendInteractiveProductResponseBody>, SdkError> {
        let response = self
            .send_request(
                request_body,
                HashMap::new(),
                Method::POST,
                PATH_SEND_INTERACTIVE_PRODUCT,
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

    /// Send an interactive multi-product message to a single recipient. Interactive multi-product
    /// messages can only be successfully delivered if the recipient has contacted the business
    /// within the last 24 hours, otherwise template message should be used.
    pub async fn send_interactive_multiproduct(
        &self,
        request_body: SendInteractiveMultiproductRequestBody,
    ) -> Result<SdkResponse<SendInteractiveMultiproductResponseBody>, SdkError> {
        let response = self
            .send_request(
                request_body,
                HashMap::new(),
                Method::POST,
                PATH_SEND_INTERACTIVE_MULTIPRODUCT,
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

    /// Create a WhatsApp template. Created template will be submitted for WhatsApp's review and
    /// approval. Once approved, template can be sent to end-users. Refer to template guidelines
    /// for additional info.
    pub async fn create_template(
        &self,
        sender: String,
        request_body: CreateTemplateRequestBody,
    ) -> Result<SdkResponse<CreateTemplateResponseBody>, SdkError> {
        let path = PATH_CREATE_TEMPLATE.replace("{sender}", &sender);

        let response = self
            .send_request(request_body, HashMap::new(), Method::POST, path.as_str())
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

    /// Get all the templates and their statuses for a given sender.
    pub async fn get_templates(
        &self,
        sender: String,
    ) -> Result<SdkResponse<GetTemplatesResponseBody>, SdkError> {
        let path = PATH_GET_TEMPLATES.replace("{sender}", &sender);

        let response = send_no_body_request(
            &self.client,
            &self.configuration,
            HashMap::new(),
            Method::GET,
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

    /// Delete a WhatsApp template.
    //
    // If registered in multiple languages, deleting the message template will also delete all its
    // languages.
    // The template will be deleted for all senders registered under the same WhatsApp Business
    // Account (WABA).
    // The system will attempt to deliver sent messages for 30 days, regardless of the template
    // deletion.
    // Once deleted, the name of the approved template cannot be reused for 30 days.
    pub async fn delete_template(
        &self,
        sender: String,
        template_id: String,
    ) -> Result<reqwest::StatusCode, SdkError> {
        let path = PATH_DELETE_TEMPLATE
            .replace("{sender}", &sender)
            .replace("{templateName}", &template_id);

        let response = send_no_body_request(
            &self.client,
            &self.configuration,
            HashMap::new(),
            Method::DELETE,
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

    /// Send a single or multiple template messages to one or more recipients. Template messages
    /// can be sent and delivered at anytime. Each template needs to be registered and pre-approved
    /// by WhatsApp.
    pub async fn send_template(
        &self,
        request_body: SendTemplateRequestBody,
    ) -> Result<SdkResponse<SendTemplateResponseBody>, SdkError> {
        let response = self
            .send_request(
                request_body,
                HashMap::new(),
                Method::POST,
                PATH_SEND_TEMPLATE,
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
