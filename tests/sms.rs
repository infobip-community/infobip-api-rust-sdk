use reqwest::StatusCode;

use infobip_sdk::api::sms::SmsClient;
use infobip_sdk::configuration;
use infobip_sdk::model::sms::PreviewSmsRequestBodyBuilder;

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
