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
    /// URL of a document sent in a WhatsApp message. Must be a valid URL starting with `https://`
    /// or `http://`. Maximum document size is 100MB.
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
pub struct ImageContent {
    /// URL of an image sent in a WhatsApp message. Must be a valid URL starting with `https://`
    /// or `http://`. Supported image types are `JPG`, `JPEG`, `PNG`. Maximum image size is 5MB.
    #[validate(url)]
    pub media_url: String,

    /// Caption of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 3000))]
    pub caption: Option<String>,
}

impl ImageContent {
    pub fn new(media_url: String) -> Self {
        ImageContent {
            media_url,
            caption: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AudioContent {
    /// URL of an audio sent in a WhatsApp message. Must be a valid URL starting with `https://`
    /// or `http://`. Supported audio types are `AAC`, `AMR`, `MP3`, `MP4`, `OPUS`. Maximum audio
    /// size is 16MB.
    #[validate(url)]
    pub media_url: String,
}

impl AudioContent {
    pub fn new(media_url: String) -> Self {
        AudioContent { media_url }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VideoContent {
    /// URL of a video sent in a WhatsApp message. Must be a valid URL starting with `https://` or
    /// `http://`. Supported video types are `MP4`, `3GPP`. Maximum video size is 16MB.
    #[validate(url)]
    pub media_url: String,
}

impl VideoContent {
    pub fn new(media_url: String) -> Self {
        VideoContent { media_url }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StickerContent {
    /// URL of a sticker sent in a WhatsApp message. Must be a valid URL starting with `https://`
    /// or `http://`. Supported sticker type is `WebP`. Sticker file should be 512x512 pixels.
    /// Maximum sticker size is 100KB.
    #[validate(url)]
    pub media_url: String,
}

impl StickerContent {
    pub fn new(media_url: String) -> Self {
        StickerContent { media_url }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LocationContent {
    /// Latitude of a location sent in the WhatsApp message.
    #[validate(range(min = -90.0, max = 90.0))]
    pub latitude: f64,

    /// Longitude of a location sent in the WhatsApp message.
    #[validate(range(min = -180.0, max = 180.0))]
    pub longitude: f64,

    /// Location name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 1000))]
    pub name: Option<String>,

    /// Location address.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 1000))]
    pub address: Option<String>,
}

impl LocationContent {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        LocationContent {
            latitude,
            longitude,
            name: None,
            address: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AddressType {
    HOME,
    WORK,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContactAddress {
    /// Street name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,

    /// City name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// State name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Zip code value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,

    /// Country name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Country code value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,

    /// Type of the address. Can be `HOME` or `WORK`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub address_type: Option<AddressType>,
}

impl ContactAddress {
    pub fn new() -> Self {
        ContactAddress {
            street: None,
            city: None,
            state: None,
            zip: None,
            country: None,
            country_code: None,
            address_type: None,
        }
    }
}

pub type EmailType = AddressType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContactName {
    /// Contact's first name.
    #[validate(length(min = 1))]
    pub first_name: String,

    /// Contact's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// Contact's middle name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,

    /// Contact's name prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_suffix: Option<String>,

    /// Contact's name prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,

    /// Contact's full name as it normally appears.
    #[validate(length(min = 1))]
    pub formatted_name: String,
}

impl ContactName {
    pub fn new(first_name: String, formatted_name: String) -> Self {
        ContactName {
            first_name,
            last_name: None,
            middle_name: None,
            name_suffix: None,
            name_prefix: None,
            formatted_name,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContactOrganization {
    /// Company name.
    pub company: Option<String>,

    /// Department name.
    pub department: Option<String>,

    /// Title value
    pub title: Option<String>,
}

impl ContactOrganization {
    pub fn new() -> Self {
        ContactOrganization {
            company: None,
            department: None,
            title: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PhoneType {
    CELL,
    MAIN,
    IPHONE,
    HOME,
    WORK,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContactPhone {
    /// Contact's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Type of the phone number. Can be `CELL`, `MAIN`, `IPHONE`, `HOME` or `WORK`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub phone_type: Option<PhoneType>,

    /// Contact's WhatsApp ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wa_id: Option<String>,
}

impl ContactPhone {
    pub fn new() -> Self {
        ContactPhone {
            phone: None,
            phone_type: None,
            wa_id: None,
        }
    }
}

pub type UrlType = AddressType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContactUrl {
    /// Contact's url.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(url)]
    pub url: Option<String>,

    /// Type of the url. Can be `HOME` or `WORK`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub url_type: Option<UrlType>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContactEmail {
    /// Contact's email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Type of the email. Can be HOME or WORK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<EmailType>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    /// Array of addresses information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<ContactAddress>>,

    /// Date of birth in `YYYY-MM-DD` format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,

    /// Array of emails information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<ContactEmail>>,

    /// Contains information about contact's name.
    pub name: ContactName,

    /// Contains information about contact's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org: Option<ContactOrganization>,

    /// Array of phones information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<ContactPhone>>,

    /// Array of urls information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<ContactUrl>>,
}

impl Contact {
    pub fn new(name: ContactName) -> Self {
        Contact {
            addresses: None,
            birthday: None,
            emails: None,
            name,
            org: None,
            phones: None,
            urls: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContactContent {
    /// An array of contacts sent in a WhatsApp message.
    #[validate(length(min = 1))]
    #[validate]
    pub contacts: Vec<Contact>,
}

impl ContactContent {
    pub fn new(contacts: Vec<Contact>) -> Self {
        ContactContent { contacts }
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

pub type SendImageRequestBody = SendContentRequestBody<ImageContent>;

impl SendImageRequestBody {
    pub fn new(from: String, to: String, content: ImageContent) -> Self {
        SendImageRequestBody {
            from,
            to,
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

pub type SendAudioRequestBody = SendContentRequestBody<AudioContent>;

impl SendAudioRequestBody {
    pub fn new(from: String, to: String, content: AudioContent) -> Self {
        SendAudioRequestBody {
            from,
            to,
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

pub type SendVideoRequestBody = SendContentRequestBody<VideoContent>;

impl SendVideoRequestBody {
    pub fn new(from: String, to: String, content: VideoContent) -> Self {
        SendVideoRequestBody {
            from,
            to,
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

pub type SendStickerRequestBody = SendContentRequestBody<StickerContent>;

impl SendStickerRequestBody {
    pub fn new(from: String, to: String, content: StickerContent) -> Self {
        SendStickerRequestBody {
            from,
            to,
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

pub type SendLocationRequestBody = SendContentRequestBody<LocationContent>;

impl SendLocationRequestBody {
    pub fn new(from: String, to: String, content: LocationContent) -> Self {
        SendLocationRequestBody {
            from,
            to,
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

pub type SendContactRequestBody = SendContentRequestBody<ContactContent>;

impl SendContactRequestBody {
    pub fn new(from: String, to: String, content: ContactContent) -> Self {
        SendContactRequestBody {
            from,
            to,
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveBody {
    /// Content of the message body.
    #[validate(length(min = 1, max = 1024))]
    pub text: String,
}

impl InteractiveBody {
    pub fn new(text: String) -> Self {
        InteractiveBody { text }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InteractiveButton {
    #[serde(rename = "REPLY")]
    ReplyButton {
        /// Unique identifier of the button containing no leading nor trailing whitespaces.
        #[serde(rename = "id")]
        id: String,

        /// Unique title of the button. Doesn't allow emojis or markdown.
        #[serde(rename = "title")]
        title: String,
    },
}

impl InteractiveButton {
    pub fn new_reply_button(id: String, title: String) -> Self {
        InteractiveButton::ReplyButton { id, title }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveButtonsAction {
    /// An array of buttons sent in a message. It can have up to three buttons.
    #[validate(length(min = 1, max = 3))]
    pub buttons: Vec<InteractiveButton>,
}

impl InteractiveButtonsAction {
    pub fn new(buttons: Vec<InteractiveButton>) -> Self {
        InteractiveButtonsAction { buttons }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InteractiveButtonsHeader {
    #[serde(rename = "DOCUMENT")]
    DocumentHeader {
        /// URL of a document sent in the header of a message containing one or more interactive
        /// buttons. Must be a valid URL starting with `https://` or `http://`. Supported document
        /// type is `PDF`. Maximum document size is 100MB.
        #[serde(rename = "mediaUrl")]
        media_url: String,

        /// Filename of the document.
        #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
    },

    #[serde(rename = "IMAGE")]
    ImageHeader {
        /// URL of an image sent in the header of a message containing one or more interactive
        /// buttons. Must be a valid URL starting with `https://` or `http://`. Supported image
        /// types are `JPG`, `JPEG`, `PNG`. Maximum image size is 5MB.
        #[serde(rename = "mediaUrl")]
        media_url: String,
    },

    #[serde(rename = "TEXT")]
    TextHeader {
        /// Content of the header used when creating interactive buttons.
        #[serde(rename = "text")]
        text: String,
    },

    #[serde(rename = "VIDEO")]
    VideoHeader {
        /// URL of a video sent in the header of a message containing one or more interactive
        /// buttons. Must be a valid URL starting with `https://` or `http://`. Supported video
        /// types are `MP4`, `3GPP`. Maximum video size is 16MB.
        #[serde(rename = "mediaUrl")]
        media_url: String,
    },
}

impl InteractiveButtonsHeader {
    pub fn new_document_header(media_url: String, filename: Option<String>) -> Self {
        InteractiveButtonsHeader::DocumentHeader {
            media_url,
            filename,
        }
    }

    pub fn new_image_header(media_url: String) -> Self {
        InteractiveButtonsHeader::ImageHeader { media_url }
    }

    pub fn new_text_header(text: String) -> Self {
        InteractiveButtonsHeader::TextHeader { text }
    }

    pub fn new_video_header(media_url: String) -> Self {
        InteractiveButtonsHeader::VideoHeader { media_url }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveFooter {
    /// Content of the message footer.
    #[validate(length(min = 1, max = 60))]
    pub text: String,
}

impl InteractiveFooter {
    pub fn new(text: String) -> Self {
        InteractiveFooter { text }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveButtonsContent {
    /// Body of a message containing one or more interactive elements.
    #[validate]
    pub body: InteractiveBody,

    /// Allows you to specify buttons sent in the message.
    #[validate]
    pub action: InteractiveButtonsAction,

    /// Header of a message containing one or more interactive elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<InteractiveButtonsHeader>,

    /// Footer of a message containing one or more interactive elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub footer: Option<InteractiveFooter>,
}

impl InteractiveButtonsContent {
    pub fn new(body: InteractiveBody, action: InteractiveButtonsAction) -> Self {
        InteractiveButtonsContent {
            body,
            action,
            header: None,
            footer: None,
        }
    }
}

pub type SendInteractiveButtonsRequestBody = SendContentRequestBody<InteractiveButtonsContent>;

impl SendInteractiveButtonsRequestBody {
    pub fn new(from: String, to: String, content: InteractiveButtonsContent) -> Self {
        SendInteractiveButtonsRequestBody {
            from,
            to,
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveRow {
    /// Identifier of the row. It must be unique across all sections.
    #[validate(length(min = 1, max = 200))]
    pub id: String,

    /// Title of the row.
    #[serde(rename = "title")]
    #[validate(length(min = 1, max = 24))]
    pub title: String,

    /// Description of the row.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 72))]
    pub description: Option<String>,
}

impl InteractiveRow {
    pub fn new(id: String, title: String) -> Self {
        InteractiveRow {
            id,
            title,
            description: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveListSection {
    /// Title of the section. Required, if the message has more than one section.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 24))]
    pub title: Option<String>,

    /// An array of rows sent within a section. Section must contain at least one row. Message can have up to ten rows.
    pub rows: Vec<InteractiveRow>,
}

impl InteractiveListSection {
    pub fn new(rows: Vec<InteractiveRow>) -> Self {
        InteractiveListSection { title: None, rows }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveListAction {
    /// Title of the list. Does not allow emojis or markdown.
    #[validate(length(min = 1, max = 20))]
    pub title: String,

    /// Array of sections in the list.
    #[validate(length(min = 1, max = 10))]
    pub sections: Vec<InteractiveListSection>,
}

impl InteractiveListAction {
    pub fn new(title: String, sections: Vec<InteractiveListSection>) -> Self {
        InteractiveListAction { title, sections }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InteractiveListHeader {
    #[serde(rename = "TEXT")]
    TextHeader {
        /// Content of the header used when creating an [interactive list](https://www.infobip.com/docs/whatsapp/message-types#interactive-lists-free-form-messages).
        text: String,
    },
}

impl InteractiveListHeader {
    pub fn new_text_header(text: String) -> Self {
        InteractiveListHeader::TextHeader { text }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveListContent {
    /// Body of a message containing one or more interactive elements.
    #[validate]
    pub body: InteractiveBody,

    /// Allows you to specify the title of the list and its sections. Each section can have a title
    /// and multiple rows to select.
    #[validate]
    pub action: InteractiveListAction,

    /// Header of a message containing one or more interactive elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<InteractiveListHeader>,

    /// Footer of a message containing one or more interactive elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<InteractiveFooter>,
}

impl InteractiveListContent {
    pub fn new(body: InteractiveBody, action: InteractiveListAction) -> Self {
        InteractiveListContent {
            body,
            action,
            header: None,
            footer: None,
        }
    }
}

pub type SendInteractiveListRequestBody = SendContentRequestBody<InteractiveListContent>;

impl SendInteractiveListRequestBody {
    pub fn new(from: String, to: String, content: InteractiveListContent) -> Self {
        SendInteractiveListRequestBody {
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

pub type SendImageResponseBody = SendContentResponseBody;

pub type SendAudioResponseBody = SendContentResponseBody;

pub type SendVideoResponseBody = SendContentResponseBody;

pub type SendStickerResponseBody = SendContentResponseBody;

pub type SendLocationResponseBody = SendContentResponseBody;

pub type SendContactResponseBody = SendContentResponseBody;

pub type SendInteractiveButtonsResponseBody = SendContentResponseBody;

pub type SendInteractiveListResponseBody = SendContentResponseBody;
