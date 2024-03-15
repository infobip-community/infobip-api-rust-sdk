use crate::api::tests::{test_configuration, mock_json_endpoint};
use crate::api::whatsapp::*;
use crate::api::SdkError::ApiRequestError;
use crate::model::whatsapp::*;

fn dummy_send_text_request_body() -> SendTextRequestBody {
    SendTextRequestBody::new("44444444444", "55555555555", TextContent::new("some text"))
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
        }"#,
    )
    .unwrap();

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

    let wa_client = WhatsappClient::with_configuration(test_configuration(&server.base_url()));

    let response = wa_client.send_text(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[tokio::test]
async fn send_text_api_error() {
    let request_body =
        SendTextRequestBody::new("44444444444", "55555555555", TextContent::new("some text"));

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

    let wa_client = WhatsappClient::with_configuration(test_configuration(&server.base_url()));

    let sdk_error = wa_client.send_text(request_body).await.err().unwrap();

    match sdk_error {
        ApiRequestError(api_error) => {
            assert_eq!(api_error.status, reqwest::StatusCode::BAD_REQUEST);
            assert_eq!(
                api_error
                    .details
                    .request_error
                    .service_exception
                    .message_id
                    .unwrap(),
                "BAD_REQUEST"
            );
        }
        _ => {
            panic!("unexpected error")
        }
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

    let wa_client = WhatsappClient::with_configuration(test_configuration(&server.base_url()));

    let sdk_error = wa_client
        .send_text(dummy_send_text_request_body())
        .await
        .err()
        .unwrap();

    match sdk_error {
        ApiRequestError(api_error) => {
            assert_eq!(api_error.status, reqwest::StatusCode::UNAUTHORIZED);
            assert_eq!(
                api_error
                    .details
                    .request_error
                    .service_exception
                    .message_id
                    .unwrap(),
                "UNAUTHORIZED"
            );
        }
        _ => {
            panic!("unexpected error")
        }
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

    let wa_client = WhatsappClient::with_configuration(test_configuration(&server.base_url()));

    let sdk_error = wa_client
        .send_text(dummy_send_text_request_body())
        .await
        .err()
        .unwrap();

    match sdk_error {
        ApiRequestError(api_error) => {
            assert_eq!(api_error.status, reqwest::StatusCode::TOO_MANY_REQUESTS);
            assert_eq!(
                api_error
                    .details
                    .request_error
                    .service_exception
                    .message_id
                    .unwrap(),
                "TOO_MANY_REQUESTS"
            );
        }
        _ => {
            panic!("unexpected error")
        }
    }
}

#[tokio::test]
async fn send_document_valid() {
    let request_body: SendDocumentRequestBody = serde_json::from_str(
        r#"
        {
          "from": "441134960000",
          "to": "441134960001",
          "messageId": "a28dd97c-1ffb-4fcf-99f1-0b557ed381da",
          "content": {
            "mediaUrl": "http://example.com/document",
            "caption": "Some document caption",
            "filename": "filename.pdf"
          },
          "callbackData": "Callback data",
          "notifyUrl": "https://www.example.com/whatsapp"
        }
        "#,
    )
    .unwrap();

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
        PATH_SEND_DOCUMENT,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let wa_client = WhatsappClient::with_configuration(test_configuration(&server.base_url()));

    let response = wa_client.send_document(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[tokio::test]
async fn send_image_valid() {
    let request_body: SendImageRequestBody = serde_json::from_str(
        r#"
            {
              "from": "441134960000",
              "to": "441134960001",
              "messageId": "a28dd97c-1ffb-4fcf-99f1-0b557ed381da",
              "content": {
                "mediaUrl": "http://example.com/image",
                "caption": "Some image caption"
              },
              "callbackData": "Callback data",
              "notifyUrl": "https://www.example.com/whatsapp"
            }
        "#,
    )
    .unwrap();

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
        PATH_SEND_IMAGE,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let wa_client = WhatsappClient::with_configuration(test_configuration(&server.base_url()));

    let response = wa_client.send_image(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[tokio::test]
async fn send_audio_valid() {
    let request_body: SendAudioRequestBody = serde_json::from_str(
        r#"
            {
              "from": "441134960000",
              "to": "441134960001",
              "messageId": "a28dd97c-1ffb-4fcf-99f1-0b557ed381da",
              "content": {
                "mediaUrl": "http://example.com/audio"
              },
              "callbackData": "Callback data",
              "notifyUrl": "https://www.example.com/whatsapp"
            }
        "#,
    )
    .unwrap();

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
        PATH_SEND_AUDIO,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let wa_client = WhatsappClient::with_configuration(test_configuration(&server.base_url()));

    let response = wa_client.send_audio(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[tokio::test]
async fn send_video_valid() {
    let request_body: SendVideoRequestBody = serde_json::from_str(
        r#"
            {
              "from": "441134960000",
              "to": "441134960001",
              "messageId": "a28dd97c-1ffb-4fcf-99f1-0b557ed381da",
              "content": {
                "mediaUrl": "http://example.com/video",
                "caption": "Some video caption"
              },
              "callbackData": "Callback data",
              "notifyUrl": "https://www.example.com/whatsapp"
            }
        "#,
    )
    .unwrap();

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
        PATH_SEND_VIDEO,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let wa_client = WhatsappClient::with_configuration(test_configuration(&server.base_url()));

    let response = wa_client.send_video(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[tokio::test]
async fn send_sticker_valid() {
    let request_body: SendStickerRequestBody = serde_json::from_str(
        r#"
            {
              "from": "441134960000",
              "to": "441134960001",
              "messageId": "a28dd97c-1ffb-4fcf-99f1-0b557ed381da",
              "content": {
                "mediaUrl": "http://example.com/sticker"
              },
              "callbackData": "Callback data",
              "notifyUrl": "https://www.example.com/whatsapp"
            }
        "#,
    )
    .unwrap();

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
        PATH_SEND_STICKER,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let wa_client = WhatsappClient::with_configuration(test_configuration(&server.base_url()));

    let response = wa_client.send_sticker(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[tokio::test]
async fn send_location_valid() {
    let request_body: SendLocationRequestBody = serde_json::from_str(
        r#"
            {
              "from": "441134960000",
              "to": "441134960001",
              "messageId": "a28dd97c-1ffb-4fcf-99f1-0b557ed381da",
              "content": {
                "latitude": 44.9526862,
                "longitude": 13.8545217,
                "name": "Infobip",
                "address": "Vodnjan, Croatia"
              },
              "callbackData": "Callback data",
              "notifyUrl": "https://www.example.com/whatsapp"
            }
        "#,
    )
    .unwrap();

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
        PATH_SEND_LOCATION,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let wa_client = WhatsappClient::with_configuration(test_configuration(&server.base_url()));

    let response = wa_client.send_location(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[tokio::test]
async fn send_contact_valid() {
    let request_body: SendContactRequestBody = serde_json::from_str(
        r#"
            {
              "from": "441134960000",
              "to": "441134960001",
              "messageId": "a28dd97c-1ffb-4fcf-99f1-0b557ed381da",
              "content": {
                "contacts": [
                  {
                    "addresses": [
                      {
                        "street": "Istarska",
                        "city": "Vodnjan",
                        "zip": "52215",
                        "country": "Croatia",
                        "countryCode": "HR",
                        "type": "WORK"
                      },
                      {
                        "street": "Istarska",
                        "city": "Vodnjan",
                        "zip": "52215",
                        "country": "Croatia",
                        "countryCode": "HR",
                        "type": "HOME"
                      }
                    ],
                    "birthday": "2010-01-01",
                    "emails": [
                      {
                        "email": "John.Smith@example.com",
                        "type": "WORK"
                      },
                      {
                        "email": "John.Smith.priv@example.com",
                        "type": "HOME"
                      }
                    ],
                    "name": {
                      "firstName": "John",
                      "lastName": "Smith",
                      "middleName": "B",
                      "namePrefix": "Mr.",
                      "formattedName": "Mr. John Smith"
                    },
                    "org": {
                      "company": "Company Name",
                      "department": "Department",
                      "title": "Director"
                    },
                    "phones": [
                      {
                        "phone": "+441134960019",
                        "type": "HOME",
                        "waId": "441134960019"
                      },
                      {
                        "phone": "+441134960000",
                        "type": "WORK",
                        "waId": "441134960000"
                      }
                    ],
                    "urls": [
                      {
                        "url": "http://example.com/John.Smith",
                        "type": "WORK"
                      },
                      {
                        "url": "http://example.com/home/John.Smith",
                        "type": "HOME"
                      }
                    ]
                  },
                  {
                    "addresses": [
                      {
                        "street": "Istarska",
                        "city": "Vodnjan",
                        "zip": "52215",
                        "country": "Croatia",
                        "countryCode": "HR",
                        "type": "WORK"
                      },
                      {
                        "street": "Istarska",
                        "city": "Vodnjan",
                        "zip": "52215",
                        "country": "Croatia",
                        "countryCode": "HR",
                        "type": "HOME"
                      }
                    ],
                    "birthday": "2010-01-01",
                    "emails": [
                      {
                        "email": "Alexander.Abraham@example.com",
                        "type": "WORK"
                      },
                      {
                        "email": "Alexander.Abraham.priv@example.com",
                        "type": "HOME"
                      }
                    ],
                    "name": {
                      "firstName": "Alexander",
                      "lastName": "Abraham",
                      "middleName": "B",
                      "namePrefix": "Mr.",
                      "formattedName": "Mr. Alexander Abraham"
                    },
                    "org": {
                      "company": "Company Name",
                      "department": "Department",
                      "title": "Director"
                    },
                    "phones": [
                      {
                        "phone": "+441134960010",
                        "type": "HOME",
                        "waId": "441134960010"
                      },
                      {
                        "phone": "+441134960011",
                        "type": "WORK",
                        "waId": "441134960011"
                      }
                    ],
                    "urls": [
                      {
                        "url": "http://example.com/Alexander.Abraham",
                        "type": "WORK"
                      },
                      {
                        "url": "http://example.com/home/Alexander.Abraham",
                        "type": "HOME"
                      }
                    ]
                  }
                ]
              },
              "callbackData": "Callback data",
              "notifyUrl": "https://www.example.com/whatsapp"
            }
        "#,
    )
    .unwrap();

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
        PATH_SEND_CONTACT,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let wa_client = WhatsappClient::with_configuration(test_configuration(&server.base_url()));

    let response = wa_client.send_contact(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.message_id.unwrap().is_empty());
}

#[tokio::test]
async fn send_template_valid() {
    let request_body: SendTemplateRequestBody = serde_json::from_str(
        r#"
            {
              "messages": [
                {
                  "from": "441134960000",
                  "to": "441134960001",
                  "messageId": "a28dd97c-1ffb-4fcf-99f1-0b557ed381da",
                  "content": {
                    "templateName": "template_name",
                    "templateData": {
                      "body": {
                        "placeholders": []
                      },
                      "header": {
                        "type": "TEXT",
                        "placeholder": "placeholder value"
                      }
                    },
                    "language": "en_GB"
                  },
                  "callbackData": "Callback data",
                  "notifyUrl": "https://www.example.com/whatsapp"
                }
              ]
            }
        "#,
    )
    .unwrap();

    let expected_response = r#"
        {
          "messages": [
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
          ],
          "bulkId": "2034072219640523073"
        }
    "#;

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        PATH_SEND_TEMPLATE,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let wa_client = WhatsappClient::with_configuration(test_configuration(&server.base_url()));

    let response = wa_client.send_template(request_body).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.messages.unwrap().is_empty());
}

#[tokio::test]
async fn templates_valid() {
    let expected_response = r#"
        {
          "templates": [
            {
              "id": "111",
              "businessAccountId": 222,
              "name": "media_template_with_buttons",
              "language": "en",
              "status": "APPROVED",
              "category": "ACCOUNT_UPDATE",
              "structure": {
                "header": {
                  "format": "IMAGE"
                },
                "body": {
                  "text": "example {{1}} body"
                },
                "footer": {
                  "text": "exampleFooter"
                },
                "buttons": [
                  {
                    "text": "Dial 911",
                    "phoneNumber": "911",
                    "type": "PHONE_NUMBER"
                  },
                  {
                    "text": "Visit our website",
                    "url": "https://www.infobip.com",
                    "type": "URL"
                  }
                ],
                "type": "MEDIA"
              }
            }
          ]
        }
    "#;

    let sender = "441134960000";
    let path = PATH_GET_TEMPLATES.replace("{sender}", &sender);

    let server = mock_json_endpoint(
        httpmock::Method::GET,
        &path,
        expected_response,
        reqwest::StatusCode::OK,
    )
    .await;

    let wa_client = WhatsappClient::with_configuration(test_configuration(&server.base_url()));

    let response = wa_client.templates(sender).await.unwrap();

    assert_eq!(response.status, reqwest::StatusCode::OK);
    assert!(!response.body.templates.unwrap().is_empty());
}

#[tokio::test]
async fn create_template_valid() {
    let request_body: CreateTemplateRequestBody = serde_json::from_str(
        r#"
            {
              "name": "media_template_with_quick_reply_buttons",
              "language": "en",
              "category": "OTP",
              "structure": {
                "body": {
                  "text": "body {{1}} content",
                  "examples": [
                    "example"
                  ]
                },
                "buttons": [
                  {
                    "text": "quick reply 1",
                    "type": "QUICK_REPLY"
                  },
                  {
                    "text": "quick reply 2",
                    "type": "QUICK_REPLY"
                  },
                  {
                    "text": "quick reply 3",
                    "type": "QUICK_REPLY"
                  }
                ],
                "type": "MEDIA"
              }
            }
        "#,
    )
    .unwrap();

    let expected_response = r#"
        {
          "id": "111",
          "businessAccountId": 222,
          "name": "media_template_with_buttons",
          "language": "en",
          "status": "APPROVED",
          "category": "ACCOUNT_UPDATE",
          "structure": {
            "header": {
              "format": "IMAGE"
            },
            "body": {
              "text": "example {{1}} body"
            },
            "footer": {
              "text": "exampleFooter"
            },
            "buttons": [
              {
                "text": "Dial 911",
                "phoneNumber": "911",
                "type": "PHONE_NUMBER"
              },
              {
                "text": "Visit our website",
                "url": "https://www.infobip.com",
                "type": "URL"
              }
            ],
            "type": "MEDIA"
          }
        }
    "#;

    let sender = "441134960000";
    let path = PATH_CREATE_TEMPLATE.replace("{sender}", &sender);

    let server = mock_json_endpoint(
        httpmock::Method::POST,
        &path,
        expected_response,
        reqwest::StatusCode::CREATED,
    )
    .await;

    let wa_client = WhatsappClient::with_configuration(test_configuration(&server.base_url()));

    let response = wa_client
        .create_template(sender, request_body)
        .await
        .unwrap();

    assert_eq!(response.status, reqwest::StatusCode::CREATED);
    assert!(!response.body.id.unwrap().is_empty());
}

#[tokio::test]
async fn delete_template_valid() {
    let template_name = "media_template_with_buttons";
    let sender = "441134960000";
    let path = PATH_DELETE_TEMPLATE
        .replace("{sender}", sender)
        .replace("{templateName}", template_name);

    let server = mock_json_endpoint(
        httpmock::Method::DELETE,
        &path,
        "",
        reqwest::StatusCode::NO_CONTENT,
    )
    .await;

    let wa_client = WhatsappClient::with_configuration(test_configuration(&server.base_url()));

    let status = wa_client
        .delete_template(sender, template_name)
        .await
        .unwrap();

    assert_eq!(status, reqwest::StatusCode::NO_CONTENT);
}
