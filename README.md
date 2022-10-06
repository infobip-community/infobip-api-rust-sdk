# Infobip API Rust SDK

![Workflow](https://github.com/infobip-community/infobip-api-rust-sdk/actions/workflows/rust.yml/badge.svg)
[![Licence](https://img.shields.io/github/license/infobip-community/infobip-api-rust-sdk)](LICENSE-MIT)
[![Licence](https://img.shields.io/github/license/infobip-community/infobip-api-rust-sdk)](LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/v/infobip_sdk)](https://crates.io/crates/infobip_sdk)
![Crate downlads](https://img.shields.io/crates/d/rust_sdk)

Client SDK to use the Infobip API with pure Rust.

This crate enables you to use multiple Infobip communication channels, like SMS, MMS,
WhatsApp, Email, etc. It abstracts the needed HTTP calls, models and validates payloads and
models errors. The module structure is divided by communication channel.

---

## 📡 Supported Channels
- [SMS](https://www.infobip.com/docs/api/channels/sms)
- [WhatsApp](https://www.infobip.com/docs/api/channels/whatsapp)
- [Email](https://www.infobip.com/docs/api/channels/email)

More Channels to be added in the near future!

## 🔐 Authentication
To use the library, you'll need to set up an [Infobip account](https://www.infobip.com/signup).
Then you can use your API Key and custom base URL to call the endpoints. You can use the
`Configuration::from_env_api_key()` method to load the configuration from the environment. To
do that, set the `IB_API_KEY` and `IB_BASE_URL` variables. Alternatively, you can use the
`Configuration::from_dotenv_api_key()` method to load the configuration from a `.env` file.

## 📦 Installation
To use the library, add the dependency to your projects `Cargo.toml`
```toml
[dependencies]
infobip_sdk = "<version>"
```
Replace <version> is the latest (or desired) release of the library. For example `0.2.0`.

## 🚀 Usage
To use the library, import the client and channel-specific models. Then create a client and
call the associated functions. For example, to send an SMS, you can do this:
```rust
use infobip_sdk::model::sms::{Destination, Message, SendRequestBody};
use infobip_sdk::api::sms::SmsClient;
use infobip_sdk::configuration::Configuration;

#[tokio::main]
async fn main() {
    // Build SMS client with configuration from the environment.
    let sms_client = SmsClient::with_configuration(
        // Load IB_API_KEY and IB_BASE_URL environment variables.
        Configuration::from_env_api_key().unwrap()
    );

    // Create a message.
    let mut message = Message::new(
        vec![Destination::new("123456789012".to_string())]
    );
    message.text = Some("Your message text".to_string());

    // Create the SendRequestBody instance.
    let request_body = SendRequestBody::new(vec![message]);

    // Send the SMS.
    let response = sms_client.send(request_body).await.unwrap();

    // Do what you want with the response.
    assert_eq!(response.status, reqwest::StatusCode::OK);
    println!("Response body:\n{}", serde_json::to_string(&response.body).unwrap());
}
```

## 👀 Examples
The best way to learn how to use the library is to look at the official
[docs.rs documentation](https://docs.rs/infobip_sdk/), which has simple examples on how to use
every endpoint. You can also look at integration tests under the [tests](./tests) directory,
which work similarly to how you would use them in a real scenario.

## 🗒 Notes

### Building payload models
Structs that represent the models have public fields, so you can either build them with the
provided `new()` functions, with `serde_json::from_str()`, or with the true constructor.
For example, to build a `Message` instance, you can do this:
```rust
let mut message = Message::new(
   vec![Destination::new("123456789012".to_string())]
);
message.text = Some("Your message text".to_string());
```
or this:
```rust
let message: Message = serde_json::from_str(
    r#"
        {
          "destinations": [
            {
              "to": "123456789012"
            }
          ],
          "text": "Your message text"
        }
    "#,
)
.unwrap();
```
or this:
```rust
let destination = Destination {
    message_id: None,
    to: "41793026727".to_string()
};
let message = Message {
    callback_data: None,
    delivery_time_window: None,
    destinations: Some(vec![destination]),
    flash: None,
    from: None,
    intermediate_report: None,
    language: None,
    notify_content_type: None,
    notify_url: None,
    regional: None,
    send_at: None,
    text: None,
    transliteration: None,
    validity_period: None
};
```

### Model validation
Some models have mandatory fields. Optional fields are wrapped in `Option` Enums. Models also
have additional checks to make sure that fields have valid values, when possible. Validation
is done automatically when calling an endpoint, or you can call the `.validate()` method of the
model.

### Using features
You can speed up compile time by turning only the needed channels as library features.
For example, to only build SMS, add the dependency like this:
```toml
infobip_sdk = { version = "0.3", features = ["sms"] }
```
You can see the complete list of features in the Cargo.toml of the project. Feature names
follow channel names.

## 🧡 Contributing

If you would like to help this project improve, please check our [contributing guide](CONTRIBUTING.md) and [code of conduct](CODE_OF_CONDUCT.md).

## ⚖️ License

This project is licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
