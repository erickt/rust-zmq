//  Reading from multiple sockets
//  This version uses zmq2::poll()

fn main() {
    let context = zmq2::Context::new();

    // Connect to task ventilator
    let receiver = context.socket(zmq2::PULL).unwrap();
    assert!(receiver.connect("tcp://localhost:5557").is_ok());

    // Connect to weather server
    let subscriber = context.socket(zmq2::SUB).unwrap();
    assert!(subscriber.connect("tcp://localhost:5556").is_ok());
    let filter = b"10001";
    assert!(subscriber.set_subscribe(filter).is_ok());

    // Process messages from both sockets
    let mut msg = zmq2::Message::new();
    loop {
        let mut items = [
            receiver.as_poll_item(zmq2::POLLIN),
            subscriber.as_poll_item(zmq2::POLLIN),
        ];
        zmq2::poll(&mut items, -1).unwrap();
        if items[0].is_readable() && receiver.recv(&mut msg, 0).is_ok() {
            //  Process task
        }
        if items[1].is_readable() && subscriber.recv(&mut msg, 0).is_ok() {
            // Process weather update
        }
    }
}
