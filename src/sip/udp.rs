use std::{io::Cursor, net::{UdpSocket, SocketAddr}};

use simple_error::{SimpleError, bail};

use lazy_static::lazy_static;

use super::{handler::handle_request, request::Request, state::connection::Connection};

lazy_static! {
    static ref UDP_SOCKET: UdpSocket = UdpSocket::bind("145.239.7.56:5060").expect("");
}

pub fn listen() {
    let mut buf = [0; 100_000];

    let local_address = UDP_SOCKET.local_addr().unwrap();

    loop {
        let (amt, address) = UDP_SOCKET.recv_from(&mut buf).unwrap();
        let packet: Vec<u8> = buf[..amt].to_vec();
        let mut cursor = Cursor::new(packet);

        let connection = Box::new(ClientConnection::new(address, local_address));
        let request = Request::from_stream(&mut cursor, connection);
        if let Ok(request) = request {
            let _response = handle_request(request);
        }
    }
}

pub struct ClientConnection {
    remote: SocketAddr,
    local: SocketAddr,
}

impl ClientConnection {
    fn new(remote: SocketAddr, local: SocketAddr) -> ClientConnection {
        ClientConnection {
            remote,
            local
        }
    }
}

impl Connection for ClientConnection {
    fn local_address(&self) -> SocketAddr {
        self.local
    }
    fn remote_address(&self) -> SocketAddr {
        self.remote
    }
    fn send_to_socket(&mut self, data: &str) -> Result<(), SimpleError>  {
        let result = UDP_SOCKET.send_to(data.to_string().as_bytes(), self.remote);
        match result {
            Ok(_) => Ok(()),
            Err(_) => bail!("Error writing to socket"),
        }
    }
}