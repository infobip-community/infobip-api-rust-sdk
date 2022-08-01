# Infobip API Rust SDK

![Workflow](https://github.com/infobip-community/infobip-api-rust-sdk/actions/workflows/rust.yml/badge.svg)
[![Licence](https://img.shields.io/github/license/infobip-community/infobip-api-rust-sdk)](LICENSE-MIT)


This is a client library to use Infobip's communication channels API through pure Rust.

This library enables you to use multiple Infobip communication channels, like SMS, MMS,
Whatsapp, Email, etc. It abstracts the needed HTTP calls, and models payloads and error
handling. The modules structure is divided by communication channels, which can be enabled as
library features.

 ---

## 📡 Supported Channels
- SMS (in progress)

More Channels to be added in the near future!

## 🔐 Authentication
To use the library, you'll need to set up an Infobip account. Then, you can use your API Key and
custom URL to call the endpoints.

## 📦 Installation
To use the library, add the dependency to your projects `Cargo.toml`
```toml
[dependencies]
infobip-sdk = "0.1"
```

## 🚀 Usage
To use the library, import the client and channel-specific models. For SMS, you can do it
like this:
```rust
```

For more examples on how to use the library, you can check the tests/ directory and the
included CLI examples.

## 👀 Examples
The best way to learn how to use the library is to look at the integration tests, which work as you
would use them in a real scenario.

## ⚖️ License
This library is distributed under the MIT license.
