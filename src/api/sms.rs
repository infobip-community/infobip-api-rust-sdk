use validator::Validate;

use crate::api::{ApiError, RequestError, SdkError, SdkResponse};
use crate::{
    configuration::Configuration,
    model::sms::{PreviewSmsRequestBody, PreviewSmsResponseBody},
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

    /*pub fn send(self, _body: SendSmsRequestBody) -> Result<SdkResponse<PreviewSmsResponseBody>, SdkError> {
        Ok(SdkResponse{
            response_body: SendSmsResponseBody{},
            status,
        })
    }*/

    pub async fn preview(
        self,
        body: PreviewSmsRequestBody,
    ) -> Result<SdkResponse<PreviewSmsResponseBody>, SdkError> {
        body.validate()?;

        let key_prefix = self
            .configuration
            .api_key
            .clone()
            .unwrap()
            .prefix
            .unwrap_or_else(|| "App ".to_string());
        let api_key = self.configuration.api_key.unwrap().key;

        let response = self
            .client
            .post(format!(
                "{}{}",
                self.configuration.base_path, "/sms/1/preview"
            ))
            .header("Authorization", format!("{}{}", key_prefix, api_key))
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        if status.is_success() {
            let response_body: PreviewSmsResponseBody = serde_json::from_str(&response_text)?;

            Ok(SdkResponse {
                response_body,
                status,
            })
        } else {
            let request_error: RequestError = serde_json::from_str(&response_text)?;
            let api_error = ApiError {
                source: request_error,
                status,
            };
            Err(SdkError::ApiRequestError(api_error))
        }
    }
}
