use validator::Validate;

use crate::model::whatsapp::*;

fn dummy_send_template_request_body() -> SendTemplateRequestBody {
    let body_content = TemplateBodyContent {
        placeholders: vec!["value1".to_string(), "value2".to_string()],
    };
    let template_data = TemplateData {
        body: body_content,
        header: Some(TemplateHeaderContent::Document {
            media_url: "https://some.url".to_string(),
            filename: "file.txt".to_string(),
        }),
        buttons: Some(vec![TemplateButtonContent::new_url("https://some.url")]),
    };
    let content = TemplateContent {
        template_name: "template_name1".to_string(),
        template_data,
        language: TemplateLanguage::EnUs.to_string(),
    };
    let message = FailoverMessage {
        from: "444444444444".to_string(),
        to: "555555555555".to_string(),
        message_id: Some("message_id".to_string()),
        content,
        callback_data: Some("callback_data".to_string()),
        notify_url: Some("https://some.url".to_string()),
        sms_failover: Some(SmsFailover {
            from: "666666666666".to_string(),
            text: "message text".to_string(),
        }),
    };

    SendTemplateRequestBody {
        messages: vec![message],
        bulk_id: Some("bulk_id".to_string()),
    }
}

fn dummy_send_text_request_body() -> SendTextRequestBody {
    SendTextRequestBody {
        from: "555555555555".to_string(),
        to: "666666666666".to_string(),
        message_id: Some("message_id".to_string()),
        content: TextContent {
            text: "message text".to_string(),
            preview_url: Some(true),
        },
        callback_data: Some("callback_data".to_string()),
        notify_url: Some("https://some.url".to_string()),
    }
}

fn dummy_send_document_request_body() -> SendDocumentRequestBody {
    SendDocumentRequestBody {
        from: "555555555555".to_string(),
        to: "666666666666".to_string(),
        message_id: Some("message_id".to_string()),
        content: DocumentContent {
            media_url: "https://some.url".to_string(),
            caption: Some("caption".to_string()),
            filename: Some("file.pdf".to_string()),
        },
        callback_data: Some("callback_data".to_string()),
        notify_url: Some("https://some.url".to_string()),
    }
}

fn dummy_send_image_request_body() -> SendImageRequestBody {
    SendImageRequestBody {
        from: "555555555555".to_string(),
        to: "666666666666".to_string(),
        message_id: Some("message_id".to_string()),
        content: ImageContent {
            media_url: "https://some.url".to_string(),
            caption: Some("caption".to_string()),
        },
        callback_data: Some("callback_data".to_string()),
        notify_url: Some("https://some.url".to_string()),
    }
}

fn dummy_send_audio_request_body() -> SendAudioRequestBody {
    SendAudioRequestBody {
        from: "555555555555".to_string(),
        to: "666666666666".to_string(),
        message_id: Some("message_id".to_string()),
        content: AudioContent {
            media_url: "https://some.url".to_string(),
        },
        callback_data: Some("callback_data".to_string()),
        notify_url: Some("https://some.url".to_string()),
    }
}

fn dummy_send_video_request_body() -> SendVideoRequestBody {
    SendVideoRequestBody {
        from: "555555555555".to_string(),
        to: "666666666666".to_string(),
        message_id: Some("message_id".to_string()),
        content: VideoContent {
            media_url: "https://some.url".to_string(),
            caption: Some("caption".to_string()),
        },
        callback_data: Some("callback_data".to_string()),
        notify_url: Some("https://some.url".to_string()),
    }
}

fn dummy_send_sticker_request_body() -> SendStickerRequestBody {
    SendStickerRequestBody {
        from: "555555555555".to_string(),
        to: "666666666666".to_string(),
        message_id: Some("message_id".to_string()),
        content: StickerContent {
            media_url: "https://some.url".to_string(),
        },
        callback_data: Some("callback_data".to_string()),
        notify_url: Some("https://some.url".to_string()),
    }
}

fn dummy_send_location_request_body() -> SendLocationRequestBody {
    SendLocationRequestBody {
        from: "555555555555".to_string(),
        to: "666666666666".to_string(),
        message_id: Some("message_id".to_string()),
        content: LocationContent {
            latitude: 1.0,
            longitude: 2.0,
            name: Some("name".to_string()),
            address: Some("address".to_string()),
        },
        callback_data: Some("callback_data".to_string()),
        notify_url: Some("https://some.url".to_string()),
    }
}

fn dummy_send_contact_request_body() -> SendContactRequestBody {
    let address = ContactAddress {
        street: Some("street".to_string()),
        city: Some("city".to_string()),
        state: Some("state".to_string()),
        zip: Some("21537".to_string()),
        country: Some("country".to_string()),

        country_code: Some("US".to_string()),
        address_type: Some(AddressType::Home),
    };

    let email = ContactEmail {
        email: Some("some@email.com".to_string()),
        email_type: Some(EmailType::Home),
    };

    let name = ContactName {
        first_name: "First".to_string(),
        last_name: Some("Last".to_string()),
        middle_name: Some("Middle".to_string()),
        name_suffix: Some("Mr.".to_string()),
        name_prefix: Some("Dr.".to_string()),
        formatted_name: "Dr. First Last".to_string(),
    };

    let org = ContactOrganization {
        company: Some("company".to_string()),
        department: Some("department".to_string()),
        title: Some("title".to_string()),
    };

    let phone = ContactPhone {
        phone: Some("555555555555".to_string()),
        phone_type: Some(PhoneType::Home),
        wa_id: Some("wid1".to_string()),
    };

    let url = ContactUrl {
        url: Some("https://some.url".to_string()),
        url_type: Some(UrlType::Home),
    };

    let contact = Contact {
        addresses: Some(vec![address]),
        birthday: Some("12-01-1990".to_string()),
        emails: Some(vec![email]),
        name,
        org: Some(org),
        phones: Some(vec![phone]),
        urls: Some(vec![url]),
    };

    SendContactRequestBody {
        from: "555555555555".to_string(),
        to: "666666666666".to_string(),
        message_id: Some("message_id".to_string()),
        content: ContactContent {
            contacts: vec![contact],
        },
        callback_data: Some("callback_data".to_string()),
        notify_url: Some("https://some.url".to_string()),
    }
}

fn dummy_send_interactive_buttons_request_body() -> SendInteractiveButtonsRequestBody {
    SendInteractiveButtonsRequestBody {
        from: "555555555555".to_string(),
        to: "666666666666".to_string(),
        message_id: Some("message_id".to_string()),
        content: InteractiveButtonsContent {
            body: InteractiveBody {
                text: "body text".to_string(),
            },
            action: InteractiveButtonsAction {
                buttons: vec![InteractiveButton::ReplyButton {
                    id: "id1".to_string(),
                    title: "title".to_string(),
                }],
            },
            header: Some(InteractiveButtonsHeader::DocumentHeader {
                media_url: "https://some.url".to_string(),
                filename: Some("filename".to_string()),
            }),
            footer: Some(InteractiveFooter {
                text: "footer".to_string(),
            }),
        },
        callback_data: Some("callback_data".to_string()),
        notify_url: Some("https://some.url".to_string()),
    }
}

fn dummy_send_interactive_list_request_body() -> SendInteractiveListRequestBody {
    let section = InteractiveListSection {
        title: Some("section title".to_string()),
        rows: vec![InteractiveRow {
            id: "id1".to_string(),
            title: "title".to_string(),
            description: Some("description".to_string()),
        }],
    };

    SendInteractiveListRequestBody {
        from: "555555555555".to_string(),
        to: "666666666666".to_string(),
        message_id: Some("message_id".to_string()),
        content: InteractiveListContent {
            body: InteractiveBody {
                text: "body text".to_string(),
            },
            action: InteractiveListAction {
                title: "list title".to_string(),
                sections: vec![section],
            },
            header: Some(InteractiveListHeader::TextHeader {
                text: "header text".to_string(),
            }),
            footer: Some(InteractiveFooter {
                text: "footer".to_string(),
            }),
        },
        callback_data: Some("callback_data".to_string()),
        notify_url: Some("https://some.url".to_string()),
    }
}

fn dummy_send_interactive_product_request_body() -> SendInteractiveProductRequestBody {
    SendInteractiveProductRequestBody {
        from: "555555555555".to_string(),
        to: "666666666666".to_string(),
        message_id: Some("message_id".to_string()),
        content: InteractiveProductContent {
            body: Some(InteractiveBody {
                text: "content text".to_string(),
            }),
            action: InteractiveProductAction {
                catalog_id: "1".to_string(),
                product_retailer_id: "2".to_string(),
            },
            footer: Some(InteractiveFooter {
                text: "footer".to_string(),
            }),
        },
        callback_data: Some("callback_data".to_string()),
        notify_url: Some("https://some.url".to_string()),
    }
}

fn dummy_send_interactive_multiproduct_request_body() -> SendInteractiveMultiproductRequestBody {
    let section = InteractiveMultiproductSection {
        title: Some("title".to_string()),
        product_retailer_ids: vec!["1".to_string(), "2".to_string()],
    };
    SendInteractiveMultiproductRequestBody {
        from: "555555555555".to_string(),
        to: "666666666666".to_string(),
        message_id: Some("message_id".to_string()),
        content: InteractiveMultiproductContent {
            header: InteractiveMultiproductHeader::TextHeader {
                text: "header text".to_string(),
            },
            body: InteractiveBody {
                text: "content text".to_string(),
            },
            action: InteractiveMultiproductAction {
                catalog_id: "1".to_string(),
                sections: vec![section],
            },
            footer: Some(InteractiveFooter {
                text: "footer".to_string(),
            }),
        },
        callback_data: Some("callback_data".to_string()),
        notify_url: Some("https://some.url".to_string()),
    }
}

fn dummy_create_template_request_body() -> CreateTemplateRequestBody {
    CreateTemplateRequestBody {
        name: "template name".to_string(),
        language: TemplateLanguage::Af,
        category: TemplateCategory::AccountUpdate,
        structure: TemplateStructure {
            header: Some(TemplateHeader::Text {
                text: "header text".to_string(),
                example: Some("example text".to_string()),
            }),
            body: TemplateBody {
                text: "body text".to_string(),
                examples: Some(vec!["example text".to_string()]),
            },
            footer: Some(TemplateFooter {
                text: "footer text".to_string(),
            }),
            buttons: Some(vec![TemplateButton::QuickReply {
                text: "reply text".to_string(),
            }]),
            template_type: Some(TemplateType::Text),
        },
    }
}

#[test]
fn send_template_request_body_valid() {
    let content = TemplateContent::new(
        "template_name1",
        TemplateData::new(TemplateBodyContent::new(vec![
            "value1".to_string(),
            "value2".to_string(),
        ])),
        TemplateLanguage::EnUs,
    );
    let message = FailoverMessage::new("444444444444", "555555555555", content);
    let request_body = SendTemplateRequestBody::new(vec![message]);

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_template_request_body_full_valid() {
    let request_body = dummy_send_template_request_body();

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_template_request_body_no_messages() {
    let request_body = SendTemplateRequestBody::new(vec![]);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_template_request_body_message_no_from() {
    let mut request_body = dummy_send_template_request_body();

    request_body.messages[0].from = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_template_request_body_long_from() {
    let mut request_body = dummy_send_template_request_body();

    request_body.messages[0].from = "4".repeat(25usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_template_request_body_message_no_to() {
    let mut request_body = dummy_send_template_request_body();

    request_body.messages[0].to = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_template_request_body_long_to() {
    let mut request_body = dummy_send_template_request_body();

    request_body.messages[0].to = "5".repeat(25usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_template_request_body_message_long_id() {
    let mut request_body = dummy_send_template_request_body();

    request_body.messages[0].message_id = Some("i".repeat(51usize));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_template_request_body_message_long_callback_data() {
    let mut request_body = dummy_send_template_request_body();

    request_body.messages[0].callback_data = Some("c".repeat(4001usize));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_template_request_body_message_invalid_notify_url() {
    let mut request_body = dummy_send_template_request_body();

    request_body.messages[0].notify_url = Some("n".repeat(2049usize));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_template_request_body_message_sms_fallback_no_from() {
    let mut request_body = dummy_send_template_request_body();

    request_body.messages[0].sms_failover.as_mut().unwrap().from = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_template_request_body_message_sms_fallback_long_from() {
    let mut request_body = dummy_send_template_request_body();

    request_body.messages[0].sms_failover.as_mut().unwrap().from = "6".repeat(25usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_template_request_body_message_sms_fallback_no_text() {
    let mut request_body = dummy_send_template_request_body();

    request_body.messages[0].sms_failover.as_mut().unwrap().text = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_template_request_body_message_sms_fallback_long_text() {
    let mut request_body = dummy_send_template_request_body();

    request_body.messages[0].sms_failover.as_mut().unwrap().text = "t".repeat(4097usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_template_request_body_message_content_no_template_name() {
    let mut request_body = dummy_send_template_request_body();

    request_body.messages[0].content.template_name = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_template_request_body_message_content_long_template_name() {
    let mut request_body = dummy_send_template_request_body();

    request_body.messages[0].content.template_name = "t".repeat(513usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_text_request_body_valid() {
    let request_body = SendTextRequestBody::new(
        "444444444444",
        "555555555555",
        TextContent::new("message text"),
    );

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_text_request_full_valid() {
    let request_body = dummy_send_text_request_body();

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_text_request_no_from() {
    let mut request_body = dummy_send_text_request_body();

    request_body.from = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_text_request_long_from() {
    let mut request_body = dummy_send_text_request_body();

    request_body.from = "7".repeat(25usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_text_request_no_to() {
    let mut request_body = dummy_send_text_request_body();

    request_body.to = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_text_request_long_to() {
    let mut request_body = dummy_send_text_request_body();

    request_body.to = "8".repeat(25usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_text_request_long_id() {
    let mut request_body = dummy_send_text_request_body();

    request_body.message_id = Some("i".repeat(51usize));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_text_request_long_callback_data() {
    let mut request_body = dummy_send_text_request_body();

    request_body.callback_data = Some("c".repeat(4001usize));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_text_request_invalid_notify_url() {
    let mut request_body = dummy_send_text_request_body();

    request_body.notify_url = Some("n".repeat(2049usize));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_text_request_sms_content_no_text() {
    let mut request_body = dummy_send_text_request_body();

    request_body.content.text = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_text_request_sms_content_long_text() {
    let mut request_body = dummy_send_text_request_body();

    request_body.content.text = "t".repeat(4097usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_document_request_valid() {
    let request_body = SendDocumentRequestBody::new(
        "444444444444",
        "555555555555",
        DocumentContent::new("https://some.url"),
    );

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_document_request_full_valid() {
    let request_body = dummy_send_document_request_body();

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_document_request_content_no_media_url() {
    let mut request_body = dummy_send_document_request_body();

    request_body.content.media_url = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_document_request_content_invalid_media_url() {
    let mut request_body = dummy_send_document_request_body();

    request_body.content.media_url = "m".repeat(2049usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_document_request_content_long_caption() {
    let mut request_body = dummy_send_document_request_body();

    request_body.content.caption = Some("c".repeat(3001usize));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_document_request_content_long_file_name() {
    let mut request_body = dummy_send_document_request_body();

    request_body.content.filename = Some("f".repeat(241usize));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_image_request_body_valid() {
    let request_body = SendImageRequestBody::new(
        "444444444444",
        "555555555555",
        ImageContent::new("https://some.url"),
    );

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_image_request_full_valid() {
    let request_body = dummy_send_image_request_body();

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_image_request_content_no_media_url() {
    let mut request_body = dummy_send_image_request_body();

    request_body.content.media_url = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_image_request_content_invalid_media_url() {
    let mut request_body = dummy_send_image_request_body();

    request_body.content.media_url = "m".repeat(2049usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_image_request_content_long_caption() {
    let mut request_body = dummy_send_image_request_body();

    request_body.content.caption = Some("c".repeat(3001usize));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_audio_request_body_valid() {
    let request_body = SendAudioRequestBody::new(
        "444444444444",
        "555555555555",
        AudioContent::new("https://some.url"),
    );

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_audio_request_full_valid() {
    let request_body = dummy_send_audio_request_body();

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_audio_request_content_no_media_url() {
    let mut request_body = dummy_send_audio_request_body();

    request_body.content.media_url = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_audio_request_content_invalid_media_url() {
    let mut request_body = dummy_send_audio_request_body();

    request_body.content.media_url = "m".repeat(2049usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_video_request_body_valid() {
    let request_body = SendVideoRequestBody::new(
        "444444444444",
        "555555555555",
        VideoContent::new("https://some.url"),
    );

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_video_request_full_valid() {
    let request_body = dummy_send_video_request_body();

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_video_request_content_no_media_url() {
    let mut request_body = dummy_send_video_request_body();

    request_body.content.media_url = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_video_request_content_invalid_media_url() {
    let mut request_body = dummy_send_video_request_body();

    request_body.content.media_url = "m".repeat(2049usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_video_request_content_long_caption() {
    let mut request_body = dummy_send_video_request_body();

    request_body.content.caption = Some("c".repeat(3001usize));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_sticker_request_body_valid() {
    let request_body = SendStickerRequestBody::new(
        "444444444444",
        "555555555555",
        StickerContent::new("https://some.url"),
    );

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_sticker_request_full_valid() {
    let request_body = dummy_send_sticker_request_body();

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_sticker_request_content_no_media_url() {
    let mut request_body = dummy_send_sticker_request_body();

    request_body.content.media_url = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_sticker_request_content_invalid_media_url() {
    let mut request_body = dummy_send_sticker_request_body();

    request_body.content.media_url = "m".repeat(2049usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_location_request_body_valid() {
    let request_body = SendLocationRequestBody::new(
        "444444444444",
        "555555555555",
        LocationContent::new(1.0, 2.0),
    );

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_location_request_full_valid() {
    let request_body = dummy_send_location_request_body();

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_location_request_content_invalid_latitude() {
    let mut request_body = dummy_send_location_request_body();

    request_body.content.latitude = 91.0;

    assert!(request_body.validate().is_err());
}

#[test]
fn send_location_request_content_invalid_longitude() {
    let mut request_body = dummy_send_location_request_body();

    request_body.content.longitude = 181.0;

    assert!(request_body.validate().is_err());
}

#[test]
fn send_location_request_content_long_address() {
    let mut request_body = dummy_send_location_request_body();

    request_body.content.address = Some("a".repeat(1001usize));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_location_request_content_long_name() {
    let mut request_body = dummy_send_location_request_body();

    request_body.content.name = Some("n".repeat(1001usize));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_contact_request_body_valid() {
    let request_body = SendContactRequestBody::new(
        "444444444444",
        "555555555555",
        ContactContent::new(vec![Contact::new(ContactName::new("John", "John Doe"))]),
    );

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_contact_request_full_valid() {
    let request_body = dummy_send_contact_request_body();

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_contact_request_content_no_contacts() {
    let mut request_body = dummy_send_contact_request_body();

    request_body.content.contacts = vec![];

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_buttons_request_body_valid() {
    let button = InteractiveButton::new_reply_button("1", "Button Title");
    let request_body = SendInteractiveButtonsRequestBody::new(
        "444444444444",
        "555555555555",
        InteractiveButtonsContent::new(
            InteractiveBody::new("Hello"),
            InteractiveButtonsAction::new(vec![button]),
        ),
    );

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_interactive_buttons_request_full_valid() {
    let request_body = dummy_send_interactive_buttons_request_body();

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_interactive_buttons_request_content_no_body_text() {
    let mut request_body = dummy_send_interactive_buttons_request_body();

    request_body.content.body.text = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_buttons_request_content_long_body_text() {
    let mut request_body = dummy_send_interactive_buttons_request_body();

    request_body.content.body.text = "t".repeat(1025usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_buttons_request_content_no_buttons() {
    let mut request_body = dummy_send_interactive_buttons_request_body();

    request_body.content.action.buttons = vec![];

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_buttons_request_content_no_footer_text() {
    let mut request_body = dummy_send_interactive_buttons_request_body();

    request_body.content.footer = Some(InteractiveFooter::new(""));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_buttons_request_content_long_footer_text() {
    let mut request_body = dummy_send_interactive_buttons_request_body();

    request_body.content.footer = Some(InteractiveFooter::new(&"t".repeat(61usize)));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_list_request_body_valid() {
    let row = InteractiveRow::new("id1", "title1");
    let section = InteractiveListSection::new(vec![row]);
    let request_body = SendInteractiveListRequestBody::new(
        "444444444444",
        "555555555555",
        InteractiveListContent::new(
            InteractiveBody::new("Hello"),
            InteractiveListAction::new("Action Title", vec![section]),
        ),
    );

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_interactive_list_request_full_valid() {
    let request_body = dummy_send_interactive_list_request_body();

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_interactive_list_request_content_no_body_text() {
    let mut request_body = dummy_send_interactive_list_request_body();

    request_body.content.body.text = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_list_request_content_long_body_text() {
    let mut request_body = dummy_send_interactive_list_request_body();

    request_body.content.body.text = "t".repeat(1025usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_list_request_content_no_action_title() {
    let mut request_body = dummy_send_interactive_list_request_body();

    request_body.content.action.title = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_list_request_content_long_action_title() {
    let mut request_body = dummy_send_interactive_list_request_body();

    request_body.content.action.title = "t".repeat(61usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_list_request_content_no_sections() {
    let mut request_body = dummy_send_interactive_list_request_body();

    request_body.content.action.sections = vec![];

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_list_request_content_many_sections() {
    let mut request_body = dummy_send_interactive_list_request_body();

    request_body.content.action.sections = vec![
        InteractiveListSection::new(vec![InteractiveRow::new("id1", "title1")]),
        InteractiveListSection::new(vec![InteractiveRow::new("id2", "title2")]),
        InteractiveListSection::new(vec![InteractiveRow::new("id3", "title3")]),
        InteractiveListSection::new(vec![InteractiveRow::new("id4", "title4")]),
        InteractiveListSection::new(vec![InteractiveRow::new("id5", "title5")]),
        InteractiveListSection::new(vec![InteractiveRow::new("id6", "title6")]),
        InteractiveListSection::new(vec![InteractiveRow::new("id7", "title7")]),
        InteractiveListSection::new(vec![InteractiveRow::new("id8", "title8")]),
        InteractiveListSection::new(vec![InteractiveRow::new("id9", "title9")]),
        InteractiveListSection::new(vec![InteractiveRow::new("id10", "title10")]),
        InteractiveListSection::new(vec![InteractiveRow::new("id11", "title11")]),
    ];

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_list_request_content_no_rows() {
    let mut request_body = dummy_send_interactive_list_request_body();

    request_body.content.action.sections[0].rows = vec![];

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_list_request_content_no_row_id() {
    let mut request_body = dummy_send_interactive_list_request_body();

    request_body.content.action.sections[0].rows[0].id = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_list_request_content_long_row_id() {
    let mut request_body = dummy_send_interactive_list_request_body();

    request_body.content.action.sections[0].rows[0].id = "t".repeat(201usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_list_request_content_no_row_title() {
    let mut request_body = dummy_send_interactive_list_request_body();

    request_body.content.action.sections[0].rows[0].title = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_list_request_content_long_row_title() {
    let mut request_body = dummy_send_interactive_list_request_body();

    request_body.content.action.sections[0].rows[0].title = "t".repeat(25usize);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_list_request_content_long_row_description() {
    let mut request_body = dummy_send_interactive_list_request_body();

    request_body.content.action.sections[0].rows[0].description = Some("t".repeat(73usize));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_list_request_content_no_footer_text() {
    let mut request_body = dummy_send_interactive_list_request_body();

    request_body.content.footer = Some(InteractiveFooter::new(""));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_list_request_content_long_footer_text() {
    let mut request_body = dummy_send_interactive_list_request_body();

    request_body.content.footer = Some(InteractiveFooter::new(&"t".repeat(61usize)));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_product_request_valid() {
    let request_body = SendInteractiveProductRequestBody::new(
        "555555555555",
        "444444444444",
        InteractiveProductContent::new(InteractiveProductAction::new("1", "2")),
    );

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_interactive_product_request_full_valid() {
    let request_body = dummy_send_interactive_product_request_body();

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_interactive_product_request_content_no_catalog_id() {
    let mut request_body = dummy_send_interactive_product_request_body();

    request_body.content.action.catalog_id = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_product_request_content_no_product_retailer_id() {
    let mut request_body = dummy_send_interactive_product_request_body();

    request_body.content.action.product_retailer_id = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_product_request_content_no_footer_text() {
    let mut request_body = dummy_send_interactive_product_request_body();

    request_body.content.footer = Some(InteractiveFooter::new(""));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_multiproduct_request_body_valid() {
    let body = InteractiveBody::new("body text");

    let section = InteractiveMultiproductSection::new(vec!["1".to_string(), "2".to_string()]);

    let action = InteractiveMultiproductAction::new("1", vec![section]);

    let request_body = SendInteractiveMultiproductRequestBody::new(
        "555555555555",
        "444444444444",
        InteractiveMultiproductContent::new(
            InteractiveMultiproductHeader::new_text_header("header text"),
            body,
            action,
        ),
    );

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_interactive_multiproduct_request_body_full_valid() {
    let request_body = dummy_send_interactive_multiproduct_request_body();

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_interactive_multiproduct_request_body_content_no_body_text() {
    let mut request_body = dummy_send_interactive_multiproduct_request_body();

    request_body.content.body = InteractiveBody::new("");

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_multiproduct_request_body_content_long_body_text() {
    let mut request_body = dummy_send_interactive_multiproduct_request_body();

    request_body.content.body = InteractiveBody::new(&"t".repeat(1025usize));

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_multiproduct_request_body_content_action_no_catalog_id() {
    let mut request_body = dummy_send_interactive_multiproduct_request_body();

    request_body.content.action.catalog_id = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_multiproduct_request_body_content_action_no_sections() {
    let mut request_body = dummy_send_interactive_multiproduct_request_body();

    request_body.content.action.sections = vec![];

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_multiproduct_request_body_content_action_many_sections() {
    let mut request_body = dummy_send_interactive_multiproduct_request_body();

    request_body.content.action.sections = vec![
        InteractiveMultiproductSection::new(vec!["1".to_string(), "2".to_string()]),
        InteractiveMultiproductSection::new(vec!["1".to_string(), "2".to_string()]),
        InteractiveMultiproductSection::new(vec!["1".to_string(), "2".to_string()]),
        InteractiveMultiproductSection::new(vec!["1".to_string(), "2".to_string()]),
        InteractiveMultiproductSection::new(vec!["1".to_string(), "2".to_string()]),
        InteractiveMultiproductSection::new(vec!["1".to_string(), "2".to_string()]),
        InteractiveMultiproductSection::new(vec!["1".to_string(), "2".to_string()]),
        InteractiveMultiproductSection::new(vec!["1".to_string(), "2".to_string()]),
        InteractiveMultiproductSection::new(vec!["1".to_string(), "2".to_string()]),
        InteractiveMultiproductSection::new(vec!["1".to_string(), "2".to_string()]),
        InteractiveMultiproductSection::new(vec!["1".to_string(), "2".to_string()]),
        InteractiveMultiproductSection::new(vec!["1".to_string(), "2".to_string()]),
    ];

    assert!(request_body.validate().is_err());
}

#[test]
fn send_interactive_multiproduct_request_body_content_action_section_long_title() {
    let mut request_body = dummy_send_interactive_multiproduct_request_body();

    request_body.content.action.sections[0].title = Some("t".repeat(25usize));

    assert!(request_body.validate().is_err());
}

#[test]
fn create_template_request_body_valid() {
    let structure = TemplateStructure::new(TemplateBody::new("hello"));
    let request_body = CreateTemplateRequestBody::new(
        "rust_sdk_test_template",
        TemplateLanguage::EnUs,
        TemplateCategory::Marketing,
        structure,
    );

    assert!(request_body.validate().is_ok());
}

#[test]
fn create_template_request_body_full_valid() {
    let request_body = dummy_create_template_request_body();

    assert!(request_body.validate().is_ok());
}

#[test]
fn create_template_request_body_no_name() {
    let mut request_body = dummy_create_template_request_body();

    request_body.name = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn create_template_request_body_structure_body_no_text() {
    let mut request_body = dummy_create_template_request_body();

    request_body.structure.body.text = "".to_string();

    assert!(request_body.validate().is_err());
}

#[test]
fn create_template_request_body_structure_long_footer_text() {
    let mut request_body = dummy_create_template_request_body();

    request_body.structure.footer = Some(TemplateFooter::new(&"t".repeat(61usize)));

    assert!(request_body.validate().is_err());
}

#[test]
fn create_template_request_body_structure_many_buttons() {
    let mut request_body = dummy_create_template_request_body();

    request_body.structure.buttons = Some(vec![
        TemplateButton::new_quick_reply("1"),
        TemplateButton::new_quick_reply("1"),
        TemplateButton::new_quick_reply("1"),
        TemplateButton::new_quick_reply("1"),
    ]);

    assert!(request_body.validate().is_err());
}
