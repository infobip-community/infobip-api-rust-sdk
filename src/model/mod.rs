//! Module that contains the modules with models to create payloads and query parameters to call
//! endpoints and get responses from them with convenient, validated structs. There is one
//! submodule for each channel.

#[cfg(feature = "sms")]
pub mod sms;

#[cfg(test)]
mod tests;
