use std::collections::HashMap;
use validator::Validate;
use crate::api::{build_api_error, send_blocking_request, send_json_request, ApiError, SdkError, SdkResponse, send_no_body_request};
use crate::{
    configuration::Configuration,
    model::sms::{PreviewSmsRequestBody, PreviewSmsResponseBody},
};
use crate::model::sms::{GetDeliveryReportsQueryParameters, GetDeliveryReportsResponseBody};

const PATH_PREVIEW: &str = "sms/1/preview";
const PATH_DELIVERY_REPORTS: &str = "sms/1/reports";

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

    pub async fn preview(
        &self,
        request_body: PreviewSmsRequestBody,
    ) -> Result<SdkResponse<PreviewSmsResponseBody>, SdkError> {
        let response = send_json_request(
            &self.client,
            &self.configuration,
            request_body,
           HashMap::new(),
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

    pub async fn get_delivery_reports(
        &self,
        query_parameters: GetDeliveryReportsQueryParameters,
    ) -> Result<SdkResponse<GetDeliveryReportsResponseBody>, SdkError> {
        query_parameters.validate()?;
        let mut parameters_map = HashMap::<String, String>::new();
        if let Some(bulk_id) = query_parameters.bulk_id {
            parameters_map.insert("bulkId".to_string(), bulk_id);
        }
        if let Some(message_id) = query_parameters.message_id {
            parameters_map.insert("messageId".to_string(), message_id);
        }
        if let Some(limit) = query_parameters.limit{
            parameters_map.insert("limit".to_string(), limit.to_string());
        }

        let response = send_no_body_request(
            &self.client,
            &self.configuration,
            parameters_map,
            reqwest::Method::GET,
            PATH_DELIVERY_REPORTS,
        ).await?;

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
