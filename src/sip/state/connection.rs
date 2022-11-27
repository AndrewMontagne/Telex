use simple_error::SimpleError;

use crate::sip::{request::Request, response::Response};


pub trait Connection {
    fn send_request(&mut self, request: Request) -> Result<(), SimpleError>;
    fn send_response(&mut self, response: Response) -> Result<(), SimpleError>;
    fn remote_name(&self) -> String;
}