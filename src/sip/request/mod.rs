use std::{collections::HashMap, fmt};

use self::method::SipRequestMethod;

use super::header::SipHeader;

pub mod from_stream;
pub mod method;

pub struct SipRequest {
    pub method: SipRequestMethod,
    pub headers: HashMap<SipHeader, String>,
    pub body: Option<String>,
    pub address: String,
}

impl fmt::Display for SipRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        _ = write!(f, "{} {} SIP/2.0\r\n", self.method, self.address);
        for (key, value) in &self.headers {
            _ = write!(f, "{}: {}\r\n", key, value);
        }
        if let Some(body) = &self.body {
            _ = write!(f, "\r\n{}\r\n", body);
        }
        Ok(())
    }
}
