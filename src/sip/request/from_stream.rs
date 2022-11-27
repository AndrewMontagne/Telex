use lazy_static::lazy_static;
use regex::Regex;
use simple_error::{bail, SimpleError};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use crate::sip::{header::Header, state::connection::{Connection}};

use super::{Method, Request};

lazy_static! {
    static ref REQUEST_LINE_REGEX: Regex = Regex::new(r"^([A-Z]+) (.+) SIP.2.0$").unwrap();
    static ref CONTENT_LENGTH_REGEX: Regex = Regex::new(r"^([\w\-]+):\s+(.+)$").unwrap();
}

impl Request {
    pub fn from_stream(data: &mut impl Read, connection: Box<dyn Connection>) -> Result<Request, SimpleError> {
        let mut buf_reader = BufReader::new(data);
        let mut headers: HashMap<Header, String> = HashMap::new();
        let mut body = None;

        // Ignore blank lines leading up to the request
        let mut request_line_str = String::new();
        while request_line_str.is_empty() {
            _ = buf_reader
                .read_line(&mut request_line_str)
                .unwrap_or_default();
            request_line_str = request_line_str.trim().to_string();
        }

        // Extract the method
        let request_line_capture = match REQUEST_LINE_REGEX.captures(&request_line_str) {
            Some(capture) => capture,
            None => bail!("Invalid Request Line: '{}'", request_line_str),
        };
        let method_str = match request_line_capture.get(1) {
            Some(matc) => matc.as_str().to_string(),
            None => bail!("SANITY: Couldn't get the method from the request line"),
        };
        let method = match Method::from_string(&method_str) {
            Ok(method) => method,
            Err(e) => return Err(e),
        };

        // Extract the size
        let address = match request_line_capture.get(2) {
            Some(matc) => matc.as_str().to_string(),
            None => bail!("SANITY: Couldn't get the address from the request line"),
        };

        // Parse the headers
        let mut content_length: usize = 0;
        loop {
            // Read the line from the stream
            let mut line = String::new();
            _ = buf_reader.read_line(&mut line).unwrap_or_default();
            line = line.trim_end().to_string();

            // Check to see if we've finished reading headers
            if line.is_empty() {
                break;
            }

            // Parse the header
            let re_captures = match CONTENT_LENGTH_REGEX.captures(&line) {
                Some(re_captures) => re_captures,
                None => bail!("Bad Header Line: {}", line),
            };
            let header = match re_captures.get(1) {
                Some(matc) => match Header::from_string(matc.as_str()) {
                    Ok(header) => header,
                    Err(_) => continue,
                },
                None => bail!("SANITY: Couldn't get the header from the header line"),
            };

            // Grab the value and validate it
            let value = match re_captures.get(2) {
                Some(matc) => {
                    let matc_str = matc.as_str().to_string();
                    match header.validate(&matc_str) {
                        Ok(_) => matc_str,
                        Err(e) => bail!("Bad Header Value: '{}'", e),
                    }
                }
                None => bail!("SANITY: Couldn't get the value from the header line"),
            };

            // If it's the content length header,
            if header == Header::ContentLength {
                content_length = value.parse().unwrap_or_default();
            }
            headers.insert(header, value.to_string());
        }

        // If we have a content length, read the body
        if content_length > 0 {
            let mut body_buf = vec![0u8];
            let mut remaining_bytes = content_length;

            // Keep reading until we have no bytes left
            while remaining_bytes > 0 {
                let mut mini_buf = vec![0u8; remaining_bytes as usize];
                let read = buf_reader.read(&mut mini_buf);
                if let Ok(read) = read {
                    body_buf.append(&mut mini_buf);
                    remaining_bytes -= read;
                } else {
                    break;
                }
            }

            // Decode into a UTF-8 string
            body = match String::from_utf8(body_buf) {
                Ok(body) => Some(body),
                Err(e) => bail!("Could not decode body: {}", e),
            }
        }

        // All done!
        Ok(Request {
            method,
            headers,
            body,
            address,
            connection,
        })
    }
}
