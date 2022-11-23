use simple_error::SimpleError;

use crate::sip::{header::SipHeader, response::SipResponse};

use super::request::{SipRequest, method::SipRequestMethod};


pub fn handle_request(request: Result<SipRequest, SimpleError>) -> SipResponse {
    if let Ok(request) = request {
        if matches!(request.method, SipRequestMethod::Register) {
            println!("{}", request);

            let mut response = SipResponse::new("401 Unauthorized".to_string(), None);

            response.copy_header_from_request(SipHeader::CSeq, &request);
            response.copy_header_from_request(SipHeader::From, &request);
            response.copy_header_from_request(SipHeader::To, &request);
            response.copy_header_from_request(SipHeader::CallID, &request);
            response.copy_header_from_request(SipHeader::Via, &request);
            _ = response.set_header(SipHeader::WWWAuthenticate, "Digest algorithm=MD5, realm=\"telex\", nonce=\"23fd5627\"".to_string());
            _ = response.set_header(SipHeader::Allow, "INVITE, ACK, CANCEL, OPTIONS, BYE, REFER, SUBSCRIBE, NOTIFY, INFO, PUBLISH, MESSAGE".to_string());

            println!("{}", response);
            response
        } else {
            println!("{} Request", request.method);
            SipResponse::new("501 Not Implemented".to_string(), None)
        }
    } else {
        eprintln!("REQUEST ERROR: {}", request.err().unwrap());
        SipResponse::new("400 Bad Request".to_string(), None)
    }
}