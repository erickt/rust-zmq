//! Hello World client

extern mod zmq;

fn main() {
    println("Conneting to hello world server...\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new();

    for x in range(0, 10) {
        printfln!("Sending Hello %d", x);
        requester.send(bytes!("Hello"), 0);

        requester.recv(&mut msg, 0).unwrap();
        do msg.with_str |s| {
            printfln!("Received World %s: %d", s, x);
        }
    }
}
