use crate::api::email::*;
use crate::api::tests::{mock_json_endpoint, test_configuration};
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

    let client = EmailClient::with_configuration(test_configuration(&server.base_url()));

    let request_body = SendRequestBody::new("some@mail.com");

    let response = client.send(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.messages.unwrap().is_empty());
}

#[tokio::test]
async fn test_send_invalid_request() {
    let client = EmailClient::with_configuration(test_configuration(DUMMY_BASE_URL));

    let request_body = SendRequestBody::new("");

    let error = client.send(request_body).await.unwrap_err();

    if let SdkError::Validation(validation_error) = error {
        assert!(!validation_error.errors().is_empty());
    } else {
        panic!("not validation error");
    }
}

#[tokio::test]
async fn test_bulks_valid() {
    let expected_response = r#"
    {
      "externalBulkId": "string",
      "bulks": [
        {
          "bulkId": "string",
          "sendAt": 1665003852352
        }
      ]
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_GET_BULKS,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = EmailClient::with_configuration(test_configuration(&server.base_url()));

    let query_params = BulksQueryParameters::new("bulk-id");

    let response = client.bulks(query_params).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.bulks.unwrap().is_empty());
}

#[tokio::test]
async fn bulks_invalid() {
    let client = EmailClient::with_configuration(test_configuration(DUMMY_BASE_URL));

    let query_params = BulksQueryParameters::new("");

    let error = client.bulks(query_params).await.unwrap_err();

    if let SdkError::Validation(validation_error) = error {
        assert!(!validation_error.errors().is_empty());
    } else {
        panic!("not a validation error");
    }
}

#[tokio::test]
async fn reschedule_valid() {
    let expected_response = r#"
    {
      "bulkId": "string",
      "sendAt": 1665003852352
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::PUT,
        PATH_RESCHEDULE,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = EmailClient::with_configuration(test_configuration(&server.base_url()));

    let query_parameters = RescheduleQueryParameters::new("bulk-id");
    let request_body = RescheduleRequestBody::new("2022-10-03T20:27:41Z");

    let response = client
        .reschedule(query_parameters, request_body)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
}

#[tokio::test]
async fn reschedule_no_bulk_id() {
    let client = EmailClient::with_configuration(test_configuration(DUMMY_BASE_URL));

    let query_parameters = RescheduleQueryParameters::new("");
    let request_body = RescheduleRequestBody::new("2022-10-03T20:27:41Z");

    let error = client
        .reschedule(query_parameters, request_body)
        .await
        .unwrap_err();

    if let SdkError::Validation(validation_error) = error {
        assert!(!validation_error.errors().is_empty());
    } else {
        panic!("not validation error");
    }
}

#[tokio::test]
async fn scheduled_status_valid() {
    let expected_response = r#"
    {
      "externalBulkId": "string",
      "bulks": [
        {
          "bulkId": "string",
          "status": "PENDING"
        }
      ]
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_GET_SCHEDULED_STATUS,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = EmailClient::with_configuration(test_configuration(&server.base_url()));

    let query_parameters = ScheduledStatusQueryParameters::new("bulk-id");

    let response = client.scheduled_status(query_parameters).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
}

#[tokio::test]
async fn update_scheduled_status_valid() {
    let expected_response = r#"
    {
      "bulkId": "string",
      "status": "CANCELED"
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::PUT,
        PATH_UPDATE_SCHEDULED_STATUS,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = EmailClient::with_configuration(test_configuration(&server.base_url()));

    let query_parameters = UpdateScheduledStatusQueryParameters::new("bulk-id");
    let request_body = UpdateScheduledStatusRequestBody::new(BulkStatus::Canceled);

    let response = client
        .update_scheduled_status(query_parameters, request_body)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
}

#[tokio::test]
async fn delivery_reports_valid() {
    let expected_response = r#"
    {
      "results": [
        {
          "bulkId": "string",
          "messageId": "string",
          "to": "string",
          "sentAt": "2022-10-03T15:11:38Z",
          "doneAt": "2022-10-03T15:11:38Z",
          "messageCount": 0,
          "price": {
            "pricePerMessage": 0,
            "currency": "string"
          },
          "status": {
            "groupId": 0,
            "groupName": "string",
            "id": 0,
            "name": "string",
            "description": "string",
            "action": "string"
          },
          "error": {
            "groupId": 0,
            "groupName": "string",
            "id": 0,
            "name": "string",
            "description": "string",
            "permanent": true
          }
        }
      ]
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_GET_DELIVERY_REPORTS,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = EmailClient::with_configuration(test_configuration(&server.base_url()));

    let query_parameters = DeliveryReportsQueryParameters::default();

    let response = client.delivery_reports(query_parameters).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
}

#[tokio::test]
async fn logs_valid() {
    let expected_response = r#"
    {
      "results": [
        {
          "messageId": "string",
          "to": "string",
          "from": "string",
          "text": "string",
          "sentAt": "2022-10-03T17:31:04Z",
          "doneAt": "2022-10-03T17:31:04Z",
          "messageCount": 0,
          "price": {
            "pricePerMessage": 0,
            "currency": "string"
          },
          "status": {
            "groupId": 0,
            "groupName": "string",
            "id": 0,
            "name": "string",
            "description": "string",
            "action": "string"
          },
          "bulkId": "string"
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

    let client = EmailClient::with_configuration(test_configuration(&server.base_url()));

    let query_parameters = LogsQueryParameters::default();

    let response = client.logs(query_parameters).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
}

#[tokio::test]
async fn validate_address_valid() {
    let expected_response = r#"
    {
      "to": "abc@zxc.com",
      "validMailbox": "unknown",
      "validSyntax": true,
      "catchAll": false,
      "disposable": false,
      "roleBased": false,
      "reason": "INBOX_FULL"
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_VALIDATE,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = EmailClient::with_configuration(test_configuration(&server.base_url()));

    let request_body = ValidateAddressRequestBody::new("address@email.com");

    let response = client.validate_address(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
}

#[tokio::test]
async fn domains_valid() {
    let expected_response = r#"
    {
      "paging": {
        "page": 0,
        "size": 0,
        "totalPages": 0,
        "totalResults": 0
      },
      "results": [
        {
          "domainId": 1,
          "domainName": "newDomain.com",
          "active": false,
          "tracking": {
            "clicks": true,
            "opens": true,
            "unsubscribe": true
          },
          "dnsRecords": [
            {
              "recordType": "string",
              "name": "string",
              "expectedValue": "string",
              "verified": true
            }
          ],
          "blocked": false,
          "createdAt": "2022-05-05T17:32:28.777+01:00"
        }
      ]
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        PATH_GET_DOMAINS,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = EmailClient::with_configuration(test_configuration(&server.base_url()));

    let query_parameters = DomainsQueryParameters::default();

    let response = client.domains(query_parameters).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
}

#[tokio::test]
async fn add_domain_valid() {
    let expected_response = r#"
    {
      "domainId": 1,
      "domainName": "newDomain.com",
      "active": false,
      "tracking": {
        "clicks": true,
        "opens": true,
        "unsubscribe": true
      },
      "dnsRecords": [
        {
          "recordType": "string",
          "name": "string",
          "expectedValue": "string",
          "verified": true
        }
      ],
      "blocked": false,
      "createdAt": "2022-05-05T17:32:28.777+01:00"
    }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_ADD_DOMAIN,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = EmailClient::with_configuration(test_configuration(&server.base_url()));

    let request_body = AddDomainRequestBody::new("domain.com");

    let response = client.add_domain(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
}

#[tokio::test]
async fn domain_valid() {
    let expected_response = r#"
    {
      "domainId": 1,
      "domainName": "newDomain.com",
      "active": false,
      "tracking": {
        "clicks": true,
        "opens": true,
        "unsubscribe": true
      },
      "dnsRecords": [
        {
          "recordType": "string",
          "name": "string",
          "expectedValue": "string",
          "verified": true
        }
      ],
      "blocked": false,
      "createdAt": "2022-05-05T17:32:28.777+01:00"
    }
    "#;

    let domain_name = "newDomain.com";
    let path = PATH_GET_DOMAIN.replace("{domainName}", domain_name);

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        path.as_str(),
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = EmailClient::with_configuration(test_configuration(&server.base_url()));

    let response = client.domain(domain_name).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
}

#[tokio::test]
async fn delete_domain_valid() {
    let domain_name = "newDomain.com";
    let path = PATH_DELETE_DOMAIN.replace("{domainName}", domain_name);

    let server = mock_json_endpoint(
        httpmock::Method::DELETE,
        path.as_str(),
        "",
        reqwest::StatusCode::NO_CONTENT,
    )
    .await;

    let client = EmailClient::with_configuration(test_configuration(&server.base_url()));

    let status = client.delete_domain(domain_name).await.unwrap();

    assert_eq!(status, reqwest::StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn update_tracking_valid() {
    let expected_response = r#"
    {
      "domainId": 1,
      "domainName": "newDomain.com",
      "active": false,
      "tracking": {
        "clicks": true,
        "opens": true,
        "unsubscribe": true
      },
      "dnsRecords": [
        {
          "recordType": "string",
          "name": "string",
          "expectedValue": "string",
          "verified": true
        }
      ],
      "blocked": false,
      "createdAt": "2022-05-05T17:32:28.777+01:00"
    }
    "#;

    let domain_name = "newDomain.com";
    let path = PATH_UPDATE_TRACKING.replace("{domainName}", domain_name);

    let server = mock_json_endpoint(
        httpmock::Method::PUT,
        path.as_str(),
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let client = EmailClient::with_configuration(test_configuration(&server.base_url()));

    let request_body = UpdateTrackingRequestBody::default();

    let response = client
        .update_tracking(domain_name, request_body)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
}

#[tokio::test]
async fn verify_domain_valid() {
    let domain_name = "newDomain.com";
    let path = PATH_VERIFY_DOMAIN.replace("{domainName}", domain_name);

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        path.as_str(),
        "",
        reqwest::StatusCode::ACCEPTED,
    )
    .await;

    let client = EmailClient::with_configuration(test_configuration(&server.base_url()));

    let status = client.verify_domain(domain_name).await.unwrap();

    assert_eq!(status, reqwest::StatusCode::ACCEPTED);
}
