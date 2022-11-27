use std::{collections::HashMap, fmt};

use simple_error::SimpleError;

use crate::strlit;

use self::status::Status;

use super::{header::Header, request::Request};

pub mod status;

#[derive(Debug)]
pub struct Response {
    pub status: Status,
    pub headers: HashMap<Header, String>,
    pub body: Option<String>,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        _ = write!(f, "SIP/2.0 {}\r\n", self.status);
        for (key, value) in &self.headers {
            _ = write!(f, "{}: {}\r\n", key.canonical_string(false), value);
        }
        if let Some(body) = &self.body {
            _ = write!(f, "\r\n{}\r\n", body);
        }
        _ = write!(f, "\r\n");
        Ok(())
    }
}

impl Response {
    pub fn new(request: &Request, status: Status, body: Option<String>) -> Result<Response, SimpleError> {
        let mut response = Response {
            status,
            headers: HashMap::new(),
            body: None,
        };

        response.copy_header_from_request(Header::CSeq, request)?;
        response.copy_header_from_request(Header::From, request)?;
        response.copy_header_from_request(Header::To, request)?;
        response.copy_header_from_request(Header::CallID, request)?;

        if request.headers.contains_key(&Header::Via) {
            let via_in = request.headers.get(&Header::Via).unwrap().clone();
            response.set_header(Header::Via, request.connection.via_header_response(via_in)?)?;
        }

        response.generate_header(Header::Date)?;
        response.generate_header(Header::Server)?;
        response.generate_header(Header::Allow)?;

        if let Some(body) = body {
            response.set_body(body)?;
        } else {
            response.set_header(Header::ContentLength, strlit!("0"))?;
        }

        Ok(response)
    }

    pub fn generate_header(&mut self, header: Header) -> Result<(), SimpleError> {
        let value = header.generate();
        self.set_header(header, value)
    }

    pub fn set_header(&mut self, header: Header, value: String) -> Result<(), SimpleError> {
        header.validate(&value)?;
        self.headers.insert(header, value);
        Ok(())
    }

    pub fn set_body(&mut self, body: String) -> Result<(), SimpleError> {
        let len = body.as_bytes().len().to_string();
        self.set_header(Header::ContentLength, len)?;
        self.body = Some(body);
        Ok(())
    }

    pub fn copy_header_from_request(
        &mut self,
        header: Header,
        request: &Request,
    ) -> Result<(), SimpleError> {
        if request.headers.contains_key(&header) {
            let value = request.headers.get(&header).unwrap().clone();
            self.set_header(header, value)
        } else {
            Ok(())
        }
    }
}
