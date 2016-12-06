#![crate_name = "helloworld_client"]

//! Hello World client

extern crate zmq;

fn main() {
    println!("Connecting to hello world server...\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new();

    for request_nbr in 0..10 {
        println!("Sending Hello {}...", request_nbr);
        requester.send("Hello", 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("Received World {}: {}", msg.as_str().unwrap(), request_nbr);
    }
}
