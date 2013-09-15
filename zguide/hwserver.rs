/*!
 * Hello World server in Rust
 * Binds REP socket to tcp://*:5555
 * Expects "Hello" from client, replies with "World"
 */

extern mod zmq;

use std::libc;

fn main() {
#[fixed_stack_segment];

    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0);
        do msg.with_str |s| {
            printfln!("Received %s", s);
        }
        responder.send_str("World", 0);
        unsafe { libc::sleep(1); }
    }
}
