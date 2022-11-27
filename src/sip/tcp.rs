use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};

use log::info;
use simple_error::{bail, SimpleError};

use super::{handler::handle_request, request::Request, state::connection::{Connection}, response::Response};

pub fn listen() {
    let listener = TcpListener::bind("[::]:5060").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        thread::spawn(move || {
            handle_connection(&mut stream);
        });
    }
}

fn handle_connection(stream: &mut TcpStream) {
    loop {
        let stream_copy = stream.try_clone();
        if let Ok(stream_copy) = stream_copy {
            let connection = Box::new(ClientConnection::new(stream_copy));
            let request = Request::from_stream(stream, connection);
            if let Ok(request) = request {
                let _result = handle_request(request);
            }
        }
    }
}

pub struct ClientConnection {
    stream: TcpStream,
    remote_name: String,
}

impl ClientConnection {
    pub fn new(stream: TcpStream) -> ClientConnection {
        let remote_name = stream.peer_addr().unwrap().to_string();
        ClientConnection {
            stream,
            remote_name
        }
    }

    fn send_to_socket(&mut self, data: &str) -> Result<(), SimpleError>  {
        let result = self.stream.write_all(data.as_bytes());
        match result {
            Ok(_) => Ok(()),
            Err(_) => bail!("Error writing to socket"),
        }
    }
}

impl Connection for ClientConnection {
    fn send_request(&mut self, request: Request) -> Result<(), SimpleError> {
        info!("=> {} Request", request.method);
        self.send_to_socket(request.to_string().as_str())
    }
    fn send_response(&mut self, response: Response) -> Result<(), SimpleError> {
        info!("=> {} Response", response.status);
        self.send_to_socket(response.to_string().as_str())
    }
    fn remote_name(&self) -> String {
        self.remote_name.clone()
    }
}