/*! 
 * Hello World server in Rust
 * Binds REP socket to tcp://*:5555
 * Expects "Hello" from client, replies with "World"
 */

extern mod zmq;

use std::libc;

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    
    assert!(responder.bind("tcp://*:5555").is_ok());

    loop {
        let mut buf = [0, ..10];
        unsafe { responder.recv_into(buf, 0) };
        println("Received Hello");
        responder.send(bytes!("World"), 0);
        unsafe { libc::sleep(1) };
    }
}
