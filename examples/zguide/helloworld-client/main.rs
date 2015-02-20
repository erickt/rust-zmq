//! Hello World client

#![crate_name = "helloworld-client"]

extern crate zmq;

fn main() {
    println!("Connecting to hello world server...\n");

    let mut context = zmq::Context::new();
    let mut requester = context.socket(zmq::REQ).ok().unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new().ok().unwrap();

    for x in 0us..10 {
        println!("Sending Hello {}", x);
        requester.send(b"Hello", 0).ok().unwrap();

        requester.recv(&mut msg, 0).ok().unwrap();
        println!("Received World {}: {}", msg.as_str().unwrap(), x);
    }
}
