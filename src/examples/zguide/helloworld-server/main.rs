/// Hello World server in Rust
/// Binds REP socket to tcp://*:5555
/// Expects "Hello" from client, replies with "World"

extern mod zmq;

use std::io;

fn main() {
    let mut context = zmq::Context::new();
    let mut responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0);
        msg.with_str(|s| {
            println!("Received {}", s);
        });
        responder.send_str("World", 0);
        io::timer::sleep(1000);
    }
}
