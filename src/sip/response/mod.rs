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
    pub fn new(status: String, body: Option<String>) -> SipResponse {
        let mut response = SipResponse {
            status,
            headers: HashMap::new(),
            body: None,
        };
        _ = response.set_header(SipHeader::Date, SipHeader::generate(SipHeader::Date));
        _ = response.set_header(SipHeader::Server, SipHeader::generate(SipHeader::Server));
        if let Some(body) = body {
            _ = response.set_body(body);
        } else {
            _ = response.set_header(SipHeader::ContentLength, "0".to_string());
        }

        response
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

    pub fn copy_header_from_request(&mut self, header: SipHeader, request: &SipRequest) {
        if request.headers.contains_key(&header) {
            _ = self.set_header(
                header.clone(),
                request.headers.get(&header).unwrap().clone(),
            );
        }
    }
}

/*
SIP/2.0 489 Bad Event
Via: SIP/2.0/UDP 10.42.0.3:58372;branch=z9hG4bKPj181065fe55e04afcb27eb870af4da564;received=10.42.0.3;rport=58372
From: "andre" <sip:3001@bifrost.lan>;tag=a799d0636d494da6afc14250860cb8fe
To: "andre" <sip:3001@bifrost.lan>;tag=as70866c73
Call-ID: 167f03e3776a48a28c33ab2a2f7d8513
CSeq: 1 PUBLISH
Server: Asterisk PBX 17.9.4
Allow: INVITE, ACK, CANCEL, OPTIONS, BYE, REFER, SUBSCRIBE, NOTIFY, INFO, PUBLISH, MESSAGE
Supported: replaces, timer
Content-Length: 0


*/
