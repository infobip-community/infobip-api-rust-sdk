use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    static ref LANGUAGE_CODES: Regex = Regex::new(r"^(TR|ES|PT|AUTODETECT)$").unwrap();
    static ref TRANSLITERATIONS: Regex = Regex::new(
        r"^(TURKISH|GREEK|CYRILLIC|SERBIAN_CYRILLIC|CENTRAL_EUROPEAN|BALTIC|NON_UNICODE)$"
    )
    .unwrap();
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate, Builder)]
#[builder(setter(strip_option))]
pub struct PreviewSmsRequestBody {
    /// Code for language character set of a message text.
    #[validate(regex = "LANGUAGE_CODES")]
    #[serde(rename = "languageCode", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub language_code: Option<String>,

    /// Message text to preview.
    #[serde(rename = "text")]
    pub text: String,

    /// Conversion of a message text from one script to another.
    #[serde(rename = "transliteration", skip_serializing_if = "Option::is_none")]
    #[validate(regex = "TRANSLITERATIONS")]
    #[builder(default = "None")]
    pub transliteration: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Language {
    /// Language code for the correct character set.
    #[serde(rename = "languageCode", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreviewLanguageConfiguration {
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    /// Conversion of a message text from one script to another.
    #[serde(rename = "transliteration", skip_serializing_if = "Option::is_none")]
    pub transliteration: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendSmsPreview {
    /// Number of remaining characters in the last SMS part.
    #[serde(
        rename = "charactersRemaining",
        skip_serializing_if = "Option::is_none"
    )]
    pub characters_remaining: Option<i32>,
    /// Configuration that, when sent with the original text, results in this preview.
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<PreviewLanguageConfiguration>,
    /// Number of SMS message parts required to deliver the message.
    #[serde(rename = "messageCount", skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i32>,
    /// Preview of the text as it should appear on the recipient’s device.
    #[serde(rename = "textPreview", skip_serializing_if = "Option::is_none")]
    pub text_preview: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreviewSmsResponseBody {
    /// Text supplied in the request.
    #[serde(rename = "originalText", skip_serializing_if = "Option::is_none")]
    pub original_text: Option<String>,
    /// Previews of applying different configurations to the original text.
    #[serde(rename = "previews", skip_serializing_if = "Option::is_none")]
    pub previews: Option<Vec<SendSmsPreview>>,
}

#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[builder(setter(strip_option))]
pub struct GetDeliveryReportsQueryParameters {
    /// Unique ID assigned to the request if messaging multiple recipients or sending multiple messages via a single API request.
    #[serde(rename = "bulkId", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub bulk_id: Option<String>,

    /// Unique message ID for which a report is requested.
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub message_id: Option<String>,

    /// Maximum number of delivery reports to be returned. If not set, the latest 50 records are returned.
    #[validate(range(max = 1000))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub limit: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsStatus {
    /// Action that should be taken to eliminate the error.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Human-readable description of the status.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Status group ID.
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,
    /// Status group name.
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// Status ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Status name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsPrice {
    /// The currency in which the price is expressed.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Price per one SMS.
    #[serde(rename = "pricePerMessage", skip_serializing_if = "Option::is_none")]
    pub price_per_message: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsError {
    /// Human-readable description of the error.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Error group ID.
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,
    /// Error group name.
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// Error ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Error name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tells if the error is permanent.
    #[serde(rename = "permanent", skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsReport {
    /// Bulk ID.
    #[serde(rename = "bulkId", skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,
    /// Callback data sent through `callbackData` field in fully featured SMS message.
    #[serde(rename = "callbackData", skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    /// Tells when the SMS was finished processing by Infobip (i.e., delivered to the destination, delivered to the destination network, etc.). Has the following format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(rename = "doneAt", skip_serializing_if = "Option::is_none")]
    pub done_at: Option<String>,
    /// Indicates whether the error occurred during the query execution.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<SmsError>,
    /// Sender ID that can be alphanumeric or numeric.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Mobile country and network codes.
    #[serde(rename = "mccMnc", skip_serializing_if = "Option::is_none")]
    pub mcc_mnc: Option<String>,
    /// Message ID.
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Sent SMS price.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<SmsPrice>,
    /// Tells when the SMS was sent. Has the following format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(rename = "sentAt", skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,
    /// The number of parts the sent SMS was split into.
    #[serde(rename = "smsCount", skip_serializing_if = "Option::is_none")]
    pub sms_count: Option<i32>,
    /// Indicates whether the message is successfully sent, not sent, delivered, not delivered, waiting for delivery or any other possible status.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<SmsStatus>,
    /// Destination address.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetDeliveryReportsResponseBody {
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<SmsReport>>,
}
