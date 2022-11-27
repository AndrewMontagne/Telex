use std::time;
use std::thread;

use log::{info, trace};
use simple_error::SimpleError;

use crate::{
    sip::{header::Header, response::{Response, status::Status}},
    strlit,
};

use super::request::{method::Method, Request};

pub fn handle_request(mut request: Request) -> Result<(), SimpleError> {
    info!("{} Request", request.method);
    if matches!(request.method, Method::Register) {
        trace!("{}", request);

        if request.headers.contains_key(&Header::Authorization) {
            let mut response = Response::new(&request, Status::OK, None)?;
            response.set_header(Header::Contact, strlit!("<sip:user@172.19.195.144>"))?;
            response.set_header(Header::Expires, strlit!("300"))?;
            trace!("{}", response);
            request.connection.send_response(response)?;
        } else {
            let mut response = Response::new(&request, Status::Unauthorized, None)?;
            response.set_header(
                Header::WWWAuthenticate,
                strlit!("Digest algorithm=MD5, realm=\"telex\", nonce=\"23fd5627\""),
            )?;
            trace!("{}", response);
            request.connection.send_response(response)?;
        }
    } else if matches!(request.method, Method::Invite) {
        let mut trying = Response::new(&request, Status::Trying, None)?;
        trying.set_header(Header::Contact, request.headers.get(&Header::To).unwrap().to_string())?;
        request.connection.send_response(trying)?;
        thread::sleep(time::Duration::from_millis(1000));
        let mut ringing = Response::new(&request, Status::Ringing, None)?;
        ringing.set_header(Header::Contact, request.headers.get(&Header::To).unwrap().to_string())?;
        request.connection.send_response(ringing)?;
    } else if ! matches!(request.method, Method::Ack) {
        let response = match request.method {
            Method::Cancel | Method:: Options => Response::new(&request, Status::OK, None),
            _ => Response::new(&request, Status::NotImplemented, None),
        }?;
        request.connection.send_response(response)?;
    }
    Ok(())
}
