extern mod zmq;

use std::iterator::Counter;

fn main() {
    println("Conneting to hello world server...\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    foreach x in Counter::new(0, 1).take_(10) {
        let mut buf = [0, ..10];
        printfln!("Sending Hello %d", x);
        requester.send(bytes!("Hello"), 0);
        unsafe { requester.recv_into(buf, 0) };
        printfln!("Received World %d", x);
    }
}
        
