#![crate_name = "helloworld-server"]
#![feature(old_io, std_misc)]

/// Hello World server in Rust
/// Binds REP socket to tcp://*:5555
/// Expects "Hello" from client, replies with "World"

extern crate zmq;

use std::old_io;
use std::time::Duration;

fn main() {
    let mut context = zmq::Context::new();
    let mut responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new().unwrap();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        responder.send_str("World", 0).unwrap();
        old_io::timer::sleep(Duration::seconds(1));
    }
}
