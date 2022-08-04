#![cfg(feature = "sms")]
#![cfg(test)]

use std::env;

use reqwest::StatusCode;

use infobip_sdk::api::sms::{BlockingSmsClient, SmsClient};
use infobip_sdk::configuration;
use infobip_sdk::model::sms::*;

const DUMMY_TEXT: &str = "Dummy text for tests. Some special chars: áéíø";

fn get_test_sms_client() -> SmsClient {
    SmsClient::with_configuration(
        configuration::Configuration::from_env_api_key()
            .expect("failed to build default test SMS client"),
    )
}

fn get_test_blocking_sms_client() -> BlockingSmsClient {
    BlockingSmsClient::with_configuration(
        configuration::Configuration::from_env_api_key()
            .expect("failed to build default test blocking SMS client"),
    )
}

#[ignore]
#[tokio::test]
async fn preview_sms() {
    let request_body = PreviewRequestBody::new(DUMMY_TEXT.to_string());

    let response = get_test_sms_client().preview(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(response.body.previews.unwrap().len() > 0usize);
}

#[ignore]
#[test]
fn preview_sms_blocking() {
    let request_body = PreviewRequestBody::new(DUMMY_TEXT.to_string());

    let response = get_test_blocking_sms_client()
        .preview(request_body)
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(response.body.previews.unwrap().len() > 0usize);
}

#[ignore]
#[tokio::test]
async fn preview_sms_multiple() {
    let sms_client = get_test_sms_client();

    let request_body1 = PreviewRequestBody::new(DUMMY_TEXT.to_string());
    let request_body2 = PreviewRequestBody::new(DUMMY_TEXT.to_string());
    let request_body3 = PreviewRequestBody::new(DUMMY_TEXT.to_string());
    let request_body4 = PreviewRequestBody::new(DUMMY_TEXT.to_string());

    let (resp1, resp2, resp3, resp4) = tokio::join!(
        sms_client.preview(request_body1),
        sms_client.preview(request_body2),
        sms_client.preview(request_body3),
        sms_client.preview(request_body4),
    );

    assert_eq!(resp1.as_ref().unwrap().status, StatusCode::OK);
    assert_eq!(resp2.as_ref().unwrap().status, StatusCode::OK);
    assert_eq!(resp3.as_ref().unwrap().status, StatusCode::OK);
    assert_eq!(resp4.as_ref().unwrap().status, StatusCode::OK);
    assert!(
        resp1
            .as_ref()
            .unwrap()
            .body
            .previews
            .as_ref()
            .unwrap()
            .len()
            > 0usize
    );
    assert!(
        resp2
            .as_ref()
            .unwrap()
            .body
            .previews
            .as_ref()
            .unwrap()
            .len()
            > 0usize
    );
    assert!(
        resp3
            .as_ref()
            .unwrap()
            .body
            .previews
            .as_ref()
            .unwrap()
            .len()
            > 0usize
    );
    assert!(
        resp4
            .as_ref()
            .unwrap()
            .body
            .previews
            .as_ref()
            .unwrap()
            .len()
            > 0usize
    );
}

#[ignore]
#[test]
fn preview_sms_multiple_blocking() {
    let sms_client = get_test_blocking_sms_client();

    let request_body1 = PreviewRequestBody::new(DUMMY_TEXT.to_string());
    let request_body2 = PreviewRequestBody::new(DUMMY_TEXT.to_string());
    let request_body3 = PreviewRequestBody::new(DUMMY_TEXT.to_string());
    let request_body4 = PreviewRequestBody::new(DUMMY_TEXT.to_string());

    let response1 = sms_client.preview(request_body1).unwrap();
    let response2 = sms_client.preview(request_body2).unwrap();
    let response3 = sms_client.preview(request_body3).unwrap();
    let response4 = sms_client.preview(request_body4).unwrap();

    assert_eq!(response1.status, StatusCode::OK);
    assert_eq!(response2.status, StatusCode::OK);
    assert_eq!(response3.status, StatusCode::OK);
    assert_eq!(response4.status, StatusCode::OK);
    assert!(response1.body.previews.unwrap().len() > 0usize);
    assert!(response2.body.previews.unwrap().len() > 0usize);
    assert!(response3.body.previews.unwrap().len() > 0usize);
    assert!(response4.body.previews.unwrap().len() > 0usize);
}

#[ignore]
#[tokio::test]
async fn get_sms_delivery_reports() {
    let mut parameters = GetDeliveryReportsQueryParameters::new();
    parameters.limit = Some(10);

    let response = get_test_sms_client()
        .get_delivery_reports(parameters)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn send_sms() {
    let mut message = Message::new(vec![Destination::new(
        env::var("IB_TEST_DESTINATION_NUMBER").unwrap(),
    )]);
    message.text = Some(DUMMY_TEXT.to_string());

    let request_body = SendRequestBody::new(vec![message]);

    let response = get_test_sms_client().send(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.body.messages.unwrap().len(), 1usize);
}

#[ignore]
#[tokio::test]
async fn send_binary_sms() {
    let mut message = BinaryMessage::new(vec![Destination::new(
        env::var("IB_TEST_DESTINATION_NUMBER").unwrap(),
    )]);
    message.binary = Some(BinaryData::new("0f c2 4a bf 34 13 ba".to_string()));

    let mut request_body = SendBinaryRequestBody::new(vec![message]);
    request_body.bulk_id = Some("test-bulk-id-5319".to_string());

    let response = get_test_sms_client()
        .send_binary(request_body)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.body.messages.unwrap().len(), 1usize);
}
