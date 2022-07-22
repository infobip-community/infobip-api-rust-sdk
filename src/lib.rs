#[macro_use]
extern crate lazy_static;

mod api;
mod configuration;
mod model;

#[cfg(test)]
mod tests {
    use validator::Validate;

    // Module organization is neat and simple.
    use crate::api::sms::SmsClient;
    use crate::configuration::Configuration;
    use crate::model::sms::PreviewSmsRequestBody;

    /*#[test]
    fn send_basic_sms() {
        // A good DX:

        // Config can be loaded from environment without boilerplate code.
        let config = Configuration::from_env_api_key().expect("error reading API key or base URL");

        // Create a client.
        // Client features may be conditionally compiled, so library compilation is fast.
        let sms_client = SmsClient::with_configuration(config);

        let req_body = SendSmsRequestBody {};

        // Gives the option to validate before calling send method.
        if req_body.validate().is_ok() {
            // Automatically validates body, and parameters. Has short, yet descriptive usage and naming.
            let response = sms_client.send(req_body);

            // Easy to print human-readable responses. Implements a nice Debug trait.
            println!("Response: {}", response)
        } else {
            eprintln!("Something wasn't valid in the model.")
        }
    }*/

    #[tokio::test]
    async fn preview_sms() {
        let config = Configuration::from_env_api_key().expect("error reading API key or base URL");
        let sms_client = SmsClient::with_configuration(config);

        let request_body = PreviewSmsRequestBody {
            language_code: Some("AUTODETECT".to_string()),
            text: "This is some text.".to_string(),
            transliteration: Some("GREEK".to_string()),
        };

        if request_body.validate().is_ok() {
            let response = sms_client.preview(request_body).await.unwrap();
            println!("{}", serde_json::to_string(&response).unwrap());
        } else {
            eprintln!("Request body is not correct.");
        }
    }
}
