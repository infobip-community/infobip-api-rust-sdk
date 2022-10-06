//! Module that contains the modules with models to create payloads and query parameters to call
//! endpoints and get responses from them with convenient, validated structs. There is one
//! submodule for each channel.

#[cfg(feature = "email")]
pub mod email;

#[cfg(feature = "sms")]
pub mod sms;

#[cfg(feature = "whatsapp")]
pub mod whatsapp;

#[cfg(test)]
mod tests;
