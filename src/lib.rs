//! # Infobip SDK
//! Client SDK to use the Infobip API with pure Rust.
//!
//! This crate enables you to use multiple Infobip communication channels, like SMS, 2FA,
//! WhatsApp, Email, etc. It abstracts the needed HTTP calls, models and validates payloads and
//! models errors. The module structure is divided by communication channel.
//!
//! ## Supported Channels
//!
//! Currently, we support the following channels:
//! - [SMS + 2FA](https://www.infobip.com/docs/api/channels/sms)
//! - [WhatsApp](https://www.infobip.com/docs/api/channels/whatsapp)
//! - [Email](https://www.infobip.com/docs/api/channels/email)
//!
//! See also `Features` below.
//!
//! More channels to be added in the near future!
//!
//! ## Authentication
//!
//! To use the library, you'll need to set up an [Infobip account](https://www.infobip.com/signup).
//! Then you can use your API Key and custom base URL to call the endpoints. You can use the
//! `Configuration::from_env_api_key()` method to load the configuration from the environment. To
//! do that, set the `IB_API_KEY` and `IB_BASE_URL` variables.
//!
//! ## Usage
//!
//! To use the library, import the client and channel-specific models. Then create a client and
//! call the associated functions. For example, to send an SMS, you can do this:
//!
//! ```no_run
//! use infobip_sdk::model::sms::{Destination, Message, SendRequestBody};
//! use infobip_sdk::api::sms::SmsClient;
//! use infobip_sdk::configuration::Configuration;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Build SMS client with configuration from the environment.
//!     let sms_client = SmsClient::with_configuration(
//!         // Load IB_API_KEY and IB_BASE_URL environment variables.
//!         Configuration::from_env_api_key().unwrap()
//!     );
//!
//!     // Create a message.
//!     let message = Message {
//!         destinations: Some(vec![Destination::new("491702384590")]),
//!         text: Some("Foobar!".to_string()),
//!         ..Default::default()
//!     };
//!
//!     // Create the SendRequestBody instance.
//!     let request_body = SendRequestBody::new(vec![message]);
//!
//!     // Send the SMS.
//!     let response = sms_client.send(request_body).await.unwrap();
//!
//!     // Do what you want with the response.
//!     assert_eq!(response.status, reqwest::StatusCode::OK);
//!     println!("Response body:\n{}", serde_json::to_string(&response.body).unwrap());
//! }
//! ```
//!
//! ## Examples
//!
//! The best way to learn how to use the library is to look at the official
//! [docs.rs documentation](https://docs.rs/infobip_sdk/), which has simple examples on how to use
//! every endpoint. You can also look at integration tests under the [tests](./tests) directory,
//! which work similarly to how you would use them in a real scenario.
//!
//! ## Notes
//!
//! ### Building Payload Models
//!
//! Structs that represent the models have public fields, so you can either build them with the
//! provided `new()` functions, with `serde_json`, or with the true constructor.
//!
//! For example, to build a `Message` instance, you can do this:
//!
//! ```rust
//! # use infobip_sdk::model::sms::{Destination, Message};
//! let mut message = Message::new(
//!    vec![Destination::new("123456789012")]
//! );
//!
//! message.text = Some("Your message text".to_string());
//! ```
//!
//! or this:
//!
//! ```rust
//! # use infobip_sdk::model::sms::{Destination, Message};
//! let message: Message = serde_json::json!(
//!     {
//!        "destinations": [
//!             {
//!               "to": "123456789012"
//!             }
//!        ],
//!        "text": "Your message text"
//!     }
//! );
//! ```
//!
//! or this:
//!
//! ```rust
//! # use infobip_sdk::model::sms::{Destination, Message};
//! let destination = Destination {
//!     message_id: None,
//!     to: "41793026727".to_string()
//! };
//!
//! let message = Message {
//!     destinations: Some(vec![destination]),
//!     ..Default::default()
//! };
//! ```
//!
//! ### Model Validation
//! Some models have mandatory fields. Optional fields are wrapped in `Option` Enums. Models also
//! have additional checks to make sure that fields have valid values, when possible. Validation
//! is done automatically when calling an endpoint, or you can call the `.validate()` method of the
//! model.
//!
//! ### Optional Features
//!
#![doc = document_features::document_features!()]
//!
//! E.g., to choose a different TLS implementation provided by the `rustls` crate:
//!
//! ```toml
//! [dependencies.infobip_sdk]
//! version = "0.6.0"
//! default-features = false
//! features = [ "rustls-tls", "email", "sms", "whatsapp"]
//! ```

pub mod api;
pub mod configuration;
pub mod model;
