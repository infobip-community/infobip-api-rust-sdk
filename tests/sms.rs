#![cfg(test)]

use reqwest::StatusCode;

use infobip_sdk::api::sms::{BlockingSmsClient, SmsClient};
use infobip_sdk::configuration;
use infobip_sdk::model::sms::{
    GetDeliveryReportsQueryParametersBuilder, PreviewSmsRequestBodyBuilder,
};

#[ignore]
#[tokio::test]
async fn preview_sms() {
    let config = configuration::Configuration::from_env_api_key()
        .expect("error reading API key or base URL");
    let sms_client = SmsClient::with_configuration(config);

    let request_body = PreviewSmsRequestBodyBuilder::default()
        .text("Some text to preview.".to_string())
        .build()
        .unwrap();

    let response = sms_client.preview(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(response.response_body.previews.unwrap().len() > 0usize);
}

#[ignore]
#[test]
fn preview_sms_blocking() {
    let config = configuration::Configuration::from_env_api_key()
        .expect("error reading API key or base URL");
    let sms_client = BlockingSmsClient::with_configuration(config);

    let request_body = PreviewSmsRequestBodyBuilder::default()
        .text("Some text to preview.".to_string())
        .build()
        .unwrap();

    let response = sms_client.preview(request_body).unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(response.response_body.previews.unwrap().len() > 0usize);
}

#[ignore]
#[tokio::test]
async fn preview_sms_multiple() {
    let config = configuration::Configuration::from_env_api_key()
        .expect("error reading API key or base URL");
    let sms_client = SmsClient::with_configuration(config);

    let request_body1 = PreviewSmsRequestBodyBuilder::default()
        .text("Some text to preview 1.".to_string())
        .build()
        .unwrap();
    let request_body2 = PreviewSmsRequestBodyBuilder::default()
        .text("Some text to preview 2.".to_string())
        .build()
        .unwrap();
    let request_body3 = PreviewSmsRequestBodyBuilder::default()
        .text("Some text to preview 3.".to_string())
        .build()
        .unwrap();
    let request_body4 = PreviewSmsRequestBodyBuilder::default()
        .text("Some text to preview 4.".to_string())
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
    let config = configuration::Configuration::from_env_api_key()
        .expect("error reading API key or base URL");
    let sms_client = BlockingSmsClient::with_configuration(config);

    let request_body1 = PreviewSmsRequestBodyBuilder::default()
        .text("Some text to preview.".to_string())
        .build()
        .unwrap();
    let request_body2 = PreviewSmsRequestBodyBuilder::default()
        .text("Some text to preview 2.".to_string())
        .build()
        .unwrap();
    let request_body3 = PreviewSmsRequestBodyBuilder::default()
        .text("Some text to preview 3.".to_string())
        .build()
        .unwrap();
    let request_body4 = PreviewSmsRequestBodyBuilder::default()
        .text("Some text to preview 4.".to_string())
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
    let config = configuration::Configuration::from_env_api_key()
        .expect("error reading API key or base URL");
    let sms_client = SmsClient::with_configuration(config);

    let parameters = GetDeliveryReportsQueryParametersBuilder::default()
        .limit(10)
        .build()
        .unwrap();

    let response = sms_client.get_delivery_reports(parameters).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
}
