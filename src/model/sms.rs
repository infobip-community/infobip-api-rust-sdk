use std::fmt;

pub struct SendSmsRequestBody {}

impl SendSmsRequestBody {
    pub fn validated(body: SendSmsRequestBody) -> Result<SendSmsRequestBody, String> {
        Ok(body)
        // validate
    }
}

pub struct SendSmsResponse {}

impl fmt::Display for SendSmsResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Displayed response!")
    }
}
