use std::thread;

mod sip;

fn main() {
    thread::spawn(move || {
        sip::udp::listen();
    });
    sip::tcp::listen();
}
