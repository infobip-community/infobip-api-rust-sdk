use validator::Validate;

use crate::model::sms::*;

const DUMMY_TEXT: &str = "Dummy text for tests. Some special chars: áéíø";

#[test]
fn test_sms_preview_request_body_valid() {
    let mut request_body = PreviewRequestBody::new(DUMMY_TEXT.to_string());
    request_body.language_code = Some("ES".to_string());
    request_body.transliteration = Some("GREEK".to_string());

    assert!(request_body.validate().is_ok())
}

#[test]
fn test_sms_preview_request_body_invalid_language_code() {
    let mut request_body = PreviewRequestBody::new(DUMMY_TEXT.to_string());
    request_body.language_code = Some("BAD".to_string());

    assert!(request_body.validate().is_err())
}

#[test]
fn test_sms_preview_request_body_invalid_transliteration() {
    let mut request_body = PreviewRequestBody::new(DUMMY_TEXT.to_string());
    request_body.transliteration = Some("BAD".to_string());

    assert!(request_body.validate().is_err())
}

#[test]
fn test_get_delivery_reports_query_parameters_valid() {
    let mut parameters = GetDeliveryReportsQueryParameters::new();
    parameters.limit = Some(10);

    assert!(parameters.validate().is_ok())
}

#[test]
fn test_get_delivery_reports_query_parameters_big_limit() {
    let mut parameters = GetDeliveryReportsQueryParameters::new();
    parameters.limit = Some(10000);

    assert!(parameters.validate().is_err())
}

#[test]
fn test_send_request_body_valid() {
    let mut message = Message::new(vec![Destination::new("123456789012".to_string())]);
    message.text = Some(DUMMY_TEXT.to_string());

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_ok())
}
