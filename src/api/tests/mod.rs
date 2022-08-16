#![cfg(test)]

use httpmock::prelude::*;

use crate::configuration::{ApiKey, Configuration};

#[cfg(test)]
mod sms;

const DUMMY_TEXT: &str = "Some text for tests.";

async fn mock_json_endpoint(
    endpoint_method: httpmock::Method,
    endpoint_path: &str,
    expected_response: &str,
    expected_status: reqwest::StatusCode,
) -> MockServer {
    let server = MockServer::start_async().await;

    server.mock(|when, then| {
        when.method(endpoint_method).path(endpoint_path);

        then.status(expected_status.as_u16())
            .header("content-type", "application/json")
            .body(expected_response);
    });

    server
}

fn mock_blocking_json_endpoint(
    endpoint_method: httpmock::Method,
    endpoint_path: &str,
    expected_response: &str,
    expected_status: reqwest::StatusCode,
) -> MockServer {
    let server = MockServer::start();

    server.mock(|when, then| {
        when.method(endpoint_method).path(endpoint_path);

        then.status(expected_status.as_u16())
            .header("content-type", "application/json")
            .body(expected_response);
    });

    server
}

fn get_test_configuration(server_url: &str) -> Configuration {
    Configuration::with_api_key(
        server_url.to_string(),
        ApiKey {
            key: "some-api-key".to_string(),
            prefix: None,
        },
    )
}
