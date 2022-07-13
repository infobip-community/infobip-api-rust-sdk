use crate::model::sms::{SendSmsRequestBody, SendSmsResponse};

pub struct Sms {}

impl Sms {
    pub fn send(self, _body: SendSmsRequestBody) -> SendSmsResponse {
        return SendSmsResponse {};
    }
}
