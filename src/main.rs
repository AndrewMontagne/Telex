#![warn(clippy::pedantic)]
#![warn(clippy::style)]

use std::thread;

use log::{info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

mod sip;

fn main() {
    _ = TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    info!("Starting Telex...");

    thread::spawn(move || {
        sip::udp::listen();
    });
    sip::tcp::listen();
}
