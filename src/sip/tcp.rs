use std::{
    io::Write,
    net::{TcpListener, TcpStream, SocketAddr},
    thread,
};

use log::info;
use simple_error::{bail, SimpleError};

use super::{handler::handle_request, request::Request, state::connection::Connection, response::Response};

pub fn listen() {
    let listener = TcpListener::bind("172.19.195.144:5060").unwrap();

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
    remote: SocketAddr,
    local: SocketAddr,
}

impl ClientConnection {
    pub fn new(stream: TcpStream) -> ClientConnection {
        let remote = stream.peer_addr().unwrap();
        let local = stream.local_addr().unwrap();
        ClientConnection {
            stream,
            remote,
            local,
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
    fn local_address(&self) -> SocketAddr {
        self.local
    }
    fn remote_address(&self) -> SocketAddr {
        self.remote
    }
}