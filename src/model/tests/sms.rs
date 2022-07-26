use validator::Validate;

use crate::model::sms::PreviewSmsRequestBodyBuilder;

#[test]
fn test_sms_preview_request_body_valid() {
    let request_body = PreviewSmsRequestBodyBuilder::default()
        .text("Some text to be previewed".to_string())
        .language_code("ES".to_string())
        .transliteration("GREEK".to_string())
        .build()
        .unwrap();

    assert!(request_body.validate().is_ok())
}

#[test]
fn test_sms_preview_request_body_invalid_language_code() {
    let request_body = PreviewSmsRequestBodyBuilder::default()
        .text("Some text to be previewed".to_string())
        .language_code("BAD".to_string())
        .build()
        .unwrap();

    assert!(request_body.validate().is_err())
}
