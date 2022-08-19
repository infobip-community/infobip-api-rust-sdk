//! Models for calling WhatsApp endpoints.

use serde_derive::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TextContent {
    /// Content of the message being sent.
    #[validate(length(min = 1, max = 4096))]
    pub text: String,

    /// Allows for URL preview from within the message. If set to true, the message content must
    /// contain a URL starting with https:// or http://. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<bool>,
}

impl TextContent {
    pub fn new(text: String) -> Self {
        TextContent {
            text,
            preview_url: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DocumentContent {
    /// URL of a document sent in a WhatsApp message. Must be a valid URL starting with https://
    /// or http://. Maximum document size is 100MB.
    #[validate(url)]
    pub media_url: String,

    /// Caption of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 3000))]
    pub caption: Option<String>,

    /// File name of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 240))]
    pub filename: Option<String>,
}

impl DocumentContent {
    pub fn new(media_url: String) -> Self {
        DocumentContent {
            media_url,
            caption: None,
            filename: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendContentRequestBody<T: serde::Serialize + Validate> {
    /// Registered WhatsApp sender number. Must be in international format and comply with
    /// WhatsApp's requirements.
    #[validate(length(min = 1, max = 24))]
    pub from: String,

    /// Message recipient number. Must be in international format.
    #[validate(length(min = 1, max = 24))]
    pub to: String,

    /// The ID that uniquely identifies the message sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 50))]
    pub message_id: Option<String>,

    /// The content object to build a message that will be sent.
    #[validate]
    pub content: T,

    /// Custom client data that will be included in a Delivery Report.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 4000))]
    pub callback_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(url)]
    pub notify_url: Option<String>,
}

pub type SendTextRequestBody = SendContentRequestBody<TextContent>;

impl SendTextRequestBody {
    pub fn new(from: String, to: String, content: TextContent) -> Self {
        SendTextRequestBody {
            from,
            to,
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

pub type SendDocumentRequestBody = SendContentRequestBody<DocumentContent>;

impl SendDocumentRequestBody {
    pub fn new(from: String, to: String, content: DocumentContent) -> Self {
        SendDocumentRequestBody {
            from,
            to,
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    /// Status group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,

    /// Status group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// Action that should be taken to eliminate the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// Status ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    /// Status name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Human-readable description of the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendContentResponseBody {
    /// The destination address of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    /// Number of messages required to deliver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i32>,

    /// The ID that uniquely identifies the message sent. If not passed, it will be automatically
    /// generated and returned in a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// Indicates the status of the message and how to recover from an error should there be any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

pub type SendTextResponseBody = SendContentResponseBody;

pub type SendDocumentResponseBody = SendContentResponseBody;
