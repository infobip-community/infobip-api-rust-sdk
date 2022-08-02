//! # Infobip SDK
//! Client SDK to use the Infobip API with pure Rust.
//!
//! This library enables you to use multiple Infobip communication channels, like SMS, MMS,
//! Whatsapp, Email, etc. It abstracts the needed HTTP calls, and models payloads and error
//! handling. The modules structure is divided by communication channel, which can be enabled as
//! library features.
//!
//! ## Supported Channels
//! - [SMS](https://www.infobip.com/docs/api/channels/sms) (in progress)
//!
//! More Channels to be added in the future.
//!
//! ## Authentication
//! To use the library, you'll need to setup an Infobip account. Then you can use your API Key and
//! custom URL to call the endpoints. You can use the `Configuration::from_env_api_key()` method to
//! load the configuration from the environment. To do that, export the variables `IB_API_KEY` and
//! `IB_BASE_URL`.
//!
//! ## Installation
//! To use the library, add the dependency to your projects `Cargo.toml`
//! ```toml
//! [dependencies]
//! infobip-sdk = "0.1"
//! ```
//!
//! ## Usage
//! To use the library, import the client and channel-specific models. Then create a client and
//! call the associated functions. For example, te send an SMS, you can do it like this:
//! ```rust
//! use infobip_sdk::model::sms::{DestinationBuilder, MessageBuilder, SendRequestBodyBuilder};
//! use infobip_sdk::api::sms::SmsClient;
//! use infobip_sdk::configuration::Configuration;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Build SMS client with configuration from the environment.
//!     let sms_client = SmsClient::with_configuration(
//!         Configuration::from_env_api_key()
//!             .expect("failed to build default test SMS client"),
//!     );
//!
//!     // Build a Destination instance.
//!     let destination = DestinationBuilder::default()
//!         .to("123456789012".to_string())
//!         .build()
//!         .unwrap();
//!
//!     // Build a Message instance.
//!     let message = MessageBuilder::default()
//!         .destinations(vec![destination])
//!         .text("Your message text".to_string())
//!         .build()
//!         .unwrap();
//!
//!     // Build the SendRequestBody instance.
//!     let request_body = SendRequestBodyBuilder::default()
//!         .messages(vec![message])
//!         .build()
//!         .unwrap();
//!
//!     // Send the SMS.
//!     let response = sms_client.send(request_body).await.unwrap();
//!
//!     // Do what you want with the response.
//!     assert_eq!(response.status, reqwest::StatusCode::OK);
//!     println!("{:?}", response.response_body);
//! }
//! ```
//!
//! For more examples on how to use the library, you can check the tests/ directory and the
//! included CLI examples.
//!
//! ## Examples
//! The best way to learn how to use the library is to look at the integration tests under the
//! [tests](./tests) directory, which work as you would use them in a real scenario.
//!
//! ## Notes
//! ### Using features
//! You can speed up compile-times a bit by turning only the needed channels as library features.
//! For example, to only build SMS, add the depedency like this:
//! ```toml
//! infobip-sdk = { version = "0.1", features = ["sms"] }
//! ```
//! You can see the complete list of features in the Cargo.toml of the project. Feature names
//! follow channel names.
//!
//! ### Building payload models
//! Structs that represent the models have public fields, so you can either build them with the
//! provided Builders, or by calling the true constructor.

#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate lazy_static;

pub mod api;
pub mod configuration;
pub mod model;
