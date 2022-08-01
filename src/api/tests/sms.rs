use crate::api::sms::*;
use crate::api::tests::{get_test_configuration, mock_blocking_json_endpoint, mock_json_endpoint};
use crate::model::sms::*;

#[tokio::test]
async fn test_valid_preview() {
    let expected_response = r#"
       {
          "originalText": "Let's see how many characters remain unused in this message.",
          "previews": [
            {
              "textPreview": "Let's see how many characters remain unused in this message.",
              "messageCount": 1,
              "charactersRemaining": 96,
              "configuration": {}
            }
          ]
       }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_PREVIEW,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(get_test_configuration(server.base_url()));

    let request_body = PreviewSmsRequestBodyBuilder::default()
        .text("Some text to preview".to_string())
        .build()
        .unwrap();

    let response = client.preview(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.response_body.original_text.unwrap().is_empty());
    assert!(response.response_body.previews.unwrap().len() > 0usize);
}

#[test]
fn test_valid_blocking_preview() {
    let expected_response = r#"
       {
          "originalText": "Let's see how many characters remain unused in this message.",
          "previews": [
            {
              "textPreview": "Let's see how many characters remain unused in this message.",
              "messageCount": 1,
              "charactersRemaining": 96,
              "configuration": {}
            }
          ]
       }
    "#;

    let mock_server = mock_blocking_json_endpoint(
        httpmock::Method::POST,
        PATH_PREVIEW,
        expected_response,
        reqwest::StatusCode::OK,
    );

    let client =
        BlockingSmsClient::with_configuration(get_test_configuration(mock_server.base_url()));

    let request_body = PreviewSmsRequestBodyBuilder::default()
        .text("Some text to preview".to_string())
        .build()
        .unwrap();

    let response = client.preview(request_body).unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.response_body.original_text.unwrap().is_empty());
    assert!(response.response_body.previews.unwrap().len() > 0usize);
}
