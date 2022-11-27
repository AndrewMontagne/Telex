use std::{collections::HashMap, fmt};

use self::method::Method;

use super::{header::Header, state::connection::Connection};

pub mod from_stream;
pub mod method;


pub struct Request {
    pub method: Method,
    pub headers: HashMap<Header, String>,
    pub body: Option<String>,
    pub address: String,
    pub connection: Box<dyn Connection>,
}

impl fmt::Display for Request {
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
