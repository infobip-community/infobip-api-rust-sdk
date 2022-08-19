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
    let content = TextContent::new(DUMMY_TEXT.to_string());

    let request_body = SendTextRequestBody::new(
        get_test_sender_number(),
        get_test_destination_number(),
        content,
    );

    let response = get_test_wa_client().send_text(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_document_whatsapp() {
    let content = DocumentContent::new(
        "https://perso.limsi.fr/pointal/_media/python:cours:mementopython3-english.pdf".to_string(),
    );

    let request_body = SendDocumentRequestBody::new(
        get_test_sender_number(),
        get_test_destination_number(),
        content,
    );

    let response = get_test_wa_client()
        .send_document(request_body)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}
