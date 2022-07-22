use super::ApiError;
use crate::{
    configuration::Configuration,
    model::sms::{
        PreviewSmsRequestBody, PreviewSmsResponseBody, SendSmsRequestBody, SendSmsResponseBody,
    },
};

pub struct SmsClient {
    configuration: Configuration,
    client: reqwest::Client,
}

impl SmsClient {
    pub fn with_configuration(configuration: Configuration) -> SmsClient {
        SmsClient {
            configuration,
            client: reqwest::Client::new(),
        }
    }

    pub fn send(self, _body: SendSmsRequestBody) -> Result<SendSmsResponseBody, ApiError> {
        Ok(SendSmsResponseBody {})
    }

    pub async fn preview(
        self,
        body: PreviewSmsRequestBody,
    ) -> Result<PreviewSmsResponseBody, ApiError> {
        Ok(PreviewSmsResponseBody {
            original_text: None,
            previews: None,
        })
    }
}
