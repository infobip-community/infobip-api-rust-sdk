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

fn get_test_wa_client() -> WhatsappClient {
    WhatsappClient::with_configuration(
        configuration::Configuration::from_env_api_key()
            .expect("failed to build default test SMS client"),
    )
}

fn get_test_destination_number() -> String {
    env::var("IB_TEST_DESTINATION_NUMBER").expect("failed to load test destination number")
}

fn get_test_sender_number() -> String {
    env::var("IB_TEST_SENDER").expect("failed to load test sender number")
}

#[ignore]
#[tokio::test]
async fn send_text_whatsapp() {
    let request_body = SendTextRequestBody::new(
        get_test_sender_number(),
        get_test_destination_number(),
        TextContent::new(DUMMY_TEXT.to_string()),
    );

    let response = get_test_wa_client().send_text(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_document_whatsapp() {
    let request_body = SendDocumentRequestBody::new(
        get_test_sender_number(),
        get_test_destination_number(),
        DocumentContent::new(
            "https://perso.limsi.fr/pointal/_media/python:cours:mementopython3-english.pdf"
                .to_string(),
        ),
    );

    let response = get_test_wa_client()
        .send_document(request_body)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_image_whatsapp() {
    let request_body = SendImageRequestBody::new(
        get_test_sender_number(),
        get_test_destination_number(),
        ImageContent::new("https://rustacean.net/assets/rustacean-flat-happy.png".to_string()),
    );

    let response = get_test_wa_client().send_image(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_audio_whatsapp() {
    let request_body = SendAudioRequestBody::new(
        get_test_sender_number(),
        get_test_destination_number(),
        AudioContent::new("https://download.samplelib.com/mp3/sample-3s.mp3".to_string()),
    );

    let response = get_test_wa_client().send_audio(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_video_whatsapp() {
    let request_body = SendVideoRequestBody::new(
        get_test_sender_number(),
        get_test_destination_number(),
        VideoContent::new("https://download.samplelib.com/mp4/sample-5s.mp4".to_string()),
    );

    let response = get_test_wa_client().send_video(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_sticker_whatsapp() {
    let request_body = SendStickerRequestBody::new(
        get_test_sender_number(),
        get_test_destination_number(),
        StickerContent::new("https://www.gstatic.com/webp/gallery/1.webp".to_string()),
    );

    let response = get_test_wa_client()
        .send_sticker(request_body)
        .await
        .unwrap();
    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_location_whatsapp() {
    let request_body = SendLocationRequestBody::new(
        get_test_sender_number(),
        get_test_destination_number(),
        LocationContent::new(0.0, 0.0),
    );

    let response = get_test_wa_client()
        .send_location(request_body)
        .await
        .unwrap();
    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_contact_whatsapp() {
    let contact = Contact::new(ContactName::new("John".to_string(), "John Doe".to_string()));
    let request_body = SendContactRequestBody::new(
        get_test_sender_number(),
        get_test_destination_number(),
        ContactContent::new(vec![contact]),
    );

    let response = get_test_wa_client()
        .send_contact(request_body)
        .await
        .unwrap();
    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}
