use std::{io::Cursor, net::{UdpSocket, SocketAddr}};

use simple_error::{SimpleError, bail};

use lazy_static::lazy_static;

use super::{handler::handle_request, request::Request, state::connection::Connection, response::Response};

lazy_static! {
    static ref UDP_SOCKET: UdpSocket = UdpSocket::bind("[::]:5060").expect("");
}

pub fn listen() {
    let mut buf = [0; 100_000];

    loop {
        let (amt, address) = UDP_SOCKET.recv_from(&mut buf).unwrap();
        let packet: Vec<u8> = buf[..amt].to_vec();
        let mut cursor = Cursor::new(packet);

        let connection = Box::new(ClientConnection::new(address));
        let request = Request::from_stream(&mut cursor, connection);
        if let Ok(request) = request {
            let _response = handle_request(request);
        }
    }
}

pub struct ClientConnection {
    address: SocketAddr,
    remote_name: String,
}

impl ClientConnection {
    fn new(address: SocketAddr) -> ClientConnection {
        let remote_name = address.to_string();
        ClientConnection {
            address,
            remote_name
        }
    }

    fn send_to_socket(&self, data: &str) -> Result<(), SimpleError>  {
        let result = UDP_SOCKET.send_to(data.to_string().as_bytes(), self.address);
        match result {
            Ok(_) => Ok(()),
            Err(_) => bail!("Error writing to socket"),
        }
    }
}

impl Connection for ClientConnection {
    fn send_request(&mut self, request: Request) -> Result<(), SimpleError> {
        self.send_to_socket(request.to_string().as_str())
    }
    fn send_response(&mut self, response: Response) -> Result<(), SimpleError> {
        self.send_to_socket(response.to_string().as_str())
    }
    fn remote_name(&self) -> String {
        self.remote_name.clone()
    }
}