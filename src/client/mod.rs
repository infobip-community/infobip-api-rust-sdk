use crate::api::sms::Sms;
use crate::client::configuration::Configuration;

pub mod configuration;

pub struct InfobipClient {
    pub client: reqwest::Client,
    pub configuration: Configuration,
    pub sms: Sms,
}

impl InfobipClient {
    pub fn new(configuration: Configuration) -> InfobipClient {
        Self {
            client: reqwest::Client::new(),
            configuration,
            sms: Sms {},
        }
    }
}
