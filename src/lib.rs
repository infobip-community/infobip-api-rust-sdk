mod api;
mod client;
mod model;

#[cfg(test)]
mod tests {
    // Module organization is neat and simple.
    use crate::client::configuration::Configuration;
    use crate::client::InfobipClient;
    use crate::model::sms::SendSmsRequestBody;

    #[test]
    fn send_basic_sms() {
        // A good DX:

        // Config can be loaded from environment without boilerplate code.
        let config = Configuration::from_env();

        // Create a client.
        // Client features may be conditionally compiled, so library compilation is fast.
        let client = InfobipClient::new(config);

        // Gives the option to validate before calling send method.
        if let Ok(send_msg_body) = SendSmsRequestBody::validated(SendSmsRequestBody {}) {
            // Automatically validates body, and parameters. Has short, yet descriptive usage and naming.
            let response = client.sms.send(send_msg_body);

            // Easy to print human-readable responses. Implements a nice Debug trait.
            println!("Response: {}", response)
        } else {
            eprintln!("Something wasn't valid in the model.")
        }
    }
}
