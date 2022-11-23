use std::{io::Cursor, net::UdpSocket};

use super::{handler::handle_request, request::SipRequest};

pub fn listen() {
    let socket: UdpSocket = UdpSocket::bind("[::]:5060").expect("");

    let mut buf = [0; 100000];

    loop {
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        let packet: Vec<u8> = buf[..amt].to_vec();
        let mut cursor = Cursor::new(packet);
        let request = SipRequest::from_stream(&mut cursor);

        let response = handle_request(request);

        _ = socket.send_to(response.to_string().as_bytes(), src);
    }
}
