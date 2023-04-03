// These tests need to be run manually, due to server state dependencies. The environment variables
// IB_API_KEY, IB_BASE_URL, IB_TEST_EMAIL_FROM, and IB_TEST_EMAIL_TO must be set.

#![cfg(feature = "email")]
#![cfg(test)]

use chrono::DateTime;
use reqwest::StatusCode;
use std::env;

use infobip_sdk::api::email::EmailClient;
use infobip_sdk::configuration;
use infobip_sdk::model::email::DkimKeyLength::L1024;
use infobip_sdk::model::email::*;

fn get_test_email_client() -> EmailClient {
    EmailClient::with_configuration(
        configuration::Configuration::from_env_api_key()
            .expect("failed to build default test client"),
    )
}

fn get_test_from() -> String {
    env::var("IB_TEST_EMAIL_FROM").expect("failed to load test email from")
}

fn get_test_to() -> String {
    env::var("IB_TEST_EMAIL_TO").expect("failed to load test email to")
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
    request_body.send_at = Some("2022-10-05T16:28:52Z".to_string());
    request_body.bulk_id = Some("test-bulk-id-rust-003".to_string());

    let response = get_test_email_client().send(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.messages.unwrap().is_empty());
}

#[ignore]
#[tokio::test]
async fn get_bulks() {
    let query_params = GetBulksQueryParameters::new("test-bulk-id-rust-003".to_string());

    let response = get_test_email_client()
        .get_bulks(query_params)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn reschedule() {
    let query_params = RescheduleQueryParameters::new("test-bulk-id-rust-003".to_string());
    let send_at = "2022-10-05T17:29:52Z".to_string();
    let expected_send_at = DateTime::parse_from_rfc3339(&send_at)
        .unwrap()
        .timestamp_millis();

    let request_body = RescheduleRequestBody::new(send_at);

    let response = get_test_email_client()
        .reschedule(query_params, request_body)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.body.send_at.unwrap(), expected_send_at as u64);
}

#[ignore]
#[tokio::test]
async fn get_scheduled_status() {
    let query_params = GetScheduledStatusQueryParameters::new("test-bulk-id-rust-003".to_string());

    let response = get_test_email_client()
        .get_scheduled_status(query_params)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn update_scheduled_status() {
    let query_params =
        UpdateScheduledStatusQueryParameters::new("test-bulk-id-rust-003".to_string());
    let request_body = UpdateScheduledStatusRequestBody::new(BulkStatus::CANCELED);

    let response = get_test_email_client()
        .update_scheduled_status(query_params, request_body)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn get_delivery_reports() {
    let query_params = GetDeliveryReportsQueryParameters::default();

    let response = get_test_email_client()
        .get_delivery_reports(query_params)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn get_logs() {
    let query_params = GetLogsQueryParameters::default();

    let response = get_test_email_client()
        .get_logs(query_params)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    println!("{}", serde_json::to_string_pretty(&response.body).unwrap());
}

#[ignore]
#[tokio::test]
async fn validate_address() {
    let query_params = ValidateAddressRequestBody::new("someone@infobip.com".to_string());

    let response = get_test_email_client()
        .validate_address(query_params)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn get_domains() {
    let query_params = GetDomainsQueryParameters::default();

    let response = get_test_email_client()
        .get_domains(query_params)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    println!("{}", serde_json::to_string_pretty(&response.body).unwrap());
}

#[ignore]
#[tokio::test]
async fn add_domain() {
    let mut request_body = AddDomainRequestBody::new("test-domain-rust-001.com".to_string());
    request_body.dkim_key_length = Some(L1024);

    let response = get_test_email_client()
        .add_domain(request_body)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    println!("{}", serde_json::to_string_pretty(&response.body).unwrap());
}

#[ignore]
#[tokio::test]
async fn get_domain() {
    let response = get_test_email_client()
        .get_domain("test-domain-rust-001.com".to_string())
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    println!("{}", serde_json::to_string_pretty(&response.body).unwrap());
}

#[ignore]
#[tokio::test]
async fn delete_domain() {
    let status = get_test_email_client()
        .delete_domain("test-domain-rust-001.com".to_string())
        .await
        .unwrap();

    assert_eq!(status, StatusCode::NO_CONTENT);
}

#[ignore]
#[tokio::test]
async fn update_tracking() {
    let mut request_body = UpdateTrackingRequestBody::new();
    request_body.opens = Some(false);

    let response = get_test_email_client()
        .update_tracking("test-domain-rust-001.com".to_string(), request_body)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    println!("{}", serde_json::to_string_pretty(&response.body).unwrap());
}

#[ignore]
#[tokio::test]
async fn verify_domain() {
    let status = get_test_email_client()
        .verify_domain("test-domain-rust-001.com".to_string())
        .await
        .unwrap();

    assert_eq!(status, StatusCode::ACCEPTED);
}
