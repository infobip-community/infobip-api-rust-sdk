// These tests need to be run manually, due to server state dependencies. The environment variables
// IB_API_KEY, IB_BASE_URL, IB_TEST_EMAIL_FROM, and IB_TEST_EMAIL_TO must be set.

#![cfg(feature = "email")]
#![cfg(test)]

use reqwest::StatusCode;

use infobip_sdk::api::email::EmailClient;
use infobip_sdk::configuration;
use infobip_sdk::model::email::*;

fn get_test_email_client() -> EmailClient {
    EmailClient::with_configuration(
        configuration::Configuration::from_dotenv_api_key()
            .expect("failed to build default test client"),
    )
}

fn get_test_from() -> String {
    dotenv::var("IB_TEST_EMAIL_FROM").expect("failed to load test email from")
}

fn get_test_to() -> String {
    dotenv::var("IB_TEST_EMAIL_TO").expect("failed to load test email to")
}

#[ignore]
#[tokio::test]
async fn send() {
    let mut request_body = SendRequestBody::new(get_test_to());
    request_body.from = Some(get_test_from());
    request_body.subject = Some("Test subject".to_string());
    request_body.text = Some("Hello world!".to_string());
    request_body.attachment = Some("tests/image.png".to_string());

    let response = get_test_email_client().send(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.messages.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn send_bulk() {
    let mut request_body = SendRequestBody::new(get_test_to());
    request_body.from = Some(get_test_from());
    request_body.subject = Some("Test subject".to_string());
    request_body.text = Some("Hello world!".to_string());
    request_body.send_at = Some("2022-10-01T17:29:52Z".to_string());
    request_body.bulk_id = Some("test-bulk-id-rust-002".to_string());

    let response = get_test_email_client().send(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.messages.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn get_bulks() {
    let query_params = GetBulksQueryParameters::new("test-bulk-id-rust-002".to_string());

    let response = get_test_email_client()
        .get_bulks(query_params)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    println!("{}", serde_json::to_string(&response.body).unwrap());
}
