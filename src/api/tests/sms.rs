use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::api::sms::*;
use crate::configuration::{ApiKey, Configuration};
use crate::model::sms::*;

#[tokio::test]
async fn test_valid_preview() {
    let mock_server = MockServer::start().await;

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

    Mock::given(method(reqwest::Method::POST.as_str()))
        .and(path(PATH_PREVIEW))
        .respond_with(
            ResponseTemplate::new(reqwest::StatusCode::OK)
                .set_body_raw(expected_response, "application/json"),
        )
        .mount(&mock_server)
        .await;

    let mut config = Configuration::with_api_key(ApiKey {
        key: "some-api-key".to_string(),
        prefix: None,
    });
    config.base_path = mock_server.uri();

    let client = SmsClient::with_configuration(config);

    let request_body = PreviewSmsRequestBodyBuilder::default()
        .text("Some text to preview".to_string())
        .build()
        .unwrap();

    let response = client.preview(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.response_body.original_text.unwrap().is_empty());
    assert!(!response.response_body.previews.unwrap().len() > 0usize);
}
