// These tests need to be run manually, due to server state dependencies. The environment variables
// IB_API_KEY, IB_BASE_URL, and IB_TEST_DESTINATION_NUMBER must be set.

#![cfg(feature = "sms")]
#![cfg(test)]

use std::env;

use reqwest::StatusCode;

use infobip_sdk::api::sms::{BlockingSmsClient, SmsClient};
use infobip_sdk::configuration;
use infobip_sdk::model::sms::*;

const DUMMY_TEXT: &str = "Dummy text for tests. Some special chars: áéíø";
const DUMMY_BULK_ID: &str = "dummy-rust-sdk-bulk-id-3";

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

fn get_test_destination_number() -> String {
    env::var("IB_TEST_DESTINATION_NUMBER").expect("failed to load test destination number")
}

#[ignore]
#[tokio::test]
async fn preview_sms() {
    let request_body = PreviewRequestBody::new(DUMMY_TEXT.to_string());

    let response = get_test_sms_client().preview(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.previews.unwrap().is_empty());
}

#[ignore]
#[test]
fn preview_sms_blocking() {
    let request_body = PreviewRequestBody::new(DUMMY_TEXT.to_string());

    let response = get_test_blocking_sms_client()
        .preview(request_body)
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert!(!response.body.previews.unwrap().is_empty());
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
    assert!(!resp1
        .as_ref()
        .unwrap()
        .body
        .previews
        .as_ref()
        .unwrap()
        .is_empty());
    assert!(!resp2
        .as_ref()
        .unwrap()
        .body
        .previews
        .as_ref()
        .unwrap()
        .is_empty());
    assert!(!resp3
        .as_ref()
        .unwrap()
        .body
        .previews
        .as_ref()
        .unwrap()
        .is_empty());
    assert!(!resp4
        .as_ref()
        .unwrap()
        .body
        .previews
        .as_ref()
        .unwrap()
        .is_empty());
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
    assert!(!response1.body.previews.unwrap().is_empty());
    assert!(!response2.body.previews.unwrap().is_empty());
    assert!(!response3.body.previews.unwrap().is_empty());
    assert!(!response4.body.previews.unwrap().is_empty());
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
    let mut message = Message::new(vec![Destination::new(get_test_destination_number())]);
    message.text = Some(DUMMY_TEXT.to_string());

    let request_body = SendRequestBody::new(vec![message]);

    let response = get_test_sms_client().send(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.body.messages.unwrap().len(), 1usize);
}

#[ignore]
#[tokio::test]
async fn send_bulk_sms() {
    let mut message = Message::new(vec![Destination::new(get_test_destination_number())]);
    message.text = Some(DUMMY_TEXT.to_string());
    message.send_at = Some("2022-10-10T00:00:00Z".to_string());

    let mut request_body = SendRequestBody::new(vec![message]);
    request_body.bulk_id = Some(DUMMY_BULK_ID.to_string());

    let response = get_test_sms_client().send(request_body).await.unwrap();

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.body.messages.unwrap().len(), 1usize);
}

#[ignore]
#[tokio::test]
async fn send_binary_sms() {
    let mut message = BinaryMessage::new(vec![Destination::new(get_test_destination_number())]);
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

#[ignore]
#[tokio::test]
async fn get_logs() {
    let query_parameters = GetLogsQueryParameters::new();
    let response = get_test_sms_client()
        .get_logs(query_parameters)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn get_inbound_reports() {
    let query_parameters = GetInboundReportsQueryParameters::new();
    let response = get_test_sms_client()
        .get_inbound_reports(query_parameters)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn send_over_query_parameters() {
    let destinations = vec!["31612345678".to_string(), "31698765432".to_string()];
    let query_parameters = SendOverQueryParametersQueryParameters::new(
        "username".to_string(),
        "password".to_string(),
        destinations,
    );

    let response = get_test_sms_client()
        .send_over_query_parameters(query_parameters)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn get_scheduled() {
    let query_parameters = GetScheduledStatusQueryParameters::new(DUMMY_BULK_ID.to_string());

    let response = get_test_sms_client()
        .get_scheduled_status(query_parameters)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn get_scheduled_status() {
    let query_parameters = GetScheduledStatusQueryParameters::new(DUMMY_BULK_ID.to_string());

    let response = get_test_sms_client()
        .get_scheduled_status(query_parameters)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn reschedule() {
    let query_parameters = RescheduleQueryParameters::new(DUMMY_BULK_ID.to_string());
    let request_body = RescheduleRequestBody::new("2022-10-02T00:00:00".to_string());

    let response = get_test_sms_client()
        .reschedule(query_parameters, request_body)
        .await
        .unwrap();

    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn update_scheduled_status() {
    let query_parameters = UpdateScheduledStatusQueryParameters::new(DUMMY_BULK_ID.to_string());
    let request_body = UpdateScheduledStatusRequestBody::new(ScheduledStatus::CANCELED);

    let response = get_test_sms_client()
        .update_scheduled_status(query_parameters, request_body)
        .await
        .unwrap();
    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn get_tfa_applications() {
    let response = get_test_sms_client().get_tfa_applications().await.unwrap();

    println!("{:?}", response.body);
    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn create_tfa_application() {
    let request_body = CreateTfaApplicationRequestBody::new("rust-application".to_string());

    let response = get_test_sms_client()
        .create_tfa_application(request_body)
        .await
        .unwrap();

    println!("{:?}", response.body);
    assert_eq!(response.status, StatusCode::CREATED);
}

#[ignore]
#[tokio::test]
async fn get_tfa_application() {
    let response = get_test_sms_client()
        .get_tfa_application("02CC3CAAFD733136AA15DFAC720A0C42")
        .await
        .unwrap();

    println!("{:?}", response.body);
    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn update_tfa_application() {
    let configuration = TfaApplicationConfiguration {
        allow_multiple_pin_verifications: Some(true),
        pin_attempts: None,
        pin_time_to_live: None,
        send_pin_per_application_limit: Some("5010/12h".to_string()),
        send_pin_per_phone_number_limit: None,
        verify_pin_limit: None,
    };
    let mut request_body = UpdateTfaApplicationRequestBody::new("rust-application-2".to_string());
    request_body.configuration = Some(configuration);

    let response = get_test_sms_client()
        .update_tfa_application("02CC3CAAFD733136AA15DFAC720A0C42", request_body)
        .await
        .unwrap();

    println!("{:?}", response.body);
    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn get_tfa_message_templates() {
    let response = get_test_sms_client()
        .get_tfa_message_templates("02CC3CAAFD733136AA15DFAC720A0C42")
        .await
        .unwrap();

    println!("{:?}", response.body);
    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn create_tfa_message_template() {
    let request_body = CreateTfaMessageTemplateRequestBody::new(
        "Your Rust PIN 2 is {{pin}}".to_string(),
        PinType::Numeric,
        6,
    );

    let response = get_test_sms_client()
        .create_tfa_message_template("02CC3CAAFD733136AA15DFAC720A0C42", request_body)
        .await
        .unwrap();

    println!("{:?}", response.body);
    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn get_tfa_message_template() {
    let response = get_test_sms_client()
        .get_tfa_message_template(
            "02CC3CAAFD733136AA15DFAC720A0C42",
            "44A45DA3067F882BB4D87D6A48F9681E",
        )
        .await
        .unwrap();

    println!("{:?}", response.body);
    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn update_tfa_message_template() {
    let request_body = UpdateTfaMessageTemplateRequestBody::new(
        "Your Rust PIN 3 is {{pin}}".to_string(),
        PinType::Numeric,
        6,
    );

    let response = get_test_sms_client()
        .update_tfa_message_template(
            "02CC3CAAFD733136AA15DFAC720A0C42",
            "44A45DA3067F882BB4D87D6A48F9681E",
            request_body,
        )
        .await
        .unwrap();

    println!("{:?}", response.body);
    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn send_pin_over_sms() {
    let query_parameters = SendPinOverSmsQueryParameters::new();
    let request_body = SendPinOverSmsRequestBody::new(
        "02CC3CAAFD733136AA15DFAC720A0C42".to_string(),
        "44A45DA3067F882BB4D87D6A48F9681E".to_string(),
        "523311800428".to_string(),
    );

    let response = get_test_sms_client()
        .send_pin_over_sms(query_parameters, request_body)
        .await
        .unwrap();

    println!("{:?}", response.body);
    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn resend_pin_over_sms() {
    let request_body = ResendPinOverSmsRequestBody::default();

    let response = get_test_sms_client()
        .resend_pin_over_sms("AAA30929B83F2ED86CC34781BCB7A546", request_body)
        .await
        .unwrap();

    println!("{:?}", response.body);
    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn send_pin_over_voice() {
    let request_body = SendPinOverVoiceRequestBody::new(
        "02CC3CAAFD733136AA15DFAC720A0C42".to_string(),
        "44A45DA3067F882BB4D87D6A48F9681E".to_string(),
        "523311800428".to_string(),
    );

    let response = get_test_sms_client()
        .send_pin_over_voice(request_body)
        .await
        .unwrap();

    println!("{:?}", response.body);
    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn resend_pin_over_voice() {
    let request_body = ResendPinOverVoiceRequestBody::default();

    let response = get_test_sms_client()
        .resend_pin_over_voice("AAA30929B83F2ED86CC34781BCB7A546", request_body)
        .await
        .unwrap();

    println!("{:?}", response.body);
    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn verify_phone_number() {
    let request_body = VerifyPhoneNumberRequestBody::new("123456".to_string());

    let response = get_test_sms_client()
        .verify_phone_number("AAA30929B83F2ED86CC34781BCB7A546", request_body)
        .await
        .unwrap();

    println!("{:?}", response.body);
    assert_eq!(response.status, StatusCode::OK);
}

#[ignore]
#[tokio::test]
async fn get_tfa_verification_status() {
    let query_parameters = GetTfaVerificationStatusQueryParameters::new("523311800428".to_string());
    let response = get_test_sms_client()
        .get_tfa_verification_status("02CC3CAAFD733136AA15DFAC720A0C42", query_parameters)
        .await
        .unwrap();

    println!("{:?}", response.body);
    assert_eq!(response.status, StatusCode::OK);
}
