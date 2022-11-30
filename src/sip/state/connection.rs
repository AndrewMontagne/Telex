use std::net::SocketAddr;

use log::info;
use regex::Regex;
use simple_error::{SimpleError, bail};

use crate::{sip::{request::Request, response::Response}};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref VIA_VALUE_REGEX: Regex = Regex::new(r"^^([^;]+);(.*)$").unwrap();
}

pub trait Connection {
    fn send_request(&mut self, request: Request) -> Result<(), SimpleError> {
        info!("=> {} Request", request.method);
        self.send_to_socket(request.to_string().as_str())
    }
    fn send_response(&mut self, response: Response) -> Result<(), SimpleError> {
        info!("=> {} Response", response.status);
        self.send_to_socket(response.to_string().as_str())
    }
    fn send_to_socket(&mut self, data: &str) -> Result<(), SimpleError>;
    fn local_address(&self) -> SocketAddr;
    fn remote_address(&self) -> SocketAddr;
    fn via_header_response(&self, via_request: String) -> Result<String, SimpleError> {
        let result = VIA_VALUE_REGEX.captures(&via_request);

        if let Some(result) = result {
            let preamble = result.get(1).unwrap().as_str();
            let payload = result.get(2).unwrap().as_str();

            let mut output = String::new();

            let splits = payload.split(';');
            for split in splits {
                let keyvalue: Vec<&str> = split.split('=').collect();
                let key = *keyvalue.first().unwrap();
                let value: String = match key {
                    "received" => {
                        format!("received={}", self.local_address().ip())
                    },
                    "rport" => {
                        format!("rport={}", self.remote_address().port())
                    },
                    _ => {
                        split.to_string()
                    }
                };
                output.push(';');
                output.push_str(&value);
            }
            Ok(format!("{}{}", preamble, output))
        } else {
            bail!("Couldn't parse Via header")
        }
    }
}
