use validator::Validate;

use crate::api::{add_auth, ApiError, SdkError, SdkResponse};
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

        let mut req_builder = self.client.post(format!(
            "{}{}",
            self.configuration.base_path, "/sms/1/preview"
        ));

        req_builder = add_auth(req_builder, &self.configuration);

        println!("Builder: {:?}", req_builder);

        let response = req_builder.json(&body).send().await?;

        let status = response.status();
        let response_text = response.text().await?;

        if status.is_success() {
            let response_body: PreviewSmsResponseBody = serde_json::from_str(&response_text)?;

            Ok(SdkResponse {
                response_body,
                status,
            })
        } else {
            let api_error = ApiError {
                details: serde_json::from_str(&response_text)?,
                status,
            };

            Err(SdkError::ApiRequestError(api_error))
        }
    }
}
