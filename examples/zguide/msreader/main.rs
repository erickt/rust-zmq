#![crate_name = "msreader"]

//! Reading from multiple sockets
//! This version uses a simple recv loop

extern crate zmq;

use std::time::Duration;
use std::thread;

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
    // We prioritize traffic from the task ventilator
    let mut msg = zmq::Message::new();
    loop {
        loop {
            if receiver.recv(&mut msg, zmq::DONTWAIT).is_err() {
                break;
            }
            // Process task
        }

        loop {
            if subscriber.recv(&mut msg, zmq::DONTWAIT).is_err() {
                break;
            }
            // Process weather update
        }

        // No activity, so sleep for 1 msec
        thread::sleep(Duration::from_millis(1))
    }
}
