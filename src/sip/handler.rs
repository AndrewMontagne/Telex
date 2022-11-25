use log::{error, info, trace};
use simple_error::SimpleError;

use crate::sip::{header::SipHeader, response::SipResponse};

use super::request::{method::SipMethod, SipRequest};

pub fn handle_request(
    request: Result<SipRequest, SimpleError>,
) -> Result<SipResponse, SimpleError> {
    if let Ok(request) = request {
        info!("{} Request", request.method);
        if matches!(request.method, SipMethod::Register) {
            trace!("{}", request);

            let mut response = SipResponse::new("401 Unauthorized".to_string(), None)?;

            response.copy_header_from_request(SipHeader::CSeq, &request)?;
            response.copy_header_from_request(SipHeader::From, &request)?;
            response.copy_header_from_request(SipHeader::To, &request)?;
            response.copy_header_from_request(SipHeader::CallID, &request)?;
            response.copy_header_from_request(SipHeader::Via, &request)?;
            response.set_header(
                SipHeader::WWWAuthenticate,
                "Digest algorithm=MD5, realm=\"telex\", nonce=\"23fd5627\"".to_string(),
            )?;
            response.set_header(
                SipHeader::Allow,
                "REGISTER, INVITE, ACK, CANCEL, OPTIONS, BYE".to_string(),
            )?;

            trace!("{}", response);
            Ok(response)
        } else {
            Ok(SipResponse::new("501 Not Implemented".to_string(), None)?)
        }
    } else {
        error!("REQUEST ERROR: {}", request.err().unwrap());
        Ok(SipResponse::new("400 Bad Request".to_string(), None)?)
    }
}
