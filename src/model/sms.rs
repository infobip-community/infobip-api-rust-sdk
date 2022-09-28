//! Models for calling SMS endpoints.

use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    static ref LANGUAGE_CODES: Regex = Regex::new(r"^(TR|ES|PT|AUTODETECT)$").unwrap();
    static ref TRANSLITERATIONS: Regex = Regex::new(
        r"^(TURKISH|GREEK|CYRILLIC|SERBIAN_CYRILLIC|CENTRAL_EUROPEAN|BALTIC|NON_UNICODE)$"
    )
    .unwrap();
    static ref CONTENT_TYPES: Regex = Regex::new(r"^(application/json|application/xml)$").unwrap();
    static ref TURKEY_RECIPIENT_TYPES: Regex = Regex::new(r"^(TACIR|BIREYSEL)$").unwrap();
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new(text: String) -> PreviewRequestBody {
        PreviewRequestBody {
            language_code: None,
            text,
            transliteration: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    /// Language code for the correct character set.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(regex = "LANGUAGE_CODES")]
    pub language_code: Option<String>,
}

impl Language {
    pub fn new(language_code: String) -> Language {
        Language {
            language_code: Some(language_code),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewLanguageConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,

    /// Conversion of a message text from one script to another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transliteration: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewResponseBody {
    /// Text supplied in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_text: Option<String>,

    /// Previews of applying different configurations to the original text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previews: Option<Vec<Preview>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new() -> GetDeliveryReportsQueryParameters {
        GetDeliveryReportsQueryParameters {
            bulk_id: None,
            message_id: None,
            limit: None,
        }
    }
}

impl Default for GetDeliveryReportsQueryParameters {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    /// The currency in which the price is expressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Price per one SMS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_per_message: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryReportsResponseBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<Report>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn new() -> Tracking {
        Tracking {
            base_url: None,
            process_key: None,
            track: None,
            tracking_type: None,
        }
    }
}

impl Default for Tracking {
    fn default() -> Self {
        Tracking::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TimeUnit {
    MINUTE,
    HOUR,
    DAY,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new(hour: i32, minute: i32) -> DeliveryTime {
        DeliveryTime { hour, minute }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn new(amount: i32) -> SpeedLimit {
        SpeedLimit {
            amount,
            time_unit: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeliveryDay {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new(days: Vec<DeliveryDay>) -> DeliveryTimeWindow {
        DeliveryTimeWindow {
            days,
            from: None,
            to: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new(to: String) -> Destination {
        Destination {
            message_id: None,
            to,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IndiaDlt {
    /// Id of your registered DTL content template that matches this message's text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_template_id: Option<String>,

    /// Your assigned DTL principal entity id.
    #[validate(length(min = 1))]
    pub principal_entity_id: String,
}

impl IndiaDlt {
    pub fn new(principal_entity_id: String) -> IndiaDlt {
        IndiaDlt {
            content_template_id: None,
            principal_entity_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new(recipient_type: String) -> TurkeyIys {
        TurkeyIys {
            brand_code: None,
            recipient_type,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new() -> RegionalOptions {
        RegionalOptions {
            india_dlt: None,
            turkey_iys: None,
        }
    }
}

impl Default for RegionalOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
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
    pub fn new(destinations: Vec<Destination>) -> Message {
        Message {
            callback_data: None,
            delivery_time_window: None,
            destinations: Some(destinations),
            flash: None,
            from: None,
            intermediate_report: None,
            language: None,
            notify_content_type: None,
            notify_url: None,
            regional: None,
            send_at: None,
            text: None,
            transliteration: None,
            validity_period: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new(hex: String) -> BinaryData {
        BinaryData {
            data_coding: None,
            esm_class: None,
            hex,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
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
    pub fn new(destinations: Vec<Destination>) -> BinaryMessage {
        BinaryMessage {
            binary: None,
            callback_data: None,
            delivery_time_window: None,
            destinations: Some(destinations),
            flash: None,
            from: None,
            intermediate_report: None,
            notify_content_type: None,
            notify_url: None,
            regional: None,
            send_at: None,
            validity_period: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
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

    /// Sets up tracking parameters to track conversion metrics and type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<Tracking>,
}

impl SendRequestBody {
    pub fn new(messages: Vec<Message>) -> SendRequestBody {
        SendRequestBody {
            messages,
            sending_speed_limit: None,
            bulk_id: None,
            tracking: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
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
    pub fn new(messages: Vec<BinaryMessage>) -> SendBinaryRequestBody {
        SendBinaryRequestBody {
            messages: Some(messages),
            sending_speed_limit: None,
            bulk_id: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct GetScheduledQueryParameters {
    #[validate(length(min = 1))]
    pub bulk_id: String,
}

impl GetScheduledQueryParameters {
    pub fn new(bulk_id: String) -> GetScheduledQueryParameters {
        GetScheduledQueryParameters { bulk_id }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScheduledResponseBody {
    pub bulk_id: String,

    pub send_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new() -> GetLogsQueryParameters {
        GetLogsQueryParameters {
            from: None,
            to: None,
            bulk_id: None,
            message_id: None,
            general_status: None,
            sent_since: None,
            sent_until: None,
            limit: None,
            mcc: None,
            mnc: None,
        }
    }
}

impl Default for GetLogsQueryParameters {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLogsResponseBody {
    /// Collection of logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<Log>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct GetInboundReportsQueryParameters {
    #[validate(range(max = 1000))]
    pub limit: Option<i32>,
}

impl GetInboundReportsQueryParameters {
    pub fn new() -> GetInboundReportsQueryParameters {
        GetInboundReportsQueryParameters { limit: None }
    }
}

impl Default for GetInboundReportsQueryParameters {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new(
        username: String,
        password: String,
        to: Vec<String>,
    ) -> SendOverQueryParametersQueryParameters {
        SendOverQueryParametersQueryParameters {
            username,
            password,
            bulk_id: None,
            from: None,
            to,
            text: None,
            flash: None,
            transliteration: None,
            language_code: None,
            intermediate_report: None,
            notify_url: None,
            notify_content_type: None,
            callback_data: None,
            validity_period: None,
            send_at: None,
            track: None,
            process_key: None,
            tracking_type: None,
            india_dlt_content_template_id: None,
            india_dlt_principal_entity_id: None,
        }
    }
}

pub type SendOverQueryParametersResponseBody = SendResponseBody;

pub type RescheduleQueryParameters = GetScheduledQueryParameters;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RescheduleRequestBody {
    /// Date and time when the message is to be sent. Used for scheduled SMS (see Scheduled SMS
    /// endpoints for more details). Has the following format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ`, and
    /// can only be scheduled for no later than 180 days in advance.
    #[validate(length(min = 1))]
    pub send_at: String,
}

impl RescheduleRequestBody {
    pub fn new(send_at: String) -> RescheduleRequestBody {
        RescheduleRequestBody { send_at }
    }
}

pub type RescheduleResponseBody = GetScheduledResponseBody;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ScheduledStatus {
    PENDING,
    PAUSED,
    PROCESSING,
    CANCELED,
    FINISHED,
    FAILED,
}

pub type GetScheduledStatusQueryParameters = GetScheduledQueryParameters;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn new(status: ScheduledStatus) -> UpdateScheduledStatusRequestBody {
        UpdateScheduledStatusRequestBody { status }
    }
}

pub type UpdateScheduledStatusResponseBody = GetScheduledStatusResponseBody;
