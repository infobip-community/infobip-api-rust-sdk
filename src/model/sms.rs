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
pub struct SmsPreview {
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
    pub previews: Option<Vec<SmsPreview>>,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsTracking {
    /// Custom base url used for shortening links from SMS text in `URL` Conversion rate tracking use-case.
    #[serde(rename = "baseUrl", skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// Key that uniquely identifies Conversion tracking process.
    #[serde(rename = "processKey", skip_serializing_if = "Option::is_none")]
    pub process_key: Option<String>,
    /// Indicates if the message has to be tracked for Conversion rates. Possible values: `SMS` and `URL`
    #[serde(rename = "track", skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
    /// User-defined type of the Conversion tracking process or flow type or message type, etc. Example: `ONE_TIME_PIN or SOCIAL_INVITES`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SmsTimeUnit {
    #[serde(rename = "MINUTE")]
    MINUTE,
    #[serde(rename = "HOUR")]
    HOUR,
    #[serde(rename = "DAY")]
    DAY,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsDeliveryTime {
    /// Hour when the time window opens when used in from property or closes when used into the property.
    pub hour: i32,
    /// Minute when the time window opens when used in from property or closes when used into the property.
    pub minute: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsSpeedLimit {
    /// The number of messages to send per time unit. By default, Infobip sends your messages as fast as the infrastructure allows. Use this parameter to reduce the traffic if you find the default sending speed too fast for your use case. Note that boosting this parameter will not result in faster sending speeds beyond infrastructure capabilities.
    pub amount: i32,
    /// The time unit in which the defined message amount will be sent. The default value is `MINUTE`.
    #[serde(rename = "timeUnit", skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<SmsTimeUnit>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SmsDeliveryDay {
    #[serde(rename = "MONDAY")]
    MONDAY,
    #[serde(rename = "TUESDAY")]
    TUESDAY,
    #[serde(rename = "WEDNESDAY")]
    WEDNESDAY,
    #[serde(rename = "THURSDAY")]
    THURSDAY,
    #[serde(rename = "FRIDAY")]
    FRIDAY,
    #[serde(rename = "SATURDAY")]
    SATURDAY,
    #[serde(rename = "SUNDAY")]
    SUNDAY,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsDeliveryTimeWindow {
    /// Days which are included in the delivery time window. Values are: `MONDAY`, `TUESDAY`, `WEDNESDAY`, `THURSDAY`, `FRIDAY`, `SATURDAY`, `SUNDAY`. At least one day must be stated.
    pub days: Vec<SmsDeliveryDay>,
    /// Exact time of day in which the sending can start. Consists of hour and minute properties, both mandatory. Time is expressed in the UTC time zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<SmsDeliveryTime>,
    /// Exact time of day in which the sending will end. Consists of an hour and minute properties, both mandatory. Time is expressed in the UTC time zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<SmsDeliveryTime>,
}

#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct Destination {
    /// The ID that uniquely identifies the message sent.
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub message_id: Option<String>,
    /// Message destination address. Addresses must be in international format (Example: `41793026727`).
    #[serde(rename = "to")]
    pub to: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndiaDltOptions {
    /// Id of your registered DTL content template that matches this message's text.
    #[serde(rename = "contentTemplateId", skip_serializing_if = "Option::is_none")]
    pub content_template_id: Option<String>,
    /// Your assigned DTL principal entity id.
    #[serde(rename = "principalEntityId")]
    pub principal_entity_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionalOptions {
    /// Distributed Ledger Technology (DLT) specific parameters required for sending SMS to phone numbers registered in India.
    #[serde(rename = "indiaDlt", skip_serializing_if = "Option::is_none")]
    pub india_dlt: Option<IndiaDltOptions>,
}

#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[builder(setter(strip_option))]
pub struct SmsMessage {
    /// Additional client's data that will be sent on the notifyUrl. The maximum value is 200 characters.
    #[serde(rename = "callbackData", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub callback_data: Option<String>,
    /// Scheduling object that allows setting up detailed time windows in which the message can be sent. Consists of `from`, `to` and `days` properties. `Days` property is mandatory. `From` and `to` properties should be either both included, to allow finer time window granulation or both omitted, to include whole days in the delivery time window.
    #[serde(rename = "deliveryTimeWindow", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub delivery_time_window: Option<SmsDeliveryTimeWindow>,
    #[serde(rename = "destinations", skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<Destination>>,
    /// Can be `true` or `false`. If the value is set to `true`, a flash SMS will be sent. Otherwise, a normal SMS will be sent. The default value is `false`.
    #[serde(rename = "flash", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub flash: Option<bool>,
    /// Represents a sender ID which can be alphanumeric or numeric. Alphanumeric sender ID length should be between 3 and 11 characters (Example: `CompanyName`). Numeric sender ID length should be between 3 and 14 characters.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub from: Option<String>,
    /// The real-time Intermediate delivery report that will be sent on your callback server. Can be `true` or `false`.
    #[serde(rename = "intermediateReport", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub intermediate_report: Option<bool>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub language: Option<Language>,
    /// Preferred Delivery report content type. Can be `application/json` or `application/xml`.
    #[serde(rename = "notifyContentType", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub notify_content_type: Option<String>,
    /// The URL on your call back server on which the Delivery report will be sent.
    #[serde(rename = "notifyUrl", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub notify_url: Option<String>,
    /// Region specific parameters, often specified by local laws. Use this if country or region that you are sending SMS to requires some extra parameters.
    #[serde(rename = "regional", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub regional: Option<RegionalOptions>,
    /// Date and time when the message is to be sent. Used for scheduled SMS (SMS not sent immediately, but at the scheduled time). Has the following format: `yyyy-MM-dd'T'HH:mm:ss.SSSZ`.
    #[serde(rename = "sendAt", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub send_at: Option<String>,
    /// Text of the message that will be sent.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub text: Option<String>,
    /// Conversion of a message text from one script to another. Possible values: `TURKISH`, `GREEK`, `CYRILLIC`, `SERBIAN_CYRILLIC`, `CENTRAL_EUROPEAN`, `BALTIC` and `NON_UNICODE`.
    #[serde(rename = "transliteration", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub transliteration: Option<String>,
    /// The message validity period in minutes. When the period expires, it will not be allowed for the message to be sent. Validity period longer than 48h is not supported (in this case, it will be automatically set to 48h).
    #[serde(rename = "validityPeriod", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub validity_period: Option<i64>,
}

#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[builder(setter(strip_option))]
pub struct SendSmsRequestBody {
    /// The ID which uniquely identifies the request. Bulk ID will be received only when you send a message to more than one destination address.
    #[serde(rename = "bulkId", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub bulk_id: Option<String>,
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<SmsMessage>>,
    /// Limit the sending speed for message bulks. In some use cases, you might want to reduce message sending speed if your message call to action involves visiting a website, calling your contact center or similar recipient activity, in which you can handle a limited amount of load. This setting helps you to spread the delivery of the messages over a longer period, allowing your systems or agents to handle incoming traffic in real-time, resulting in better customer satisfaction.
    #[serde(rename = "sendingSpeedLimit", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub sending_speed_limit: Option<SmsSpeedLimit>,
    #[serde(rename = "tracking", skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub tracking: Option<SmsTracking>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Status {
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
pub struct SentMessageDetails {
    /// The ID that uniquely identifies the message sent.
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Indicates whether the message is successfully sent, not sent, delivered, not delivered, waiting for delivery or any other possible status.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The message destination address.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendSmsResponseBody {
    /// The ID that uniquely identifies the request. Bulk ID will be received only when you send a message to more than one destination address.
    #[serde(rename = "bulkId", skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,
    /// Array of sent message objects, one object per every message.
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<SentMessageDetails>>,
}
