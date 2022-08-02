use crate::api::sms::*;
use crate::api::tests::{get_test_configuration, mock_blocking_json_endpoint, mock_json_endpoint};
use crate::api::SdkError;
use crate::model::sms::*;

const DUMMY_BASE_URL: &str = "https://some.url";

#[tokio::test]
async fn test_preview_valid() {
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

    let client = SmsClient::with_configuration(get_test_configuration(&server.base_url()));

    let request_body = PreviewRequestBodyBuilder::default()
        .text("Some text to preview".to_string())
        .build()
        .unwrap();

    let response = client.preview(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.response_body.original_text.unwrap().is_empty());
    assert!(response.response_body.previews.unwrap().len() > 0usize);
}

#[tokio::test]
async fn test_preview_bad_request() {
    let client = SmsClient::with_configuration(get_test_configuration(DUMMY_BASE_URL));

    let request_body = PreviewRequestBodyBuilder::default()
        .text("Some text to preview".to_string())
        .language_code("XX".to_string())
        .build()
        .unwrap();

    let error = client.preview(request_body).await.unwrap_err();

    if let SdkError::Validation(validation_error) = error {
        assert!(!validation_error.errors().is_empty());
    } else {
        assert!(false, "not validation error")
    }
}

#[test]
fn test_blocking_preview_valid() {
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
        BlockingSmsClient::with_configuration(get_test_configuration(&mock_server.base_url()));

    let request_body = PreviewRequestBodyBuilder::default()
        .text("Some text to preview".to_string())
        .build()
        .unwrap();

    let response = client.preview(request_body).unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.response_body.original_text.unwrap().is_empty());
    assert!(response.response_body.previews.unwrap().len() > 0usize);
}

#[tokio::test]
async fn test_preview_server_error() {
    let expected_response = r#"
        {
          "requestError": {
            "serviceException": {
              "messageId": "string",
              "text": "string"
            }
          }
        }
    "#;
    let expected_status = reqwest::StatusCode::UNAUTHORIZED;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_PREVIEW,
        expected_response,
        expected_status,
    )
    .await;

    let client = SmsClient::with_configuration(get_test_configuration(&server.base_url()));

    let request_body = PreviewRequestBodyBuilder::default()
        .text("Some text to preview".to_string())
        .build()
        .unwrap();

    let error = client.preview(request_body).await.unwrap_err();
    if let SdkError::ApiRequestError(api_error) = error {
        assert_eq!(api_error.status, expected_status);
        assert!(!api_error
            .details
            .request_error
            .service_exception
            .text
            .is_empty())
    } else {
        assert!(false, "not an API error")
    }
}

#[tokio::test]
async fn test_get_delivery_reports_valid() {
    let expected_response = r#"
        {
          "results": [
            {
              "bulkId": "BULK-ID-123-xyz",
              "messageId": "MESSAGE-ID-123-xyz",
              "to": "41793026727",
              "sentAt": "2019-11-09T16:00:00.000+0000",
              "doneAt": "2019-11-09T16:00:00.000+0000",
              "smsCount": 1,
              "price": {
                "pricePerMessage": 0.01,
                "currency": "EUR"
              },
              "status": {
                "groupId": 3,
                "groupName": "DELIVERED",
                "id": 5,
                "name": "DELIVERED_TO_HANDSET",
                "description": "Message delivered to handset"
              },
              "error": {
                "groupId": 0,
                "groupName": "Ok",
                "id": 0,
                "name": "NO_ERROR",
                "description": "No Error",
                "permanent": false
              }
            },
            {
              "bulkId": "BULK-ID-123-xyz",
              "messageId": "12db39c3-7822-4e72-a3ec-c87442c0ffc5",
              "to": "41793026834",
              "sentAt": "2019-11-09T17:00:00.000+0000",
              "doneAt": "2019-11-09T17:00:00.000+0000",
              "smsCount": 1,
              "price": {
                "pricePerMessage": 0.01,
                "currency": "EUR"
              },
              "status": {
                "groupId": 3,
                "groupName": "DELIVERED",
                "id": 5,
                "name": "DELIVERED_TO_HANDSET",
                "description": "Message delivered to handset"
              },
              "error": {
                "groupId": 0,
                "groupName": "Ok",
                "id": 0,
                "name": "NO_ERROR",
                "description": "No Error",
                "permanent": false
              }
            }
          ]
        }
    "#;
    let expected_status = reqwest::StatusCode::OK;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_DELIVERY_REPORTS,
        expected_response,
        expected_status,
    )
    .await;

    let client = SmsClient::with_configuration(get_test_configuration(&server.base_url()));

    let query_parameters = GetDeliveryReportsQueryParametersBuilder::default()
        .limit(10)
        .build()
        .unwrap();

    let response = client.get_delivery_reports(query_parameters).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(response.response_body.results.as_ref().unwrap().len() > 1);
    assert!(!response.response_body.results.as_ref().unwrap()[0]
        .bulk_id
        .as_ref()
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn test_get_delivery_reports_bad_parameters() {
    let client = SmsClient::with_configuration(get_test_configuration(DUMMY_BASE_URL));

    let query_parameters = GetDeliveryReportsQueryParametersBuilder::default()
        .limit(10000)
        .build()
        .unwrap();

    let error = client
        .get_delivery_reports(query_parameters)
        .await
        .unwrap_err();
    if let SdkError::Validation(validation_error) = error {
        assert!(!validation_error.errors().is_empty());
    } else {
        assert!(false, "not validation error")
    }
}
