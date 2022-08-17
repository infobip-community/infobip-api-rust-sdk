//! Models for calling WhatsApp endpoints.

use serde_derive::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TextMessageContent {
    /// Content of the message being sent.
    #[validate(length(min = 1, max = 4096))]
    pub text: String,

    /// Allows for URL preview from within the message. If set to true, the message content must
    /// contain a URL starting with https:// or http://. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<bool>,
}

impl TextMessageContent {
    pub fn new(text: String) -> Self {
        TextMessageContent {
            text,
            preview_url: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendTextRequestBody {
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
    pub content: TextMessageContent,

    /// Custom client data that will be included in a Delivery Report.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 4000))]
    pub callback_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(url)]
    pub notify_url: Option<String>,
}

impl SendTextRequestBody {
    pub fn new(from: String, to: String, content: TextMessageContent) -> Self {
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
pub struct SendTextResponseBody {
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
