use log::{error, info, trace};
use simple_error::SimpleError;

use crate::{
    sip::{header::Header, response::{Response, status::Status}},
    strlit,
};

use super::request::{method::Method, Request};

pub fn handle_request(request: Result<Request, SimpleError>) -> Result<Response, SimpleError> {
    if let Ok(request) = request {
        info!("{} Request", request.method);
        if matches!(request.method, Method::Register) {
            trace!("{}", request);

            if request.headers.contains_key(&Header::Authorization) {
                let mut response = Response::new(&request, Status::OK, None)?;
                response.set_header(Header::Contact, strlit!("<sip:user@172.19.195.144>"))?;
                response.set_header(Header::Expires, strlit!("300"))?;
                trace!("{}", response);
                Ok(response)
            } else {
                let mut response = Response::new(&request, Status::Unauthorized, None)?;
                response.set_header(
                    Header::WWWAuthenticate,
                    strlit!("Digest algorithm=MD5, realm=\"telex\", nonce=\"23fd5627\""),
                )?;
                trace!("{}", response);
                Ok(response)
            }
            
        } else if matches!(request.method, Method::Invite) { 
            Ok(Response::new(&request, Status::BusyHere, None)?)
        } else if let Method::Cancel | Method:: Options = request.method { 
            Ok(Response::new(&request, Status::OK, None)?)
        } else if matches!(request.method, Method::Ack) { 
            Ok(Response::empty())
        } else {
            Ok(Response::new(&request, Status::NotImplemented, None)?)
        }
    } else {
        let error = request.err().unwrap();
        error!("REQUEST ERROR: {}", &error);
        Err(error)
    }
}
