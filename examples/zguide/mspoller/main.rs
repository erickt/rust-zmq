//  Reading from multiple sockets
//  This version uses zmq::poll()

extern crate zmq;

fn main() {
    let context = zmq::Context::new();

    // Connect to task ventilator
    let receiver = context.socket(zmq::PULL).unwrap();
    assert!(receiver.connect("tcp://localhost:5557").is_ok());

    // Connect to weather server
    let subscriber = context.socket(zmq::SUB).unwrap();
    assert!(subscriber.connect("tcp://localhost:5556").is_ok());
    let filter = b"10001";
    assert!(subscriber.set_subscribe(filter).is_ok());


    // Process messages from both sockets
    let mut msg = zmq::Message::new();
    loop {
        let mut items = [
            receiver.as_poll_item(zmq::POLLIN),
            subscriber.as_poll_item(zmq::POLLIN),
        ];
        zmq::poll(&mut items, -1).unwrap();
        if items[0].is_readable() {
            if receiver.recv(&mut msg, 0).is_ok() {
                //  Process task
            }
        }
        if items[1].is_readable() {
            if subscriber.recv(&mut msg, 0).is_ok() {
                // Process weather update
            }
        }
    }
}
