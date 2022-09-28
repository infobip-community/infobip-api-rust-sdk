use crate::api::sms::*;
use crate::api::tests::{
    get_test_configuration, mock_blocking_json_endpoint, mock_json_endpoint, DUMMY_TEXT,
};
use crate::api::SdkError;
use crate::model::sms::ScheduledStatus::PAUSED;
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

    let request_body = PreviewRequestBody::new(DUMMY_TEXT.to_string());

    let response = client.preview(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.original_text.unwrap().is_empty());
    assert!(!response.body.previews.unwrap().is_empty());
}

#[tokio::test]
async fn test_preview_bad_request() {
    let client = SmsClient::with_configuration(get_test_configuration(DUMMY_BASE_URL));

    let mut request_body = PreviewRequestBody::new(DUMMY_TEXT.to_string());
    request_body.language_code = Some("XX".to_string());

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

    let request_body = PreviewRequestBody::new(DUMMY_TEXT.to_string());

    let response = client.preview(request_body).unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.original_text.unwrap().is_empty());
    assert!(!response.body.previews.unwrap().is_empty());
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

    let request_body = PreviewRequestBody::new(DUMMY_TEXT.to_string());

    let error = client.preview(request_body).await.unwrap_err();
    if let SdkError::ApiRequestError(api_error) = error {
        assert_eq!(api_error.status, expected_status);
        assert!(!api_error
            .details
            .request_error
            .service_exception
            .text
            .unwrap()
            .is_empty());
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
        PATH_GET_DELIVERY_REPORTS,
        expected_response,
        expected_status,
    )
    .await;

    let client = SmsClient::with_configuration(get_test_configuration(&server.base_url()));

    let mut query_parameters = GetDeliveryReportsQueryParameters::new();
    query_parameters.limit = Some(10);

    let response = client.get_delivery_reports(query_parameters).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(response.body.results.as_ref().unwrap().len() > 1);
    assert!(!response.body.results.as_ref().unwrap()[0]
        .bulk_id
        .as_ref()
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn test_get_delivery_reports_bad_parameters() {
    let client = SmsClient::with_configuration(get_test_configuration(DUMMY_BASE_URL));

    let mut query_parameters = GetDeliveryReportsQueryParameters::new();
    query_parameters.limit = Some(10000);

    let error = client
        .get_delivery_reports(query_parameters)
        .await
        .unwrap_err();
    if let SdkError::Validation(validation_error) = error {
        assert!(!validation_error.errors().is_empty());
    }
}

#[tokio::test]
async fn test_send_valid() {
    let expected_response = r#"
    {
      "bulkId": "2034072219640523073",
      "messages": [
        {
          "messageId": "41793026727",
          "status": {
            "description": "Message sent to next instance",
            "groupId": 1,
            "groupName": "PENDING",
            "id": 26,
            "name": "MESSAGE_ACCEPTED"
          },
          "to": "2033247207850523791"
        },
        {
          "messageId": "41793026834",
          "status": {
            "description": "Message sent to next instance",
            "groupId": 1,
            "groupName": "PENDING",
            "id": 26,
            "name": "MESSAGE_ACCEPTED"
          },
          "to": "2033247207850523792"
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

    let client = SmsClient::with_configuration(get_test_configuration(&server.base_url()));

    let message = Message::new(vec![Destination::new("123456789101".to_string())]);
    let request_body = SendRequestBody::new(vec![message]);

    let response = client.send(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.messages.unwrap().is_empty());
}

#[tokio::test]
async fn test_send_binary_valid() {
    let expected_response = r#"
    {
      "bulkId": "2034072219640523073",
      "messages": [
        {
          "messageId": "41793026727",
          "status": {
            "description": "Message sent to next instance",
            "groupId": 1,
            "groupName": "PENDING",
            "id": 26,
            "name": "MESSAGE_ACCEPTED"
          },
          "to": "2033247207850523791"
        },
        {
          "messageId": "41793026834",
          "status": {
            "description": "Message sent to next instance",
            "groupId": 1,
            "groupName": "PENDING",
            "id": 26,
            "name": "MESSAGE_ACCEPTED"
          },
          "to": "2033247207850523792"
        }
      ]
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_SEND_BINARY,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(get_test_configuration(&server.base_url()));

    let message = BinaryMessage::new(vec![Destination::new("123456789101".to_string())]);
    let request_body = SendBinaryRequestBody::new(vec![message]);

    let response = client.send_binary(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.messages.unwrap().is_empty());
}

#[tokio::test]
async fn test_send_over_query_parameters_valid() {
    let expected_response = r#"
    {
      "bulkId": "1478260834465349756",
      "messages": [
        {
          "to": "41793026727",
          "status": {
            "groupId": 1,
            "groupName": "PENDING",
            "id": 26,
            "name": "PENDING_ACCEPTED",
            "description": "Message sent to next instance"
          },
          "messageId": "2250be2d4219-3af1-78856-aabe-1362af1edfd2"
        }
      ]
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_SEND_OVER_QUERY_PARAMS,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(get_test_configuration(&server.base_url()));

    let query_parameters = SendOverQueryParametersQueryParameters::new(
        "username".to_string(),
        "password".to_string(),
        vec!["41793026727".to_string()],
    );

    let response = client
        .send_over_query_parameters(query_parameters)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.messages.unwrap().is_empty());
}

#[tokio::test]
async fn test_get_scheduled_valid() {
    let expected_response = r#"
        {
          "bulkId": "BULK-ID-123-xyz",
          "sendAt": "2021-08-25T16:00:00.000+0000"
        }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_GET_SCHEDULED,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(get_test_configuration(&server.base_url()));

    let query_parameters = GetScheduledQueryParameters::new("BULK-ID-123-xyz".to_string());

    let response = client.get_scheduled(query_parameters).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.bulk_id, "BULK-ID-123-xyz");
}

#[tokio::test]
async fn test_get_scheduled_empty_bulk_id() {
    let client = SmsClient::with_configuration(get_test_configuration("https://some.url"));

    let query_parameters = GetScheduledQueryParameters::new("".to_string());

    assert!(client.get_scheduled(query_parameters).await.is_err());
}

#[tokio::test]
async fn test_reschedule_valid() {
    let expected_response = r#"
    {
      "bulkId": "BULK-ID-123-xyz",
      "sendAt": "2021-08-25T16:00:00.000+0000"
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::PUT,
        PATH_RESCHEDULE,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(get_test_configuration(&server.base_url()));

    let query_parameters = RescheduleQueryParameters::new("BULK-ID-123-xyz".to_string());
    let request_body = RescheduleRequestBody::new("2021-08-25T16:00:00.000+0000".to_string());

    let response = client
        .reschedule(query_parameters, request_body)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.bulk_id, "BULK-ID-123-xyz");
    assert_eq!(response.body.send_at, "2021-08-25T16:00:00.000+0000");
}

#[tokio::test]
async fn test_reschedule_empty_bulk_id() {
    let client = SmsClient::with_configuration(get_test_configuration("https://some.url"));

    let query_parameters = RescheduleQueryParameters::new("".to_string());
    let request_body = RescheduleRequestBody::new("2021-08-25T16:00:00.000+0000".to_string());

    assert!(client
        .reschedule(query_parameters, request_body)
        .await
        .is_err());
}

#[tokio::test]
async fn test_reschedule_empty_send_at() {
    let client = SmsClient::with_configuration(get_test_configuration("https://some.url"));

    let query_parameters = RescheduleQueryParameters::new("BULK-ID-123-xyz".to_string());
    let request_body = RescheduleRequestBody::new("".to_string());

    assert!(client
        .reschedule(query_parameters, request_body)
        .await
        .is_err());
}

#[tokio::test]
async fn test_get_scheduled_status_valid() {
    let expected_response = r#"
    {
      "bulkId": "BULK-ID-123-xyz",
      "status": "PAUSED"
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_GET_SCHEDULED_STATUS,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(get_test_configuration(&server.base_url()));

    let query_parameters = GetScheduledStatusQueryParameters::new("BULK-ID-123-xyz".to_string());

    let response = client.get_scheduled_status(query_parameters).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.bulk_id.unwrap(), "BULK-ID-123-xyz");
    assert_eq!(response.body.status.unwrap(), PAUSED);
}

#[tokio::test]
async fn test_get_scheduled_status_empty_bulk_id() {
    let client = SmsClient::with_configuration(get_test_configuration("https://some.url"));

    let query_parameters = GetScheduledStatusQueryParameters::new("".to_string());

    assert!(client.get_scheduled_status(query_parameters).await.is_err());
}

#[tokio::test]
async fn test_update_scheduled_status_valid() {
    let expected_response = r#"
    {
      "bulkId": "BULK-ID-123-xyz",
      "status": "PAUSED"
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::PUT,
        PATH_UPDATE_SCHEDULED_STATUS,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(get_test_configuration(&server.base_url()));

    let query_parameters = UpdateScheduledStatusQueryParameters::new("BULK-ID-123-xyz".to_string());
    let request_body = UpdateScheduledStatusRequestBody::new(PAUSED);

    let response = client
        .update_scheduled_status(query_parameters, request_body)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.bulk_id.unwrap(), "BULK-ID-123-xyz");
    assert_eq!(response.body.status.unwrap(), PAUSED);
}

#[tokio::test]
async fn test_update_scheduled_status_empty_bulk_id() {
    let client = SmsClient::with_configuration(get_test_configuration("https://some.url"));

    let query_parameters = UpdateScheduledStatusQueryParameters::new("".to_string());
    let request_body = UpdateScheduledStatusRequestBody::new(PAUSED);

    assert!(client
        .update_scheduled_status(query_parameters, request_body)
        .await
        .is_err());
}

#[tokio::test]
async fn test_get_inbound_reports_valid() {
    let expected_response = r#"
    {
      "results": [
        {
          "messageId": "817790313235066447",
          "from": "385916242493",
          "to": "385921004026",
          "text": "QUIZ Correct answer is Paris",
          "cleanText": "Correct answer is Paris",
          "keyword": "QUIZ",
          "receivedAt": "2019-11-09T16:00:00.000+0000",
          "smsCount": 1,
          "price": {
            "pricePerMessage": 0,
            "currency": "EUR"
          },
          "callbackData": "callbackData"
        }
      ],
      "messageCount": 1,
      "pendingMessageCount": 0
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_GET_INBOUND,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(get_test_configuration(&server.base_url()));

    let query_parameters = GetInboundReportsQueryParameters::new();

    let response = client.get_inbound_reports(query_parameters).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.message_count.unwrap(), 1);
}

#[tokio::test]
async fn test_get_inbound_reports_big_limit() {
    let client = SmsClient::with_configuration(get_test_configuration("https://some.url"));

    let mut query_parameters = GetInboundReportsQueryParameters::new();
    query_parameters.limit = Some(1001);

    assert!(client.get_inbound_reports(query_parameters).await.is_err());
}

#[tokio::test]
async fn test_get_logs_valid() {
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
          "mccMnc": "22801",
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
          "messageId": "MESSAGE-ID-ijkl-45",
          "to": "41793026834",
          "sentAt": "2019-11-09T17:00:00.000+0000",
          "doneAt": "2019-11-09T17:00:00.000+0000",
          "smsCount": 1,
          "mccMnc": "22801",
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

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_GET_LOGS,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = SmsClient::with_configuration(get_test_configuration(&server.base_url()));

    let query_parameters = GetLogsQueryParameters::new();

    let response = client.get_logs(query_parameters).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert_eq!(response.body.results.unwrap().len(), 2usize);
}

#[tokio::test]
async fn test_get_logs_big_limit() {
    let client = SmsClient::with_configuration(get_test_configuration("https://some.url"));

    let mut query_parameters = GetLogsQueryParameters::new();
    query_parameters.limit = Some(1001);

    assert!(client.get_logs(query_parameters).await.is_err());
}
