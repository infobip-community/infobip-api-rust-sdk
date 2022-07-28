use crate::api::{
    build_api_error, send_blocking_request, send_request, ApiError, SdkError, SdkResponse,
};
use crate::{
    configuration::Configuration,
    model::sms::{PreviewSmsRequestBody, PreviewSmsResponseBody},
};

const PATH_PREVIEW: &str = "sms/1/preview";

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
        &self,
        request_body: PreviewSmsRequestBody,
    ) -> Result<SdkResponse<PreviewSmsResponseBody>, SdkError> {
        let response = send_request(
            &self.client,
            &self.configuration,
            request_body,
            None,
            reqwest::Method::POST,
            PATH_PREVIEW,
        )
        .await?;

        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            Ok(SdkResponse {
                response_body: serde_json::from_str(&text)?,
                status,
            })
        } else {
            Err(build_api_error(status, &text))
        }
    }
}

pub struct BlockingSmsClient {
    configuration: Configuration,
    client: reqwest::blocking::Client,
}

impl BlockingSmsClient {
    pub fn with_configuration(configuration: Configuration) -> BlockingSmsClient {
        BlockingSmsClient {
            configuration,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn preview(
        &self,
        body: PreviewSmsRequestBody,
    ) -> Result<SdkResponse<PreviewSmsResponseBody>, SdkError> {
        let response = send_blocking_request(
            &self.client,
            &self.configuration,
            body,
            None,
            reqwest::Method::POST,
            PATH_PREVIEW,
        )?;

        let status = response.status();
        let text = response.text()?;

        if status.is_success() {
            Ok(SdkResponse {
                response_body: serde_json::from_str(&text)?,
                status,
            })
        } else {
            let api_error = ApiError {
                details: serde_json::from_str(&text)?,
                status,
            };

            Err(SdkError::ApiRequestError(api_error))
        }
    }
}
