use crate::api::SdkError::ApiRequestError;
use crate::api::tests::{get_test_configuration, mock_json_endpoint};
use crate::api::whatsapp::*;
use crate::model::whatsapp::*;

fn get_dummy_send_text_request_body() -> SendTextRequestBody {
    SendTextRequestBody::new(
        "44444444444".to_string(),
        "55555555555".to_string(),
        TextContent::new("some text".to_string()),
    )
}

#[tokio::test]
async fn send_text_valid() {
    let request_body: SendTextRequestBody = serde_json::from_str(
        r#"
        {
          "from": "441134960000",
          "to": "441134960001",
          "messageId": "a28dd97c-1ffb-4fcf-99f1-0b557ed381da",
          "content": {
            "text": "Some text with url: http://example.com",
            "previewUrl": true
          },
          "callbackData": "Callback data",
          "notifyUrl": "https://www.example.com/whatsapp"
        }"#
    ).unwrap();

    let expected_response = r#"
        {
          "to": "441134960001",
          "messageCount": 1,
          "messageId": "a28dd97c-1ffb-4fcf-99f1-0b557ed381da",
          "status": {
            "groupId": 1,
            "groupName": "PENDING",
            "id": 7,
            "name": "PENDING_ENROUTE",
            "description": "Message sent to next instance"
          }
        }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_SEND_TEXT,
        expected_response,
        reqwest::StatusCode::OK,
    )
        .await;

    let wa_client = WhatsappClient::with_configuration(get_test_configuration(&server.base_url()));

    let response = wa_client.send_text(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[tokio::test]
async fn send_text_api_error() {
    let request_body = SendTextRequestBody::new(
        "44444444444".to_string(),
        "55555555555".to_string(),
        TextContent::new("some text".to_string()),
    );

    let expected_response = r#"
        {
          "requestError": {
            "serviceException": {
              "messageId": "BAD_REQUEST",
              "text": "Bad request",
              "validationErrors": {
                "content.text": [
                  "size must be between 1 and 4096",
                  "must not be blank"
                ]
              }
            }
          }
        }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_SEND_TEXT,
        expected_response,
        reqwest::StatusCode::BAD_REQUEST,
    )
        .await;

    let wa_client = WhatsappClient::with_configuration(get_test_configuration(&server.base_url()));

    let sdk_error = wa_client.send_text(request_body).await.err().unwrap();

    match sdk_error {
        ApiRequestError(api_error) => {
            assert_eq!(api_error.status, reqwest::StatusCode::BAD_REQUEST);
            assert_eq!(api_error.details.request_error.service_exception.message_id.unwrap(), "BAD_REQUEST".to_string());
        }
        _ => {
            panic!("unexpected error")
        },
    }
}

#[tokio::test]
async fn send_text_api_error_401() {
    let expected_response = r#"
        {
          "requestError": {
            "serviceException": {
              "messageId": "UNAUTHORIZED",
              "text": "Invalid login details"
            }
          }
        }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_SEND_TEXT,
        expected_response,
        reqwest::StatusCode::UNAUTHORIZED,
    )
        .await;

    let wa_client = WhatsappClient::with_configuration(get_test_configuration(&server.base_url()));

    let sdk_error = wa_client.send_text(get_dummy_send_text_request_body()).await.err().unwrap();

    match sdk_error {
        ApiRequestError(api_error) => {
            assert_eq!(api_error.status, reqwest::StatusCode::UNAUTHORIZED);
            assert_eq!(api_error.details.request_error.service_exception.message_id.unwrap(), "UNAUTHORIZED".to_string());
        }
        _ => {
            panic!("unexpected error")
        },
    }
}

#[tokio::test]
async fn send_text_api_error_429() {
    let expected_response = r#"
        {
            "requestError": {
                "serviceException": {
                    "messageId": "TOO_MANY_REQUESTS",
                    "text": "Too many requests"
                }
            }
        }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_SEND_TEXT,
        expected_response,
        reqwest::StatusCode::TOO_MANY_REQUESTS,
    )
        .await;

    let wa_client = WhatsappClient::with_configuration(get_test_configuration(&server.base_url()));

    let sdk_error = wa_client.send_text(get_dummy_send_text_request_body()).await.err().unwrap();

    match sdk_error {
        ApiRequestError(api_error) => {
            assert_eq!(api_error.status, reqwest::StatusCode::TOO_MANY_REQUESTS);
            assert_eq!(api_error.details.request_error.service_exception.message_id.unwrap(), "TOO_MANY_REQUESTS".to_string());
        }
        _ => {
            panic!("unexpected error")
        },
    }
}
