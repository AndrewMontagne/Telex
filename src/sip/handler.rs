use log::{error, info, trace};
use simple_error::SimpleError;

use crate::sip::{header::Header, response::Response};

use super::request::{method::Method, Request};

pub fn handle_request(request: Result<Request, SimpleError>) -> Result<Response, SimpleError> {
    if let Ok(request) = request {
        info!("{} Request", request.method);
        if matches!(request.method, Method::Register) {
            trace!("{}", request);

            let mut response = Response::new("401 Unauthorized".to_string(), None)?;

            response.copy_header_from_request(Header::CSeq, &request)?;
            response.copy_header_from_request(Header::From, &request)?;
            response.copy_header_from_request(Header::To, &request)?;
            response.copy_header_from_request(Header::CallID, &request)?;
            response.copy_header_from_request(Header::Via, &request)?;
            response.set_header(
                Header::WWWAuthenticate,
                "Digest algorithm=MD5, realm=\"telex\", nonce=\"23fd5627\"".to_string(),
            )?;
            response.set_header(
                Header::Allow,
                "REGISTER, INVITE, ACK, CANCEL, OPTIONS, BYE".to_string(),
            )?;

            trace!("{}", response);
            Ok(response)
        } else {
            Ok(Response::new("501 Not Implemented".to_string(), None)?)
        }
    } else {
        error!("REQUEST ERROR: {}", request.err().unwrap());
        Ok(Response::new("400 Bad Request".to_string(), None)?)
    }
}
