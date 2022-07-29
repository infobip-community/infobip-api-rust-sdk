//! # Infobip SDK
//! This is a client library to use Infobip's communication channels API through pure Rust.
//!
//! This library enables you to use multiple Infobip communication channels, like SMS, MMS,
//! Whatsapp, Email, etc. It abstracts the needed HTTP calls, and models payloads and error
//! handling. The modules structure is divided by communication channels, which can be enabled as
//! library features.
//!
//! ## Supported Channels
//! - SMS
//!
//! More Channels to be added in the future.
//!
//! ## Authentication
//! To use the library, you'll need to setup an Infobip account. Then you can use your API Key and
//! custom URL to call the endpoints.
//!
//! ## Installation
//! To use the library, add the dependency to your projects `Cargo.toml`
//! ```toml
//! [dependencies]
//! infobip-sdk = "0.1"
//! ```
//!
//! ## Usage
//! To use the library, import the client and channel-specific models. For SMS, you can do it
//! like this:
//! ```rust
//! ```
//!
//! For more examples on how to use the library, you can check the tests/ directory and the
//! included CLI examples.

#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate lazy_static;

pub mod api;
pub mod configuration;
pub mod model;
