use crate::api::email::*;
use crate::api::tests::{get_test_configuration, mock_json_endpoint};
use crate::api::SdkError;
use crate::model::email::*;

const DUMMY_BASE_URL: &str = "https://some.url";

#[tokio::test]
async fn test_send_valid() {
    let expected_response = r#"
    {
      "bulkId": "4pk1xihiy4rln2f1g2se",
      "messages": [
        {
          "to": "john.smith@somecompany.com",
          "messageId": "tu5k6tdo7df1bpgk7ggs",
          "status": {
            "groupId": 1,
            "groupName": "PENDING",
            "id": 26,
            "name": "PENDING_ACCEPTED",
            "description": "Message accepted, pending for delivery."
          }
        },
        {
          "to": "jane.doe@somecompany.com",
          "messageId": "e7zzb1v9yirml2se9zo4",
          "status": {
            "groupId": 1,
            "groupName": "PENDING",
            "id": 26,
            "name": "PENDING_ACCEPTED",
            "description": "Message accepted, pending for delivery."
          }
        }
      ]
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_SEND,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = EmailClient::with_configuration(get_test_configuration(&server.base_url()));

    let request_body = SendRequestBody::new("some@mail.com".to_string());

    let response = client.send(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.messages.unwrap().is_empty());
}

#[tokio::test]
async fn test_send_invalid_request() {
    let client = EmailClient::with_configuration(get_test_configuration(DUMMY_BASE_URL));

    let request_body = SendRequestBody::new("".to_string());

    let error = client.send(request_body).await.unwrap_err();

    if let SdkError::Validation(validation_error) = error {
        assert!(!validation_error.errors().is_empty());
    } else {
        assert!(false, "not validation error");
    }
}
