use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};

use log::error;

use super::{handler::handle_request, request::SipRequest};

pub fn listen() {
    let listener = TcpListener::bind("[::]:5060").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let request = SipRequest::from_stream(&mut &stream);
        let response = handle_request(request);
        if let Ok(response) = response {
            let result = stream.write_all(response.to_string().as_bytes());
            if result.is_err() {
                error!("TCP send error");
            }
        }
    }
}
