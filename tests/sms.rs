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
    let request_body = PreviewSmsRequestBodyBuilder::default()
        .text(DUMMY_TEXT.to_string())
        .build()
        .unwrap();

    let response = get_test_sms_client().preview(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(response.response_body.previews.unwrap().len() > 0usize);
}

#[ignore]
#[test]
fn preview_sms_blocking() {
    let request_body = PreviewSmsRequestBodyBuilder::default()
        .text(DUMMY_TEXT.to_string())
        .build()
        .unwrap();

    let response = get_test_blocking_sms_client()
        .preview(request_body)
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(response.response_body.previews.unwrap().len() > 0usize);
}

#[ignore]
#[tokio::test]
async fn preview_sms_multiple() {
    let sms_client = get_test_sms_client();

    let request_body1 = PreviewSmsRequestBodyBuilder::default()
        .text(DUMMY_TEXT.to_string())
        .build()
        .unwrap();
    let request_body2 = PreviewSmsRequestBodyBuilder::default()
        .text(DUMMY_TEXT.to_string())
        .build()
        .unwrap();
    let request_body3 = PreviewSmsRequestBodyBuilder::default()
        .text(DUMMY_TEXT.to_string())
        .build()
        .unwrap();
    let request_body4 = PreviewSmsRequestBodyBuilder::default()
        .text(DUMMY_TEXT.to_string())
        .build()
        .unwrap();

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
            .response_body
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
            .response_body
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
            .response_body
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
            .response_body
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

    let request_body1 = PreviewSmsRequestBodyBuilder::default()
        .text(DUMMY_TEXT.to_string())
        .build()
        .unwrap();
    let request_body2 = PreviewSmsRequestBodyBuilder::default()
        .text(DUMMY_TEXT.to_string())
        .build()
        .unwrap();
    let request_body3 = PreviewSmsRequestBodyBuilder::default()
        .text(DUMMY_TEXT.to_string())
        .build()
        .unwrap();
    let request_body4 = PreviewSmsRequestBodyBuilder::default()
        .text(DUMMY_TEXT.to_string())
        .build()
        .unwrap();

    let response1 = sms_client.preview(request_body1).unwrap();
    let response2 = sms_client.preview(request_body2).unwrap();
    let response3 = sms_client.preview(request_body3).unwrap();
    let response4 = sms_client.preview(request_body4).unwrap();

    assert_eq!(response1.status, StatusCode::OK);
    assert_eq!(response2.status, StatusCode::OK);
    assert_eq!(response3.status, StatusCode::OK);
    assert_eq!(response4.status, StatusCode::OK);
    assert!(response1.response_body.previews.unwrap().len() > 0usize);
    assert!(response2.response_body.previews.unwrap().len() > 0usize);
    assert!(response3.response_body.previews.unwrap().len() > 0usize);
    assert!(response4.response_body.previews.unwrap().len() > 0usize);
}

#[ignore]
#[tokio::test]
async fn get_sms_delivery_reports() {
    let parameters = GetDeliveryReportsQueryParametersBuilder::default()
        .limit(10)
        .build()
        .unwrap();

    let response = get_test_sms_client()
        .get_delivery_reports(parameters)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn send_sms() {
    let destination = DestinationBuilder::default()
        .to(env::var("IB_TEST_DESTINATION_NUMBER").unwrap())
        .build()
        .unwrap();

    let message = SmsMessageBuilder::default()
        .destinations(vec![destination])
        .text(DUMMY_TEXT.to_string())
        .build()
        .unwrap();

    let request_body = SendSmsRequestBodyBuilder::default()
        .messages(vec![message])
        .build()
        .unwrap();

    let response = get_test_sms_client().send(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    println!("{:?}", response.response_body);
}
