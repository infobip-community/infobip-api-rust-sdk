//! Models for calling SMS endpoints.

use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

lazy_static::lazy_static! {
    static ref LANGUAGE_CODES: Regex = Regex::new(r"^(TR|ES|PT|AUTODETECT)$").unwrap();
    static ref TRANSLITERATIONS: Regex = Regex::new(
        r"^(TURKISH|GREEK|CYRILLIC|SERBIAN_CYRILLIC|CENTRAL_EUROPEAN|BALTIC|NON_UNICODE)$"
    )
    .unwrap();
    static ref CONTENT_TYPES: Regex = Regex::new(r"^(application/json|application/xml)$").unwrap();
    static ref TURKEY_RECIPIENT_TYPES: Regex = Regex::new(r"^(TACIR|BIREYSEL)$").unwrap();
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PreviewRequestBody {
    /// Code for language character set of a message text.
    #[validate(regex = "LANGUAGE_CODES")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    /// Message text to preview.
    pub text: String,

    /// Conversion of a message text from one script to another.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(regex = "TRANSLITERATIONS")]
    pub transliteration: Option<String>,
}

impl PreviewRequestBody {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    /// Language code for the correct character set.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(regex = "LANGUAGE_CODES")]
    pub language_code: Option<String>,
}

impl Language {
    pub fn new(language_code: &str) -> Self {
        Self {
            language_code: Some(language_code.into()),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewLanguageConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,

    /// Conversion of a message text from one script to another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transliteration: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    /// Number of remaining characters in the last SMS part.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters_remaining: Option<i32>,

    /// Configuration that, when sent with the original text, results in this preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<PreviewLanguageConfiguration>,

    /// Number of SMS message parts required to deliver the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i32>,

    /// Preview of the text as it should appear on the recipient’s device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_preview: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewResponseBody {
    /// Text supplied in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_text: Option<String>,

    /// Previews of applying different configurations to the original text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previews: Option<Vec<Preview>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryReportsQueryParameters {
    /// Unique ID assigned to the request if messaging multiple recipients or sending multiple
    /// messages via a single API request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,

    /// Unique message ID for which a report is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// Maximum number of delivery reports to be returned. If not set, the latest 50 records are
    /// returned.
    #[validate(range(max = 1000))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl GetDeliveryReportsQueryParameters {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    /// Action that should be taken to eliminate the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// Human-readable description of the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Status group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,
    /// Status group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// Status ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Status name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    /// The currency in which the price is expressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Price per one SMS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_per_message: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    /// Human-readable description of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Error group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,

    /// Error group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// Error ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    /// Error name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Tells if the error is permanent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    /// Bulk ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,

    /// Callback data sent through `callbackData` field in fully featured SMS message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,

    /// Tells when the SMS was finished processing by Infobip (i.e., delivered to the destination,
    /// delivered to the destination network, etc.). Has the following format:
    /// `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_at: Option<String>,

    /// Indicates whether the error occurred during the query execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,

    /// Sender ID that can be alphanumeric or numeric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// Mobile country and network codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc_mnc: Option<String>,

    /// Message ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// Sent SMS price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,

    /// Tells when the SMS was sent. Has the following format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,

    /// The number of parts the sent SMS was split into.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_count: Option<i32>,

    /// Indicates whether the message is successfully sent, not sent, delivered, not delivered,
    /// waiting for delivery or any other possible status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,

    /// Destination address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryReportsResponseBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<Report>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracking {
    /// Custom base url used for shortening links from SMS text in `URL` Conversion rate tracking
    /// use-case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,

    /// Key that uniquely identifies Conversion tracking process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_key: Option<String>,

    /// Indicates if the message has to be tracked for Conversion rates. Possible values:
    /// `SMS` and `URL`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,

    /// User-defined type of the Conversion tracking process or flow type or message type, etc.
    /// Example: `ONE_TIME_PIN or SOCIAL_INVITES`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_type: Option<String>,
}

impl Tracking {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeUnit {
    Minute,
    Hour,
    Day,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct DeliveryTime {
    /// Hour when the time window opens when used in from property or closes when used into the
    /// property.
    #[validate(range(min = 0, max = 23))]
    pub hour: i32,

    /// Minute when the time window opens when used in from property or closes when used into the
    /// property.
    #[validate(range(min = 0, max = 59))]
    pub minute: i32,
}

impl DeliveryTime {
    pub fn new(hour: i32, minute: i32) -> Self {
        Self { hour, minute }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeedLimit {
    /// The number of messages to be sent per timeUnit. By default, the system sends messages as
    /// fast as the infrastructure allows. Use this parameter to adapt sending capacity to your
    /// needs. The system is only able to work against its maximum capacity for ambitious message
    /// batches.
    pub amount: i32,

    /// The time unit in which the defined message amount will be sent. The default value is `MINUTE`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<TimeUnit>,
}

impl SpeedLimit {
    pub fn new(amount: i32) -> Self {
        Self {
            amount,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlOptions {
    /// Enable shortening of the URLs within a message. Set this to `true`, if you want to set up other URL options.
    #[serde(rename = "shortenUrl", skip_serializing_if = "Option::is_none")]
    pub shorten_url: Option<bool>,

    /// Enable tracking of short URL clicks within a message: which URL was clicked, how many times, and by whom.
    #[serde(rename = "trackClicks", skip_serializing_if = "Option::is_none")]
    pub track_clicks: Option<bool>,

    /// The URL of your callback server on to which the Click report will be sent.
    #[serde(rename = "trackingUrl", skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,

    /// Remove a protocol, such as `https://`, from links to shorten a message. Note that some mobiles may not recognize such links as a URL.
    #[serde(rename = "removeProtocol", skip_serializing_if = "Option::is_none")]
    pub remove_protocol: Option<bool>,

    /// Select a predefined custom domain to use when generating a short URL.
    #[serde(rename = "customDomain", skip_serializing_if = "Option::is_none")]
    pub custom_domain: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeliveryDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct DeliveryTimeWindow {
    /// Days which are included in the delivery time window. Values are: `MONDAY`, `TUESDAY`,
    /// `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`, `SUNDAY`. At least one day must be stated.
    #[validate(length(min = 1, max = 7))]
    pub days: Vec<DeliveryDay>,

    /// Exact time of day in which the sending can start. Consists of hour and minute properties,
    /// both mandatory. Time is expressed in the UTC time zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub from: Option<DeliveryTime>,

    /// Exact time of day in which the sending will end. Consists of an hour and minute properties,
    /// both mandatory. Time is expressed in the UTC time zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub to: Option<DeliveryTime>,
}

impl DeliveryTimeWindow {
    pub fn new(days: Vec<DeliveryDay>) -> Self {
        Self {
            days,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Destination {
    /// The ID that uniquely identifies the message sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// Message destination address. Addresses must be in international format (Example:
    /// `41793026727`).
    #[validate(length(min = 1, max = 50))]
    pub to: String,
}

impl Destination {
    pub fn new(to: &str) -> Self {
        Self {
            to: to.into(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IndiaDlt {
    /// Id of your registered DTL content template that matches this message's text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 30))]
    pub content_template_id: Option<String>,

    /// Your assigned DTL principal entity id.
    #[validate(length(min = 1))]
    pub principal_entity_id: String,
}

impl IndiaDlt {
    pub fn new(principal_entity_id: &str) -> Self {
        Self {
            principal_entity_id: principal_entity_id.into(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TurkeyIys {
    /// Brand code is an ID of the company based on a company VAT number. If not provided in
    /// request, default value is used from your Infobip account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_code: Option<i32>,

    /// Recipient Type must be `TACIR` or `BIREYSEL`.
    #[validate(regex = "TURKEY_RECIPIENT_TYPES")]
    pub recipient_type: String,
}

impl TurkeyIys {
    pub fn new(recipient_type: &str) -> Self {
        TurkeyIys {
            recipient_type: recipient_type.into(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegionalOptions {
    /// Distributed Ledger Technology (DLT) specific parameters required for sending SMS to phone
    /// numbers registered in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub india_dlt: Option<IndiaDlt>,

    /// IYS regulations specific parameters required for sending promotional SMS to phone numbers
    /// registered in Turkey.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub turkey_iys: Option<TurkeyIys>,
}

impl RegionalOptions {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    /// Additional data that can be used for identifying, managing, or monitoring a message.
    /// Data included here will also be automatically included in the message Delivery Report.
    /// The maximum value is 4000 characters and any overhead may be truncated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 4000))]
    pub callback_data: Option<String>,

    /// Sets specific scheduling options to send a message within daily or hourly intervals.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub delivery_time_window: Option<DeliveryTimeWindow>,

    /// An array of destination objects for where messages are being sent. A valid destination is
    /// required.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    #[validate]
    pub destinations: Option<Vec<Destination>>,

    /// Allows for sending a flash SMS to automatically appear on recipient devices without
    /// interaction. Set to true to enable flash SMS, or leave the default value, false to send a
    /// standard SMS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash: Option<bool>,

    /// The sender ID which can be alphanumeric or numeric (e.g., CompanyName). Make sure you don't
    /// exceed character limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 3, max = 15))]
    pub from: Option<String>,

    /// The real-time intermediate delivery report containing GSM error codes, messages status,
    /// pricing, network and country codes, etc., which will be sent on your callback server.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediate_report: Option<bool>,

    /// Sets the language parameters for the message being sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,

    /// Preferred Delivery report content type. Can be `application/json` or `application/xml`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(regex = "CONTENT_TYPES")]
    pub notify_content_type: Option<String>,

    /// The URL on your call back server on to which a delivery report will be sent. The retry
    /// cycle for when your URL becomes unavailable uses the following formula:
    /// 1min + (1min * retryNumber * retryNumber).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(url)]
    pub notify_url: Option<String>,

    /// Region specific parameters, often specified by local laws. Use this if country or region
    /// that you are sending SMS to requires some extra parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub regional: Option<RegionalOptions>,

    /// Date and time when the message is to be sent. Used for scheduled SMS (SMS not sent
    /// immediately, but at the scheduled time). Has the following format:
    /// `yyyy-MM-dd'T'HH:mm:ss.SSSZ`, and can only be scheduled for no later than 180 days in
    /// advance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at: Option<String>,

    /// Content of the message that will be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// The transliteration of your sent message from one script to another. Transliteration is
    /// used to replace characters which are not recognized as part of your defaulted alphabet.
    /// Possible values: TURKISH, GREEK, CYRILLIC, SERBIAN_CYRILLIC, CENTRAL_EUROPEAN, BALTIC
    /// and NON_UNICODE.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(regex = "TRANSLITERATIONS")]
    pub transliteration: Option<String>,

    /// The message validity period in minutes. When the period expires, it will not be allowed for
    /// the message to be sent. Validity period longer than 48h is not supported. Any bigger value
    /// will automatically default back to 2880.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<i64>,
}

impl Message {
    pub fn new(destinations: Vec<Destination>) -> Self {
        Self {
            destinations: Some(destinations),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BinaryData {
    /// Binary content data coding. The default value is (0) for GSM7. Example: (8) for Unicode
    /// data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_coding: Option<i32>,

    /// Indicate special message attributes associated with the SMS. Default value is (0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esm_class: Option<i32>,

    /// Hexadecimal string. This is the representation of your binary data. Two hex digits
    /// represent one byte. They should be separated by the space character (Example: `0f c2 4a bf
    /// 34 13 ba`).
    #[validate(length(min = 1))]
    pub hex: String,
}

impl BinaryData {
    pub fn new(hex: &str) -> Self {
        Self {
            hex: hex.into(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BinaryMessage {
    #[validate]
    pub binary: Option<BinaryData>,

    /// Additional client data that will be sent on the notifyUrl. The maximum value is 4000
    /// characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 4000))]
    pub callback_data: Option<String>,

    /// Sets specific scheduling options to send a message within daily or hourly intervals.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub delivery_time_window: Option<DeliveryTimeWindow>,

    /// An array of destination objects for where messages are being sent. A valid destination is
    /// required.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    #[validate]
    pub destinations: Option<Vec<Destination>>,

    /// Allows for sending a flash SMS to automatically appear on recipient devices without
    /// interaction. Set to true to enable flash SMS, or leave the default value, false to send a
    /// standard SMS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash: Option<bool>,

    /// The sender ID which can be alphanumeric or numeric (e.g., CompanyName). Make sure you don't
    /// exceed character limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 3, max = 15))]
    pub from: Option<String>,

    /// The real-time intermediate delivery report containing GSM error codes, messages status,
    /// pricing, network and country codes, etc., which will be sent on your callback server.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediate_report: Option<bool>,

    /// Preferred Delivery report content type. Can be `application/json` or `application/xml`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(regex = "CONTENT_TYPES")]
    pub notify_content_type: Option<String>,

    /// The URL on your call back server on which the Delivery report will be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(url)]
    pub notify_url: Option<String>,

    /// Region-specific parameters, often imposed by local laws. Use this, if country or region
    /// that you are sending an SMS to requires additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub regional: Option<RegionalOptions>,

    /// Date and time when the message is to be sent. Used for scheduled SMS. Has the following
    /// format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ`, and can only be scheduled for no later than
    /// 180 days in advance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at: Option<String>,

    /// The message validity period in minutes. When the period expires, it will not be allowed for
    /// the message to be sent. Validity period longer than 48h is not supported (in this case,
    /// it will be automatically set to 48h).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<i64>,
}

impl BinaryMessage {
    pub fn new(destinations: Vec<Destination>) -> Self {
        Self {
            destinations: Some(destinations),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendRequestBody {
    /// Unique ID assigned to the request if messaging multiple recipients or sending multiple
    /// messages via a single API request. If not provided, it will be auto-generated and returned
    /// in the API response. Typically, used to fetch delivery reports and message logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,

    /// An array of message objects of a single message or multiple messages sent under one bulk ID.
    #[validate(length(min = 1))]
    #[validate]
    pub messages: Vec<Message>,

    /// Limits the send speed when sending messages in bulk to deliver messages over a longer
    /// period of time. You may wish to use this to allow your systems or agents to handle large
    /// amounts of incoming traffic, e.g., if you are expecting recipients to follow through with a
    /// call-to-action option from a message you sent. Not setting a send speed limit can
    /// overwhelm your resources with incoming traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_speed_limit: Option<SpeedLimit>,

    /// Sets up URL shortening and tracking feature. Not compatible with old tracking feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_options: Option<UrlOptions>,

    /// Sets up tracking parameters to track conversion metrics and type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<Tracking>,
}

impl SendRequestBody {
    pub fn new(messages: Vec<Message>) -> Self {
        Self {
            messages,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendBinaryRequestBody {
    /// The ID which uniquely identifies the request. Bulk ID will be received only when you send a
    /// message to more than one destination address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    #[validate]
    pub messages: Option<Vec<BinaryMessage>>,

    /// Limit the sending speed for message bulks. In some use cases, you might want to reduce
    /// message sending speed if your message call to action involves visiting a website, calling
    /// your contact center or similar recipient activity, in which you can handle a limited amount
    /// of load. This setting helps you to spread the delivery of the messages over a longer
    /// period, allowing your systems or agents to handle incoming traffic in real-time,
    /// resulting in better customer satisfaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_speed_limit: Option<SpeedLimit>,
}

impl SendBinaryRequestBody {
    pub fn new(messages: Vec<BinaryMessage>) -> Self {
        Self {
            messages: Some(messages),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentMessageDetails {
    /// The ID that uniquely identifies the message sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// Indicates whether the message is successfully sent, not sent, delivered, not delivered,
    /// waiting for delivery or any other possible status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,

    /// The message destination address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendResponseBody {
    /// The ID that uniquely identifies the request. Bulk ID will be received only when you send a
    /// message to more than one destination address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,

    /// Array of sent message objects, one object per every message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<SentMessageDetails>>,
}

pub type SendBinaryResponseBody = SendResponseBody;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct GetScheduledQueryParameters {
    #[validate(length(min = 1))]
    pub bulk_id: String,
}

impl GetScheduledQueryParameters {
    pub fn new(bulk_id: &str) -> Self {
        Self {
            bulk_id: bulk_id.into(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScheduledResponseBody {
    pub bulk_id: String,

    pub send_at: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetLogsQueryParameters {
    /// The sender ID which can be alphanumeric or numeric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// Message destination address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    /// Unique ID assigned to the request if messaging multiple recipients or sending multiple
    /// messages via a single API request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,

    /// Unique message ID for which a log is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// Sent message status. Possible values: ACCEPTED, PENDING, UNDELIVERABLE, DELIVERED,
    /// REJECTED, EXPIRED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_status: Option<String>,

    /// The logs will only include messages sent after this date. Use it together with sentUntil
    /// to return a time range or if you want to fetch more than 1000 logs allowed per call. Has
    /// the following format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_since: Option<String>,

    /// The logs will only include messages sent before this date. Use it together with sentBefore
    /// to return a time range or if you want to fetch more than 1000 logs allowed per call. Has
    /// the following format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_until: Option<String>,

    /// Maximum number of messages to include in logs. If not set, the latest 50 records are
    /// returned. Maximum limit value is 1000 and you can only access logs for the last 48h. If
    /// you want to fetch more than 1000 logs allowed per call, use `sentBefore` and `sentUntil` to
    /// retrieve them in pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Mobile Country Code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,

    /// Mobile Network Code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnc: Option<String>,
}

impl GetLogsQueryParameters {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    /// Unique ID assigned to the request if messaging multiple recipients or sending multiple
    /// messages via a single API request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,

    /// Date and time when the Infobip services finished processing the message (i.e. delivered
    /// to the destination, delivered to the destination network, etc.). Has the following
    /// format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ.`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,

    /// Sender ID that can be alphanumeric or numeric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// Mobile country and network codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc_mnc: Option<String>,

    /// Unique message ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,

    /// Date and time when the message was scheduled to be sent. Has the following format:
    /// `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,

    /// The number of parts the message content was split into.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_count: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,

    /// Content of the message being sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// The destination address of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLogsResponseBody {
    /// Collection of logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<Log>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct GetInboundReportsQueryParameters {
    #[validate(range(max = 1000))]
    pub limit: Option<i32>,
}

impl GetInboundReportsQueryParameters {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInboundReportsResponseBody {
    /// The number of messages returned in the `results` array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i32>,

    /// The number of messages that have not been pulled in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_message_count: Option<i32>,

    /// An array of result objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<InboundSmsReport>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboundSmsReport {
    /// Custom callback data sent over the notifyUrl.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,

    /// Content of the message without a keyword (if a keyword was sent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_text: Option<String>,

    /// Sender ID that can be alphanumeric or numeric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// Keyword extracted from the message content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,

    /// Unique message ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// A price object showing currency and a price per each message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,

    /// Indicates when the Infobip platform received the message. Has the following format:
    /// `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<String>,

    /// The number of characters within a message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_count: Option<i32>,

    /// Full content of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// The destination address of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct SendOverQueryParametersQueryParameters {
    /// Username for authentication.
    pub username: String,

    /// Password for authentication.
    pub password: String,

    /// Unique ID assigned to the request if messaging multiple recipients or sending multiple
    /// messages via a single API request.
    pub bulk_id: Option<String>,

    /// The sender ID which can be alphanumeric or numeric (e.g., `CompanyName`).
    pub from: Option<String>,

    /// List of message recipients.
    pub to: Vec<String>,

    /// Content of the message being sent.
    pub text: Option<String>,

    /// Sends a flash SMS if set to true.
    pub flash: Option<bool>,

    /// Conversion of a message text from one script to another.
    pub transliteration: Option<String>,

    /// Code for language character set of a message content.
    pub language_code: Option<String>,

    /// Use a real-time intermediate delivery report that will be sent on your callback server.
    pub intermediate_report: Option<bool>,

    /// The URL on your call back server on to which a delivery report will be sent.
    #[validate(url)]
    pub notify_url: Option<String>,

    /// Preferred delivery report content type, `application/json` or `application/xml`.
    #[validate(regex = "CONTENT_TYPES")]
    pub notify_content_type: Option<String>,

    /// Additional client data to be sent over the notifyUrl.
    pub callback_data: Option<String>,

    /// The message validity period in minutes. When the period expires, it will not be allowed for
    /// the message to be sent. Validity period longer than 48h is not supported. Any bigger value
    /// will automatically default back to 2880.
    pub validity_period: Option<i32>,

    /// Date and time when the message is to be sent. Used for scheduled SMS. Has the following
    /// format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ`. Must be sooner than 180 days from now.
    pub send_at: Option<String>,

    /// Sets the conversion element to be tracked.
    pub track: Option<String>,

    /// The process key which uniquely identifies conversion tracking.
    pub process_key: Option<String>,

    /// Sets a custom conversion type naming convention, e.g. ONE_TIME_PIN, SOCIAL_INVITES, etc.
    pub tracking_type: Option<String>,

    /// The ID of your registered DLT (Distributed Ledger Technology) content template.
    pub india_dlt_content_template_id: Option<String>,

    /// Your DLT (Distributed Ledger Technology) entity id.
    pub india_dlt_principal_entity_id: Option<String>,
}

impl SendOverQueryParametersQueryParameters {
    pub fn new(username: &str, password: &str, to: Vec<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            to,
            ..Default::default()
        }
    }
}

pub type SendOverQueryParametersResponseBody = SendResponseBody;

pub type RescheduleQueryParameters = GetScheduledQueryParameters;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RescheduleRequestBody {
    /// Date and time when the message is to be sent. Used for scheduled SMS (see Scheduled SMS
    /// endpoints for more details). Has the following format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ`, and
    /// can only be scheduled for no later than 180 days in advance.
    #[validate(length(min = 1))]
    pub send_at: String,
}

impl RescheduleRequestBody {
    pub fn new(send_at: &str) -> Self {
        Self {
            send_at: send_at.into(),
        }
    }
}

pub type RescheduleResponseBody = GetScheduledResponseBody;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScheduledStatus {
    Pending,
    Paused,
    Processing,
    Canceled,
    Finished,
    Failed,
}

pub type GetScheduledStatusQueryParameters = GetScheduledQueryParameters;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScheduledStatusResponseBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ScheduledStatus>,
}

pub type UpdateScheduledStatusQueryParameters = RescheduleQueryParameters;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateScheduledStatusRequestBody {
    pub status: ScheduledStatus,
}

impl UpdateScheduledStatusRequestBody {
    pub fn new(status: ScheduledStatus) -> Self {
        Self { status }
    }
}

pub type UpdateScheduledStatusResponseBody = GetScheduledStatusResponseBody;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TfaApplicationConfiguration {
    /// Indicates whether multiple PIN verification is allowed.
    #[serde(
        rename = "allowMultiplePinVerifications",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_multiple_pin_verifications: Option<bool>,

    /// Number of possible PIN attempts.
    #[serde(rename = "pinAttempts", skip_serializing_if = "Option::is_none")]
    pub pin_attempts: Option<i32>,

    /// Validity period of PIN in specified time unit. Required format: `{timeLength}{timeUnit}`. `timeLength` is optional with a default value of 1. `timeUnit` can be set to: `ms`, `s`, `m`, `h` or `d` representing milliseconds, seconds, minutes, hours, and days respectively. Must not exceed one year, although much lower value is recommended.
    #[serde(rename = "pinTimeToLive", skip_serializing_if = "Option::is_none")]
    pub pin_time_to_live: Option<String>,

    /// Overall number of requests over a specified time period for generating a PIN and sending an SMS using a single application. Required format: `{attempts}/{timeLength}{timeUnit}`. `attempts` is mandatory and `timeLength` is optional with a default value of 1. `timeUnit` is one of: `ms`, `s`, `m`, `h` or `d` representing milliseconds, seconds, minutes, hours, and days respectively. Must not exceed one year, although much lower value is recommended.
    #[serde(
        rename = "sendPinPerApplicationLimit",
        skip_serializing_if = "Option::is_none"
    )]
    pub send_pin_per_application_limit: Option<String>,

    /// Number of requests over a specified time period for generating a PIN and sending an SMS to one phone number (MSISDN). Required format: `{attempts}/{timeLength}{timeUnit}`. `attempts` is mandatory and `timeLength` is optional with a default value of 1. `timeUnit` is one of: `ms`, `s`, `m`, `h` or `d` representing milliseconds, seconds, minutes, hours, and days respectively. Must not exceed one year, although much lower value is recommended.
    #[serde(
        rename = "sendPinPerPhoneNumberLimit",
        skip_serializing_if = "Option::is_none"
    )]
    pub send_pin_per_phone_number_limit: Option<String>,

    /// The number of PIN verification requests over a specified time period from one phone number (MSISDN). Required format: `{attempts}/{timeLength}{timeUnit}`. `attempts` is mandatory and `timeLength` is optional with a default value of 1. `timeUnit` is one of: `ms`, `s`, `m`, `h` or `d` representing milliseconds, seconds, minutes, hours, and days respectively. Must not exceed one day, although much lower value is recommended.
    #[serde(rename = "verifyPinLimit", skip_serializing_if = "Option::is_none")]
    pub verify_pin_limit: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TfaApplication {
    /// The ID of the application that represents your service, e.g. 2FA for login, 2FA for changing the password, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,

    /// Created 2FA application configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<TfaApplicationConfiguration>,

    /// Indicates whether the created application is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// 2FA application name.
    #[validate(length(min = 1))]
    pub name: String,
}

pub type GetTfaApplicationsResponseBody = Vec<TfaApplication>;

pub type CreateTfaApplicationRequestBody = TfaApplication;

pub type CreateTfaApplicationResponseBody = TfaApplication;

impl CreateTfaApplicationRequestBody {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

pub type GetTfaApplicationResponseBody = TfaApplication;

pub type UpdateTfaApplicationRequestBody = TfaApplication;

pub type UpdateTfaApplicationResponseBody = TfaApplication;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TfaLanguage {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "pt-pt")]
    PtPt,
    #[serde(rename = "pt-br")]
    PtBr,
    #[serde(rename = "zh-cn")]
    ZhCn,
    #[serde(rename = "zh-tw")]
    ZhTw,
}

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PinType {
    #[default]
    Numeric,
    Alpha,
    Hex,
    Alphanumeric,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TfaRegional {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub india_dlt: Option<IndiaDlt>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TfaMessageTemplate {
    /// The ID of the application that represents your service (e.g. 2FA for login, 2FA for changing the password, etc.) for which the requested message has been created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,

    /// The language code which message is written in used when sending text-to-speech messages. If not defined, it will default to English (`en`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<TfaLanguage>,

    /// The ID of the message template (message body with the PIN placeholder) that is sent to the recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    /// Text of a message that will be sent. Message text must contain `pinPlaceholder`.
    #[validate(length(min = 1))]
    pub message_text: String,

    /// PIN code length.
    #[validate(range(min = 1))]
    pub pin_length: i32,

    /// The PIN code placeholder that will be replaced with a generated PIN code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_placeholder: Option<String>,

    /// The type of PIN code that will be generated and sent as part of 2FA message. You can set PIN type to numeric, alpha, alphanumeric or hex.
    pub pin_type: PinType,

    /// Region-specific parameters, often imposed by local laws. Use this, if country or region that you are sending a message to requires additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub regional: Option<TfaRegional>,

    /// In case PIN message is sent by Voice, DTMF code will enable replaying the message.
    #[serde(rename = "repeatDTMF", skip_serializing_if = "Option::is_none")]
    pub repeat_dtmf: Option<String>,

    /// The name that will appear as the sender of the 2FA message (Example: CompanyName).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,

    /// In case PIN message is sent by Voice, the speed of speech can be set for the message. Supported range is from `0.5` to `2`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_rate: Option<f64>,
}

impl TfaMessageTemplate {
    pub fn new(message_text: &str, pin_type: PinType, pin_length: i32) -> Self {
        Self {
            message_text: message_text.into(),
            pin_length,
            pin_type,
            ..Default::default()
        }
    }
}

pub type GetTfaMessageTemplatesResponseBody = Vec<TfaMessageTemplate>;

pub type CreateTfaMessageTemplateRequestBody = TfaMessageTemplate;

pub type CreateTfaMessageTemplateResponseBody = TfaMessageTemplate;

pub type GetTfaMessageTemplateResponseBody = TfaMessageTemplate;

pub type UpdateTfaMessageTemplateRequestBody = TfaMessageTemplate;

pub type UpdateTfaMessageTemplateResponseBody = TfaMessageTemplate;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendPinOverSmsQueryParameters {
    pub nc_needed: Option<bool>,
}

impl SendPinOverSmsQueryParameters {
    pub fn new() -> Self {
        Self { nc_needed: None }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendPinOverSmsRequestBody {
    /// The ID of the application that represents your service, e.g. 2FA for login, 2FA for changing the password, etc.
    #[validate(length(min = 1))]
    pub application_id: String,

    /// Use this parameter if you wish to override the sender ID from the [created](#channels/sms/create-2fa-message-template) message template parameter `senderId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// The ID of the message template (message body with the PIN placeholder) that is sent to the recipient.
    #[validate(length(min = 1))]
    pub message_id: String,

    /// Key value pairs that will be replaced during message sending. Placeholder keys should NOT contain curly brackets and should NOT contain a `pin` placeholder. Valid example: `\"placeholders\":{\"firstName\":\"John\"}`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholders: Option<HashMap<String, String>>,

    /// Phone number to which the 2FA message will be sent. Example: 41793026727.
    #[validate(length(min = 1))]
    pub to: String,
}

impl SendPinOverSmsRequestBody {
    pub fn new(application_id: &str, message_id: &str, to: &str) -> Self {
        Self {
            application_id: application_id.into(),
            message_id: message_id.into(),
            to: to.into(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendPinResponseBody {
    /// Call status, e.g. `PENDING_ACCEPTED`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_status: Option<String>,

    /// Status of sent [Number Lookup](https://www.infobip.com/docs/number-lookup). Number Lookup status can have one of the following values: `NC_DESTINATION_UNKNOWN`, `NC_DESTINATION_REACHABLE`, `NC_DESTINATION_NOT_REACHABLE`, `NC_NOT_CONFIGURED`. Contact your Account Manager, if you get the `NC_NOT_CONFIGURED` status. SMS will not be sent only if Number Lookup status is `NC_NOT_REACHABLE`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nc_status: Option<String>,

    /// Sent PIN code ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_id: Option<String>,

    /// Sent SMS status. Can have one of the following values: `MESSAGE_SENT`, `MESSAGE_NOT_SENT`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_status: Option<String>,

    /// Phone number to which the 2FA message will be sent. Example: `41793026727`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

pub type SendPinOverSmsResponseBody = SendPinResponseBody;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResendPinRequestBody {
    /// Key value pairs that will be replaced during message sending. Placeholder keys should NOT contain curly brackets and should NOT contain a pin placeholder. Valid example: "placeholders":{"firstName":"John"}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholders: Option<HashMap<String, String>>,
}

impl ResendPinRequestBody {
    pub fn new() -> Self {
        Self { placeholders: None }
    }
}

pub type ResendPinOverSmsRequestBody = ResendPinRequestBody;

pub type ResendPinOverSmsResponseBody = SendPinResponseBody;

pub type SendPinOverVoiceRequestBody = SendPinOverSmsRequestBody;

pub type SendPinOverVoiceResponseBody = SendPinResponseBody;

pub type ResendPinOverVoiceRequestBody = ResendPinRequestBody;

pub type ResendPinOverVoiceResponseBody = SendPinResponseBody;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct VerifyPhoneNumberRequestBody {
    /// ID of the pin code that has to be verified.
    #[validate(length(min = 1))]
    pub pin: String,
}

impl VerifyPhoneNumberRequestBody {
    pub fn new(pin: &str) -> Self {
        Self { pin: pin.into() }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VerifyPhoneNumberResponseBody {
    /// Number of remaining PIN attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts_remaining: Option<i32>,
    /// Phone number (`MSISDN`) to which the 2FA message was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msisdn: Option<String>,
    /// Indicates whether an error has occurred during PIN verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_error: Option<String>,
    /// Sent PIN code ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_id: Option<String>,
    /// Indicates whether the phone number (`MSISDN`) was successfully verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct GetTfaVerificationStatusQueryParameters {
    /// Filter by msisdn (phone number) for which verification status is checked.
    pub msisdn: String,

    /// Filter by verified (true or false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,

    /// Filter by message sent status (true or false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent: Option<bool>,
}

impl GetTfaVerificationStatusQueryParameters {
    pub fn new(msisdn: &str) -> Self {
        Self {
            msisdn: msisdn.into(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TfaVerification {
    /// Phone number (MSISDN) for which verification status is checked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msisdn: Option<String>,

    /// Sent UNIX timestamp (in millis), if the phone number (MSISDN) is verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<i64>,

    /// Indicates if the phone number (MSISDN) is already verified for 2FA application with given ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,

    /// Verification UNIX timestamp (in millis), if the phone number (MSISDN) is verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetTfaVerificationStatusResponseBody {
    /// Collection of verifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifications: Option<Vec<TfaVerification>>,
}
