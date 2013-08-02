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
        responder.send(bytes!("Hello"), 0);
        unsafe { libc::sleep(1) };
    }
}
