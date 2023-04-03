//! Models for calling WhatsApp endpoints.

use serde_derive::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new(text: &str) -> Self {
        TextContent {
            text: text.into(),
            preview_url: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new(media_url: &str) -> Self {
        DocumentContent {
            media_url: media_url.into(),
            caption: None,
            filename: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new(media_url: &str) -> Self {
        ImageContent {
            media_url: media_url.into(),
            caption: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AudioContent {
    /// URL of an audio sent in a WhatsApp message. Must be a valid URL starting with `https://`
    /// or `http://`. Supported audio types are `AAC`, `AMR`, `MP3`, `MP4`, `OPUS`. Maximum audio
    /// size is 16MB.
    #[validate(url)]
    pub media_url: String,
}

impl AudioContent {
    pub fn new(media_url: &str) -> Self {
        AudioContent {
            media_url: media_url.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VideoContent {
    /// URL of a video sent in a WhatsApp message. Must be a valid URL starting with `https://` or
    /// `http://`. Supported video types are `MP4`, `3GPP`. Maximum video size is 16MB.
    #[validate(url)]
    pub media_url: String,

    /// Caption of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 0, max = 3000))]
    pub caption: Option<String>,
}

impl VideoContent {
    pub fn new(media_url: &str) -> Self {
        VideoContent {
            media_url: media_url.into(),
            caption: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StickerContent {
    /// URL of a sticker sent in a WhatsApp message. Must be a valid URL starting with `https://`
    /// or `http://`. Supported sticker type is `WebP`. Sticker file should be 512x512 pixels.
    /// Maximum sticker size is 100KB.
    #[validate(url)]
    pub media_url: String,
}

impl StickerContent {
    pub fn new(media_url: &str) -> Self {
        StickerContent {
            media_url: media_url.into(),
        }
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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

impl Default for ContactAddress {
    fn default() -> Self {
        Self::new()
    }
}

pub type EmailType = AddressType;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new(first_name: &str, formatted_name: &str) -> Self {
        ContactName {
            first_name: first_name.into(),
            last_name: None,
            middle_name: None,
            name_suffix: None,
            name_prefix: None,
            formatted_name: formatted_name.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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

impl Default for ContactOrganization {
    fn default() -> Self {
        Self::new()
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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

impl Default for ContactPhone {
    fn default() -> Self {
        Self::new()
    }
}

pub type UrlType = AddressType;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContactEmail {
    /// Contact's email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Type of the email. Can be HOME or WORK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<EmailType>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new(from: &str, to: &str, content: TextContent) -> Self {
        SendTextRequestBody {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

pub type SendDocumentRequestBody = SendContentRequestBody<DocumentContent>;

impl SendDocumentRequestBody {
    pub fn new(from: &str, to: &str, content: DocumentContent) -> Self {
        SendDocumentRequestBody {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

pub type SendImageRequestBody = SendContentRequestBody<ImageContent>;

impl SendImageRequestBody {
    pub fn new(from: &str, to: &str, content: ImageContent) -> Self {
        SendImageRequestBody {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

pub type SendAudioRequestBody = SendContentRequestBody<AudioContent>;

impl SendAudioRequestBody {
    pub fn new(from: &str, to: &str, content: AudioContent) -> Self {
        SendAudioRequestBody {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

pub type SendVideoRequestBody = SendContentRequestBody<VideoContent>;

impl SendVideoRequestBody {
    pub fn new(from: &str, to: &str, content: VideoContent) -> Self {
        SendVideoRequestBody {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

pub type SendStickerRequestBody = SendContentRequestBody<StickerContent>;

impl SendStickerRequestBody {
    pub fn new(from: &str, to: &str, content: StickerContent) -> Self {
        SendStickerRequestBody {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

pub type SendLocationRequestBody = SendContentRequestBody<LocationContent>;

impl SendLocationRequestBody {
    pub fn new(from: &str, to: &str, content: LocationContent) -> Self {
        SendLocationRequestBody {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

pub type SendContactRequestBody = SendContentRequestBody<ContactContent>;

impl SendContactRequestBody {
    pub fn new(from: &str, to: &str, content: ContactContent) -> Self {
        SendContactRequestBody {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveBody {
    /// Content of the message body.
    #[validate(length(min = 1, max = 1024))]
    pub text: String,
}

impl InteractiveBody {
    pub fn new(text: &str) -> Self {
        InteractiveBody { text: text.into() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn new_reply_button(id: &str, title: &str) -> Self {
        InteractiveButton::ReplyButton {
            id: id.into(),
            title: title.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn new_document_header(media_url: &str, filename: Option<String>) -> Self {
        InteractiveButtonsHeader::DocumentHeader {
            media_url: media_url.into(),
            filename,
        }
    }

    pub fn new_image_header(media_url: &str) -> Self {
        InteractiveButtonsHeader::ImageHeader {
            media_url: media_url.into(),
        }
    }

    pub fn new_text_header(text: &str) -> Self {
        InteractiveButtonsHeader::TextHeader { text: text.into() }
    }

    pub fn new_video_header(media_url: &str) -> Self {
        InteractiveButtonsHeader::VideoHeader {
            media_url: media_url.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveFooter {
    /// Content of the message footer.
    #[validate(length(min = 1, max = 60))]
    pub text: String,
}

impl InteractiveFooter {
    pub fn new(text: &str) -> Self {
        InteractiveFooter { text: text.into() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new(from: &str, to: &str, content: InteractiveButtonsContent) -> Self {
        SendInteractiveButtonsRequestBody {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    pub fn new(id: &str, title: &str) -> Self {
        InteractiveRow {
            id: id.into(),
            title: title.into(),
            description: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveListSection {
    /// Title of the section. Required, if the message has more than one section.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 24))]
    pub title: Option<String>,

    /// An array of rows sent within a section. Section must contain at least one row. Message can have up to ten rows.
    #[validate(length(min = 1))]
    #[validate]
    pub rows: Vec<InteractiveRow>,
}

impl InteractiveListSection {
    pub fn new(rows: Vec<InteractiveRow>) -> Self {
        InteractiveListSection { title: None, rows }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveListAction {
    /// Title of the list. Does not allow emojis or markdown.
    #[validate(length(min = 1, max = 20))]
    pub title: String,

    /// Array of sections in the list.
    #[validate(length(min = 1, max = 10))]
    #[validate]
    pub sections: Vec<InteractiveListSection>,
}

impl InteractiveListAction {
    pub fn new(title: &str, sections: Vec<InteractiveListSection>) -> Self {
        InteractiveListAction {
            title: title.into(),
            sections,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InteractiveListHeader {
    #[serde(rename = "TEXT")]
    TextHeader {
        /// Content of the header used when creating an [interactive list](https://www.infobip.com/docs/whatsapp/message-types#interactive-lists-free-form-messages).
        text: String,
    },
}

impl InteractiveListHeader {
    pub fn new_text_header(text: &str) -> Self {
        InteractiveListHeader::TextHeader { text: text.into() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
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
    #[validate]
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
    pub fn new(from: &str, to: &str, content: InteractiveListContent) -> Self {
        SendInteractiveListRequestBody {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveProductAction {
    /// The ID that uniquely identifies the catalog registered with Meta and connected to the
    /// WhatsApp Business Account the sender belongs to.
    #[validate(length(min = 1))]
    pub catalog_id: String,

    /// Product-unique identifier, as defined in catalog.
    #[validate(length(min = 1))]
    pub product_retailer_id: String,
}

impl InteractiveProductAction {
    pub fn new(catalog_id: &str, product_retailer_id: &str) -> Self {
        InteractiveProductAction {
            catalog_id: catalog_id.into(),
            product_retailer_id: product_retailer_id.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveProductContent {
    /// Allows you to specify catalog and product details sent in the product message.
    #[validate]
    pub action: InteractiveProductAction,

    /// Body of a message containing one or more interactive elements.
    #[validate]
    pub body: Option<InteractiveBody>,

    /// Footer of a message containing one or more interactive elements.
    #[validate]
    pub footer: Option<InteractiveFooter>,
}

impl InteractiveProductContent {
    pub fn new(action: InteractiveProductAction) -> Self {
        InteractiveProductContent {
            action,
            body: None,
            footer: None,
        }
    }
}

pub type SendInteractiveProductRequestBody = SendContentRequestBody<InteractiveProductContent>;

impl SendInteractiveProductRequestBody {
    pub fn new(from: &str, to: &str, content: InteractiveProductContent) -> Self {
        SendInteractiveProductRequestBody {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InteractiveMultiproductHeader {
    #[serde(rename = "TEXT")]
    TextHeader {
        /// Content of the multi-product message header.
        text: String,
    },
}

impl InteractiveMultiproductHeader {
    pub fn new_text_header(text: &str) -> Self {
        InteractiveMultiproductHeader::TextHeader { text: text.into() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveMultiproductSection {
    /// Title of the section. Required, if the message has more than one section.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 24))]
    pub title: Option<String>,

    /// An array of product-unique identifiers as defined in the catalog. If product retailer ID
    /// doesn't exist in your catalog, the product won't be displayed.
    pub product_retailer_ids: Vec<String>,
}

impl InteractiveMultiproductSection {
    pub fn new(product_retailer_ids: Vec<String>) -> Self {
        InteractiveMultiproductSection {
            title: None,
            product_retailer_ids,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveMultiproductAction {
    /// The ID that uniquely identifies the catalog registered with Meta and connected to the
    /// WhatsApp Business Account the sender belongs to.
    #[validate(length(min = 1))]
    pub catalog_id: String,

    /// An array of multi-product sections.
    #[validate(length(min = 1, max = 10))]
    #[validate]
    pub sections: Vec<InteractiveMultiproductSection>,
}

impl InteractiveMultiproductAction {
    pub fn new(catalog_id: &str, sections: Vec<InteractiveMultiproductSection>) -> Self {
        InteractiveMultiproductAction {
            catalog_id: catalog_id.into(),
            sections,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveMultiproductContent {
    /// Header of a message containing one or more interactive elements.
    pub header: InteractiveMultiproductHeader,

    /// Body of a message containing one or more interactive elements.
    #[validate]
    pub body: InteractiveBody,

    /// Allows you to specify catalog and product details sent in the multi-product message.
    #[validate]
    pub action: InteractiveMultiproductAction,

    /// Footer of a message containing one or more interactive elements.
    #[validate]
    pub footer: Option<InteractiveFooter>,
}

impl InteractiveMultiproductContent {
    pub fn new(
        header: InteractiveMultiproductHeader,
        body: InteractiveBody,
        action: InteractiveMultiproductAction,
    ) -> Self {
        InteractiveMultiproductContent {
            header,
            body,
            action,
            footer: None,
        }
    }
}

pub type SendInteractiveMultiproductRequestBody =
    SendContentRequestBody<InteractiveMultiproductContent>;

impl SendInteractiveMultiproductRequestBody {
    pub fn new(from: &str, to: &str, content: InteractiveMultiproductContent) -> Self {
        SendInteractiveMultiproductRequestBody {
            from: from.into(),
            to: to.into(),
            message_id: None,
            content,
            callback_data: None,
            notify_url: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TemplateCategory {
    #[serde(rename = "ACCOUNT_UPDATE")]
    AccountUpdate,
    #[serde(rename = "PAYMENT_UPDATE")]
    PaymentUpdate,
    #[serde(rename = "PERSONAL_FINANCE_UPDATE")]
    PersonalFinanceUpdate,
    #[serde(rename = "SHIPPING_UPDATE")]
    ShippingUpdate,
    #[serde(rename = "RESERVATION_UPDATE")]
    ReservationUpdate,
    #[serde(rename = "ISSUE_RESOLUTION")]
    IssueResolution,
    #[serde(rename = "APPOINTMENT_UPDATE")]
    AppointmentUpdate,
    #[serde(rename = "TRANSPORTATION_UPDATE")]
    TransportationUpdate,
    #[serde(rename = "TICKET_UPDATE")]
    TicketUpdate,
    #[serde(rename = "ALERT_UPDATE")]
    AlertUpdate,
    #[serde(rename = "AUTO_REPLY")]
    AutoReply,
    #[serde(rename = "MARKETING")]
    Marketing,
    #[serde(rename = "TRANSACTIONAL")]
    Transactional,
    #[serde(rename = "OTP")]
    Otp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TemplateLanguage {
    #[serde(rename = "af")]
    Af,
    #[serde(rename = "sq")]
    Sq,
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "az")]
    Az,
    #[serde(rename = "bn")]
    Bn,
    #[serde(rename = "bg")]
    Bg,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "zh_CN")]
    ZhCn,
    #[serde(rename = "zh_HK")]
    ZhHk,
    #[serde(rename = "zh_TW")]
    ZhTw,
    #[serde(rename = "hr")]
    Hr,
    #[serde(rename = "cs")]
    Cs,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "en_GB")]
    EnGb,
    #[serde(rename = "en_US")]
    EnUs,
    #[serde(rename = "et")]
    Et,
    #[serde(rename = "fil")]
    Fil,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "ka")]
    Ka,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "el")]
    El,
    #[serde(rename = "gu")]
    Gu,
    #[serde(rename = "ha")]
    Ha,
    #[serde(rename = "he")]
    He,
    #[serde(rename = "hi")]
    Hi,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "ga")]
    Ga,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "kn")]
    Kn,
    #[serde(rename = "kk")]
    Kk,
    #[serde(rename = "rw_RW")]
    RwRw,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "ky_KG")]
    KyKg,
    #[serde(rename = "lo")]
    Lo,
    #[serde(rename = "lv")]
    Lv,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "mk")]
    Mk,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "ml")]
    Ml,
    #[serde(rename = "mr")]
    Mr,
    #[serde(rename = "nb")]
    Nb,
    #[serde(rename = "fa")]
    Fa,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt_BR")]
    PtBr,
    #[serde(rename = "pt_PT")]
    PtPt,
    #[serde(rename = "pa")]
    Pa,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "ru")]
    Ru,
    #[serde(rename = "sr")]
    Sr,
    #[serde(rename = "sk")]
    Sk,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "es_AR")]
    EsAr,
    #[serde(rename = "es_ES")]
    EsEs,
    #[serde(rename = "es_MX")]
    EsMx,
    #[serde(rename = "sw")]
    Sw,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "ta")]
    Ta,
    #[serde(rename = "te")]
    Te,
    #[serde(rename = "th")]
    Th,
    #[serde(rename = "tr")]
    Tr,
    #[serde(rename = "uk")]
    Uk,
    #[serde(rename = "ur")]
    Ur,
    #[serde(rename = "uz")]
    Uz,
    #[serde(rename = "vi")]
    Vi,
    #[serde(rename = "zu")]
    Zu,
    #[serde(rename = "unknown")]
    Unknown,
}

impl ToString for TemplateLanguage {
    fn to_string(&self) -> String {
        match self {
            Self::Af => String::from("af"),
            Self::Sq => String::from("sq"),
            Self::Ar => String::from("ar"),
            Self::Az => String::from("az"),
            Self::Bn => String::from("bn"),
            Self::Bg => String::from("bg"),
            Self::Ca => String::from("ca"),
            Self::ZhCn => String::from("zh_CN"),
            Self::ZhHk => String::from("zh_HK"),
            Self::ZhTw => String::from("zh_TW"),
            Self::Hr => String::from("hr"),
            Self::Cs => String::from("cs"),
            Self::Da => String::from("da"),
            Self::Nl => String::from("nl"),
            Self::En => String::from("en"),
            Self::EnGb => String::from("en_GB"),
            Self::EnUs => String::from("en_US"),
            Self::Et => String::from("et"),
            Self::Fil => String::from("fil"),
            Self::Fi => String::from("fi"),
            Self::Fr => String::from("fr"),
            Self::Ka => String::from("ka"),
            Self::De => String::from("de"),
            Self::El => String::from("el"),
            Self::Gu => String::from("gu"),
            Self::Ha => String::from("ha"),
            Self::He => String::from("he"),
            Self::Hi => String::from("hi"),
            Self::Hu => String::from("hu"),
            Self::Id => String::from("id"),
            Self::Ga => String::from("ga"),
            Self::It => String::from("it"),
            Self::Ja => String::from("ja"),
            Self::Kn => String::from("kn"),
            Self::Kk => String::from("kk"),
            Self::RwRw => String::from("rw_RW"),
            Self::Ko => String::from("ko"),
            Self::KyKg => String::from("ky_KG"),
            Self::Lo => String::from("lo"),
            Self::Lv => String::from("lv"),
            Self::Lt => String::from("lt"),
            Self::Mk => String::from("mk"),
            Self::Ms => String::from("ms"),
            Self::Ml => String::from("ml"),
            Self::Mr => String::from("mr"),
            Self::Nb => String::from("nb"),
            Self::Fa => String::from("fa"),
            Self::Pl => String::from("pl"),
            Self::PtBr => String::from("pt_BR"),
            Self::PtPt => String::from("pt_PT"),
            Self::Pa => String::from("pa"),
            Self::Ro => String::from("ro"),
            Self::Ru => String::from("ru"),
            Self::Sr => String::from("sr"),
            Self::Sk => String::from("sk"),
            Self::Sl => String::from("sl"),
            Self::Es => String::from("es"),
            Self::EsAr => String::from("es_AR"),
            Self::EsEs => String::from("es_ES"),
            Self::EsMx => String::from("es_MX"),
            Self::Sw => String::from("sw"),
            Self::Sv => String::from("sv"),
            Self::Ta => String::from("ta"),
            Self::Te => String::from("te"),
            Self::Th => String::from("th"),
            Self::Tr => String::from("tr"),
            Self::Uk => String::from("uk"),
            Self::Ur => String::from("ur"),
            Self::Uz => String::from("uz"),
            Self::Vi => String::from("vi"),
            Self::Zu => String::from("zu"),
            Self::Unknown => String::from("unknown"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "format")]
pub enum TemplateHeader {
    #[serde(rename = "DOCUMENT")]
    Document {
        /// An example of a template header document a user could create. Should be a valid URL
        /// that starts with `http` or `https`. Supported document type is `PDF`. Maximum document
        /// size is 16MB. Cannot contain placeholders.
        #[serde(skip_serializing_if = "Option::is_none")]
        example: Option<String>,
    },
    #[serde(rename = "IMAGE")]
    Image {
        /// An example of a template header image a user could create. Should be a valid URL that
        /// starts with `http` or `https`. Supported image types are `JPG`, `JPEG`, `PNG`. Maximum
        /// image size is 16MB. Cannot contain placeholders.
        #[serde(skip_serializing_if = "Option::is_none")]
        example: Option<String>,
    },
    #[serde(rename = "LOCATION")]
    Location {},
    #[serde(rename = "TEXT")]
    Text {
        /// Template header text. Can contain up to 60 characters, with one placeholder {{1}}.
        text: String,
        /// An example of the content for the template header a user could use. Cannot contain
        /// placeholders.
        #[serde(skip_serializing_if = "Option::is_none")]
        example: Option<String>,
    },
    #[serde(rename = "VIDEO")]
    Video {
        /// An example of a template header video a user could use. Should be a valid URL that
        /// starts with `http` or `https`. Supported video type is `MP4`. Maximum video size is
        /// 16MB. Cannot contain placeholders.
        #[serde(skip_serializing_if = "Option::is_none")]
        example: Option<String>,
    },
}

impl TemplateHeader {
    pub fn new_text(text: &str) -> Self {
        Self::Text {
            text: text.into(),
            example: None,
        }
    }

    pub fn new_image(example: &str) -> Self {
        Self::Image {
            example: Some(example.into()),
        }
    }

    pub fn new_video(example: &str) -> Self {
        Self::Video {
            example: Some(example.into()),
        }
    }

    pub fn new_document(example: &str) -> Self {
        Self::Document {
            example: Some(example.into()),
        }
    }

    pub fn new_location() -> Self {
        Self::Location {}
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TemplateFooter {
    /// Plain text, up to 60 characters.
    #[validate(length(max = 60))]
    pub text: String,
}

impl TemplateFooter {
    pub fn new(text: &str) -> Self {
        TemplateFooter { text: text.into() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TemplateButton {
    #[serde(rename = "PHONE_NUMBER")]
    Number {
        /// Button text.
        text: String,
        /// Phone number to which a phone call would be placed by end-user when hitting the button.
        #[serde(rename = "phoneNumber")]
        phone_number: String,
    },
    #[serde(rename = "QUICK_REPLY")]
    QuickReply {
        /// Button text.
        text: String,
    },
    #[serde(rename = "URL")]
    Url {
        /// Button text.
        text: String,
        /// URL to which the end-user will be directed when hitting the button. URL is expected to
        /// start with `https://` or `http://`. Can be static or dynamic. For dynamic URL
        /// registration, add a placeholder {{1}} at the end of the link. Example:
        /// `https://www.infobip.com/{{1}}`.
        url: String,
        /// An example of a URL a user could use. Should be a valid URL that starts with
        /// `https://` or `http://`. Cannot contain placeholders.
        #[serde(skip_serializing_if = "Option::is_none")]
        example: Option<String>,
    },
}

impl TemplateButton {
    pub fn new_number(text: &str, phone_number: &str) -> Self {
        Self::Number {
            text: text.into(),
            phone_number: phone_number.into(),
        }
    }

    pub fn new_quick_reply(text: &str) -> Self {
        Self::QuickReply { text: text.into() }
    }

    pub fn new_url(text: &str, url: &str) -> Self {
        Self::Url {
            text: text.into(),
            url: url.into(),
            example: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TemplateBody {
    /// Plain text or text with placeholders. Placeholders have to be correctly formatted and in
    /// the correct order, regardless of other sections. Example: {{1}}, {{2}}, {{3}}...
    #[validate(length(min = 1))]
    pub text: String,

    /// Placeholders examples. The number of examples has to be the same as the number of
    /// placeholders. Examples cannot contain placeholders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<String>>,
}

impl TemplateBody {
    pub fn new(text: &str) -> Self {
        TemplateBody {
            text: text.into(),
            examples: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TemplateType {
    TEXT,
    MEDIA,
    UNSUPPORTED,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TemplateStructure {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Template header. Can be `image`, `document`, `video`, `location` or `text`.
    pub header: Option<TemplateHeader>,

    #[validate]
    /// Template body.
    pub body: TemplateBody,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    /// Template footer.
    pub footer: Option<TemplateFooter>,

    /// Template buttons. Can be either up to 3 `quick reply` buttons or up to 2 `call to action`
    /// buttons. Call to action buttons must be unique in type.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 3))]
    pub buttons: Option<Vec<TemplateButton>>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub template_type: Option<TemplateType>,
}

impl TemplateStructure {
    pub fn new(body: TemplateBody) -> Self {
        TemplateStructure {
            header: None,
            body,
            footer: None,
            buttons: None,
            template_type: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateTemplateRequestBody {
    /// Template name. Must only contain lowercase alphanumeric characters and underscores.
    #[validate(length(min = 1))]
    pub name: String,

    /// Template language, one template with same name can have multiple transliterations.
    pub language: TemplateLanguage,

    /// Category of the template.
    pub category: TemplateCategory,

    /// Template structure.
    #[validate]
    pub structure: TemplateStructure,
}

impl CreateTemplateRequestBody {
    pub fn new(
        name: &str,
        language: TemplateLanguage,
        category: TemplateCategory,
        structure: TemplateStructure,
    ) -> Self {
        CreateTemplateRequestBody {
            name: name.into(),
            language,
            category,
            structure,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TemplateHeaderContent {
    #[serde(rename = "DOCUMENT")]
    Document {
        /// URL of a document sent in the header. It is expected to be a valid URL starting with
        /// `https://` or `http://`. Supported document type is `PDF`. Maximum document size
        /// is 100MB.
        #[serde(rename = "mediaUrl")]
        media_url: String,

        /// Filename of the document.
        #[serde(rename = "filename")]
        filename: String,
    },
    #[serde(rename = "IMAGE")]
    Image {
        /// URL of an image sent in the header. It is expected to be a valid URL starting with
        /// `https://` or `http://`. Supported image types are `JPG`, `JPEG`, `PNG`. Maximum image
        /// size is 5MB.
        #[serde(rename = "mediaUrl")]
        media_url: String,
    },
    #[serde(rename = "LOCATION")]
    Location {
        /// Latitude of a location sent in the header.
        #[serde(rename = "latitude")]
        latitude: f64,

        /// Longitude of a location sent in the header.
        #[serde(rename = "longitude")]
        longitude: f64,
    },
    #[serde(rename = "TEXT")]
    Text {
        /// Value of a placeholder in the text header.
        #[serde(rename = "placeholder")]
        placeholder: String,
    },
    #[serde(rename = "VIDEO")]
    Video {
        /// URL of a video sent in the header. It is expected to be a valid URL starting with
        /// `https://` or `http://`. Supported video types are `MP4`, `3GPP`. Maximum video size
        /// is 16MB.
        #[serde(rename = "mediaUrl")]
        media_url: String,
    },
}

impl TemplateHeaderContent {
    pub fn new_document(media_url: &str, filename: &str) -> Self {
        TemplateHeaderContent::Document {
            media_url: media_url.into(),
            filename: filename.into(),
        }
    }

    pub fn new_image(media_url: &str) -> Self {
        TemplateHeaderContent::Image {
            media_url: media_url.into(),
        }
    }

    pub fn new_location(latitude: f64, longitude: f64) -> Self {
        TemplateHeaderContent::Location {
            latitude,
            longitude,
        }
    }

    pub fn new_text(placeholder: &str) -> Self {
        TemplateHeaderContent::Text {
            placeholder: placeholder.into(),
        }
    }

    pub fn new_video(media_url: &str) -> Self {
        TemplateHeaderContent::Video {
            media_url: media_url.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TemplateBodyContent {
    /// Template's parameter values submitted in the same order as in the registered template.
    /// The value must not be null, but it can be an empty array, if the template was registered
    /// without placeholders. Values within the array must not be null or empty.
    pub placeholders: Vec<String>,
}

impl TemplateBodyContent {
    pub fn new(placeholders: Vec<String>) -> Self {
        TemplateBodyContent { placeholders }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TemplateButtonContent {
    #[serde(rename = "QUICK_REPLY")]
    QuickReply {
        /// Payload of a `quick reply` button.
        #[serde(rename = "parameter")]
        parameter: String,
    },
    #[serde(rename = "URL")]
    Url {
        /// URL extension of a `dynamic URL` defined in the registered template.
        #[serde(rename = "parameter")]
        parameter: String,
    },
}

impl TemplateButtonContent {
    pub fn new_quick_reply(parameter: &str) -> Self {
        TemplateButtonContent::QuickReply {
            parameter: parameter.into(),
        }
    }

    pub fn new_url(parameter: &str) -> Self {
        TemplateButtonContent::Url {
            parameter: parameter.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TemplateData {
    /// Template body.
    #[validate]
    pub body: TemplateBodyContent,

    /// Template header. Should be defined, only if placeholders or media have been registered in
    /// the template's header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<TemplateHeaderContent>,

    /// Template buttons. Should be defined in correct order, only if `quick reply` or
    /// `dynamic URL` buttons have been registered. It can have up to three `quick reply` buttons
    /// or only one `dynamic URL` button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<TemplateButtonContent>>,
}

impl TemplateData {
    pub fn new(body: TemplateBodyContent) -> Self {
        TemplateData {
            body,
            header: None,
            buttons: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TemplateContent {
    /// Template name. Should only contain lowercase alphanumeric characters and underscores.
    #[serde(rename = "templateName")]
    #[validate(length(min = 1, max = 512))]
    pub template_name: String,

    /// Template data. Values have to be set as registered in the template.
    #[serde(rename = "templateData")]
    #[validate]
    pub template_data: TemplateData,

    /// The code of language or locale to use. Must be the same code used when registering the template.
    #[serde(rename = "language")]
    pub language: String,
}

impl TemplateContent {
    pub fn new(template_name: &str, template_data: TemplateData, language: &str) -> Self {
        TemplateContent {
            template_name: template_name.into(),
            template_data,
            language: language.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SmsFailover {
    /// SMS sender number. Must be in international format.
    #[validate(length(min = 1, max = 24))]
    pub from: String,

    /// Content of the SMS that will be sent.
    #[validate(length(min = 1, max = 4096))]
    pub text: String,
}

impl SmsFailover {
    pub fn new(from: &str, text: &str) -> Self {
        SmsFailover {
            from: from.into(),
            text: text.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FailoverMessage {
    /// Registered WhatsApp sender number. Must be in international format and comply with
    /// WhatsApp's requirements.
    #[serde(rename = "from")]
    #[validate(length(min = 1, max = 24))]
    pub from: String,

    /// Message recipient number. Must be in international format.
    #[serde(rename = "to")]
    #[validate(length(min = 1, max = 24))]
    pub to: String,

    /// The ID that uniquely identifies the message sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub message_id: Option<String>,

    /// The content object to build a message that will be sent.
    #[validate]
    pub content: TemplateContent,

    /// Custom client data that will be included in a Delivery Report.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 4000))]
    pub callback_data: Option<String>,

    /// The URL on your callback server to which delivery and seen reports will be sent. Delivery
    /// report format, Seen report format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(url)]
    pub notify_url: Option<String>,

    /// SMS message to be sent if the WhatsApp template message could not be delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub sms_failover: Option<SmsFailover>,
}

impl FailoverMessage {
    pub fn new(from: &str, to: &str, content: TemplateContent) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            content,
            message_id: None,
            callback_data: None,
            notify_url: None,
            sms_failover: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateRequestBody {
    /// An array of messages being sent.
    #[validate(length(min = 1))]
    #[validate]
    pub messages: Vec<FailoverMessage>,

    /// The ID that uniquely identifies the request. Bulk ID will be received only when you send a
    /// message to more than one destination address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,
}

impl SendTemplateRequestBody {
    pub fn new(messages: Vec<FailoverMessage>) -> Self {
        Self {
            messages,
            bulk_id: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

pub type SendInteractiveProductResponseBody = SendContentResponseBody;

pub type SendInteractiveMultiproductResponseBody = SendContentResponseBody;

/// Status of the template.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TemplateStatus {
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "IN_APPEAL")]
    InAppeal,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "PENDING_DELETION")]
    PendingDeletion,
    #[serde(rename = "DELETED")]
    Deleted,
    #[serde(rename = "DISABLED")]
    Disabled,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTemplateResponseBody {
    /// Template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Business account ID to which template belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_account_id: Option<i64>,

    /// Name of the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<TemplateLanguage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TemplateStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<TemplateCategory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<TemplateStructure>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    /// Template ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Business account ID to which template belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_account_id: Option<i64>,

    /// Name of the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Template language, one template with same name can have multiple transliterations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<TemplateLanguage>,

    /// Status of the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TemplateStatus>,

    /// Category of the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<TemplateCategory>,

    /// Template structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<TemplateStructure>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTemplatesResponseBody {
    /// List of all templates for given sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<Template>>,
}

pub type SentMessageInfo = SendContentResponseBody;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendTemplateResponseBody {
    /// Array of sent message objects, one object per every message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<SentMessageInfo>>,

    /// The ID that uniquely identifies the request. Bulk ID will be received only when you send a
    /// message to more than one destination address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_id: Option<String>,
}
