use std::fmt;

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendSmsRequestBody {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendSmsResponseBody {}

impl fmt::Display for SendSmsResponseBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Displayed response!")
    }
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
    pub language: Option<Box<Language>>,
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
    pub configuration: Option<Box<PreviewLanguageConfiguration>>,
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
