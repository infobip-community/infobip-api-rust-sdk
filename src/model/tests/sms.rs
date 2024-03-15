use validator::Validate;

use crate::model::sms::*;

const DUMMY_TEXT: &str = "Dummy text for tests. Some special chars: áéíø";

#[test]
fn sms_preview_request_body_valid() {
    let mut request_body = PreviewRequestBody::new(DUMMY_TEXT);
    request_body.language_code = Some("ES".to_string());
    request_body.transliteration = Some("GREEK".to_string());

    assert!(request_body.validate().is_ok())
}

#[test]
fn sms_preview_request_body_invalid_language_code() {
    let mut request_body = PreviewRequestBody::new(DUMMY_TEXT);
    request_body.language_code = Some("BAD".to_string());

    assert!(request_body.validate().is_err())
}

#[test]
fn sms_preview_request_body_invalid_transliteration() {
    let mut request_body = PreviewRequestBody::new(DUMMY_TEXT);
    request_body.transliteration = Some("BAD".to_string());

    assert!(request_body.validate().is_err())
}

#[test]
fn delivery_reports_query_parameters_valid() {
    let mut parameters = GetDeliveryReportsQueryParameters::new();
    parameters.limit = Some(10);

    assert!(parameters.validate().is_ok())
}

#[test]
fn delivery_reports_query_parameters_big_limit() {
    let mut parameters = GetDeliveryReportsQueryParameters::new();
    parameters.limit = Some(10000);

    assert!(parameters.validate().is_err())
}

#[test]
fn send_request_body_valid() {
    let mut message = Message::new(vec![Destination::new("123456789012")]);
    message.text = Some(DUMMY_TEXT.into());

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_ok())
}

#[test]
fn send_request_body_no_messages() {
    let request_body = SendRequestBody::new(vec![]);

    assert!(request_body.validate().is_err())
}

#[test]
fn send_request_body_no_destination() {
    let message = Message::new(vec![]);
    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_request_body_no_destination_to() {
    let message = Message::new(vec![Destination::new("")]);
    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_request_body_no_principal_entity_id() {
    let mut regional = RegionalOptions::new();
    regional.india_dlt = Some(IndiaDlt::new(""));
    let mut message = Message::new(vec![Destination::new("123456789012")]);
    message.regional = Some(regional);
    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_request_body_no_turkey_recipient_type() {
    let mut regional = RegionalOptions::new();
    regional.turkey_iys = Some(TurkeyIys::new(""));
    let mut message = Message::new(vec![Destination::new("123456789012")]);
    message.regional = Some(regional);
    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_request_body_bad_turkey_recipient_type() {
    let mut regional = RegionalOptions::new();
    regional.turkey_iys = Some(TurkeyIys::new("BAD"));
    let mut message = Message::new(vec![Destination::new("123456789012")]);
    message.regional = Some(regional);
    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn message_from_str() {
    let message: Message = serde_json::from_str(
        r#"
        {
          "destinations": [
            {
              "to": "41793026727"
            }
          ],
          "from": "InfoSMS",
          "text": "This is a sample message"
        }
    "#,
    )
    .unwrap();

    assert_eq!(message.text.unwrap(), "This is a sample message");
}

#[test]
fn send_request_body_zero_speed_limit_amount() {
    let message = Message::new(vec![Destination::new("123456789012")]);
    let mut request_body = SendRequestBody::new(vec![message]);
    request_body.sending_speed_limit = Some(SpeedLimit::new(0));

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_request_body_speed_limit_time_unit() {
    let message = Message::new(vec![Destination::new("123456789012")]);
    let mut speed_limit = SpeedLimit::new(5);
    speed_limit.time_unit = Some(TimeUnit::Day);

    let mut request_body = SendRequestBody::new(vec![message]);
    request_body.sending_speed_limit = Some(speed_limit);

    let serialized = serde_json::to_string(&request_body).unwrap();
    assert!(request_body.validate().is_ok());
    assert!(serialized.contains(r#""amount":5,"timeUnit":"DAY""#));
}

#[test]
fn send_request_body_with_delivery_time_window() {
    let delivery_time_window =
        DeliveryTimeWindow::new(vec![DeliveryDay::Monday, DeliveryDay::Tuesday]);

    let mut message = Message::new(vec![Destination::new("123456789012")]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    let serialized = serde_json::to_string(&request_body).unwrap();
    assert!(request_body.validate().is_ok());
    assert!(serialized.contains(r#""days":["MONDAY","TUESDAY"]"#));
}

#[test]
fn send_request_body_delivery_time_window_no_days() {
    let delivery_time_window = DeliveryTimeWindow::new(vec![]);

    let mut message = Message::new(vec![Destination::new("123456789012")]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_request_body_delivery_time_window_to_hour() {
    let mut delivery_time_window = DeliveryTimeWindow::new(vec![DeliveryDay::Monday]);
    delivery_time_window.to = Some(DeliveryTime::new(23, 59));

    let mut message = Message::new(vec![Destination::new("123456789012")]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_request_body_delivery_time_window_bad_to_hour() {
    let mut delivery_time_window = DeliveryTimeWindow::new(vec![DeliveryDay::Monday]);
    delivery_time_window.to = Some(DeliveryTime::new(24, 0));

    let mut message = Message::new(vec![Destination::new("123456789012")]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_request_body_delivery_time_window_bad_to_minute() {
    let mut delivery_time_window = DeliveryTimeWindow::new(vec![DeliveryDay::Monday]);
    delivery_time_window.to = Some(DeliveryTime::new(23, 60));

    let mut message = Message::new(vec![Destination::new("123456789012")]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_request_body_delivery_time_window_from_hour() {
    let mut delivery_time_window = DeliveryTimeWindow::new(vec![DeliveryDay::Monday]);
    delivery_time_window.from = Some(DeliveryTime::new(23, 59));

    let mut message = Message::new(vec![Destination::new("123456789012")]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_request_body_delivery_time_window_bad_from_hour() {
    let mut delivery_time_window = DeliveryTimeWindow::new(vec![DeliveryDay::Monday]);
    delivery_time_window.from = Some(DeliveryTime::new(24, 0));

    let mut message = Message::new(vec![Destination::new("123456789012")]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_request_body_delivery_time_window_bad_from_minute() {
    let mut delivery_time_window = DeliveryTimeWindow::new(vec![DeliveryDay::Monday]);
    delivery_time_window.from = Some(DeliveryTime::new(23, 60));

    let mut message = Message::new(vec![Destination::new("123456789012")]);
    message.delivery_time_window = Some(delivery_time_window);

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_request_body_long_callback_data() {
    let mut message = Message::new(vec![Destination::new("123456789012")]);
    message.callback_data = Some("longstring ".repeat(1000));

    let request_body = SendRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_binary_request_body_long_to() {
    let message = BinaryMessage::new(vec![Destination::new(&"123456789012".repeat(10))]);

    let request_body = SendBinaryRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_binary_request_body_empty_hex() {
    let binary_data = BinaryData::new("");
    let mut message = BinaryMessage::new(vec![Destination::new("123456789012")]);
    message.binary = Some(binary_data);

    let request_body = SendBinaryRequestBody::new(vec![message]);

    assert!(request_body.validate().is_err());
}

#[test]
fn reschedule_request_body_valid() {
    let request_body = RescheduleRequestBody::new("2021-08-25T16:00:00.000+0000");

    assert!(request_body.validate().is_ok());
}

#[test]
fn reschedule_request_body_empty_send_at() {
    let request_body = RescheduleRequestBody::new("");

    assert!(request_body.validate().is_err());
}

#[test]
fn scheduled_query_parameters_valid() {
    let query_parameters = GetScheduledQueryParameters::new("some_bulk_id");

    assert!(query_parameters.validate().is_ok());
}

#[test]
fn scheduled_query_parameters_empty_bulk_id() {
    let query_parameters = GetScheduledQueryParameters::new("");

    assert!(query_parameters.validate().is_err());
}

#[test]
fn inbound_reports_query_parameters_valid() {
    let mut query_parameters = GetInboundReportsQueryParameters::new();
    query_parameters.limit = Some(10);

    assert!(query_parameters.validate().is_ok());
}

#[test]
fn inbound_reports_query_parameters_big_limit() {
    let mut query_parameters = GetInboundReportsQueryParameters::new();
    query_parameters.limit = Some(10001);

    assert!(query_parameters.validate().is_err());
}

#[test]
fn create_tfa_application_request_body_valid() {
    let request_body = CreateTfaApplicationRequestBody::new("some_name");

    assert!(request_body.validate().is_ok());
}

#[test]
fn create_tfa_application_request_body_empty_name() {
    let request_body = CreateTfaApplicationRequestBody::new("");

    assert!(request_body.validate().is_err());
}

#[test]
fn create_tfa_message_template_request_body_valid() {
    let request_body = CreateTfaMessageTemplateRequestBody::new("some_name", PinType::Alpha, 6);

    assert!(request_body.validate().is_ok());
}

#[test]
fn create_tfa_message_template_request_body_empty_name() {
    let request_body = CreateTfaMessageTemplateRequestBody::new("", PinType::Alpha, 6);

    assert!(request_body.validate().is_err());
}

#[test]
fn create_tfa_message_template_request_body_no_principal_entity_id() {
    let mut request_body = CreateTfaMessageTemplateRequestBody::new("some_name", PinType::Alpha, 6);
    let regional = TfaRegional {
        india_dlt: Some(IndiaDlt::new("")),
    };
    request_body.regional = Some(regional);

    assert!(request_body.validate().is_err());
}

#[test]
fn update_tfa_message_template_request_body_no_principal_entity_id() {
    let mut request_body = UpdateTfaMessageTemplateRequestBody::new("some_name", PinType::Alpha, 6);
    let regional = TfaRegional {
        india_dlt: Some(IndiaDlt::new("")),
    };
    request_body.regional = Some(regional);

    assert!(request_body.validate().is_err());
}

#[test]
fn send_pin_over_sms_request_body_valid() {
    let request_body =
        SendPinOverSmsRequestBody::new("some-app-id", "some-message-id", "555555555555");

    assert!(request_body.validate().is_ok());
}

#[test]
fn send_pin_over_sms_request_body_empty_app_id() {
    let request_body = SendPinOverSmsRequestBody::new("", "some-message-id", "555555555555");

    assert!(request_body.validate().is_err());
}

#[test]
fn send_pin_over_sms_request_body_empty_message_id() {
    let request_body = SendPinOverSmsRequestBody::new("some-app-id", "", "555555555555");

    assert!(request_body.validate().is_err());
}

#[test]
fn send_pin_over_sms_request_body_empty_to() {
    let request_body = SendPinOverSmsRequestBody::new("some-app-id", "some-message-id", "");

    assert!(request_body.validate().is_err());
}

#[test]
fn verify_phone_number_request_body_valid() {
    let request_body = VerifyPhoneNumberRequestBody::new("1234");

    assert!(request_body.validate().is_ok());
}

#[test]
fn verify_phone_number_request_body_empty_pin() {
    let request_body = VerifyPhoneNumberRequestBody::new("");

    assert!(request_body.validate().is_err());
}
