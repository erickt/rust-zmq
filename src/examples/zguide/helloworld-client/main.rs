//! Hello World client

extern crate zmq;

fn main() {
    println!("Connecting to hello world server...\n");

    let mut context = zmq::Context::new();
    let mut requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new();

    for x in range(0, 10) {
        println!("Sending Hello {}", x);
        requester.send(bytes!("Hello"), 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        msg.with_str(|s| {
            println!("Received World {}: {}", s, x);
        })
    }
}
