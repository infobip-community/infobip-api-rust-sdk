use validator::Validate;

use crate::model::sms::*;

const DUMMY_TEXT: &str = "Dummy text for tests. Some special chars: áéíø";

#[test]
fn test_sms_preview_request_body_valid() {
    let request_body = PreviewRequestBodyBuilder::default()
        .text(DUMMY_TEXT.to_string())
        .language_code("ES".to_string())
        .transliteration("GREEK".to_string())
        .build()
        .unwrap();

    assert!(request_body.validate().is_ok())
}

#[test]
fn test_sms_preview_request_body_invalid_language_code() {
    let request_body = PreviewRequestBodyBuilder::default()
        .text(DUMMY_TEXT.to_string())
        .language_code("BAD".to_string())
        .build()
        .unwrap();

    assert!(request_body.validate().is_err())
}

#[test]
fn test_sms_preview_request_body_invalid_transliteration() {
    let request_body = PreviewRequestBodyBuilder::default()
        .text(DUMMY_TEXT.to_string())
        .transliteration("BAD".to_string())
        .build()
        .unwrap();

    assert!(request_body.validate().is_err())
}

#[test]
fn test_get_delivery_reports_query_parameters_valid() {
    let parameters = GetDeliveryReportsQueryParametersBuilder::default()
        .limit(10)
        .build()
        .unwrap();

    assert!(parameters.validate().is_ok())
}

#[test]
fn test_get_delivery_reports_query_parameters_big_limit() {
    let parameters = GetDeliveryReportsQueryParametersBuilder::default()
        .limit(10000)
        .build()
        .unwrap();

    assert!(parameters.validate().is_err())
}
