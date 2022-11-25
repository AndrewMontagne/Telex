use std::{collections::HashMap, fmt};

use simple_error::SimpleError;

use super::{header::SipHeader, request::SipRequest};

pub struct SipResponse {
    pub status: String,
    pub headers: HashMap<SipHeader, String>,
    pub body: Option<String>,
}

impl fmt::Display for SipResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        _ = write!(f, "SIP/2.0 {}\r\n", self.status);
        for (key, value) in &self.headers {
            _ = write!(f, "{}: {}\r\n", key, value);
        }
        if let Some(body) = &self.body {
            _ = write!(f, "\r\n{}\r\n", body);
        }
        _ = write!(f, "\r\n");
        Ok(())
    }
}

impl SipResponse {
    pub fn new(status: String, body: Option<String>) -> Result<SipResponse, SimpleError> {
        let mut response = SipResponse {
            status,
            headers: HashMap::new(),
            body: None,
        };
        response.set_header(SipHeader::Date, SipHeader::generate(SipHeader::Date))?;
        response.set_header(SipHeader::Server, SipHeader::generate(SipHeader::Server))?;
        if let Some(body) = body {
            response.set_body(body)?;
        } else {
            response.set_header(SipHeader::ContentLength, "0".to_string())?;
        }

        Ok(response)
    }

    pub fn set_header(&mut self, header: SipHeader, value: String) -> Result<(), SimpleError> {
        header.validate(&value)?;
        self.headers.insert(header, value);
        Ok(())
    }

    pub fn set_body(&mut self, body: String) -> Result<(), SimpleError> {
        let len = body.as_bytes().len().to_string();
        self.set_header(SipHeader::ContentLength, len)?;
        self.body = Some(body);
        Ok(())
    }

    pub fn copy_header_from_request(
        &mut self,
        header: SipHeader,
        request: &SipRequest,
    ) -> Result<(), SimpleError> {
        if request.headers.contains_key(&header) {
            let value = request.headers.get(&header).unwrap().clone();
            self.set_header(header, value)
        } else {
            Ok(())
        }
    }
}
