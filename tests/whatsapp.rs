// These tests need to be run manually, due to server state dependencies. The environment variables
// IB_API_KEY, IB_BASE_URL, IB_TEST_SENDER, and IB_TEST_DESTINATION_NUMBER must be set.

#![cfg(feature = "whatsapp")]
#![cfg(test)]

use std::env;

use reqwest::StatusCode;

use infobip_sdk::api::whatsapp::WhatsappClient;
use infobip_sdk::configuration;
use infobip_sdk::model::whatsapp::*;

const DUMMY_TEXT: &str = "Dummy text for tests. Some special chars: áéíø";

fn test_wa_client() -> WhatsappClient {
    WhatsappClient::with_configuration(
        configuration::Configuration::from_env_api_key()
            .expect("failed to build default test client"),
    )
}

fn test_destination_number() -> String {
    env::var("IB_TEST_DESTINATION_NUMBER").expect("failed to load test destination number")
}

fn test_sender_number() -> String {
    env::var("IB_TEST_SENDER").expect("failed to load test sender number")
}

#[ignore]
#[tokio::test]
async fn send_text() {
    let request_body = SendTextRequestBody::new(
        &test_sender_number(),
        &test_destination_number(),
        TextContent::new(DUMMY_TEXT),
    );

    let response = test_wa_client().send_text(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_document() {
    let request_body = SendDocumentRequestBody::new(
        &test_sender_number(),
        &test_destination_number(),
        DocumentContent::new(
            "https://perso.limsi.fr/pointal/_media/python:cours:mementopython3-english.pdf",
        ),
    );

    let response = test_wa_client()
        .send_document(request_body)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_image() {
    let request_body = SendImageRequestBody::new(
        &test_sender_number(),
        &test_destination_number(),
        ImageContent::new("https://rustacean.net/assets/rustacean-flat-happy.png"),
    );

    let response = test_wa_client().send_image(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_audio() {
    let request_body = SendAudioRequestBody::new(
        &test_sender_number(),
        &test_destination_number(),
        AudioContent::new("https://download.samplelib.com/mp3/sample-3s.mp3"),
    );

    let response = test_wa_client().send_audio(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_video() {
    let request_body = SendVideoRequestBody::new(
        &test_sender_number(),
        &test_destination_number(),
        VideoContent::new("https://download.samplelib.com/mp4/sample-5s.mp4"),
    );

    let response = test_wa_client().send_video(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_sticker() {
    let request_body = SendStickerRequestBody::new(
        &test_sender_number(),
        &test_destination_number(),
        StickerContent::new("https://www.gstatic.com/webp/gallery/1.webp"),
    );

    let response = test_wa_client()
        .send_sticker(request_body)
        .await
        .unwrap();
    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_location() {
    let request_body = SendLocationRequestBody::new(
        &test_sender_number(),
        &test_destination_number(),
        LocationContent::new(0.0, 0.0),
    );

    let response = test_wa_client()
        .send_location(request_body)
        .await
        .unwrap();
    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_contact() {
    let contact = Contact::new(ContactName::new("John", "John Doe"));
    let request_body = SendContactRequestBody::new(
        &test_sender_number(),
        &test_destination_number(),
        ContactContent::new(vec![contact]),
    );

    let response = test_wa_client()
        .send_contact(request_body)
        .await
        .unwrap();
    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_interactive_buttons() {
    let button = InteractiveButton::new_reply_button("1", "Button Title");
    let request_body = SendInteractiveButtonsRequestBody::new(
        &test_sender_number(),
        &test_destination_number(),
        InteractiveButtonsContent::new(
            InteractiveBody::new("Hello"),
            InteractiveButtonsAction::new(vec![button]),
        ),
    );

    let response = test_wa_client()
        .send_interactive_buttons(request_body)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_interactive_list() {
    let row = InteractiveRow::new("1", "Row Title");

    let section = InteractiveListSection::new(vec![row]);

    let request_body = SendInteractiveListRequestBody::new(
        &test_sender_number(),
        &test_destination_number(),
        InteractiveListContent::new(
            InteractiveBody::new("Hello"),
            InteractiveListAction::new("Section Title", vec![section]),
        ),
    );

    let response = test_wa_client()
        .send_interactive_list(request_body)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_interactive_product() {
    let request_body = SendInteractiveProductRequestBody::new(
        &test_sender_number(),
        &test_destination_number(),
        InteractiveProductContent::new(InteractiveProductAction::new("1", "2")),
    );

    let response = test_wa_client()
        .send_interactive_product(request_body)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_interactive_multiproduct() {
    let section = InteractiveMultiproductSection::new(vec!["1".to_string(), "2".to_string()]);
    let request_body = SendInteractiveMultiproductRequestBody::new(
        &test_sender_number(),
        &test_destination_number(),
        InteractiveMultiproductContent::new(
            InteractiveMultiproductHeader::new_text_header("Header text"),
            InteractiveBody::new("Body text"),
            InteractiveMultiproductAction::new("1", vec![section]),
        ),
    );

    let response = test_wa_client()
        .send_interactive_multiproduct(request_body)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn create_template() {
    let structure = TemplateStructure::new(TemplateBody::new("hello"));
    let request_body = CreateTemplateRequestBody::new(
        "rust_sdk_test_template",
        TemplateLanguage::EnUs,
        TemplateCategory::Marketing,
        structure,
    );

    let response = test_wa_client()
        .create_template(&test_sender_number(), request_body)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::CREATED);
    assert!(!response.body.id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn templates() {
    let response = test_wa_client()
        .templates(&test_sender_number())
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.templates.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn delete_template() {
    let status = test_wa_client()
        .delete_template(&test_sender_number(), "rust_sdk_test_template")
        .await
        .unwrap();

    assert_eq!(status, StatusCode::NO_CONTENT);
}

#[ignore]
#[tokio::test]
async fn send_template() {
    let template_content = TemplateContent::new(
        "rust_sdk_test_template",
        TemplateData::new(TemplateBodyContent::new(vec!["hello".to_string()])),
        TemplateLanguage::EnUs,
    );
    let message = FailoverMessage::new(
        &test_sender_number(),
        &test_destination_number(),
        template_content,
    );

    let request_body = SendTemplateRequestBody::new(vec![message]);

    let response = test_wa_client()
        .send_template(request_body)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.messages.unwrap().is_empty());
}
